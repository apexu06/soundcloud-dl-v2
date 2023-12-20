use std::{path::PathBuf, sync::OnceLock, time::Duration};

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};
use directories::UserDirs;
use id3::{frame, TagLike};
use indicatif::ProgressBar;
use regex::Regex;
use soundcloud::{download_track, DownloadError};
use types::{FieldLabel, Metadata, MetadataField};

mod soundcloud;
mod types;

pub static FILENAME: OnceLock<String> = OnceLock::new();

/// cli tool to download soundcloud tracks
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// soundcloud url to download (skips metadata options)
    #[arg(short, long)]
    url: Option<String>,

    #[arg(short, long)]
    songname: Option<String>,

    #[arg(short, long)]
    artist: Option<String>,

    #[arg(short = 'A', long)]
    album: Option<String>,

    #[arg(short, long)]
    genre: Option<String>,

    /// specify download directory
    #[arg(short, long)]
    download_directory: Option<PathBuf>,

    /// use default metadata
    #[arg(short = 'U', long)]
    use_default_metadata: bool,
}

fn get_default_dir() -> PathBuf {
    let working_dir = PathBuf::new();
    if let Some(dir) = UserDirs::new() {
        dir.audio_dir().unwrap_or(&working_dir).to_path_buf()
    } else {
        working_dir
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if let Some(dir) = args.download_directory {
        std::env::set_current_dir(dir)?;
    } else {
        std::env::set_current_dir(get_default_dir())?;
    }

    let url = if let Some(url) = args.url {
        url
    } else {
        prompt_url()?
    };

    let default_metadata = download(url).await?;
    FILENAME.get_or_init(|| format!("{}.mp3", &default_metadata.title));
    let tag = create_default_tag(default_metadata);

    let mut parameter_fields: Vec<MetadataField> = Vec::new();

    if let Some(title) = args.songname {
        parameter_fields.push(MetadataField {
            label: FieldLabel::Title,
            value: title,
        });
    }

    if let Some(artist) = args.artist {
        parameter_fields.push(MetadataField {
            label: FieldLabel::Artist,
            value: artist,
        });
    }

    if let Some(genre) = args.genre {
        parameter_fields.push(MetadataField {
            label: FieldLabel::Genre,
            value: genre,
        });
    }

    if let Some(album) = args.album {
        parameter_fields.push(MetadataField {
            label: FieldLabel::Album,
            value: album,
        });
    }

    apply_metadata(parameter_fields, tag)?;

    println!("finished!");

    Ok(())
}

fn create_default_tag(metadata: Metadata) -> id3::Tag {
    let mut tag = id3::Tag::new();

    tag.set_title(metadata.title.value);
    tag.set_artist(metadata.artist.value);
    tag.set_album(metadata.album_name.value);
    tag.set_genre(metadata.genre.value);
    tag.add_frame(frame::Picture {
        mime_type: "image/jpeg".to_string(),
        picture_type: frame::PictureType::CoverFront,
        description: "Cover".to_string(),
        data: metadata.album_art,
    });
    tag
}

fn apply_metadata(
    metadata_fields: Vec<MetadataField>,
    mut tag: id3::Tag,
) -> Result<(), Box<dyn std::error::Error>> {
    let selected = prompt_metadata(&metadata_fields)?;

    for index in selected {
        let value = prompt_field(&metadata_fields[index])?;

        match &metadata_fields[index].label {
            FieldLabel::Title => tag.set_title(&value),
            FieldLabel::Artist => tag.set_artist(&value),
            FieldLabel::Album => tag.set_album(&value),
            FieldLabel::Genre => tag.set_genre(&value),
        }
    }

    let mut path = std::env::current_dir()?;
    path.push(format!(
        "{}.mp3",
        FILENAME.get().unwrap_or(&"soundcloud".to_string())
    ));
    tag.write_to_path(path, id3::Version::Id3v24)?;
    Ok(())
}

fn prompt_url() -> Result<String, dialoguer::Error> {
    let re = Regex::new("https://soundcloud.com/.*/.*").expect("invalid regex");

    let url = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("enter SoundCloud URL")
        .validate_with(|input: &String| -> Result<(), &str> {
            if re.is_match(input) {
                Ok(())
            } else {
                Err("Invalid URL")
            }
        })
        .interact_text()?;

    Ok(url)
}

fn prompt_metadata(items: &Vec<MetadataField>) -> Result<Vec<usize>, dialoguer::Error> {
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("choose which fields you want to change\n  (space: select / enter: continue):")
        .items(items)
        .interact()?;

    Ok(selection)
}

fn prompt_field(field: &MetadataField) -> Result<String, dialoguer::Error> {
    let updated: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("new {}", field.label))
        .show_default(false)
        .default(field.value.clone())
        .interact_text()?;

    Ok(updated)
}

async fn download(url: String) -> Result<types::Metadata, DownloadError> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("downloading...");
    spinner.enable_steady_tick(Duration::from_millis(50));

    Ok(download_track(url).await?)
}

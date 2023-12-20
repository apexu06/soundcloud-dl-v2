use std::{path::PathBuf, sync::OnceLock, time::Duration};

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};
use directories::UserDirs;
use id3::TagLike;
use indicatif::ProgressBar;
use regex::Regex;
use soundcloud::{download_track, DownloadError};
use types::{FieldLabel, MetadataField};

mod soundcloud;
mod types;

static FILENAME: OnceLock<String> = OnceLock::new();

/// cli tool to download soundcloud tracks
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// soundcloud url to download (skips metadata options)
    #[arg(short, long)]
    url: Option<String>,

    /// song name
    #[arg(short, long)]
    songname: Option<String>,

    /// artist name
    #[arg(short, long)]
    artist: Option<String>,

    /// album name
    #[arg(short = 'A', long)]
    album: Option<String>,

    /// genre name
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
    let working_dir = std::env::current_dir().unwrap_or_default();
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
    FILENAME.get_or_init(|| format!("{}.mp3", &default_metadata.title.value));

    let mut default_fields: Vec<MetadataField> = Vec::new();
    let mut param_fields: Vec<MetadataField> = Vec::new();

    if let Some(title) = args.songname {
        param_fields.push(MetadataField {
            label: FieldLabel::Title,
            value: title,
        })
    } else {
        default_fields.push(MetadataField {
            label: FieldLabel::Title,
            value: default_metadata.title.value,
        });
    }

    if let Some(artist) = args.artist {
        param_fields.push(MetadataField {
            label: FieldLabel::Artist,
            value: artist,
        });
    } else {
        default_fields.push(MetadataField {
            label: FieldLabel::Artist,
            value: default_metadata.artist.value,
        });
    }

    if let Some(genre) = args.genre {
        param_fields.push(MetadataField {
            label: FieldLabel::Genre,
            value: genre,
        });
    } else {
        default_fields.push(MetadataField {
            label: FieldLabel::Genre,
            value: default_metadata.genre.value,
        });
    }

    if let Some(album) = args.album {
        param_fields.push(MetadataField {
            label: FieldLabel::Album,
            value: album,
        });
    } else {
        default_fields.push(MetadataField {
            label: FieldLabel::Album,
            value: default_metadata.album_name.value,
        });
    };

    let tag = create_base_tag(&default_fields, &param_fields);
    apply_metadata(default_fields, tag)?;

    let mut location = std::env::current_dir()?;
    location.push(FILENAME.get().unwrap_or(&"soundcloud.mp3".to_string()));
    println!("finished: {}", location.display());

    Ok(())
}

fn create_base_tag(
    default_metadata: &[MetadataField],
    param_metadata: &[MetadataField],
) -> id3::Tag {
    let mut tag = id3::Tag::new();

    default_metadata.iter().for_each(|field| match field.label {
        FieldLabel::Title => tag.set_title(field.value.as_str()),
        FieldLabel::Artist => tag.set_artist(field.value.as_str()),
        FieldLabel::Genre => tag.set_genre(field.value.as_str()),
        FieldLabel::Album => tag.set_album(field.value.as_str()),
    });

    param_metadata.iter().for_each(|field| match field.label {
        FieldLabel::Title => tag.set_title(field.value.as_str()),
        FieldLabel::Artist => tag.set_artist(field.value.as_str()),
        FieldLabel::Genre => tag.set_genre(field.value.as_str()),
        FieldLabel::Album => tag.set_album(field.value.as_str()),
    });

    tag
}

fn apply_metadata(
    metadata_fields: Vec<MetadataField>,
    mut tag: id3::Tag,
) -> Result<(), Box<dyn std::error::Error>> {
    if !metadata_fields.is_empty() {
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
    }

    let mut path = std::env::current_dir()?;
    path.push(FILENAME.get().unwrap_or(&"soundcloud.mp3".to_string()));

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

fn prompt_metadata(items: &[MetadataField]) -> Result<Vec<usize>, dialoguer::Error> {
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

    download_track(url).await
}

use std::{time::Duration, vec};

use clap::Parser;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};
use id3::{frame, Tag, TagLike};
use indicatif::ProgressBar;
use regex::Regex;
use soundcloud::{download_track, DownloadError};
use types::{FieldLabel, Metadata, MetadataField};

mod soundcloud;
mod types;

/// cli tool to download soundcloud tracks
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// soundcloud url to download (skips metadata options)
    #[arg(short, long)]
    url: Option<String>,

    /// directory to download tracks to
    #[arg(short, long)]
    download_directory: Option<String>,

    /// show current download directory
    #[arg(short, long)]
    show_download_directory: bool,

    /// use default metadata
    #[arg(short = 'U', long)]
    use_default_metadata: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term = Term::stdout();
    let args = Args::parse();

    if let Some(url) = args.url {
        let metadata = download(url).await?;
        if !args.use_default_metadata {
            term.clear_screen()?;
            apply_metadata(metadata)?;
        }
    } else {
        term.clear_screen()?;
        let metadata = download(prompt_url()?).await?;
        apply_metadata(metadata)?;
    }

    println!("finished!");

    Ok(())
}

async fn download(url: String) -> Result<types::Metadata, DownloadError> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("downloading...");
    spinner.enable_steady_tick(Duration::from_millis(50));

    Ok(download_track(url).await?)
}

fn apply_metadata(metadata: Metadata) -> Result<(), Box<dyn std::error::Error>> {
    let fields = vec![
        metadata.title,
        metadata.artist,
        metadata.album_name,
        metadata.genre,
    ];

    let mut tag = Tag::new();
    tag.set_title(&fields[0].value);
    tag.set_artist(&fields[1].value);
    tag.set_album(&fields[2].value);
    tag.set_genre(&fields[3].value);
    tag.add_frame(frame::Picture {
        mime_type: "image/jpeg".to_string(),
        picture_type: frame::PictureType::CoverFront,
        description: "Cover".to_string(),
        data: metadata.album_art,
    });

    let selected = prompt_metadata(&fields)?;

    for index in selected {
        let value = prompt_field(&fields[index])?;

        match &fields[index].label {
            FieldLabel::Title => tag.set_title(&value),
            FieldLabel::Artist => tag.set_artist(&value),
            FieldLabel::Album => tag.set_album(&value),
            FieldLabel::Genre => tag.set_genre(&value),
        }
    }
    tag.write_to_path(format!("{}.mp3", &fields[0].value), id3::Version::Id3v24)?;
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

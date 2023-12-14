use std::{time::Duration, vec};

use console::Term;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};
use id3::{frame, Tag, TagLike};
use indicatif::ProgressBar;
use regex::Regex;
use soundcloud::download_track;
use types::{FieldLabel, MetadataField};

mod soundcloud;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term = Term::stdout();
    term.clear_screen()?;
    term.set_title("soundcloud");

    let url = prompt_url()?;
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("downloading...");
    spinner.enable_steady_tick(Duration::from_millis(50));

    let metadata = download_track(url).await?;
    spinner.finish_with_message("finished download!");

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

    println!("finished!");

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

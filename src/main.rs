use std::{
    fs::{self, OpenOptions},
    time::Duration,
    vec,
};

use console::Term;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};
use id3::{
    frame,
    v1v2::{read_from_path, write_to_file},
    ErrorKind, Tag, TagLike,
};
use indicatif::ProgressBar;
use regex::Regex;
use soundcloud::download_track;
use types::MetaDataField;

mod soundcloud;
mod types;

pub const CLIENT_ID: &str = "bX15WAb1KO8PbF0ZxzrtUNTgliPQqV55";
pub const TRACK_INFO_URL: &str = "https://api-v2.soundcloud.com/resolve";
pub const TRACK_DOWNLOAD_URL: &str = "https://api-v2.soundcloud.com/tracks";

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

    let mut tag = Tag::new();
    tag.set_title(&metadata.title.value);
    tag.set_artist(&metadata.artist.value);
    tag.set_album(&metadata.album_name.value);
    tag.set_genre(&metadata.genre.value);
    tag.add_frame(frame::Picture {
        mime_type: "image/jpeg".to_string(),
        picture_type: frame::PictureType::CoverFront,
        description: "Cover".to_string(),
        data: metadata.album_art,
    });

    let fields = &vec![
        metadata.title,
        metadata.artist,
        metadata.genre,
        metadata.album_name,
    ];

    let selected = prompt_metadata(fields)?;

    for index in selected {
        fields[index].value = prompt_field(&fields[index].label)?;

        match fields[index].label.as_ref() {
            "title" => tag.set_title(&fields[index].value),
            "artist" => tag.set_artist(&fields[index].value),
            "album" => tag.set_album(&fields[index].value),
            "genre" => tag.set_genre(&fields[index].value),
            &_ => {}
        }
    }

    tag.write_to_path(&format!("{}.mp3", &metadata.title), id3::Version::Id3v24)?;

    Ok(())
}

fn prompt_url() -> Result<String, dialoguer::Error> {
    let re = Regex::new("https://soundcloud.com/.*/.*").unwrap();

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

fn prompt_metadata(items: &Vec<MetaDataField>) -> Result<Vec<usize>, dialoguer::Error> {
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("choose which fields you want to change\n  (space: select / enter: continue):")
        .items(&items)
        .interact()?;

    Ok(selection)
}

fn prompt_field(field: &String) -> Result<String, dialoguer::Error> {
    let updated: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("new {field}"))
        .interact_text()?;

    Ok(updated)
}

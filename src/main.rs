use std::{time::Duration, vec};

use audiotags::Tag;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};
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

    let track_info = download_track(url).await?;
    spinner.finish_with_message("finished download!");

    let mut fields = vec![
        MetaDataField::new("title".to_string(), track_info.title.clone()),
        MetaDataField::new("artist".to_string(), track_info.user.username.clone()),
        MetaDataField::new("album".to_string(), "".to_string()),
    ];

    let selected = prompt_metadata(&fields)?;

    let mut tag = Tag::new().read_from_path(format!(
        "{} - {}.mp3",
        track_info.user.username, track_info.title
    ))?;

    for index in selected {
        fields[index].value = prompt_field(&fields[index].label)?;

        match fields[index].label.as_ref() {
            "title" => tag.set_title(&fields[index].value),
            "artist" => tag.set_artist(&fields[index].value),
            "album" => tag.set_album_title(&fields[index].value),
            &_ => {}
        }
    }

    tag.write_to_path(&format!(
        "{} - {}.mp3",
        track_info.user.username, track_info.title
    ))?;

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

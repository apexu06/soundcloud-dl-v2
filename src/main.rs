use std::{time::Duration, vec};

use console::Term;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};
use indicatif::ProgressBar;
use regex::Regex;
use sc::download_track;

mod sc;
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
    spinner.set_message("Downloading...");
    spinner.enable_steady_tick(Duration::from_millis(50));

    let track_info = download_track(url).await?;
    spinner.finish_with_message("Finished Download!");

    let title = &track_info.title;
    let username = &track_info.user.username;

    let items = vec![
        format!("Title: {title}"),
        format!("Artist: {username}"),
        "Album".to_string(),
    ];

    prompt_metadata(items)?;

    Ok(())
}

fn prompt_url() -> Result<String, dialoguer::Error> {
    let re = Regex::new("https://soundcloud.com/.*/.*").unwrap();

    let url = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter SoundCloud URL")
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

fn prompt_metadata(items: Vec<String>) -> Result<Vec<usize>, dialoguer::Error> {
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose what metadata you want to change:")
        .items(&items)
        .interact()?;

    Ok(selection)
}

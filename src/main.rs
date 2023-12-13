use console::Term;
use dialoguer::{theme::ColorfulTheme, Error, Input};
use regex::Regex;

mod sc;
mod types;

pub const CLIENT_ID: &str = "bX15WAb1KO8PbF0ZxzrtUNTgliPQqV55";
pub const TRACK_INFO_URL: &str = "https://api-v2.soundcloud.com/resolve";
pub const TRACK_DOWNLOAD_URL: &str = "https://api-v2.soundcloud.com/tracks";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let term = Term::stdout();
    term.clear_screen()?;

    let url = prompt_url()?;

    Ok(())
}

fn prompt_url() -> Result<String, Error> {
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

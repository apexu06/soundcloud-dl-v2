use std::path::PathBuf;

use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};
use regex::Regex;

use crate::types::MetadataField;

pub fn prompt_url() -> Result<String, dialoguer::Error> {
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

pub fn prompt_dir(default_loc: String) -> Result<PathBuf, dialoguer::Error> {
    let url = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("enter directory to download to")
        .validate_with(|input: &String| -> Result<(), &str> {
            let path = PathBuf::from(input);

            if path.is_dir() {
                Ok(())
            } else {
                Err("Invalid path, make sure the directory exists")
            }
        })
        .default(default_loc)
        .interact_text()?;

    Ok(PathBuf::from(url))
}

pub fn prompt_metadata(items: &[MetadataField]) -> Result<Vec<usize>, dialoguer::Error> {
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("choose which fields you want to change\n  (space: select / enter: continue):")
        .items(items)
        .interact()?;

    Ok(selection)
}

pub fn prompt_field(field: &MetadataField) -> Result<String, dialoguer::Error> {
    let updated: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("new {}", field.label))
        .show_default(false)
        .default(field.value.clone())
        .interact_text()?;

    Ok(updated)
}

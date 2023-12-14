use std::{fs, path::PathBuf};

use crate::types::{FieldLabel, Metadata, MetadataField, TrackInfo};
use thiserror::Error;

const CLIENT_ID: &str = "bX15WAb1KO8PbF0ZxzrtUNTgliPQqV55";
const TRACK_INFO_URL: &str = "https://api-v2.soundcloud.com/resolve";

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("network error:\n{0}")]
    Network(#[from] reqwest::Error),
    #[error("could not write to file:\n{0}")]
    File(#[from] std::io::Error),
}

async fn get_track_info(url: String) -> Result<TrackInfo, reqwest::Error> {
    let client = reqwest::Client::new();
    let res: TrackInfo = client
        .get(TRACK_INFO_URL)
        .query(&[("url", url.as_str()), ("client_id", CLIENT_ID)])
        .send()
        .await?
        .json()
        .await?;

    Ok(res)
}

pub async fn download_track(url: String) -> Result<Metadata, DownloadError> {
    let track_info = match get_track_info(url).await {
        Ok(track_info) => track_info,
        Err(err) => return Err(DownloadError::Network(err)),
    };

    let album_cover = match get_track_cover(track_info.artwork_url).await {
        Ok(cover) => cover,
        Err(err) => return Err(DownloadError::Network(err)),
    };

    let metadata = Metadata {
        title: MetadataField {
            label: FieldLabel::Title,
            value: track_info.title,
        },
        artist: MetadataField {
            label: FieldLabel::Artist,
            value: track_info.user.username,
        },
        genre: MetadataField {
            label: FieldLabel::Genre,
            value: track_info.genre,
        },
        album_name: MetadataField {
            label: FieldLabel::Album,
            value: String::new(),
        },
        album_art: album_cover,
    };

    let client = reqwest::Client::new();

    #[derive(serde::Deserialize)]
    struct Mp3Link {
        url: String,
    }

    let res: Mp3Link = client
        .get(&track_info.media.transcodings[1].url)
        .query(&[("client_id", CLIENT_ID)])
        .send()
        .await?
        .json()
        .await?;

    let mp3_url = res.url;
    let res = client.get(mp3_url).send().await?.bytes().await?;

    let mut path = std::env::current_dir().unwrap();
    path.push(format!("{}.mp3", metadata.title.value));

    fs::write(path, res)?;
    Ok(metadata)
}

pub async fn get_track_cover(url: String) -> Result<Vec<u8>, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?.bytes().await?;
    Ok(res.iter().map(|b| *b).collect())
}

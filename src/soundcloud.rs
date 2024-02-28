use std::{fs, path::PathBuf};

use crate::{
    get_filename, get_filepath,
    types::{DownloadError, FieldLabel, FromResponse, Metadata, MetadataField, TrackInfo},
    FILENAME,
};

const CLIENT_ID: &str = "tPycpzX7dXV3LN9SC9RpDUI9s4lKl9cc";
const TRACK_INFO_URL: &str = "https://api-v2.soundcloud.com/resolve";

const INVALID_FILENAME_SYMBOLS: [char; 9] = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];

async fn get_track_info(url: String) -> Result<TrackInfo, DownloadError> {
    let client = reqwest::Client::new();
    let res: TrackInfo = client
        .get(TRACK_INFO_URL)
        .query(&[("url", url.as_str()), ("client_id", CLIENT_ID)])
        .send()
        .await
        .from_response()?
        .json()
        .await?;

    Ok(res)
}

pub async fn download_track(url: String) -> Result<Metadata, DownloadError> {
    let track_info = get_track_info(url).await?;
    let album_cover = get_track_cover(track_info.artwork_url).await?;

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
        .await
        .from_response()?
        .json()
        .await?;

    let mp3_url = res.url;
    let res = client.get(mp3_url).send().await?.bytes().await?;

    let mut path = get_filepath();

    FILENAME.get_or_init(|| {
        let mut title = metadata.title.value.replace(" ", "_");
        INVALID_FILENAME_SYMBOLS.iter().for_each(|symbol| {
            title = title.replace(*symbol, "");
        });

        PathBuf::from(title).with_extension("mp3")
    });

    path.push(get_filename());

    fs::write(path, res)?;
    Ok(metadata)
}

pub async fn get_track_cover(url: String) -> Result<Vec<u8>, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = url.replace("large", "t500x500");
    let res = client.get(url).send().await?.bytes().await?;
    Ok(res.iter().copied().collect())
}

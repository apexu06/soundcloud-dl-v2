use std::fs;

use crate::types::TrackInfo;
use thiserror::Error;

use crate::{CLIENT_ID, TRACK_INFO_URL};

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

pub async fn download_track(url: String) -> Result<TrackInfo, DownloadError> {
    let track_info = match get_track_info(url).await {
        Ok(track_info) => track_info,
        Err(err) => return Err(DownloadError::Network(err)),
    };

    let client = reqwest::Client::new();

    #[derive(serde::Deserialize)]
    struct Mp3Link {
        url: String,
    }

    if !track_info.downloadable {
        let res: Mp3Link = client
            .get(&track_info.media.transcodings[0].url)
            .query(&[("client_id", CLIENT_ID)])
            .send()
            .await?
            .json()
            .await?;

        let mp3_url = res.url;
        let res = client.get(mp3_url).send().await?.bytes().await?;

        fs::write(
            format!("{} - {}.mp3", track_info.user.username, track_info.title),
            res,
        )?;
        Ok(track_info)
    } else {
        // let res = client
        //     .get(format!("{}/{}/download", TRACK_DOWNLOAD_URL, track_info.id))
        //     .query(&[("client_id", CLIENT_ID)])
        //     .send()
        //     .await
        //     .map_err(|_| "Failed to download track".to_string())?;

        Ok(track_info)
    }
}

pub async fn get_track_cover<'a>(url: String) -> Result<&'a [u8], DownloadError> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?.bytes().await?;

    //Ok(res.to_vec().as_slice())
    todo!()
}

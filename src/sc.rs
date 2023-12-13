use crate::types::TrackInfo;
use std::fs;

use crate::{CLIENT_ID, TRACK_INFO_URL};

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

async fn download_track(url: String) -> Result<(), String> {
    let track_info = match get_track_info(url).await {
        Ok(track_info) => track_info,
        Err(_) => return Err("Failed to get track info".to_string()),
    };

    println!("{:?}", track_info.title);

    // if !track_info.downloadable {
    //     return Err("Track is not downloadable".to_string());
    // }

    let client = reqwest::Client::new();
    // let res = client
    //     .get(format!("{}/{}/download", TRACK_DOWNLOAD_URL, track_info.id))
    //     .query(&[("client_id", CLIENT_ID)])
    //     .send()
    //     .await
    //     .map_err(|_| "Failed to download track".to_string())?;
    //
    //

    #[derive(serde::Deserialize, Debug)]
    struct Test {
        url: String,
    }

    let res: Test = client
        .get("https://api-v2.soundcloud.com/media/soundcloud:tracks:1007008426/c851067c-d685-49b3-8186-cd309246cac9/stream/progressive")
        .query(&[("client_id", CLIENT_ID)])
        .send()
        .await.map_err(|_| "fail")?
        .json()
        .await.map_err(|_| "fail")?;

    let mp3_url = res.url;

    println!("{:?}", mp3_url);

    let res = client
        .get(mp3_url)
        .send()
        .await
        .map_err(|_| "fail")?
        .bytes()
        .await
        .map_err(|_| "fail")?;

    println!("{:?}", res);

    fs::write("test.mp3", res).unwrap();

    Ok(())
}

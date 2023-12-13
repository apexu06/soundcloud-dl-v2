use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TrackInfo {
    pub artwork_url: String,
    pub downloadable: bool,
    pub genre: String,
    pub id: u32,
    pub title: String,
    pub kind: String,
    pub user: User,
    pub media: Vec<Transcoding>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct Transcoding {
    pub url: String,
    pub preset: String,
    pub format: TranscodingFormat,
}

#[derive(Debug, Deserialize)]
pub struct TranscodingFormat {
    pub protocol: String,
    pub mime_type: String,
}

use std::fmt::Display;

use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct TrackInfo {
    pub artwork_url: String,
    pub genre: String,
    pub id: u32,
    pub title: String,
    pub user: User,
    pub media: Transcodings,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct Transcodings {
    pub transcodings: Vec<Transcoding>,
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

pub enum FieldLabel {
    Title,
    Artist,
    Album,
    Genre,
}

impl Display for FieldLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            FieldLabel::Title => "title",
            FieldLabel::Artist => "artist",
            FieldLabel::Album => "album",
            FieldLabel::Genre => "genre",
        };

        write!(f, "{}", label)
    }
}

pub struct Metadata {
    pub title: MetadataField,
    pub artist: MetadataField,
    pub genre: MetadataField,
    pub album_name: MetadataField,
    pub album_art: Vec<u8>,
}

pub struct MetadataField {
    pub label: FieldLabel,
    pub value: String,
}

impl Display for MetadataField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)?;
        if !self.value.is_empty() {
            write!(f, ": {}", self.value)?;
        }

        Ok(())
    }
}

pub trait FromResponse {
    fn from_response(self) -> Result<reqwest::Response, DownloadError>;
}

impl FromResponse for Result<reqwest::Response, reqwest::Error> {
    fn from_response(self) -> Result<reqwest::Response, DownloadError> {
        let response = self?;

        match response.status() {
            code if code == reqwest::StatusCode::NOT_FOUND => Err(DownloadError::NetworkNotFound(
                extract_soundcloud_url(Some(response.url())),
            )),
            code if code == reqwest::StatusCode::FORBIDDEN => Err(DownloadError::NetworkForbidden(
                extract_soundcloud_url(Some(response.url())),
            )),
            _ => Ok(response),
        }
    }
}

impl From<reqwest::Error> for DownloadError {
    fn from(value: reqwest::Error) -> Self {
        match value.status() {
            Some(code) if code == reqwest::StatusCode::NOT_FOUND => {
                DownloadError::NetworkNotFound(extract_soundcloud_url(value.url()))
            }
            Some(code) if code == reqwest::StatusCode::FORBIDDEN => {
                DownloadError::NetworkForbidden(extract_soundcloud_url(value.url()))
            }
            _ => DownloadError::NetworkOther(value),
        }
    }
}

fn extract_soundcloud_url(url: Option<&reqwest::Url>) -> String {
    match url {
        Some(url) => url
            .query_pairs()
            .find(|(key, _)| key == "url")
            .map(|(_, value)| value.to_string())
            .unwrap_or_else(|| url.to_string()),

        None => "".to_string(),
    }
}

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("could not write to file: {0}")]
    File(#[from] std::io::Error),
    #[error("not found: {0}")]
    NetworkNotFound(String),
    #[error("forbidden: {0}")]
    NetworkForbidden(String),
    #[error("{0}")]
    NetworkOther(reqwest::Error),
}

#[derive(Error, Debug)]
pub enum SoundcloudError {
    #[error("{0}")]
    Download(#[from] DownloadError),
}

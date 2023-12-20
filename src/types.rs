use std::fmt::Display;

use serde::Deserialize;

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

use std::fmt::Display;

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

pub struct MetaDataField {
    pub label: String,
    pub value: String,
}

impl MetaDataField {
    pub fn new(label: String, value: String) -> Self {
        MetaDataField { label, value }
    }
}

impl Display for MetaDataField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label.as_ref())?;

        if self.value.len() > 0 {
            f.write_str(format!(": {}", self.value).as_ref())?;
        }

        Ok(())
    }
}

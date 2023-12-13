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
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
}

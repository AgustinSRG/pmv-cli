// Albums models

use serde::{Deserialize, Serialize};

use super::MediaListItem;

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumListItem {
    #[serde(rename = "id")]
    pub id: u64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "size")]
    pub size: u32,

    #[serde(rename = "thumbnail")]
    pub thumbnail: String,

    #[serde(rename = "lm")]
    pub lm: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumNameBody {
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumIdResponse {
    #[serde(rename = "album_id")]
    pub album_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    #[serde(rename = "id")]
    pub id: u64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "list")]
    pub list: Vec<MediaListItem>,

    #[serde(rename = "lm")]
    pub lm: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumSetOrderBody {
    #[serde(rename = "list")]
    pub list: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumMediaBody {
    #[serde(rename = "media_id")]
    pub media_id: u64,
}

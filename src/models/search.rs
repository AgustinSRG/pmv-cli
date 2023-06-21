// Search API models

use serde::{Deserialize, Serialize};

use super::MediaType;

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaListItem {
    #[serde(rename = "id")]
    pub id: u64,

    #[serde(rename = "type")]
    pub media_type: MediaType,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "thumbnail")]
    pub thumbnail: String,

    #[serde(rename = "tags")]
    pub tags: Vec<u64>,

    #[serde(rename = "duration")]
    pub duration: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchMediaResult {
    #[serde(rename = "total_count")]
    pub total_count: u64,

    #[serde(rename = "page_index")]
    pub page_index: u32,

    #[serde(rename = "page_count")]
    pub page_count: u32,

    #[serde(rename = "page_size")]
    pub page_size: u32,

    #[serde(rename = "page_items")]
    pub page_items: Vec<MediaListItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomMediaResult {
    #[serde(rename = "seed")]
    pub seed: i64,

    #[serde(rename = "page_size")]
    pub page_size: u32,

    #[serde(rename = "page_items")]
    pub page_items: Vec<MediaListItem>,
}

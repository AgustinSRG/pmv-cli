// Media API models

use serde::{Deserialize, Serialize};

use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum MediaType {
    Deleted = 0,
    Image = 1,
    Video = 2,
    Audio = 3,
}

impl MediaType {
    pub fn to_string(&self) -> String {
        match self {
            MediaType::Deleted => {
                return "N/A".to_string();
            },
            MediaType::Image => {
                return "Image".to_string();
            },
            MediaType::Video => {
                return "Video".to_string();
            },
            MediaType::Audio => {
                return "Audio".to_string();
            },
        }
    }
}

pub fn parse_media_type(s: &str) -> Result<MediaType, ()> {
    let s_lower = s.to_lowercase();

    if s_lower == "video" || s_lower == "videos" {
        return Ok(MediaType::Video);
    } else if s_lower == "audio" || s_lower == "audios" {
        return Ok(MediaType::Audio);
    } else if s_lower == "image" || s_lower == "images" || s_lower == "picture" || s_lower == "pictures" {
        return Ok(MediaType::Image);
    } else {
        return Err(());
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub thumbnail: Option<String>,

    #[serde(rename = "tags")]
    pub tags: Vec<u64>,

    #[serde(rename = "duration")]
    pub duration: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediaMetadata {
    #[serde(rename = "id")]
    pub id: u64,

    #[serde(rename = "type")]
    pub media_type: MediaType,

    #[serde(rename = "upload_time")]
    pub upload_time: i64,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "thumbnail")]
    pub thumbnail: String,

    #[serde(rename = "tags")]
    pub tags: Vec<u64>,

    #[serde(rename = "duration")]
    pub duration: Option<f64>,

    #[serde(rename = "width")]
    pub width: Option<i32>,

    #[serde(rename = "height")]
    pub height: Option<i32>,

    #[serde(rename = "fps")]
    pub fps: Option<i32>,

    #[serde(rename = "ready")]
    pub ready: bool,

    #[serde(rename = "ready_p")]
    pub ready_p: Option<i32>,

    #[serde(rename = "encoded")]
    pub encoded: bool,

    #[serde(rename = "task")]
    pub task: Option<u64>,

    #[serde(rename = "url")]
    pub url: Option<String>,

    #[serde(rename = "video_previews")]
    pub video_previews: Option<String>,

    #[serde(rename = "video_previews_interval")]
    pub video_previews_interval: Option<f64>,

    #[serde(rename = "force_start_beginning")]
    pub force_start_beginning: Option<bool>,

    #[serde(rename = "resolutions")]
    pub resolutions: Option<Vec<MediaResolution>>,

    #[serde(rename = "subtitles")]
    pub subtitles: Option<Vec<MediaSubtitle>>, 

    #[serde(rename = "audios")]
    pub audios: Option<Vec<MediaAudioTrack>>, 

    #[serde(rename = "time_slices")]
    pub time_slices: Option<Vec<MediaTimeSlice>>,

    #[serde(rename = "img_notes")]
    pub img_notes: Option<bool>,

    #[serde(rename = "img_notes_url")]
    pub img_notes_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediaResolution {
    #[serde(rename = "width")]
    pub width: i32,

    #[serde(rename = "height")]
    pub height: i32,

    #[serde(rename = "fps")]
    pub fps: Option<i32>,

    #[serde(rename = "ready")]
    pub ready: bool,

    #[serde(rename = "task")]
    pub task: Option<u64>,

    #[serde(rename = "url")]
    pub url: Option<String>,
}

impl MediaResolution {
    pub fn to_string(&self) -> String {
        let w = self.width;
        let h = self.height;
        let fps = self.fps.unwrap_or(0);
        if fps > 0 {
            return format!("{w}x{h}:{fps}");
        } else {
            return format!("{w}x{h}");
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediaSubtitle {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediaAudioTrack {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediaTimeSlice {
    #[serde(rename = "time")]
    pub time: f64,

    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediaUploadResponse {
    #[serde(rename = "media_id")]
    pub media_id: u64,
}

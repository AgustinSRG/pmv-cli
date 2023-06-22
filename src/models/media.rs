// Media API models

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


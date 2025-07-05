// Models for home page API

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::{AlbumListItem, MediaListItem};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum HomePageGroupType {
    #[serde(other)]
    Custom = 0,
    RecentMedia = 1,
    RecentAlbums = 2,
}

impl HomePageGroupType {
    pub fn as_string(&self) -> String {
        match self {
            HomePageGroupType::Custom => "CUSTOM".to_string(),
            HomePageGroupType::RecentMedia => "RECENT_MEDIA".to_string(),
            HomePageGroupType::RecentAlbums => "RECENT_ALBUMS".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "CUSTOM" => Some(HomePageGroupType::Custom),
            "RECENT_MEDIA" => Some(HomePageGroupType::RecentMedia),
            "RECENT_ALBUMS" => Some(HomePageGroupType::RecentAlbums),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomePageGroup {
    #[serde(rename = "id")]
    pub id: u64,

    #[serde(rename = "type")]
    pub group_type: HomePageGroupType,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "elementsCount")]
    pub elements_count: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomePageAddGroupBody {
    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "type")]
    pub group_type: HomePageGroupType,

    #[serde(rename = "prepend")]
    pub prepend: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomePageElement {
    #[serde(rename = "media")]
    media: Option<MediaListItem>,

    #[serde(rename = "album")]
    album: Option<AlbumListItem>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum HomePageElementType {
    #[serde(other)]
    Media = 0,
    Album = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomePageElementRef {
    #[serde(rename = "t")]
    element_type: HomePageElementType,

    #[serde(rename = "i")]
    id: u64,
}

impl HomePageElementRef {
    pub fn from_home_page_elements(elements: &[HomePageElement]) -> Vec<HomePageElementRef> {
        elements
            .iter()
            .map(|e| Self::from_home_page_element(e))
            .flatten()
            .collect()
    }

    pub fn from_home_page_element(element: &HomePageElement) -> Option<HomePageElementRef> {
        match &element.media {
            Some(media) => Some(HomePageElementRef {
                element_type: HomePageElementType::Media,
                id: media.id,
            }),
            None => match &element.album {
                Some(album) => Some(HomePageElementRef {
                    element_type: HomePageElementType::Album,
                    id: album.id,
                }),
                None => None,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomePageGroupSetElementsBody {
    #[serde(rename = "elements")]
    pub elements: Vec<HomePageElementRef>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomePageGroupRenameBody {
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomePageGroupMoveBody {
    #[serde(rename = "position")]
    pub position: u32,
}

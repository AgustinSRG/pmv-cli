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

impl HomePageElement {
    pub fn is_valid_element(&self) -> bool {
        self.album.is_some() || self.media.is_some()
    }

    pub fn get_element_type(&self) -> HomePageElementType {
        if self.album.is_some() {
            HomePageElementType::Album
        } else {
            HomePageElementType::Media
        }
    }

    pub fn get_element_id(&self) -> u64 {
        if let Some(a) = &self.album {
            return a.id;
        }

        if let Some(m) = &self.media {
            return m.id;
        }

        0
    }

    pub fn get_element_title(&self) -> String {
        if let Some(a) = &self.album {
            return a.name.clone();
        }

        if let Some(m) = &self.media {
            return m.title.clone();
        }

        "".to_string()
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum HomePageElementType {
    #[serde(other)]
    Media = 0,
    Album = 1,
}

impl HomePageElementType {
    pub fn as_string(&self) -> String {
        match self {
            HomePageElementType::Media => "MEDIA".to_string(),
            HomePageElementType::Album => "ALBUM".to_string(),
        }
    }

    pub fn as_prefix(&self) -> String {
        match self {
            HomePageElementType::Media => "M".to_string(),
            HomePageElementType::Album => "A".to_string(),
        }
    }

    pub fn parse_prefix(prefix: &str) -> Option<HomePageElementType> {
        match prefix.to_uppercase().as_str() {
            "M" => Some(HomePageElementType::Media),
            "A" => Some(HomePageElementType::Album),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomePageElementRef {
    #[serde(rename = "t")]
    element_type: HomePageElementType,

    #[serde(rename = "i")]
    id: u64,
}

impl HomePageElementRef {
    pub fn as_string(&self) -> String {
        format!("{}{}", self.element_type.as_prefix(), self.id)
    }

    pub fn as_list_string(element_refs: &[Self]) -> String {
        let elements_strings: Vec<String> = element_refs.iter().map(|r| r.as_string()).collect();
        elements_strings.join(", ")
    }

    pub fn from_string(s: &str) -> Option<Self> {
        if s.len() < 2 {
            return None;
        }

        let prefix = s[0..1].to_string();
        let id_str = s[1..].to_string();

        let element_type = match HomePageElementType::parse_prefix(&prefix) {
            Some(t) => t,
            None => {
                return None;
            }
        };

        let id = match id_str.parse::<u64>() {
            Ok(i) => i,
            Err(_) => {
                return None;
            }
        };

        Some(Self { element_type, id })
    }

    pub fn from_list_string(list: &str) -> Result<Vec<Self>, usize> {
        let list_split: Vec<String> = list.split(",").map(|i| i.trim().to_uppercase()).collect();

        let mut res: Vec<Self> = Vec::with_capacity(list_split.len());

        for (i, item) in list_split.iter().enumerate() {
            let element_ref = match Self::from_string(item) {
                Some(e) => e,
                None => {
                    return Err(i);
                }
            };

            res.push(element_ref);
        }

        Ok(res)
    }

    pub fn from_home_page_elements(elements: &[HomePageElement]) -> Vec<Self> {
        elements
            .iter()
            .filter_map(Self::from_home_page_element)
            .collect()
    }

    pub fn from_home_page_element(element: &HomePageElement) -> Option<Self> {
        match &element.media {
            Some(media) => Some(Self {
                element_type: HomePageElementType::Media,
                id: media.id,
            }),
            None => element.album.as_ref().map(|album| Self {
                element_type: HomePageElementType::Album,
                id: album.id,
            }),
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

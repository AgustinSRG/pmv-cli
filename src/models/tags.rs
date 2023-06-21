// Tags models

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaTag {
    #[serde(rename = "id")]
    pub id: u64,

    #[serde(rename = "name")]
    pub name: String,
}

pub fn tags_map_from_list(list: &Vec<MediaTag>) -> HashMap<u64, String> {
    let mut res: HashMap<u64, String> = HashMap::new();

    for tag in list {
        res.insert(tag.id, tag.name.clone());
    }

    return res;
}

pub fn tags_names_from_ids(ids: &Vec<u64>, tags_map: &HashMap<u64, String>) -> Vec<String> {
    let mut res:  Vec<String> = Vec::with_capacity(ids.len());

    for id in ids {
        let default_name = "#".to_string() + &id.to_string();
        let tag_name = tags_map.get(id).unwrap_or(&default_name);
        res.push((*tag_name).clone());
    }

    return res;
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AddTagBody {
    #[serde(rename = "media_id")]
    pub media_id: u64,

    #[serde(rename = "tag_name")]
    pub tag_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveTagBody {
    #[serde(rename = "media_id")]
    pub media_id: u64,

    #[serde(rename = "tag_id")]
    pub tag_id: u64,
}


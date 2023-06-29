// Config models

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ConfigVideoResolution {
    #[serde(rename = "width")]
    pub width: i32,

    #[serde(rename = "height")]
    pub height: i32,

    #[serde(rename = "fps")]
    pub fps: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ConfigImageResolution {
    #[serde(rename = "width")]
    pub width: i32,

    #[serde(rename = "height")]
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaultConfig {
    #[serde(rename = "title")]
    pub title: Option<String>,

    #[serde(rename = "css")]
    pub css: Option<String>,

    #[serde(rename = "max_tasks")]
    pub max_tasks: i32,

    #[serde(rename = "encoding_threads")]
    pub encoding_threads: i32,

    #[serde(rename = "resolutions")]
    pub resolutions: Vec<ConfigVideoResolution>,

    #[serde(rename = "image_resolutions")]
    pub image_resolutions: Vec<ConfigImageResolution>,
}

impl ConfigVideoResolution {
    pub fn to_string(&self) -> String {
        let w = self.width;
        let h = self.height;
        let fps = self.fps;
        return format!("{w}x{h}:{fps}");
    }

    pub fn from_str(res_str: &str) -> Result<Self, ()> {
        let parts: Vec<&str> = res_str.split(":").collect();

        if parts.len() != 2 {
            return Err(());
        }

        let fps = parts[1].parse::<i32>();

        if fps.is_err() {
            return Err(());
        }

        let parts2: Vec<&str> = parts[0].split("x").collect();

        if parts2.len() != 2 {
            return Err(());
        }

        let width = parts2[0].parse::<i32>();

        if width.is_err() {
            return Err(());
        }

        let height = parts2[1].parse::<i32>();

        if height.is_err() {
            return Err(());
        }

        return Ok(ConfigVideoResolution{
            width: width.unwrap(),
            height: height.unwrap(),
            fps: fps.unwrap(),
        });
    }
}

impl ConfigImageResolution {
    pub fn to_string(&self) -> String {
        let w = self.width;
        let h = self.height;
        return format!("{w}x{h}");
    }

    pub fn from_str(res_str: &str) -> Result<Self, ()> {
        let parts: Vec<&str> = res_str.split("x").collect();

        if parts.len() != 2 {
            return Err(());
        }

        let width = parts[0].parse::<i32>();

        if width.is_err() {
            return Err(());
        }

        let height = parts[1].parse::<i32>();

        if height.is_err() {
            return Err(());
        }

        return Ok(ConfigImageResolution{
            width: width.unwrap(),
            height: height.unwrap(),
        });
    }
}

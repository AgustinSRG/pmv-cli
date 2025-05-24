// Models for about API

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInformation {
  #[serde(rename = "version")]
  pub version: String,

  #[serde(rename = "last_release")]
  pub last_release: String,

  #[serde(rename = "ffmpeg_version")]
  pub ffmpeg_version: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ServerDiskUsage {
  #[serde(rename = "usage")]
  pub usage: f32,

  #[serde(rename = "available")]
  pub available: u64,

  #[serde(rename = "free")]
  pub free: u64,

  #[serde(rename = "total")]
  pub total: u64,
}

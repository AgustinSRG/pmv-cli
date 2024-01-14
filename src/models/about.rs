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

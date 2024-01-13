// Models for invites API

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteCodeStatus {
  #[serde(rename = "has_code")]
  pub has_code: bool,

  #[serde(rename = "code")]
  pub code: Option<String>,

  #[serde(rename = "duration")]
  pub duration: Option<i64>,

  #[serde(rename = "expiration_remaining")]
  pub expiration_remaining: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvitedSession {
  #[serde(rename = "index")]
  pub index: u64,

  #[serde(rename = "timestamp")]
  pub timestamp: i64,

  #[serde(rename = "expiration")]
  pub expiration: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteCodeGenerateBody {
  #[serde(rename = "duration")]
  pub duration: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteCodeLoginBody {
  #[serde(rename = "code")]
  pub code: String,
}
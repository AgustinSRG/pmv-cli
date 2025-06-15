// Models for authentication API

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
  #[serde(rename = "username")]
  pub username: String,

  #[serde(rename = "password")]
  pub password: String,

  #[serde(rename = "duration")]
  pub duration: Option<String>,

  #[serde(rename = "tfaCode")]
  pub tfa_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
  #[serde(rename = "session_id")]
  pub session_id: String,

  #[serde(rename = "vault_fingerprint")]
  pub vault_fingerprint: Option<String>,
}

// Models for account API

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountContext {
  #[serde(rename = "username")]
  pub username: String,

  #[serde(rename = "title")]
  pub title: Option<String>,

  #[serde(rename = "css")]
  pub css: Option<String>,

  #[serde(rename = "root")]
  pub root: bool,

  #[serde(rename = "write")]
  pub write: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeUsernameBody {
  #[serde(rename = "username")]
  pub username: String,

  #[serde(rename = "password")]
  pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordBody {
  #[serde(rename = "old_password")]
  pub old_password: String,

  #[serde(rename = "password")]
  pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountListItem {
  #[serde(rename = "username")]
  pub username: String,

  #[serde(rename = "write")]
  pub write: bool,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AccountCreateBody {
  #[serde(rename = "username")]
  pub username: String,

  #[serde(rename = "password")]
  pub password: String,

  #[serde(rename = "write")]
  pub write: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountUpdateBody {
  #[serde(rename = "username")]
  pub username: String,

  #[serde(rename = "newUsername")]
  pub new_username: Option<String>,

  #[serde(rename = "write")]
  pub write: Option<bool>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AccountDeleteBody {
  #[serde(rename = "username")]
  pub username: String,
}

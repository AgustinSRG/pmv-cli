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


#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSecuritySettings {
  #[serde(rename = "tfa")]
  pub tfa: bool,

  #[serde(rename = "tfaMethod")]
  pub tfa_method: String,

  #[serde(rename = "authConfirmation")]
  pub auth_confirmation: bool,

  #[serde(rename = "authConfirmationMethod")]
  pub auth_confirmation_method: String,

  #[serde(rename = "authConfirmationPeriodSeconds")]
  pub auth_confirmation_period_seconds: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSetSecuritySettingsBody {
  #[serde(rename = "authConfirmation")]
  pub auth_confirmation: bool,

  #[serde(rename = "authConfirmationMethod")]
  pub auth_confirmation_method: String,

  #[serde(rename = "authConfirmationPeriodSeconds")]
  pub auth_confirmation_period_seconds: i32,
}


#[derive(Debug, Copy, Clone)]
pub enum TimeOtpAlgorithm {
  Sha1,
  Sha256,
  Sha512,
}

impl TimeOtpAlgorithm {
  pub fn parse(input: &str) -> Result<TimeOtpAlgorithm, ()> {
    match input.to_lowercase().as_str() {
      "sha1" | "sha-1" => Ok(TimeOtpAlgorithm::Sha1),
      "sha256" | "sha-256" => Ok(TimeOtpAlgorithm::Sha256),
      "sha512" | "sha-512" => Ok(TimeOtpAlgorithm::Sha512),
      _ => Err(()),
    }
  }
}

impl ToString for TimeOtpAlgorithm {
    fn to_string(&self) -> String {
        match self {
            TimeOtpAlgorithm::Sha1 => "sha1".to_string(),
            TimeOtpAlgorithm::Sha256 => "sha256".to_string(),
            TimeOtpAlgorithm::Sha512 => "sha512".to_string(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TimeOtpPeriod {
  P30,
  P60,
  P120,
}

impl TimeOtpPeriod {
  pub fn parse(input: &str) -> Result<TimeOtpPeriod, ()> {
    match input.to_lowercase().as_str() {
      "30s" | "30" => Ok(TimeOtpPeriod::P30),
      "1m" | "60s" | "60" => Ok(TimeOtpPeriod::P60),
      "2m" | "120s" | "120" => Ok(TimeOtpPeriod::P120),
      _ => Err(()),
    }
  }
}

impl ToString for TimeOtpPeriod {
    fn to_string(&self) -> String {
        match self {
            TimeOtpPeriod::P30 => "30".to_string(),
            TimeOtpPeriod::P60 => "60".to_string(),
            TimeOtpPeriod::P120 => "120".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct TimeOtpOptions {
  pub issuer: Option<String>,

  pub account: Option<String>,

  pub algorithm: TimeOtpAlgorithm,

  pub period: TimeOtpPeriod,

  pub skew: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeOtpSettings {
  #[serde(rename = "secret")]
  pub secret: String,

  #[serde(rename = "method")]
  pub method: String,

  #[serde(rename = "url")]
  pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeOtpEnableBody {
  #[serde(rename = "secret")]
  pub secret: String,

  #[serde(rename = "method")]
  pub method: String,

  #[serde(rename = "password")]
  pub password: String,

  #[serde(rename = "code")]
  pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TfaDisableBody {
  #[serde(rename = "code")]
  pub code: String,
}

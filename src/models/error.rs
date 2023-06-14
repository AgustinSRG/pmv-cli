// API error model

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct APIErrorResponse {
    #[serde(rename = "code")]
    pub code: String,
  
    #[serde(rename = "message")]
    pub message: String,
  }

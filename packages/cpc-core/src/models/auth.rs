// DEPRECATED: This file has been replaced by token.rs. Use AuthToken in token.rs instead.
// This file is kept for reference only.
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthToken {
    pub access_token: String,
    pub refresh_token: String,
    #[serde(with = "crate::utils::datetime")]
    pub expires_at: DateTime<Utc>,
}
// DateTime handling follows the standard defined in [DATETIME_STANDARD.md](../../docs/DATETIME_STANDARD.md)
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthToken {
    pub access_token: String,
    pub refresh_token: String,
    #[serde(with = "crate::utils::datetime")]
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceFingerprint(String);

impl DeviceFingerprint {
    pub fn new(user_agent: &str, ip: &str) -> Self {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(user_agent.as_bytes());
        hasher.update(ip.as_bytes());
        let result = hasher.finalize();
        DeviceFingerprint(format!("{:x}", result))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub user_id: Uuid,
    pub token: String,
    pub token_type: String, // "access" or "refresh"
    pub device_fingerprint: String,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NewToken {
    pub user_id: Uuid,
    pub token: String,
    pub token_type: String, // "access" or "refresh"
    pub device_fingerprint: String,
    pub expires_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_auth_token_serialization() {
        let dt = Utc.with_ymd_and_hms(2025, 7, 22, 1, 42, 45).unwrap().with_nanosecond(82000000).unwrap();
        let token = AuthToken {
            access_token: "access-token".to_string(),
            refresh_token: "refresh-token".to_string(),
            expires_at: dt,
        };

        let json = serde_json::to_string(&token).unwrap();
        assert!(json.contains("\"expires_at\":\"2025-07-22T01:42:45.082Z\""));
    }
}
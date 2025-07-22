// DateTime handling follows the standard defined in [DATETIME_STANDARD.md](../../docs/DATETIME_STANDARD.md)
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,  // Store hashed passwords only
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,  // Plaintext password for registration
}

impl NewUser {
    /// Validates user input with a focus on security
    pub fn validate(&self) -> Result<(), String> {
        // Username validation
        if self.username.len() < 3 {
            return Err("Username must be at least 3 characters".to_string());
        }
        
        // Email validation
        if !self.email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        
        // Password strength requirements
        if self.password.len() < 8 {
            return Err("Password must be at least 8 characters".to_string());
        }
        
        // Add more security checks as needed (special characters, etc)
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_user_serialization() {
        let dt = Utc.with_ymd_and_hms(2025, 7, 22, 1, 42, 45).unwrap().with_nanosecond(82000000).unwrap();
        let user = User {
            id: "test-id".to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            created_at: dt,
        };

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("\"created_at\":\"2025-07-22T01:42:45.082Z\""));
    }
}
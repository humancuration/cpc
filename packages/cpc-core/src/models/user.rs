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
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub friends: Vec<uuid::Uuid>,
    pub followers: Vec<uuid::Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,  // Plaintext password for registration
    pub display_name: Option<String>,
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
        
        // Display name validation (if provided)
        if let Some(display_name) = &self.display_name {
            if display_name.len() > 50 {
                return Err("Display name must be 50 characters or less".to_string());
            }
        }
        
        Ok(())
    }

    /// Updates the user's password with a new one
    pub fn update_password(&mut self, new_password: String) {
        self.password = new_password;
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
            id: uuid::Uuid::nil(), // Using nil UUID for test
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            created_at: dt,
            updated_at: dt,
            display_name: Some("Test User".to_string()),
            bio: Some("Test bio".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            friends: vec![],
            followers: vec![],
        };

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("\"created_at\":\"2025-07-22T01:42:45.082Z\""));
        assert!(json.contains("\"displayName\":\"Test User\""));
        assert!(json.contains("\"bio\":\"Test bio\""));
        assert!(json.contains("\"avatarUrl\":\"https://example.com/avatar.jpg\""));
    }
}
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

impl User {
    pub fn new(username: String, display_name: String, email: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            username,
            display_name,
            email,
            created_at: now,
            updated_at: now,
            is_active: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(
            "testuser".to_string(),
            "Test User".to_string(),
            "test@example.com".to_string(),
        );
        
        assert_eq!(user.username, "testuser");
        assert_eq!(user.display_name, "Test User");
        assert_eq!(user.email, "test@example.com");
        assert!(user.is_active);
    }
}
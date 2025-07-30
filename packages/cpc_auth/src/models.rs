use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

impl Credentials {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    User,
    Admin,
    Moderator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub roles: Vec<Role>,
}

impl User {
    pub fn new(email: String, password_hash: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            password_hash,
            is_verified: false,
            created_at: Utc::now(),
            roles: vec![Role::User],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub device_info: String,
}

impl Session {
    pub fn new(user_id: Uuid, device_info: String) -> Self {
        let now = Utc::now();
        let expires_at = now + chrono::Duration::minutes(30); // 30 minute session
        
        Self {
            id: Uuid::new_v4(),
            user_id,
            created_at: now,
            expires_at,
            device_info,
        }
    }
    
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}
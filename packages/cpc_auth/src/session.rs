//! # Session Management
//!
//! Redis-based session store implementation for the CPC authentication system.

use crate::models::Session;
use crate::error::AuthError;
use redis::Commands;
use uuid::Uuid;

/// Redis-based session store
pub struct RedisSessionStore {
    client: redis::Client,
}

impl RedisSessionStore {
    /// Create a new Redis session store
    pub fn new(redis_url: &str) -> Result<Self, AuthError> {
        let client = redis::Client::open(redis_url)
            .map_err(|e| AuthError::DatabaseError(format!("Failed to connect to Redis: {}", e)))?;
        Ok(Self { client })
    }

    /// Get a session by its ID
    pub fn get_session(&self, session_id: &str) -> Result<Option<Session>, AuthError> {
        let mut con = self.client.get_connection()
            .map_err(|e| AuthError::DatabaseError(format!("Failed to get Redis connection: {}", e)))?;
        
        let session_data: Option<String> = con.get(session_id)
            .map_err(|e| AuthError::DatabaseError(format!("Failed to get session from Redis: {}", e)))?;
        
        if let Some(data) = session_data {
            let session: Session = serde_json::from_str(&data)
                .map_err(|e| AuthError::DatabaseError(format!("Failed to deserialize session: {}", e)))?;
            Ok(Some(session))
        } else {
            Ok(None)
        }
    }

    /// Save a session to Redis
    pub fn save_session(&self, session: &Session) -> Result<(), AuthError> {
        let mut con = self.client.get_connection()
            .map_err(|e| AuthError::DatabaseError(format!("Failed to get Redis connection: {}", e)))?;
        
        let session_data = serde_json::to_string(session)
            .map_err(|e| AuthError::DatabaseError(format!("Failed to serialize session: {}", e)))?;
        
        // Set expiration time (30 minutes)
        let ttl: usize = 30 * 60;
        con.set_ex(&session.id.to_string(), session_data, ttl)
            .map_err(|e| AuthError::DatabaseError(format!("Failed to save session to Redis: {}", e)))?;
        
        Ok(())
    }

    /// Delete a session from Redis
    pub fn delete_session(&self, session_id: &str) -> Result<(), AuthError> {
        let mut con = self.client.get_connection()
            .map_err(|e| AuthError::DatabaseError(format!("Failed to get Redis connection: {}", e)))?;
        
        con.del(session_id)
            .map_err(|e| AuthError::DatabaseError(format!("Failed to delete session from Redis: {}", e)))?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::User;
    use crate::models::Role;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_redis_session_store() {
        // This test requires a running Redis instance
        // For now, we'll just test that the code compiles
        // In a real implementation, you would run integration tests with a Redis instance
        
        /*
        let store = RedisSessionStore::new("redis://127.0.0.1/").unwrap();
        let user = User::new("test@example.com".to_string(), "hashed_password".to_string());
        let session = Session::new(user.id, "test-device".to_string());
        
        // Save session
        store.save_session(&session).unwrap();
        
        // Get session
        let retrieved = store.get_session(&session.id.to_string()).unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.id, session.id);
        assert_eq!(retrieved.user_id, session.user_id);
        
        // Delete session
        store.delete_session(&session.id.to_string()).unwrap();
        
        // Verify deletion
        let retrieved = store.get_session(&session.id.to_string()).unwrap();
        assert!(retrieved.is_none());
        */
    }
}
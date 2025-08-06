//! Session management utilities

use crate::{CacheManager, RedisError, RedisResult};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Session manager for Redis-based session storage
pub struct SessionManager {
    cache: CacheManager,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new(cache: CacheManager) -> Self {
        Self { cache }
    }

    /// Create a new session with the given data and TTL
    pub async fn create_session<T: Serialize>(&self, data: &T, ttl: Duration) -> RedisResult<String> {
        let session_id = uuid::Uuid::new_v4().to_string();
        self.cache.set(&session_id, data, Some(ttl)).await?;
        Ok(session_id)
    }

    /// Get session data by session ID
    pub async fn get_session<T: for<'de> Deserialize<'de>>(&self, session_id: &str) -> RedisResult<Option<T>> {
        // Check if session exists
        if !self.cache.exists(session_id).await? {
            return Err(RedisError::SessionExpired);
        }
        
        // Get session data
        self.cache.get(session_id).await
    }

    /// Extend session TTL
    pub async fn extend_session(&self, session_id: &str, ttl: Duration) -> RedisResult<()> {
        // Check if session exists
        if !self.cache.exists(session_id).await? {
            return Err(RedisError::SessionExpired);
        }
        
        // Extend session TTL
        self.cache.expire(session_id, ttl).await
    }

    /// Destroy a session
    pub async fn destroy_session(&self, session_id: &str) -> RedisResult<()> {
        self.cache.delete(session_id).await
    }

    /// Refresh session (get data and extend TTL)
    pub async fn refresh_session<T: for<'de> Deserialize<'de> + Serialize>(
        &self, 
        session_id: &str, 
        ttl: Duration
    ) -> RedisResult<Option<T>> {
        // Get session data
        let data = self.get_session(session_id).await?;
        
        // Extend session TTL if data exists
        if data.is_some() {
            self.extend_session(session_id, ttl).await?;
        }
        
        Ok(data)
    }
}
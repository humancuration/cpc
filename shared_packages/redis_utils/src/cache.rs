//! Caching utilities with automatic expiration

use crate::{RedisManager, RedisError, RedisResult};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Cache manager for Redis-based caching
pub struct CacheManager {
    redis: RedisManager,
    default_ttl: Duration,
}

impl CacheManager {
    /// Create a new cache manager
    pub fn new(redis: RedisManager, default_ttl: Duration) -> Self {
        Self { redis, default_ttl }
    }

    /// Set a value in the cache with an optional TTL
    pub async fn set<T: Serialize>(&self, key: &str, value: &T, ttl: Option<Duration>) -> RedisResult<()> {
        let serialized = serde_json::to_string(value)?;
        let ttl = ttl.unwrap_or(self.default_ttl);
        
        let mut conn = self.redis.get_connection().await?;
        let _: () = conn.set_ex(key, serialized, ttl.as_secs() as usize).await?;
        
        Ok(())
    }

    /// Set a value in the cache with MessagePack serialization
    pub async fn set_msgpack<T: Serialize>(&self, key: &str, value: &T, ttl: Option<Duration>) -> RedisResult<()> {
        let serialized = rmp_serde::to_vec(value)?;
        let ttl = ttl.unwrap_or(self.default_ttl);
        
        let mut conn = self.redis.get_connection().await?;
        let _: () = conn.set_ex(key, serialized, ttl.as_secs() as usize).await?;
        
        Ok(())
    }

    /// Get a value from the cache
    pub async fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> RedisResult<Option<T>> {
        let mut conn = self.redis.get_connection().await?;
        let result: Option<String> = conn.get(key).await?;
        
        match result {
            Some(data) => {
                let value = serde_json::from_str(&data)?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    /// Get a value from the cache with MessagePack deserialization
    pub async fn get_msgpack<T: for<'de> Deserialize<'de>>(&self, key: &str) -> RedisResult<Option<T>> {
        let mut conn = self.redis.get_connection().await?;
        let result: Option<Vec<u8>> = conn.get(key).await?;
        
        match result {
            Some(data) => {
                let value = rmp_serde::from_slice(&data)?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    /// Delete a value from the cache
    pub async fn delete(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.redis.get_connection().await?;
        let _: () = conn.del(key).await?;
        
        Ok(())
    }

    /// Check if a key exists in the cache
    pub async fn exists(&self, key: &str) -> RedisResult<bool> {
        let mut conn = self.redis.get_connection().await?;
        let result: bool = conn.exists(key).await?;
        
        Ok(result)
    }

    /// Set expiration time for a key
    pub async fn expire(&self, key: &str, ttl: Duration) -> RedisResult<()> {
        let mut conn = self.redis.get_connection().await?;
        let _: () = conn.expire(key, ttl.as_secs() as usize).await?;
        
        Ok(())
    }
}
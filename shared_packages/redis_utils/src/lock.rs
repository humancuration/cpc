//! Distributed locking utilities

use crate::{RedisManager, RedisError, RedisResult};
use std::time::Duration;
use tokio::time::{sleep, timeout};

/// Distributed lock implementation
pub struct DistributedLock {
    redis: RedisManager,
    key: String,
    ttl: Duration,
}

/// Lock guard that represents an acquired lock
pub struct LockGuard {
    redis: RedisManager,
    key: String,
    value: String,
}

impl DistributedLock {
    /// Create a new distributed lock
    pub fn new(redis: RedisManager, key: String, ttl: Duration) -> Self {
        Self { redis, key, ttl }
    }

    /// Acquire the lock with a timeout
    pub async fn acquire(&self) -> RedisResult<LockGuard> {
        let value = uuid::Uuid::new_v4().to_string();
        let timeout_duration = Duration::from_secs(10); // 10 second timeout
        
        match timeout(timeout_duration, self.try_acquire_lock(&value)).await {
            Ok(Ok(())) => Ok(LockGuard {
                redis: self.redis.clone(),
                key: self.key.clone(),
                value,
            }),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(RedisError::LockTimeout),
        }
    }

    /// Try to acquire the lock
    async fn try_acquire_lock(&self, value: &str) -> RedisResult<()> {
        let mut conn = self.redis.get_connection().await?;
        
        loop {
            // Try to set the lock key with NX (only if not exists) and EX (expire time)
            let result: redis::RedisResult<bool> = redis::cmd("SET")
                .arg(&self.key)
                .arg(value)
                .arg("NX")
                .arg("EX")
                .arg(self.ttl.as_secs())
                .query_async(&mut *conn)
                .await;
            
            match result {
                Ok(true) => return Ok(()), // Lock acquired
                Ok(false) => {
                    // Lock not acquired, wait a bit and try again
                    sleep(Duration::from_millis(100)).await;
                }
                Err(e) => return Err(RedisError::Redis(e)),
            }
        }
    }

    /// Release the lock
    pub async fn release(&self, guard: LockGuard) -> RedisResult<()> {
        // Use a Lua script to atomically check and delete the lock
        let script = redis::Script::new(
            r#"
            if redis.call("GET", KEYS[1]) == ARGV[1] then
                return redis.call("DEL", KEYS[1])
            else
                return 0
            end
            "#,
        );
        
        let mut conn = self.redis.get_connection().await?;
        let result: redis::RedisResult<i32> = script
            .key(&self.key)
            .arg(&guard.value)
            .invoke_async(&mut *conn)
            .await;
        
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(RedisError::Redis(e)),
        }
    }

    /// Extend the lock TTL
    pub async fn extend(&self, guard: &LockGuard, additional_time: Duration) -> RedisResult<()> {
        // Use a Lua script to atomically check and extend the lock
        let script = redis::Script::new(
            r#"
            if redis.call("GET", KEYS[1]) == ARGV[1] then
                return redis.call("EXPIRE", KEYS[1], ARGV[2])
            else
                return 0
            end
            "#,
        );
        
        let mut conn = self.redis.get_connection().await?;
        let result: redis::RedisResult<i32> = script
            .key(&self.key)
            .arg(&guard.value)
            .arg(additional_time.as_secs())
            .invoke_async(&mut *conn)
            .await;
        
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(RedisError::Redis(e)),
        }
    }
}
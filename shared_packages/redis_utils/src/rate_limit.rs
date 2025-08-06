//! Rate limiting utilities

use crate::{RedisManager, RedisError, RedisResult};
use std::time::Duration;

/// Rate limit result
#[derive(Debug, Clone)]
pub struct RateLimitResult {
    /// Whether the request is allowed
    pub allowed: bool,
    
    /// Number of requests remaining in the current window
    pub remaining: u64,
    
    /// Time until the rate limit resets
    pub reset_time: Duration,
}

/// Rate limiter implementation
pub struct RateLimiter {
    redis: RedisManager,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(redis: RedisManager) -> Self {
        Self { redis }
    }

    /// Check rate limit using token bucket algorithm
    pub async fn check_token_bucket(
        &self,
        key: &str,
        capacity: u64,
        refill_rate: u64,
    ) -> RedisResult<RateLimitResult> {
        let script = redis::Script::new(
            r#"
            local key = KEYS[1]
            local capacity = tonumber(ARGV[1])
            local refill_rate = tonumber(ARGV[2])
            local now = tonumber(ARGV[3])
            
            local bucket = redis.call("HMGET", key, "tokens", "last_refill")
            local tokens = tonumber(bucket[1]) or capacity
            local last_refill = tonumber(bucket[2]) or now
            
            -- Refill tokens
            local time_passed = now - last_refill
            local new_tokens = math.min(capacity, tokens + time_passed * refill_rate)
            
            if new_tokens >= 1 then
                -- Allow request and consume token
                new_tokens = new_tokens - 1
                redis.call("HMSET", key, "tokens", new_tokens, "last_refill", now)
                redis.call("EXPIRE", key, 86400) -- Expire after 24 hours
                return {1, new_tokens, 0}
            else
                -- Reject request
                redis.call("HMSET", key, "tokens", new_tokens, "last_refill", now)
                redis.call("EXPIRE", key, 86400) -- Expire after 24 hours
                return {0, new_tokens, math.ceil(1 / refill_rate)}
            end
            "#,
        );
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| RedisError::InvalidConfiguration(e.to_string()))?
            .as_secs();
        
        let mut conn = self.redis.get_connection().await?;
        let result: redis::RedisResult<Vec<i64>> = script
            .key(key)
            .arg(capacity)
            .arg(refill_rate)
            .arg(now)
            .invoke_async(&mut *conn)
            .await;
        
        match result {
            Ok(values) => {
                let allowed = values[0] == 1;
                let remaining = values[1] as u64;
                let reset_time = Duration::from_secs(values[2] as u64);
                
                Ok(RateLimitResult {
                    allowed,
                    remaining,
                    reset_time,
                })
            }
            Err(e) => Err(RedisError::Redis(e)),
        }
    }

    /// Check rate limit using sliding window algorithm
    pub async fn check_sliding_window(
        &self,
        key: &str,
        max_requests: u64,
        window_size: Duration,
    ) -> RedisResult<RateLimitResult> {
        let script = redis::Script::new(
            r#"
            local key = KEYS[1]
            local max_requests = tonumber(ARGV[1])
            local window_size = tonumber(ARGV[2])
            local now = tonumber(ARGV[3])
            local window_start = now - window_size
            
            -- Remove old requests outside the window
            redis.call("ZREMRANGEBYSCORE", key, 0, window_start)
            
            -- Count requests in current window
            local current_requests = redis.call("ZCARD", key)
            
            if current_requests < max_requests then
                -- Allow request and add timestamp
                redis.call("ZADD", key, now, now)
                redis.call("EXPIRE", key, window_size * 2) -- Expire after 2 windows
                return {1, max_requests - current_requests - 1, 0}
            else
                -- Reject request
                local oldest = redis.call("ZRANGE", key, 0, 0, "WITHSCORES")
                local reset_time = window_size - (now - tonumber(oldest[2]))
                return {0, 0, reset_time}
            end
            "#,
        );
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| RedisError::InvalidConfiguration(e.to_string()))?
            .as_secs();
        
        let window_size_secs = window_size.as_secs();
        
        let mut conn = self.redis.get_connection().await?;
        let result: redis::RedisResult<Vec<i64>> = script
            .key(key)
            .arg(max_requests)
            .arg(window_size_secs)
            .arg(now)
            .invoke_async(&mut *conn)
            .await;
        
        match result {
            Ok(values) => {
                let allowed = values[0] == 1;
                let remaining = values[1] as u64;
                let reset_time = Duration::from_secs(values[2] as u64);
                
                Ok(RateLimitResult {
                    allowed,
                    remaining,
                    reset_time,
                })
            }
            Err(e) => Err(RedisError::Redis(e)),
        }
    }
}
//! Redis configuration utilities

use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

/// Redis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub pool_max_size: u32,
    pub connection_timeout: Duration,
    pub retry_attempts: u32,
    pub retry_delay: Duration,
}

impl RedisConfig {
    /// Create a new Redis configuration
    pub fn new(
        url: String,
        pool_max_size: u32,
        connection_timeout: Duration,
        retry_attempts: u32,
        retry_delay: Duration,
    ) -> Self {
        Self {
            url,
            pool_max_size,
            connection_timeout,
            retry_attempts,
            retry_delay,
        }
    }

    /// Create a Redis configuration from environment variables
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string()),
            pool_max_size: env::var("REDIS_POOL_MAX_SIZE")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
            connection_timeout: Duration::from_secs(
                env::var("REDIS_CONNECTION_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()?,
            ),
            retry_attempts: env::var("REDIS_RETRY_ATTEMPTS")
                .unwrap_or_else(|_| "3".to_string())
                .parse()?,
            retry_delay: Duration::from_secs(
                env::var("REDIS_RETRY_DELAY")
                    .unwrap_or_else(|_| "1".to_string())
                    .parse()?,
            ),
        })
    }

    /// Get the Redis URL
    pub fn url(&self) -> &str {
        &self.url
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://127.0.0.1/".to_string(),
            pool_max_size: 10,
            connection_timeout: Duration::from_secs(30),
            retry_attempts: 3,
            retry_delay: Duration::from_secs(1),
        }
    }
}
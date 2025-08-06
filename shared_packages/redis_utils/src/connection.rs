//! Redis connection management with bb8 pooling

use crate::{RedisConfig, RedisError, RedisResult};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;
use std::time::Instant;

/// Redis manager for connection pooling
#[derive(Clone)]
pub struct RedisManager {
    pool: Pool<RedisConnectionManager>,
    url: String,
}

impl RedisManager {
    /// Create a new Redis manager
    pub async fn new(config: &RedisConfig) -> RedisResult<Self> {
        let manager = RedisConnectionManager::new(config.url())?;
        let pool = Pool::builder()
            .max_size(config.pool_max_size)
            .connection_timeout(config.connection_timeout)
            .build(manager)
            .await
            .map_err(RedisError::Connection)?;
        
        Ok(Self { 
            pool,
            url: config.url().to_string(),
        })
    }

    /// Get a connection from the pool
    pub async fn get_connection(&self) -> RedisResult<bb8::PooledConnection<'_, RedisConnectionManager>> {
        let conn = self.pool.get().await?;
        Ok(conn)
    }

    /// Get the Redis URL
    pub fn get_url(&self) -> &str {
        &self.url
    }

    /// Check the health of the Redis connection
    pub async fn health_check(&self) -> RedisResult<HealthStatus> {
        let start = Instant::now();
        
        match self.pool.get().await {
            Ok(mut conn) => {
                // Execute a simple ping command to test the connection
                let result: redis::RedisResult<String> = redis::cmd("PING").query_async(&mut *conn).await;
                
                match result {
                    Ok(response) if response == "PONG" => {
                        let duration = start.elapsed();
                        if duration.as_millis() > 1000 {
                            Ok(HealthStatus::Degraded(format!("Slow response: {}ms", duration.as_millis())))
                        } else {
                            Ok(HealthStatus::Healthy)
                        }
                    }
                    Ok(_) => Ok(HealthStatus::Degraded("Unexpected PING response".to_string())),
                    Err(e) => Ok(HealthStatus::Unhealthy(format!("PING failed: {}", e))),
                }
            }
            Err(e) => Ok(HealthStatus::Unhealthy(format!("Connection failed: {}", e))),
        }
    }
}

/// Health status enum
#[derive(Debug, Clone)]
pub enum HealthStatus {
    Healthy,
    Degraded(String),
    Unhealthy(String),
}
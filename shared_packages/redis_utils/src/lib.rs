//! Enhanced Redis utilities package
//!
//! This module provides enhanced Redis functionality including connection management,
//! serialization, caching, distributed locking, pub/sub, rate limiting, and session management.

pub mod config;
pub mod connection;
pub mod cache;
pub mod lock;
pub mod pubsub;
pub mod rate_limit;
pub mod session;

pub use config::RedisConfig;
pub use connection::RedisManager;
pub use cache::CacheManager;
pub use lock::DistributedLock;
pub use pubsub::PubSubManager;
pub use rate_limit::RateLimiter;
pub use session::SessionManager;

use thiserror::Error;
use std::time::Duration;

/// Unified error type for Redis operations
#[derive(Debug, Error)]
pub enum RedisError {
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    
    #[error("Connection error: {0}")]
    Connection(#[from] bb8::RunError<redis::RedisError>),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("MessagePack serialization error: {0}")]
    MessagePackSerialization(#[from] rmp_serde::encode::Error),
    
    #[error("MessagePack deserialization error: {0}")]
    MessagePackDeserialization(#[from] rmp_serde::decode::Error),
    
    #[error("Lock acquisition timeout")]
    LockTimeout,
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Session expired")]
    SessionExpired,
    
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
}

/// Result type for Redis operations
pub type RedisResult<T> = Result<T, RedisError>;
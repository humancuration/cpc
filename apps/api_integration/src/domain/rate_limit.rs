//! Rate limit domain entities for the API & Integration Hub module

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use thiserror::Error;

/// Error types for rate limit operations
#[derive(Error, Debug)]
pub enum RateLimitError {
    #[error("Invalid rate limit data: {0}")]
    InvalidData(String),
    
    #[error("Rate limit rule not found: {0}")]
    NotFound(String),
    
    #[error("Rate limit exceeded: {0}")]
    Exceeded(String),
}

/// Rate limiting rule
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RateLimitRule {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    /// Maximum number of requests allowed per time window
    pub requests_per_window: u32,
    /// Time window in seconds
    pub window_seconds: u32,
    /// Maximum burst requests allowed
    pub burst_limit: u32,
    /// Whether the rate limit is enabled
    pub enabled: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// Owner of the rate limit rule
    pub owner_id: Uuid,
}

impl RateLimitRule {
    /// Create a new rate limit rule
    pub fn new(
        name: String,
        requests_per_window: u32,
        window_seconds: u32,
        burst_limit: u32,
        owner_id: Uuid,
        description: Option<String>,
    ) -> Result<Self, RateLimitError> {
        if name.is_empty() {
            return Err(RateLimitError::InvalidData("Rate limit rule name cannot be empty".to_string()));
        }
        
        if requests_per_window == 0 {
            return Err(RateLimitError::InvalidData("Requests per window must be greater than 0".to_string
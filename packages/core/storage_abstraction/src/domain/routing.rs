//! Routing logic for storage operations
//! 
//! This module contains the logic for determining which storage backend to use for a given operation.

use serde::{Deserialize, Serialize};

/// Location where data should be stored
#[derive(Debug, Clone, PartialEq)]
pub enum StorageLocation {
    /// Edge storage (Sled)
    Edge,
    /// Cloud storage (PostgreSQL)
    Cloud,
}

/// Configuration for storage routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingConfig {
    /// Default location for data storage
    pub default_location: StorageLocation,
    /// Patterns that should be stored in edge storage
    pub edge_patterns: Vec<String>,
    /// Patterns that should be stored in cloud storage
    pub cloud_patterns: Vec<String>,
}

impl Default for RoutingConfig {
    fn default() -> Self {
        Self {
            default_location: StorageLocation::Cloud,
            edge_patterns: vec![
                "temp:*".to_string(),
                "cache:*".to_string(),
                "session:*".to_string(),
            ],
            cloud_patterns: vec![
                "user:*".to_string(),
                "consent:*".to_string(),
                "finance:*".to_string(),
                "health:*".to_string(),
            ],
        }
    }
}

/// Router for determining storage location
pub struct StorageRouter {
    config: RoutingConfig,
}

impl StorageRouter {
    /// Create a new storage router
    pub fn new(config: RoutingConfig) -> Self {
        Self { config }
    }
    
    /// Determine where to read data from
    pub fn route_read(&self, key: &str) -> StorageLocation {
        // Check for specific patterns
        for pattern in &self.config.edge_patterns {
            if Self::matches_pattern(key, pattern) {
                return StorageLocation::Edge;
            }
        }
        
        for pattern in &self.config.cloud_patterns {
            if Self::matches_pattern(key, pattern) {
                return StorageLocation::Cloud;
            }
        }
        
        // Default location
        self.config.default_location.clone()
    }
    
    /// Determine where to write data to
    pub fn route_write(&self, key: &str) -> StorageLocation {
        // For writes, we might have different logic than reads
        // For now, we'll use the same logic
        self.route_read(key)
    }
    
    /// Check if a key matches a pattern
    fn matches_pattern(key: &str, pattern: &str) -> bool {
        if pattern.ends_with("*") {
            let prefix = &pattern[..pattern.len() - 1];
            key.starts_with(prefix)
        } else {
            key == pattern
        }
    }
}
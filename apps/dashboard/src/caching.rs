//! Caching layer for visualization data
//!
//! This module implements a caching layer for visualization data using Sled
//! as specified in the visualization architecture documentation.

use sled::Db;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Visualization cache implementation
pub struct VisualizationCache {
    /// Sled database instance
    db: Db,
    /// Default TTL in seconds
    default_ttl: u64,
}

impl VisualizationCache {
    /// Create a new cache instance
    pub fn new(db_path: &str, default_ttl: u64) -> Result<Self, sled::Error> {
        let db = sled::open(db_path)?;
        Ok(Self { db, default_ttl })
    }
    
    /// Generate a cache key using SHA-256
    pub fn generate_key(&self, visualization_id: Uuid, width: Option<u32>, height: Option<u32>, lod_level: Option<u8>) -> String {
        let mut hasher = Sha256::new();
        hasher.update(visualization_id.to_string());
        hasher.update(width.unwrap_or(0).to_string());
        hasher.update(height.unwrap_or(0).to_string());
        hasher.update(lod_level.unwrap_or(0).to_string());
        format!("{:x}", hasher.finalize())
    }
    
    /// Store visualization data in cache
    pub fn store(&self, key: &str, data: &CacheEntry) -> Result<(), sled::Error> {
        let serialized = serde_json::to_vec(data)?;
        self.db.insert(key, serialized)?;
        Ok(())
    }
    
    /// Retrieve visualization data from cache
    pub fn retrieve(&self, key: &str) -> Result<Option<CacheEntry>, sled::Error> {
        match self.db.get(key)? {
            Some(data) => {
                let entry: CacheEntry = serde_json::from_slice(&data)?;
                // Check if entry is still valid
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                
                if entry.expires_at > now {
                    Ok(Some(entry))
                } else {
                    // Remove expired entry
                    self.db.remove(key)?;
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
    
    /// Remove an entry from cache
    pub fn remove(&self, key: &str) -> Result<(), sled::Error> {
        self.db.remove(key)?;
        Ok(())
    }
    
    /// Clear all entries from cache
    pub fn clear(&self) -> Result<(), sled::Error> {
        self.db.clear()?;
        Ok(())
    }
}

/// Cached visualization entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Cached data
    pub data: serde_json::Value,
    /// Expiration timestamp
    pub expires_at: u64,
    /// Entry creation timestamp
    pub created_at: u64,
    /// Cache TTL in seconds
    pub ttl: u64,
}

impl CacheEntry {
    /// Create a new cache entry
    pub fn new(data: serde_json::Value, ttl: u64) -> Result<Self, Box<dyn std::error::Error>> {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();
        
        Ok(Self {
            data,
            expires_at: created_at + ttl,
            created_at,
            ttl,
        })
    }
    
    /// Check if entry is expired
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.expires_at <= now
    }
}

/// TTL configuration for different visualization types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtlConfig {
    /// TTL for static images in seconds
    pub static_image: u64,
    /// TTL for 3D scene data in seconds
    pub scene_3d: u64,
    /// TTL for real-time data in seconds
    pub real_time: u64,
}

impl Default for TtlConfig {
    fn default() -> Self {
        Self {
            static_image: 300,  // 5 minutes
            scene_3d: 3600,     // 1 hour
            real_time: 30,      // 30 seconds
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cache_key_generation() {
        let cache = VisualizationCache::new("test_db", 300).unwrap();
        let uuid = Uuid::new_v4();
        let key = cache.generate_key(uuid, Some(800), Some(600), Some(2));
        assert_eq!(key.len(), 64); // SHA-256 produces 64 hex characters
    }
    
    #[test]
    fn test_cache_entry_creation() {
        let data = serde_json::json!({"test": "data"});
        let entry = CacheEntry::new(data, 300).unwrap();
        assert!(!entry.is_expired());
    }
    
    #[test]
    fn test_ttl_config_default() {
        let config = TtlConfig::default();
        assert_eq!(config.static_image, 300);
        assert_eq!(config.scene_3d, 3600);
        assert_eq!(config.real_time, 30);
    }
}
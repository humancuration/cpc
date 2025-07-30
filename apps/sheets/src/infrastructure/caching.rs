//! Multi-tier caching implementation for visualization data
//!
//! This module implements a caching layer for visualization data using both
//! Sled (edge cache) and Redis (regional cache) as specified in the
//! visualization architecture documentation.

use sled::Db;
use redis::{Client, Commands, RedisResult};
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use crate::application::collaboration_service::SheetEvent;

/// Multi-tier cache implementation
pub struct VisualizationCache {
    /// Sled database instance (edge cache)
    sled_db: Db,
    /// Redis client (regional cache)
    redis_client: Client,
    /// Default TTL configuration
    ttl_config: TtlConfig,
}

impl VisualizationCache {
    /// Create a new cache instance
    pub fn new(sled_db_path: &str, redis_url: &str, ttl_config: TtlConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let sled_db = sled::open(sled_db_path)?;
        let redis_client = Client::open(redis_url)?;
        Ok(Self { sled_db, redis_client, ttl_config })
    }
    
    /// Generate a cache key using SHA-256
    pub fn generate_key(&self, visualization_id: Uuid, params: &CacheKeyParams, version: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(version);
        hasher.update(visualization_id.to_string());
        hasher.update(params.width.unwrap_or(0).to_string());
        hasher.update(params.height.unwrap_or(0).to_string());
        hasher.update(params.lod_level.unwrap_or(0).to_string());
        hasher.update(params.accessibility_mode.clone().unwrap_or("standard".to_string()));
        format!("{:x}", hasher.finalize())
    }
    
    /// Get TTL based on visualization type
    pub fn get_ttl(&self, visualization_type: &VisualizationType) -> u64 {
        match visualization_type {
            VisualizationType::StaticImage => self.ttl_config.static_image,
            VisualizationType::Scene3D => self.ttl_config.scene_3d,
            VisualizationType::RealTime => self.ttl_config.real_time,
        }
    }
    
    /// Store visualization data in cache
    pub fn store(
        &self,
        key: &str,
        data: &CacheEntry,
        visualization_type: &VisualizationType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ttl = self.get_ttl(visualization_type);
        
        // Store in Sled (edge cache)
        let serialized = serde_json::to_vec(data)?;
        self.sled_db.insert(key, serialized)?;
        
        // Store in Redis (regional cache) with TTL
        let mut conn = self.redis_client.get_connection()?;
        let serialized_str = serde_json::to_string(data)?;
        conn.set_ex(key, serialized_str, ttl as usize)?;
        
        Ok(())
    }
    
    /// Retrieve visualization data from cache
    pub fn retrieve(&self, key: &str) -> Result<Option<CacheEntry>, Box<dyn std::error::Error>> {
        // Try to get from Sled first (faster)
        if let Some(entry) = self.retrieve_from_sled(key)? {
            return Ok(Some(entry));
        }
        
        // If not in Sled, try Redis
        if let Some(entry) = self.retrieve_from_redis(key)? {
            // Store in Sled for future requests
            let serialized = serde_json::to_vec(&entry)?;
            self.sled_db.insert(key, serialized)?;
            return Ok(Some(entry));
        }
        
        Ok(None)
    }
    
    /// Retrieve from Sled cache
    fn retrieve_from_sled(&self, key: &str) -> Result<Option<CacheEntry>, sled::Error> {
        match self.sled_db.get(key)? {
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
                    self.sled_db.remove(key)?;
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
    
    /// Retrieve from Redis cache
    fn retrieve_from_redis(&self, key: &str) -> Result<Option<CacheEntry>, Box<dyn std::error::Error>> {
        let mut conn = self.redis_client.get_connection()?;
        let result: RedisResult<String> = conn.get(key);
        
        match result {
            Ok(data) => {
                let entry: CacheEntry = serde_json::from_str(&data)?;
                // Check if entry is still valid
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                
                if entry.expires_at > now {
                    Ok(Some(entry))
                } else {
                    // Remove expired entry
                    let _: RedisResult<()> = conn.del(key);
                    Ok(None)
                }
            }
            Err(_) => Ok(None),
        }
    }
    
    /// Remove an entry from cache
    pub fn remove(&self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.sled_db.remove(key)?;
        let mut conn = self.redis_client.get_connection()?;
        let _: RedisResult<()> = conn.del(key);
        Ok(())
    }
    
    /// Clear all entries from cache
    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.sled_db.clear()?;
        let mut conn = self.redis_client.get_connection()?;
        let _: RedisResult<()> = redis::cmd("FLUSHDB").query(&mut conn);
        Ok(())
    /// Handle SheetEvent for cache invalidation
    pub fn handle_sheet_event(&self, event: &SheetEvent) -> Result<(), Box<dyn std::error::Error>> {
        if let SheetEvent::CellUpdated { sheet_id, cache_version, .. } = event {
            // In a real implementation, we would need to generate cache keys for all
            // visualizations affected by this cell update and remove them from cache
            // This is a simplified example that just logs the invalidation
            println!("Invalidating cache for sheet {} with version {}", sheet_id, cache_version);
        }
        Ok(())
    }
}
    }
}

/// Parameters for cache key generation
#[derive(Debug, Clone)]
pub struct CacheKeyParams {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub lod_level: Option<u8>,
    pub accessibility_mode: Option<String>,
    pub cache_version: Option<String>,
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

/// Visualization types for TTL configuration
#[derive(Debug, Clone)]
pub enum VisualizationType {
    StaticImage,
    Scene3D,
    RealTime,
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
        let cache = VisualizationCache::new("test_db", "redis://127.0.0.1/", TtlConfig::default()).unwrap();
        let uuid = Uuid::new_v4();
        let params = CacheKeyParams {
            width: Some(800),
            height: Some(600),
            lod_level: Some(2),
            accessibility_mode: Some("screen-reader".to_string()),
            cache_version: Some("v1.0".to_string()),
        };
        let key = cache.generate_key(uuid, &params, "v1.0");
        assert_eq!(key.len(), 64); // SHA-256 produces 64 hex characters
    }
    
    #[test]
    fn test_ttl_config_default() {
        let config = TtlConfig::default();
        assert_eq!(config.static_image, 300);
        assert_eq!(config.scene_3d, 3600);
        assert_eq!(config.real_time, 30);
    }
    
    #[test]
    fn test_visualization_type_ttl() {
        let cache = VisualizationCache::new("test_db", "redis://127.0.0.1/", TtlConfig::default()).unwrap();
        assert_eq!(cache.get_ttl(&VisualizationType::StaticImage), 300);
        assert_eq!(cache.get_ttl(&VisualizationType::Scene3D), 3600);
        assert_eq!(cache.get_ttl(&VisualizationType::RealTime), 30);
    }
}
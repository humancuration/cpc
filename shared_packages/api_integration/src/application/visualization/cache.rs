//! Caching implementation for visualization responses

use sled::Db;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Sled database error: {0}")]
    DatabaseError(#[from] sled::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Cache entry expired")]
    EntryExpired,
}

/// Cache entry with TTL support
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheEntry {
    /// The actual cached data
    pub data: Vec<u8>,
    /// Expiration timestamp (Unix timestamp in seconds)
    pub expires_at: u64,
}

/// Visualization cache implementation using Sled
pub struct VisualizationCache {
    db: Db,
}

impl VisualizationCache {
    /// Create a new cache instance
    pub fn new(path: &str) -> Result<Self, CacheError> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }
    
    /// Get a cached visualization response
    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>, CacheError> {
        if let Some(bytes) = self.db.get(key)? {
            let entry: CacheEntry = serde_json::from_slice(&bytes)?;
            
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
                
            if entry.expires_at > now {
                Ok(Some(entry.data))
            } else {
                // Entry expired, remove it
                let _ = self.db.remove(key);
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    
    /// Cache a visualization response
    pub fn set(&self, key: &str, value: Vec<u8>, ttl: Duration) -> Result<(), CacheError> {
        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + ttl.as_secs();
            
        let entry = CacheEntry {
            data: value,
            expires_at,
        };
        
        let bytes = serde_json::to_vec(&entry)?;
        self.db.insert(key, bytes)?;
        self.db.flush()?;
        
        Ok(())
    }
    
    /// Remove a cached entry
    pub fn remove(&self, key: &str) -> Result<(), CacheError> {
        self.db.remove(key)?;
        Ok(())
    }
    
    /// Clear all cached entries
    pub fn clear(&self) -> Result<(), CacheError> {
        self.db.clear()?;
        Ok(())
    }
    
    /// Generate cache key for a visualization request
    pub fn generate_key(
        report_id: Uuid,
        width: u32,
        height: u32,
        lod_level: u8,
        accessibility_mode: &str,
        originating_app: &str,
    ) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        report_id.hash(&mut hasher);
        width.hash(&mut hasher);
        height.hash(&mut hasher);
        lod_level.hash(&mut hasher);
        accessibility_mode.hash(&mut hasher);
        originating_app.hash(&mut hasher);
        
        let hash = hasher.finish();
        format!("viz:{}:{}", report_id, hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_cache_operations() {
        let temp_dir = tempdir().unwrap();
        let cache = VisualizationCache::new(temp_dir.path().to_str().unwrap()).unwrap();
        
        let key = "test-key";
        let data = b"test-data".to_vec();
        
        // Test set and get
        cache.set(key, data.clone(), Duration::from_secs(60)).unwrap();
        let retrieved = cache.get(key).unwrap();
        assert_eq!(retrieved, Some(data));
        
        // Test expiration
        cache.set(key, data.clone(), Duration::from_nanos(1)).unwrap();
        std::thread::sleep(Duration::from_millis(10));
        let expired = cache.get(key).unwrap();
        assert_eq!(expired, None);
        
        // Test remove
        cache.set(key, data.clone(), Duration::from_secs(60)).unwrap();
        cache.remove(key).unwrap();
        let removed = cache.get(key).unwrap();
        assert_eq!(removed, None);
    }

    #[test]
    fn test_generate_cache_key() {
        let report_id = Uuid::new_v4();
        let key = VisualizationCache::generate_key(
            report_id,
            800,
            600,
            2,
            "standard",
            "dashboard",
        );
        
        assert!(key.starts_with("viz:"));
        assert!(key.contains(&report_id.to_string()));
    }
}
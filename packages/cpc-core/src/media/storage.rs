use crate::media::types::*;
use anyhow::Result;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Media storage interface for different storage backends
#[async_trait::async_trait]
pub trait MediaStorage: Send + Sync {
    /// Store media file and return storage path/identifier
    async fn store(&self, data: &[u8], filename: &str) -> Result<String>;
    
    /// Retrieve media file by storage identifier
    async fn retrieve(&self, storage_id: &str) -> Result<Vec<u8>>;
    
    /// Delete media file by storage identifier
    async fn delete(&self, storage_id: &str) -> Result<()>;
    
    /// Check if media file exists
    async fn exists(&self, storage_id: &str) -> Result<bool>;
    
    /// Get metadata for stored media
    async fn get_metadata(&self, storage_id: &str) -> Result<Option<MediaMetadata>>;
}

/// Local filesystem storage implementation
pub struct LocalStorage {
    base_path: PathBuf,
}

impl LocalStorage {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    pub async fn initialize(&self) -> Result<()> {
        tokio::fs::create_dir_all(&self.base_path).await?;
        Ok(())
    }

    fn get_file_path(&self, storage_id: &str) -> PathBuf {
        self.base_path.join(storage_id)
    }
}

#[async_trait::async_trait]
impl MediaStorage for LocalStorage {
    async fn store(&self, data: &[u8], filename: &str) -> Result<String> {
        let storage_id = format!("{}_{}", Uuid::new_v4(), filename);
        let file_path = self.get_file_path(&storage_id);
        
        // Ensure parent directory exists
        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        tokio::fs::write(&file_path, data).await?;
        
        Ok(storage_id)
    }

    async fn retrieve(&self, storage_id: &str) -> Result<Vec<u8>> {
        let file_path = self.get_file_path(storage_id);
        let data = tokio::fs::read(&file_path).await?;
        Ok(data)
    }

    async fn delete(&self, storage_id: &str) -> Result<()> {
        let file_path = self.get_file_path(storage_id);
        if file_path.exists() {
            tokio::fs::remove_file(&file_path).await?;
        }
        Ok(())
    }

    async fn exists(&self, storage_id: &str) -> Result<bool> {
        let file_path = self.get_file_path(storage_id);
        Ok(file_path.exists())
    }

    async fn get_metadata(&self, storage_id: &str) -> Result<Option<MediaMetadata>> {
        let file_path = self.get_file_path(storage_id);
        
        if !file_path.exists() {
            return Ok(None);
        }
        
        let metadata = tokio::fs::metadata(&file_path).await?;
        let file_name = file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        // Basic metadata - in a real implementation, this would extract more details
        let media_metadata = MediaMetadata {
            id: Uuid::new_v4(),
            file_name,
            file_size: metadata.len(),
            media_type: MediaType::Image, // TODO: Detect from file
            duration: None,
            width: None,
            height: None,
            frame_rate: None,
            bitrate: None,
            codec: None,
            created_at: chrono::Utc::now(),
            checksum: "".to_string(), // TODO: Calculate checksum
        };
        
        Ok(Some(media_metadata))
    }
}

/// P2P storage implementation using p2panda
pub struct P2PStorage {
    inner: crate::p2p::P2PMediaStorage,
}

impl P2PStorage {
    pub fn new(config: String) -> Self {
        Self {
            inner: crate::p2p::P2PMediaStorage::new(config),
        }
    }

    pub async fn initialize(&self) -> Result<()> {
        self.inner.initialize().await
    }

    pub fn get_stats(&self) -> crate::p2p::ContentStats {
        self.inner.get_stats()
    }

    pub async fn cleanup(&self, max_age_days: u64) -> Result<usize> {
        self.inner.cleanup(max_age_days).await
    }
}

#[async_trait::async_trait]
impl MediaStorage for P2PStorage {
    async fn store(&self, data: &[u8], filename: &str) -> Result<String> {
        self.inner.store(data, filename).await
    }

    async fn retrieve(&self, storage_id: &str) -> Result<Vec<u8>> {
        self.inner.retrieve(storage_id).await
    }

    async fn delete(&self, storage_id: &str) -> Result<()> {
        self.inner.delete(storage_id).await
    }

    async fn exists(&self, storage_id: &str) -> Result<bool> {
        self.inner.exists(storage_id).await
    }

    async fn get_metadata(&self, storage_id: &str) -> Result<Option<MediaMetadata>> {
        self.inner.get_metadata(storage_id).await
    }
}

/// Hybrid storage that combines local and P2P storage
pub struct HybridStorage {
    local: LocalStorage,
    p2p: P2PStorage,
}

impl HybridStorage {
    pub fn new(local_path: PathBuf) -> Self {
        // Default P2P configuration
        let p2p_config = serde_json::json!({
            "bootstrap_node": "127.0.0.1:4001"
        }).to_string();
        
        Self {
            local: LocalStorage::new(local_path),
            p2p: P2PStorage::new(p2p_config),
        }
    }

    pub fn new_with_p2p_config(local_path: PathBuf, p2p_config: String) -> Self {
        Self {
            local: LocalStorage::new(local_path),
            p2p: P2PStorage::new(p2p_config),
        }
    }

    pub async fn initialize(&self) -> Result<()> {
        self.local.initialize().await?;
        self.p2p.initialize().await?;
        Ok(())
    }

    pub fn get_p2p_stats(&self) -> crate::p2p::ContentStats {
        self.p2p.get_stats()
    }

    pub async fn cleanup_p2p(&self, max_age_days: u64) -> Result<usize> {
        self.p2p.cleanup(max_age_days).await
    }
}

#[async_trait::async_trait]
impl MediaStorage for HybridStorage {
    async fn store(&self, data: &[u8], filename: &str) -> Result<String> {
        // Store locally first
        let storage_id = self.local.store(data, filename).await?;
        
        // TODO: Also store in P2P network
        // let p2p_id = self.p2p.store(data, filename).await?;
        
        Ok(storage_id)
    }

    async fn retrieve(&self, storage_id: &str) -> Result<Vec<u8>> {
        // Try local storage first
        match self.local.retrieve(storage_id).await {
            Ok(data) => Ok(data),
            Err(_) => {
                // Fallback to P2P storage
                self.p2p.retrieve(storage_id).await
            }
        }
    }

    async fn delete(&self, storage_id: &str) -> Result<()> {
        // Delete from both storages
        let _ = self.local.delete(storage_id).await;
        let _ = self.p2p.delete(storage_id).await;
        Ok(())
    }

    async fn exists(&self, storage_id: &str) -> Result<bool> {
        // Check both storages
        let local_exists = self.local.exists(storage_id).await.unwrap_or(false);
        let p2p_exists = self.p2p.exists(storage_id).await.unwrap_or(false);
        Ok(local_exists || p2p_exists)
    }

    async fn get_metadata(&self, storage_id: &str) -> Result<Option<MediaMetadata>> {
        // Try local first, then P2P
        if let Ok(Some(metadata)) = self.local.get_metadata(storage_id).await {
            Ok(Some(metadata))
        } else {
            self.p2p.get_metadata(storage_id).await
        }
    }
}
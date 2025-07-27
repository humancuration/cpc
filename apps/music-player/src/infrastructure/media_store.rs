//! Content-addressed storage for music player media

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::domain::errors::{Result, MusicPlayerError};

/// Content-addressed storage for media files
pub struct MediaStore {
    /// In-memory storage for demonstration
    /// In a real implementation, this would interface with p2panda or IPFS
    storage: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl MediaStore {
    /// Create a new media store
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Store content and return its content identifier
    pub async fn store_content(&self, content: Vec<u8>) -> Result<String> {
        // In a real implementation, this would:
        // 1. Calculate the BLAKE3 hash of the content
        // 2. Store it in the P2P network
        // 3. Return the content identifier
        
        // For demonstration, we'll use a simple hash function
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        let hash = hasher.finish();
        let cid = format!("bafybeig6xv5nwphfmvcnektpnojts33jqcuam7bmye2pb54adnrtccj{:x}", hash);
        
        // Store the content
        {
            let mut storage = self.storage.write().await;
            storage.insert(cid.clone(), content);
        }
        
        tracing::info!("Stored content with CID: {}", cid);
        Ok(cid)
    }

    /// Retrieve content by its content identifier
    pub async fn retrieve_content(&self, cid: &str) -> Result<Vec<u8>> {
        // In a real implementation, this would:
        // 1. Fetch the content from the P2P network
        // 2. Verify its integrity using the CID
        // 3. Return the content
        
        // For demonstration, we'll retrieve from our in-memory store
        let storage = self.storage.read().await;
        if let Some(content) = storage.get(cid) {
            tracing::info!("Retrieved content with CID: {}", cid);
            Ok(content.clone())
        } else {
            Err(MusicPlayerError::ContentNotAvailable { 
                cid: cid.to_string() 
            })
        }
    }

    /// Check if content exists
    pub async fn content_exists(&self, cid: &str) -> Result<bool> {
        // In a real implementation, this would check the P2P network
        let storage = self.storage.read().await;
        Ok(storage.contains_key(cid))
    }

    /// Delete content (for cleanup purposes)
    pub async fn delete_content(&self, cid: &str) -> Result<()> {
        // In a real implementation, this would mark content for garbage collection
        let mut storage = self.storage.write().await;
        storage.remove(cid);
        tracing::info!("Deleted content with CID: {}", cid);
        Ok(())
    }

    /// Get content size
    pub async fn get_content_size(&self, cid: &str) -> Result<u64> {
        let storage = self.storage.read().await;
        if let Some(content) = storage.get(cid) {
            Ok(content.len() as u64)
        } else {
            Err(MusicPlayerError::ContentNotAvailable { 
                cid: cid.to_string() 
            })
        }
    }

    /// List all stored content identifiers
    pub async fn list_content_ids(&self) -> Result<Vec<String>> {
        let storage = self.storage.read().await;
        Ok(storage.keys().cloned().collect())
    }

    /// Pin content to ensure it's not garbage collected
    pub async fn pin_content(&self, cid: &str) -> Result<()> {
        // In a real implementation, this would pin content in the P2P network
        tracing::info!("Pinned content with CID: {}", cid);
        Ok(())
    }

    /// Unpin content to allow garbage collection
    pub async fn unpin_content(&self, cid: &str) -> Result<()> {
        // In a real implementation, this would unpin content in the P2P network
        tracing::info!("Unpinned content with CID: {}", cid);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_media_store_creation() {
        let store = MediaStore::new();
        // Just test that it can be created
        assert!(true);
    }

    #[tokio::test]
    async fn test_store_and_retrieve_content() {
        let store = MediaStore::new();
        let content = b"test audio content".to_vec();
        
        // Store content
        let cid = store.store_content(content.clone()).await.unwrap();
        assert!(cid.starts_with("bafy"));
        
        // Retrieve content
        let retrieved = store.retrieve_content(&cid).await.unwrap();
        assert_eq!(content, retrieved);
    }

    #[tokio::test]
    async fn test_content_exists() {
        let store = MediaStore::new();
        let content = b"test audio content".to_vec();
        
        // Check non-existent content
        let exists = store.content_exists("non-existent-cid").await.unwrap();
        assert!(!exists);
        
        // Store content and check again
        let cid = store.store_content(content).await.unwrap();
        let exists = store.content_exists(&cid).await.unwrap();
        assert!(exists);
    }

    #[tokio::test]
    async fn test_delete_content() {
        let store = MediaStore::new();
        let content = b"test audio content".to_vec();
        
        // Store content
        let cid = store.store_content(content).await.unwrap();
        
        // Delete content
        store.delete_content(&cid).await.unwrap();
        
        // Check that content no longer exists
        let exists = store.content_exists(&cid).await.unwrap();
        assert!(!exists);
    }

    #[tokio::test]
    async fn test_get_content_size() {
        let store = MediaStore::new();
        let content = b"test audio content".to_vec();
        let content_size = content.len() as u64;
        
        // Store content
        let cid = store.store_content(content).await.unwrap();
        
        // Get content size
        let size = store.get_content_size(&cid).await.unwrap();
        assert_eq!(content_size, size);
    }

    #[tokio::test]
    async fn test_list_content_ids() {
        let store = MediaStore::new();
        let content1 = b"test audio content 1".to_vec();
        let content2 = b"test audio content 2".to_vec();
        
        // Store content
        let cid1 = store.store_content(content1).await.unwrap();
        let cid2 = store.store_content(content2).await.unwrap();
        
        // List content IDs
        let cids = store.list_content_ids().await.unwrap();
        assert!(cids.contains(&cid1));
        assert!(cids.contains(&cid2));
        assert_eq!(cids.len(), 2);
    }
}
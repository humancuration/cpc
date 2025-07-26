use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Metadata store for P2P operations
pub struct MetadataStore {
    data: Arc<Mutex<HashMap<String, StoredMetadata>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMetadata {
    pub id: String,
    pub content_hash: String,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub peers: Vec<String>, // Peer IDs that have this content
}

impl MetadataStore {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn store_metadata(&self, id: String, metadata: StoredMetadata) {
        let mut data = self.data.lock().unwrap();
        data.insert(id, metadata);
    }

    pub fn get_metadata(&self, id: &str) -> Option<StoredMetadata> {
        let data = self.data.lock().unwrap();
        data.get(id).cloned()
    }

    pub fn remove_metadata(&self, id: &str) -> Option<StoredMetadata> {
        let mut data = self.data.lock().unwrap();
        data.remove(id)
    }

    pub fn list_all(&self) -> Vec<(String, StoredMetadata)> {
        let data = self.data.lock().unwrap();
        data.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
}

impl Default for MetadataStore {
    fn default() -> Self {
        Self::new()
    }
}
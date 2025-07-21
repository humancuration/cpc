use crate::metadata::FileMetadata;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct InMemoryMetadataStore {
    store: Mutex<HashMap<String, Vec<FileMetadata>>>,
}

impl InMemoryMetadataStore {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
pub trait FileMetadataStore: Send + Sync {
    async fn get_metadata(&self, file_path: &str) -> Result<FileMetadata, String>;
    async fn store_metadata(&self, file_path: &str, metadata: FileMetadata) -> Result<(), String>;
    async fn get_versioned_metadata(&self, file_path: &str, version: u64) -> Result<FileMetadata, String>;
}

#[async_trait::async_trait]
impl FileMetadataStore for InMemoryMetadataStore {
    async fn get_metadata(&self, file_path: &str) -> Result<FileMetadata, String> {
        let store = self.store.lock().map_err(|e| e.to_string())?;
        store.get(file_path)
            .and_then(|versions| versions.last().cloned())
            .ok_or_else(|| "Metadata not found".to_string())
    }

    async fn store_metadata(&self, file_path: &str, metadata: FileMetadata) -> Result<(), String> {
        let mut store = self.store.lock().map_err(|e| e.to_string())?;
        let versions = store.entry(file_path.to_string()).or_insert_with(Vec::new);
        versions.push(metadata);
        Ok(())
    }
    
    async fn get_versioned_metadata(&self, file_path: &str, version: u64) -> Result<FileMetadata, String> {
        let store = self.store.lock().map_err(|e| e.to_string())?;
        store.get(file_path)
            .and_then(|versions| versions.iter().find(|m| m.version == version).cloned())
            .ok_or_else(|| "Versioned metadata not found".to_string())
    }
}
use notify::{RecommendedWatcher, Watcher, RecursiveMode, Event, EventKind};
use std::path::Path;
use tokio::sync::mpsc;
use crate::network::{Network, NetworkMessage, FileChangeNotification};
use crate::file_utils::compute_file_diff;
use crate::metadata::FileMetadata;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    network: Arc<Mutex<dyn Network + Send + Sync>>,
    metadata_store: Arc<Mutex<dyn FileMetadataStore + Send + Sync>>,
}

impl FileWatcher {
    pub fn new(
        network: Arc<Mutex<dyn Network + Send + Sync>>,
        metadata_store: Arc<Mutex<dyn FileMetadataStore + Send + Sync>>
    ) -> Result<(Self, mpsc::Receiver<Event>), String> {
        let (tx, rx) = mpsc::channel(100);
        let network_clone = Arc::clone(&network);
        let metadata_clone = Arc::clone(&metadata_store);
        
        let watcher = notify::recommended_watcher(move |res| {
            if let Ok(event) = res {
                tx.blocking_send(event).unwrap();
                
                // Handle file change events
                if let EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_) = event.kind {
                    for path in event.paths {
                        let net = Arc::clone(&network_clone);
                        let meta = Arc::clone(&metadata_clone);
                        tokio::spawn(async move {
                            if let Err(e) = Self::handle_file_change(&path, net, meta).await {
                                log::error!("Error handling file change: {}", e);
                            }
                        });
                    }
                }
            }
        }).map_err(|e| e.to_string())?;
        
        Ok(Self { watcher, network, metadata_store }, rx)
    }

    pub fn watch(&mut self, path: &Path) -> notify::Result<()> {
        self.watcher.watch(path, RecursiveMode::Recursive)
    }

    async fn handle_file_change(
        path: &Path,
        network: Arc<Mutex<dyn Network + Send + Sync>>,
        metadata_store: Arc<Mutex<dyn FileMetadataStore + Send + Sync>>
    ) -> Result<(), String> {
        let file_path = path.to_string_lossy().to_string();
        
        // Get current file metadata
        let mut meta_store = metadata_store.lock().await;
        let current_meta = meta_store.get_metadata(&file_path).await?;
        
        // Compute new metadata (this would involve actual file processing)
        // For simplicity, we'll just increment version
        let new_meta = FileMetadata {
            content_address: "new_merkle_root".to_string(),
            version: current_meta.version + 1,
            ..current_meta.clone()
        };
        
        // Compute diff between versions
        let diff = compute_file_diff(&current_meta, &new_meta);
        
        // Update metadata store
        meta_store.store_metadata(&file_path, new_meta).await?;
        
        // Broadcast change notification
        let msg = NetworkMessage::FileChangeNotification(FileChangeNotification {
            file_path: file_path.clone(),
            merkle_root: "new_merkle_root".to_string(),
            version: current_meta.version + 1,
        });
        
        let mut net = network.lock().await;
        net.broadcast(msg).await.map_err(|e| e.to_string())
    }
}

pub trait FileMetadataStore {
    async fn get_metadata(&self, file_path: &str) -> Result<FileMetadata, String>;
    async fn store_metadata(&self, file_path: &str, metadata: FileMetadata) -> Result<(), String>;
}
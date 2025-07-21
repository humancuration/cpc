//! Personal Data Server for Cooperative Peer Cloud

use std::sync::{Arc, Mutex};
use cpc_lib::{storage::LruStorage, net::{NetworkBuilder, Network}, crypto::KeyPair};
use tauri::Builder;
use tokio::sync::mpsc;

mod commands;
mod metadata;
mod file_watcher;
mod conflict_resolver;
use commands::{get_network_status, get_storage_usage, set_storage_limit, list_files, upload_file, download_file};
use metadata::MetadataStore;
use file_watcher::FileWatcher;
use conflict_resolver::ConflictResolver;

/// Application state shared with Tauri commands
pub struct AppState {
    storage: Arc<Mutex<LruStorage>>,
    network: Arc<Mutex<dyn Network>>,
    metadata_store: Arc<Mutex<MetadataStore>>,
    config: Config,
    file_watcher: Mutex<FileWatcher>,
    file_event_rx: Mutex<mpsc::Receiver<notify::Event>>,
    conflict_resolver: Mutex<ConflictResolver>,
}

/// Configuration for the application
pub struct Config {
    encryption_key: [u8; 32],
}

fn main() {
    println!("Personal Data Server starting...");

    // Initialize local storage
    let storage = Arc::new(Mutex::new(LruStorage::new(1024 * 1024 * 1024))); // 1GB limit

    // Generate keys for encryption
    let keys = KeyPair::generate_x25519();
    let encryption_key = keys.private_key().as_ref().try_into().expect("Invalid key length");

    // Initialize metadata store
    let metadata_store = Arc::new(Mutex::new(
        MetadataStore::new("pds_metadata.db").expect("Failed to create metadata store")
    ));

    // Connect to P2P network
    let network = Arc::new(Mutex::new(
        NetworkBuilder::new()
            .with_websocket()
            .build()
    ));

    // Initialize file watcher
    let (file_watcher, file_event_rx) = FileWatcher::new().expect("Failed to create file watcher");

    // Initialize conflict resolver
    let conflict_resolver = ConflictResolver::new();

    // Create application state
    let app_state = AppState {
        storage,
        network,
        metadata_store,
        config: Config { encryption_key },
        file_watcher: Mutex::new(file_watcher),
        file_event_rx: Mutex::new(file_event_rx),
        conflict_resolver: Mutex::new(conflict_resolver),
    };

    // Initialize Tauri application with state
    Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_network_status,
            get_storage_usage,
            set_storage_limit,
            list_files,
            upload_file,
            download_file
        ])
        .run(tauri::generate_context!())
        .expect("error running tauri application");
}
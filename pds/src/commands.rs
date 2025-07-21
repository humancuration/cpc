//! Tauri commands for PDS frontend interaction

use crate::AppState;
use tauri::State;
use std::sync::{Mutex, Arc};
use cpc_lib::{storage::LruStorage, net::Network};

/// Network status information
#[derive(serde::Serialize)]
pub struct NetworkStatus {
    peers: usize,
    is_online: bool,
    bandwidth_up: f64,
    bandwidth_down: f64,
}

/// Get current network status
#[tauri::command]
pub fn get_network_status(state: State<AppState>) -> Result<NetworkStatus, String> {
    let network = state.network.lock().map_err(|e| e.to_string())?;
    
    // Get actual network metrics from rust-libp2p
    let peers = network.get_peer_count();
    let is_online = network.is_online();
    let (bandwidth_up, bandwidth_down) = network.get_bandwidth_usage();
    
    Ok(NetworkStatus {
        peers,
        is_online,
        bandwidth_up: bandwidth_up as f64 / 1024.0, // Convert to KB/s
        bandwidth_down: bandwidth_down as f64 / 1024.0,
    })
}

/// Set storage limit
/// Storage metrics information
#[derive(serde::Serialize)]
pub struct StorageMetrics {
    used: u64,
    limit: u64,
}

/// Get current storage usage
#[tauri::command]
pub fn get_storage_usage(state: State<AppState>) -> Result<StorageMetrics, String> {
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    let used = storage.get_used_space().map_err(|e| e.to_string())?;
    let limit = storage.get_limit();
    Ok(StorageMetrics { used, limit })
}

/// Set storage limit
#[tauri::command]
pub fn set_storage_limit(limit: u64, state: State<AppState>) -> Result<(), String> {
    let mut storage = state.storage.lock().map_err(|e| e.to_string())?;
    storage.set_limit(limit).map_err(|e| e.to_string())
}

/// List files in a directory
#[tauri::command]
pub fn list_files(path: String, state: State<AppState>) -> Result<Vec<String>, String> {
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    storage.list_files(&path).map_err(|e| e.to_string())
}
use sha2::{Sha256, Digest};
use crate::{file_utils::{FileProcessor, EncryptedChunk}, metadata::{FileMetadata, MetadataStore}};
use std::path::Path;
use crate::storage::NetworkError;

/// Upload a file to the network using content addressing
#[tauri::command]
pub async fn upload_file(
    path: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let processor = FileProcessor::new(state.config.encryption_key);
    let (chunks, metadata) = processor.chunk_and_encrypt(Path::new(&path))
        .map_err(|e| e.to_string())?;

    let mut network = state.network.lock()
        .map_err(|e| e.to_string())?;
    let metadata_store = state.metadata_store.lock()
        .map_err(|e| e.to_string())?;

    // Store file metadata
    network.store_metadata(&metadata)
        .await
        .map_err(|e| e.to_string())?;
    metadata_store.store_file_metadata(&metadata)
        .map_err(|e| e.to_string())?;

    // Store each chunk with its content address
    for (i, chunk) in chunks.iter().enumerate() {
        let chunk_hash = hex::encode(&chunk.data);
        metadata_store.store_chunk_metadata(&ChunkMetadata {
            file_content_address: metadata.content_address.clone(),
            index: i,
            hash: chunk_hash.clone(),
            size: chunk.data.len(),
            nonce: chunk.nonce.to_vec(),
        }).map_err(|e| e.to_string())?;

        network.store_chunk(&chunk_hash, &chunk.data)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(metadata.content_address)
}

/// Download a file from the network using its content address
#[tauri::command]
pub async fn download_file(
    content_address: String,
    state: State<'_, AppState>,
) -> Result<Vec<u8>, String> {
    let network = state.network.lock()
        .map_err(|e| e.to_string())?;
    let metadata_store = state.metadata_store.lock()
        .map_err(|e| e.to_string())?;

    // Get file metadata
    let metadata = network.get_metadata(&content_address)
        .await
        .map_err(|e| e.to_string())?;

    // Get chunk metadata
    let chunks_meta = metadata_store.get_chunks_for_file(&content_address)
        .map_err(|e| e.to_string())?;

    // Retrieve chunks with retry logic
    let mut chunks = Vec::new();
    for chunk_meta in chunks_meta {
        let mut retries = 3;
        loop {
            match network.get_chunk(&chunk_meta.hash).await {
            Ok(data) => {
                // Validate chunk hash
                let mut hasher = Sha256::new();
                hasher.update(&data);
                let computed_hash = hex::encode(hasher.finalize());

                if computed_hash != chunk_meta.hash {
                    return Err(NetworkError::VerificationFailed(format!(
                        "Chunk {} hash mismatch",
                        chunk_meta.index
                    )).to_string());
                }

                chunks.push(EncryptedChunk {
                    data,
                    nonce: chunk_meta.nonce.clone().try_into()
                        .map_err(|_| "Invalid nonce length".to_string())?,
                });
                break;
            }
                Err(NetworkError::NotFound) if retries > 0 => {
                    retries -= 1;
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
                Err(e) => return Err(e.to_string()),
            }
        }
    }

    // Verify Merkle tree
    crate::file_utils::verify_merkle_tree(&chunks, &metadata.content_address)
        .map_err(|e| NetworkError::VerificationFailed(e).to_string())?;

    let processor = FileProcessor::new(state.config.encryption_key);
    processor.decrypt_and_assemble(chunks)
}
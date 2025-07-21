# File Change Plan for Phase 2 Implementation

## Core Changes

### 1. `pds/src/metadata.rs` (New File)
```rust
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use rusqlite::{Connection, params};

#[derive(Serialize, Deserialize, Clone)]
pub struct FileMetadata {
    pub file_id: String,       // Content address (Merkle root)
    pub file_name: String,
    pub size: u64,
    pub mime_type: String,
    pub created_at: DateTime<Utc>,
    pub chunks: Vec<ChunkMetadata>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChunkMetadata {
    pub index: u32,
    pub hash: String,          // SHA256 of encrypted chunk
    pub size: usize,
    pub nonce: [u8; 12],       // AES-GCM nonce
}

pub struct MetadataStore {
    conn: Connection,
}

impl MetadataStore {
    pub fn new(path: &str) -> Result<Self, String> {
        let conn = Connection::open(path).map_err(|e| e.to_string())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS files (
                file_id TEXT PRIMARY KEY,
                file_name TEXT NOT NULL,
                size INTEGER NOT NULL,
                mime_type TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        ).map_err(|e| e.to_string())?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS chunks (
                chunk_hash TEXT PRIMARY KEY,
                file_id TEXT NOT NULL,
                index INTEGER NOT NULL,
                size INTEGER NOT NULL,
                nonce BLOB NOT NULL,
                FOREIGN KEY(file_id) REFERENCES files(file_id)
            )",
            [],
        ).map_err(|e| e.to_string())?;
        
        Ok(Self { conn })
    }

    pub fn store_metadata(&self, metadata: &FileMetadata) -> Result<(), String> {
        let tx = self.conn.transaction().map_err(|e| e.to_string())?;
        tx.execute(
            "INSERT INTO files (file_id, file_name, size, mime_type, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                &metadata.file_id,
                &metadata.file_name,
                metadata.size,
                &metadata.mime_type,
                metadata.created_at.to_rfc3339()
            ],
        ).map_err(|e| e.to_string())?;

        for chunk in &metadata.chunks {
            tx.execute(
                "INSERT INTO chunks (chunk_hash, file_id, index, size, nonce) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    &chunk.hash,
                    &metadata.file_id,
                    chunk.index,
                    chunk.size,
                    &chunk.nonce[..]
                ],
            ).map_err(|e| e.to_string())?;
        }

        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_metadata(&self, file_id: &str) -> Result<FileMetadata, String> {
        // Implementation to retrieve metadata
    }
}
```

### 2. `pds/src/file_utils.rs` - Major Update
- Return metadata from `chunk_and_encrypt`
- Add Merkle tree verification
- Fix nonce handling
- Add content-based hashing

```rust
use aes_gcm::{Aes256Gcm, KeyInit, aead::{Aead, generic_array::GenericArray}};
use merkle_cbt::{merkle_tree::Merge, MerkleTree};
use sha2::{Sha256, Digest};
use crate::metadata::{FileMetadata, ChunkMetadata};

impl FileProcessor {
    pub fn chunk_and_encrypt(&self, path: &Path) -> Result<(Vec<Vec<u8>>, FileMetadata), String> {
        let mut file = std::fs::File::open(path).map_err(|e| e.to_string())?;
        let mut chunks = Vec::new();
        let mut hashes = Vec::new();
        let mut chunk_metadata = Vec::new();

        let cipher = Aes256Gcm::new(GenericArray::from_slice(&self.encryption_key));
        let mut total_size = 0;

        loop {
            let mut chunk = vec![0; CHUNK_SIZE];
            let bytes_read = file.read(&mut chunk).map_err(|e| e.to_string())?;
            if bytes_read == 0 { break }

            chunk.truncate(bytes_read);
            total_size += bytes_read as u64;
            
            let nonce: [u8; 12] = rand::random();
            let encrypted_chunk = cipher.encrypt(&nonce.into(), chunk.as_ref())
                .map_err(|e| e.to_string())?;

            let mut hasher = Sha256::new();
            hasher.update(&encrypted_chunk);
            let chunk_hash = hasher.finalize();
            let chunk_hash_hex = hex::encode(chunk_hash);
            
            chunks.push(encrypted_chunk);
            hashes.push(chunk_hash.to_vec());
            
            chunk_metadata.push(ChunkMetadata {
                index: chunks.len() as u32 - 1,
                hash: chunk_hash_hex,
                size: bytes_read,
                nonce,
            });
        }

        let merkle_tree = MerkleTree::<Vec<u8>, Sha256>::build(hashes);
        let merkle_root = merkle_tree.root();
        let file_id = hex::encode(merkle_root);

        let metadata = FileMetadata {
            file_id,
            file_name: path.file_name().unwrap().to_string_lossy().into_owned(),
            size: total_size,
            mime_type: mime_guess::from_path(path).first_or_octet_stream().to_string(),
            created_at: Utc::now(),
            chunks: chunk_metadata,
        };

        Ok((chunks, metadata))
    }

    pub fn verify_chunks(chunks: &[Vec<u8>], metadata: &FileMetadata) -> Result<(), String> {
        // Merkle tree verification implementation
    }
}
```

### 3. `pds/src/commands.rs` - Update Upload/Download
- Integrate metadata storage
- Use content addressing
- Add error handling

```rust
use crate::metadata::{FileMetadata, MetadataStore};

#[tauri::command]
pub async fn upload_file(
    path: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let processor = FileProcessor::new(state.config.encryption_key);
    let (chunks, metadata) = processor.chunk_and_encrypt(Path::new(&path))?;
    
    let mut network = state.network.lock().map_err(|e| e.to_string())?;
    
    // Store chunks using content addresses
    for chunk_meta in &metadata.chunks {
        let chunk = &chunks[chunk_meta.index as usize];
        network.store_chunk(&chunk_meta.hash, chunk.clone()).await?;
    }

    // Store metadata
    network.store_metadata(&metadata).await?;

    // Store locally for fast access
    let metadata_store = state.metadata_store.lock().map_err(|e| e.to_string())?;
    metadata_store.store_metadata(&metadata)?;

    Ok(metadata.file_id)
}

#[tauri::command]
pub async fn download_file(
    file_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<u8>, String> {
    let metadata = {
        // Try local metadata store first
        let metadata_store = state.metadata_store.lock().map_err(|e| e.to_string())?;
        metadata_store.get_metadata(&file_id)
            .or_else(|_| {
                // Fallback to network
                let network = state.network.lock().map_err(|e| e.to_string())?;
                network.get_metadata(&file_id).await
            })?
    };

    let mut network = state.network.lock().map_err(|e| e.to_string())?;
    let mut chunks = Vec::new();
    
    for chunk_meta in &metadata.chunks {
        match network.get_chunk(&chunk_meta.hash).await {
            Ok(chunk) => chunks.push(chunk),
            Err(e) => return Err(format!("Failed to fetch chunk {}: {}", chunk_meta.hash, e)),
        }
    }

    // Verify chunks
    FileProcessor::verify_chunks(&chunks, &metadata)?;

    let processor = FileProcessor::new(state.config.encryption_key);
    let nonces: Vec<_> = metadata.chunks.iter().map(|c| c.nce).collect();
    processor.decrypt_and_assemble(chunks, nonces)
}
```

### 4. `pds/src/storage.rs` - Network Trait Update
```rust
use crate::metadata::FileMetadata;

pub enum NetworkError {
    ChunkNotFound(String),
    VerificationFailed,
    DecryptionError(String),
    StorageFull,
    MetadataNotFound(String),
    NetworkUnavailable,
    InvalidData,
}

pub trait Network {
    async fn store_chunk(&mut self, hash: &str, data: Vec<u8>) -> Result<(), NetworkError>;
    async fn get_chunk(&self, hash: &str) -> Result<Vec<u8>, NetworkError>;
    async fn store_metadata(&mut self, metadata: &FileMetadata) -> Result<(), NetworkError>;
    async fn get_metadata(&self, file_id: &str) -> Result<FileMetadata, NetworkError>;
}
```

### 5. `pds/src/main.rs` - Initialize Metadata Store
```rust
use crate::metadata::MetadataStore;

struct AppState {
    config: Config,
    network: Mutex<Box<dyn Network>>,
    storage: Mutex<LruStorage>,
    metadata_store: Mutex<MetadataStore>,
}

fn main() {
    // ... existing setup ...
    
    let metadata_store = MetadataStore::new("metadata.db")
        .expect("Failed to create metadata store");
        
    let state = AppState {
        config,
        network: Mutex::new(Box::new(rust-libp2p_network)),
        storage: Mutex::new(storage),
        metadata_store: Mutex::new(metadata_store),
    };
    
    // ... run tauri app ...
}
```

### 6. `pds/Cargo.toml` - Add Dependencies
```toml
[dependencies]
chrono = { version = "0.4", features = ["serde"] }
hex = "0.4"
mime_guess = "2.0"
```

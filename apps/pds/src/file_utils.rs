use aes_gcm::{Aes256Gcm, KeyInit, aead::{Aead, generic_array::GenericArray}};
use std::path::Path;
use std::io::{Read};
use merkle_cbt::{merkle_tree::Merge, MerkleTree};
use sha2::{Sha256, Digest};
use crate::metadata::{FileMetadata, ChunkMetadata};
use chrono::Utc;
use hex;
use mime_guess;
use std::fs;

const CHUNK_SIZE: usize = 262_144; // 256KB

/// Represents an encrypted chunk with its associated nonce
pub struct EncryptedChunk {
    pub data: Vec<u8>,
    pub nonce: [u8; 12],
}

pub struct FileProcessor {
    encryption_key: [u8; 32],
}

impl FileProcessor {
    pub fn new(encryption_key: [u8; 32]) -> Self {
        Self { encryption_key }
    }

    /// Chunk, encrypt, and generate metadata for a file
    pub fn chunk_and_encrypt(&self, path: &Path) -> Result<(Vec<EncryptedChunk>, FileMetadata), String> {
        let mut file = fs::File::open(path).map_err(|e| e.to_string())?;
        let total_size = fs::metadata(path).map_err(|e| e.to_string())?.len();
        let mime_type = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();

        let mut chunks = Vec::new();
        let mut hashes = Vec::new();

        let cipher = Aes256Gcm::new(GenericArray::from_slice(&self.encryption_key));

        loop {
            let mut chunk = vec![0; CHUNK_SIZE];
            let bytes_read = file.read(&mut chunk).map_err(|e| e.to_string())?;
            if bytes_read == 0 { break }

            chunk.truncate(bytes_read);
            let nonce: [u8; 12] = rand::random(); // Generate unique nonce
            let nonce_generic = GenericArray::from_slice(&nonce);
            let encrypted_chunk = cipher.encrypt(nonce_generic, chunk.as_ref())
                .map_err(|e| e.to_string())?;

            let mut hasher = Sha256::new();
            hasher.update(&encrypted_chunk);
            let hash = hasher.finalize().to_vec();
            hashes.push(hash.clone());

            chunks.push(EncryptedChunk {
                data: encrypted_chunk,
                nonce,
            });
        }

        // Build Merkle tree and get root hash
        let merkle_tree = MerkleTree::<Vec<u8>, Sha256>::build(hashes);
        let merkle_root = merkle_tree.root();
        let content_address = hex::encode(merkle_root);

        let file_metadata = FileMetadata {
            content_address: content_address.clone(),
            version: 1,
            original_path: path.to_string_lossy().to_string(),
            size: total_size,
            chunk_count: chunks.len(),
            mime_type,
            created_at: Utc::now().timestamp(),
        };

        Ok((chunks, file_metadata))
    }
/// Decrypt and assemble chunks using stored nonces
pub fn decrypt_and_assemble(&self, chunks: Vec<EncryptedChunk>) -> Result<Vec<u8>, String> {
    let cipher = Aes256Gcm::new(GenericArray::from_slice(&self.encryption_key));
    let mut assembled = Vec::new();

    for chunk in chunks {
        let nonce = GenericArray::from_slice(&chunk.nonce);
        let decrypted = cipher.decrypt(nonce, chunk.data.as_ref())
            .map_err(|e| e.to_string())?;
        assembled.extend(decrypted);
    }

    Ok(assembled)
}

/// Chunk, encrypt, and generate metadata for byte data
/// Chunk, encrypt, and generate metadata for byte data
#[tracing::instrument(skip(self, data))]
pub fn chunk_and_encrypt_bytes(&self, data: &[u8]) -> Result<(Vec<EncryptedChunk>, FileMetadata), PublishError> {
    let total_size = data.len() as u64;
    let mime_type = "application/octet-stream".to_string();

    tracing::debug!("Chunking and encrypting {} bytes of data", total_size);

    let mut chunks = Vec::new();
    let mut hashes = Vec::new();

    let cipher = Aes256Gcm::new(GenericArray::from_slice(&self.encryption_key));

    for chunk_data in data.chunks(CHUNK_SIZE) {
        let nonce: [u8; 12] = rand::random(); // Generate unique nonce
        let nonce_generic = GenericArray::from_slice(&nonce);
        let encrypted_chunk = cipher.encrypt(nonce_generic, chunk_data)
            .map_err(|e| PublishError::PdsProcessing(format!("Encryption failed: {}", e)))?;

        let mut hasher = Sha256::new();
        hasher.update(&encrypted_chunk);
        let hash = hasher.finalize().to_vec();
        hashes.push(hash.clone());

        chunks.push(EncryptedChunk {
            data: encrypted_chunk,
            nonce,
        });
    }

    tracing::debug!("Created {} chunks for {} bytes", chunks.len(), total_size);

    // Build Merkle tree and get root hash
    let merkle_tree = MerkleTree::<Vec<u8>, Sha256>::build(hashes);
    let merkle_root = merkle_tree.root();
    let content_address = hex::encode(merkle_root);

    let file_metadata = FileMetadata {
        content_address: content_address.clone(),
        version: 1,
        original_path: "".to_string(),
        size: total_size,
        chunk_count: chunks.len(),
        mime_type,
        created_at: Utc::now().timestamp(),
    };

    Ok((chunks, file_metadata))
}

    /// Process project data by serializing it and then chunking/encrypting
    #[tracing::instrument(skip(self, project_data))]
    pub fn process_project(&self, project_data: &cpc_core::project::ProjectData) -> Result<String, PublishError> {
        // Serialize project data to bytes
        let mut buf = Vec::new();
        project_data.serialize(&mut rmp_serde::Serializer::new(&mut buf))
            .map_err(|e| PublishError::Serialization(e.into()))?;

        tracing::info!("Serialized project data to {} bytes", buf.len());

        // Process the serialized bytes
        let (_, metadata) = self.chunk_and_encrypt_bytes(&buf)?;
        Ok(metadata.content_address)
    }
}

/// Verify Merkle tree structure of downloaded chunks
pub fn verify_merkle_tree(metadata: &FileMetadata) -> Result<(), String> {
    // This would be implemented using actual chunk data
    // For now, we'll just simulate verification
    if metadata.content_address.is_empty() {
        return Err("Empty content address".to_string());
    }
    Ok(())
}

/// Compare two file metadata versions and return chunk differences
pub fn compute_file_diff(
    old_metadata: &FileMetadata,
    new_metadata: &FileMetadata
) -> Vec<ChunkDiff> {
    let mut diffs = Vec::new();
    
    // Simple version comparison - in real implementation we'd compare Merkle trees
    if old_metadata.version != new_metadata.version {
        diffs.push(ChunkDiff {
            chunk_index: 0,
            old_hash: old_metadata.content_address.clone(),
            new_hash: new_metadata.content_address.clone(),
        });
    }
    
    diffs
}

/// Apply diff to file metadata
pub fn apply_file_diff(
    metadata: &mut FileMetadata,
    diff: Vec<ChunkDiff>
) -> Result<(), String> {
    for chunk_diff in diff {
        // In real implementation we'd update specific chunks
        metadata.content_address = chunk_diff.new_hash;
        metadata.version += 1;
    }
    Ok(())
}
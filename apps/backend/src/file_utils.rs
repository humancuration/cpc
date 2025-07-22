use aes_gcm::{Aes256Gcm, KeyInit, aead::{Aead, generic_array::GenericArray}};
use merkle_cbt::{merkle_tree::Merge, MerkleTree};
use sha2::{Sha256, Digest};
use chrono::Utc;
use hex;
use mime_guess;
use std::fs;
use crate::error::PublishError;

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
        // Validate key length at creation time
        assert_eq!(encryption_key.len(), 32, "Encryption key must be 32 bytes");
        Self { encryption_key }
    }

    /// Chunk and encrypt byte data
    #[tracing::instrument(skip(self, data))]
    pub fn chunk_and_encrypt_bytes(&self, data: &[u8]) -> Result<(Vec<EncryptedChunk>, FileMetadata), PublishError> {
        let total_size = data.len() as u64;
        let mime_type = "application/octet-stream".to_string();

        tracing::debug!("Chunking and encrypting {} bytes of data", total_size);

        let mut chunks = Vec::new();
        let mut hashes = Vec::new();

        let cipher = Aes256Gcm::new(GenericArray::from_slice(&self.encryption_key));

        for chunk_data in data.chunks(CHUNK_SIZE) {
            let nonce: [u8; 12] = rand::random();
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
//! Encryption module for health data
//!
//! This module provides encryption functionality for health data,
//! including audit logs, using AES-256 encryption as required for HIPAA compliance.

use aes::Aes256;
use aes::cipher::{BlockEncrypt, BlockDecrypt, KeyInit, generic_array::GenericArray};
use rand::Rng;
use thiserror::Error;
use base64::{Engine as _, engine::general_purpose};

/// Error types for encryption operations
#[derive(Debug, Error)]
pub enum EncryptionError {
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    
    #[error("Invalid key size")]
    InvalidKeySize,
    
    #[error("Invalid data format")]
    InvalidDataFormat,
}

/// AES-256 encryption service
pub struct AESEncryptionService {
    key: [u8; 32], // 256-bit key
}

impl AESEncryptionService {
    /// Create a new encryption service with a provided key
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
    
    /// Generate a random 256-bit key
    pub fn generate_key() -> [u8; 32] {
        let mut rng = rand::thread_rng();
        let mut key = [0u8; 32];
        rng.fill(&mut key);
        key
    }
    
    /// Encrypt data using AES-256
    pub fn encrypt(&self, plaintext: &str) -> Result<String, EncryptionError> {
        // Convert plaintext to bytes
        let mut data = plaintext.as_bytes().to_vec();
        
        // Pad data to block size (16 bytes for AES)
        let padding = 16 - (data.len() % 16);
        for _ in 0..padding {
            data.push(padding as u8);
        }
        
        // Encrypt each block
        let cipher = Aes256::new(&GenericArray::from_slice(&self.key));
        let mut encrypted = Vec::new();
        
        for chunk in data.chunks_exact(16) {
            let mut block = GenericArray::clone_from_slice(chunk);
            cipher.encrypt_block(&mut block);
            encrypted.extend_from_slice(block.as_slice());
        }
        
        // Encode as base64 for storage
        Ok(general_purpose::STANDARD.encode(&encrypted))
    }
    
    /// Decrypt data using AES-256
    pub fn decrypt(&self, encrypted_data: &str) -> Result<String, EncryptionError> {
        // Decode from base64
        let encrypted_bytes = general_purpose::STANDARD
            .decode(encrypted_data)
            .map_err(|e| EncryptionError::InvalidDataFormat)?;
        
        // Decrypt each block
        let cipher = Aes256::new(&GenericArray::from_slice(&self.key));
        let mut decrypted = Vec::new();
        
        for chunk in encrypted_bytes.chunks_exact(16) {
            let mut block = GenericArray::clone_from_slice(chunk);
            cipher.decrypt_block(&mut block);
            decrypted.extend_from_slice(block.as_slice());
        }
        
        // Remove padding
        if let Some(&last_byte) = decrypted.last() {
            let padding_size = last_byte as usize;
            if padding_size > 0 && padding_size <= 16 {
                decrypted.truncate(decrypted.len() - padding_size);
            }
        }
        
        // Convert to string
        String::from_utf8(decrypted)
            .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))
    }
}

/// Trait for encryptable data
pub trait Encryptable {
    /// Encrypt the data
    fn encrypt(&self, encryption_service: &AESEncryptionService) -> Result<String, EncryptionError>;
    
    /// Decrypt the data
    fn decrypt(encrypted_data: &str, encryption_service: &AESEncryptionService) -> Result<Self, EncryptionError> where Self: Sized;
}

/// Configuration for encryption
#[derive(Debug, Clone)]
pub struct EncryptionConfig {
    /// Whether encryption is enabled
    pub enabled: bool,
    
    /// Key management strategy
    pub key_management: KeyManagementStrategy,
}

/// Key management strategies
#[derive(Debug, Clone)]
pub enum KeyManagementStrategy {
    /// Use a fixed key (not recommended for production)
    FixedKey,
    
    /// Use environment-based key management
    Environment,
    
    /// Use a key management service
    KeyManagementService,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            key_management: KeyManagementStrategy::Environment,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let key = AESEncryptionService::generate_key();
        let encryption_service = AESEncryptionService::new(key);
        
        let original_text = "This is a test message for encryption";
        
        // Encrypt
        let encrypted = encryption_service.encrypt(original_text).unwrap();
        assert_ne!(original_text, encrypted);
        
        // Decrypt
        let decrypted = encryption_service.decrypt(&encrypted).unwrap();
        assert_eq!(original_text, decrypted);
    }
    
    #[test]
    fn test_encryption_with_empty_string() {
        let key = AESEncryptionService::generate_key();
        let encryption_service = AESEncryptionService::new(key);
        
        let original_text = "";
        
        // Encrypt
        let encrypted = encryption_service.encrypt(original_text).unwrap();
        
        // Decrypt
        let decrypted = encryption_service.decrypt(&encrypted).unwrap();
        assert_eq!(original_text, decrypted);
    }
}
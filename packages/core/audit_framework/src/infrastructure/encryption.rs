//! Audit log encryption
//! 
//! This module provides encryption capabilities for audit logs.

use crypto::{aes, blockmodes, buffer, symmetriccipher};
use crypto::aes::KeySize;
use crypto::hmac::Hmac;
use crypto::pbkdf2::pbkdf2;
use crypto::sha2::Sha256;
use rand::{RngCore, rngs::OsRng};
use crate::domain::AuditError;

/// Audit log encryption service
pub struct AuditEncryption {
    /// Encryption key
    key: [u8; 32],
}

impl AuditEncryption {
    /// Create a new audit encryption service
    pub fn new(password: &str, salt: &[u8]) -> Self {
        let mut key = [0u8; 32];
        pbkdf2(&Sha256::new(), password.as_bytes(), salt, 10000, &mut key);
        Self { key }
    }
    
    /// Encrypt audit log data
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, AuditError> {
        // Generate a random IV
        let mut iv = [0u8; 16];
        OsRng.fill_bytes(&mut iv);
        
        // Create AES encryptor
        let mut encryptor = aes::cbc_encryptor(
            KeySize::KeySize256,
            &self.key,
            &iv,
            blockmodes::PkcsPadding,
        );
        
        // Encrypt the data
        let mut encrypted_data = Vec::new();
        let mut buffer = buffer::RefReadBuffer::new(data);
        let mut output_buffer = vec![0; data.len() + 16]; // Add padding space
        let mut buffer_out = buffer::RefWriteBuffer::new(&mut output_buffer);
        
        loop {
            let result = encryptor.encrypt(&mut buffer, &mut buffer_out, true)
                .map_err(|e| AuditError::EncryptionError(format!("Encryption failed: {:?}", e)))?;
            
            encrypted_data.extend_from_slice(buffer_out.take_read_buffer().take_remaining());
            
            match result {
                symmetriccipher::BufferResult::BufferUnderflow => break,
                symmetriccipher::BufferResult::BufferOverflow => continue,
            }
        }
        
        // Prepend the IV to the encrypted data
        let mut result = Vec::with_capacity(16 + encrypted_data.len());
        result.extend_from_slice(&iv);
        result.extend_from_slice(&encrypted_data);
        
        Ok(result)
    }
    
    /// Decrypt audit log data
    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, AuditError> {
        if encrypted_data.len() < 16 {
            return Err(AuditError::EncryptionError("Invalid encrypted data".to_string()));
        }
        
        // Extract the IV
        let (iv, ciphertext) = encrypted_data.split_at(16);
        
        // Create AES decryptor
        let mut decryptor = aes::cbc_decryptor(
            KeySize::KeySize256,
            &self.key,
            iv,
            blockmodes::PkcsPadding,
        );
        
        // Decrypt the data
        let mut decrypted_data = Vec::new();
        let mut buffer = buffer::RefReadBuffer::new(ciphertext);
        let mut output_buffer = vec![0; ciphertext.len() + 16]; // Add padding space
        let mut buffer_out = buffer::RefWriteBuffer::new(&mut output_buffer);
        
        loop {
            let result = decryptor.decrypt(&mut buffer, &mut buffer_out, true)
                .map_err(|e| AuditError::EncryptionError(format!("Decryption failed: {:?}", e)))?;
            
            decrypted_data.extend_from_slice(buffer_out.take_read_buffer().take_remaining());
            
            match result {
                symmetriccipher::BufferResult::BufferUnderflow => break,
                symmetriccipher::BufferResult::BufferOverflow => continue,
            }
        }
        
        Ok(decrypted_data)
    }
    
    /// Generate a random salt
    pub fn generate_salt() -> [u8; 32] {
        let mut salt = [0u8; 32];
        OsRng.fill_bytes(&mut salt);
        salt
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encryption_decryption() {
        let salt = AuditEncryption::generate_salt();
        let encryption = AuditEncryption::new("test_password", &salt);
        
        let original_data = b"This is test audit data";
        let encrypted_data = encryption.encrypt(original_data).unwrap();
        let decrypted_data = encryption.decrypt(&encrypted_data).unwrap();
        
        assert_eq!(original_data, decrypted_data.as_slice());
    }
}
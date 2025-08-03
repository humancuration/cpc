//! Encryption utilities
//!
//! This module provides symmetric encryption functions using AES-256.

use crypto::{aes, blockmodes, buffer, symmetriccipher};
use crypto::aes::KeySize;
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use rand::{RngCore, thread_rng};
use crate::error::{CommonError, Result};

/// Generate a random encryption key
pub fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    thread_rng().fill_bytes(&mut key);
    key
}

/// Encrypt data using AES-256
pub fn encrypt(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>> {
    let mut encryptor = aes::ctr(aes::KeySize::KeySize256, key, &generate_iv());
    
    let mut encrypted_data = Vec::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut encrypted_data);
    
    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)
            .map_err(|e| CommonError::crypto(format!("Encryption error: {:?}", e)))?;
        
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => continue,
        }
    }
    
    Ok(encrypted_data)
}

/// Decrypt data using AES-256
pub fn decrypt(encrypted_data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>> {
    let mut decryptor = aes::ctr(aes::KeySize::KeySize256, key, &generate_iv());
    
    let mut decrypted_data = Vec::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut decrypted_data);
    
    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)
            .map_err(|e| CommonError::crypto(format!("Decryption error: {:?}", e)))?;
        
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => continue,
        }
    }
    
    Ok(decrypted_data)
}

/// Generate a random initialization vector
fn generate_iv() -> [u8; 16] {
    let mut iv = [0u8; 16];
    thread_rng().fill_bytes(&mut iv);
    iv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_key() {
        let key1 = generate_key();
        let key2 = generate_key();
        
        assert_eq!(key1.len(), 32);
        assert_eq!(key2.len(), 32);
        assert_ne!(key1, key2); // Very unlikely to be equal
    }
    
    #[test]
    fn test_encrypt_decrypt() {
        let key = generate_key();
        let data = b"Hello, World! This is a test message.";
        
        let encrypted = encrypt(data, &key).unwrap();
        let decrypted = decrypt(&encrypted, &key).unwrap();
        
        assert_ne!(encrypted, data); // Encrypted data should be different
        assert_eq!(decrypted, data); // Decrypted data should match original
    }
    
    #[test]
    fn test_encrypt_decrypt_empty() {
        let key = generate_key();
        let data = b"";
        
        let encrypted = encrypt(data, &key).unwrap();
        let decrypted = decrypt(&encrypted, &key).unwrap();
        
        assert_eq!(decrypted, data);
    }
    
    #[test]
    fn test_decrypt_with_wrong_key() {
        let key1 = generate_key();
        let key2 = generate_key();
        let data = b"Secret message";
        
        let encrypted = encrypt(data, &key1).unwrap();
        let result = decrypt(&encrypted, &key2);
        
        // Decryption with wrong key should fail or produce garbage
        assert!(result.is_ok()); // AES-CTR doesn't inherently detect wrong keys
    }
}
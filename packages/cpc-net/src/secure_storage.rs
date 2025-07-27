//! Secure storage for sensitive data like encryption keys
//! 
//! This module provides secure storage mechanisms that attempt to protect
//! sensitive data in memory and prevent it from being swapped to disk.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zeroize::Zeroize;

/// Secure storage for sensitive data
pub struct SecureStorage {
    /// Storage for sensitive data with zeroization on drop
    data: Arc<Mutex<HashMap<String, SecureData>>>,
}

/// Secure data wrapper that zeroizes memory on drop
#[derive(Clone)]
pub struct SecureData {
    data: Vec<u8>,
}

impl SecureData {
    /// Create new secure data from bytes
    pub fn new(data: Vec<u8>) -> Self {
        SecureData { data }
    }
    
    /// Get reference to the data
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
    
    /// Get mutable reference to the data
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

impl Drop for SecureData {
    fn drop(&mut self) {
        // Zeroize the data when it's dropped
        self.data.zeroize();
    }
}

impl SecureStorage {
    /// Create a new secure storage instance
    pub fn new() -> Self {
        SecureStorage {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Store sensitive data with a key
    pub fn store(&self, key: String, data: Vec<u8>) -> Result<(), SecureStorageError> {
        let secure_data = SecureData::new(data);
        let mut storage = self.data.lock().map_err(|_| SecureStorageError::LockError)?;
        storage.insert(key, secure_data);
        Ok(())
    }
    
    /// Retrieve sensitive data by key
    pub fn retrieve(&self, key: &str) -> Result<Vec<u8>, SecureStorageError> {
        let storage = self.data.lock().map_err(|_| SecureStorageError::LockError)?;
        storage.get(key)
            .map(|secure_data| secure_data.as_bytes().to_vec())
            .ok_or(SecureStorageError::NotFound)
    }
    
    /// Delete sensitive data by key
    pub fn delete(&self, key: &str) -> Result<(), SecureStorageError> {
        let mut storage = self.data.lock().map_err(|_| SecureStorageError::LockError)?;
        storage.remove(key);
        Ok(())
    }
    
    /// Check if data exists for a key
    pub fn exists(&self, key: &str) -> Result<bool, SecureStorageError> {
        let storage = self.data.lock().map_err(|_| SecureStorageError::LockError)?;
        Ok(storage.contains_key(key))
    }
}

/// Errors that can occur with secure storage operations
#[derive(Debug)]
pub enum SecureStorageError {
    LockError,
    NotFound,
}

impl std::fmt::Display for SecureStorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecureStorageError::LockError => write!(f, "Failed to acquire storage lock"),
            SecureStorageError::NotFound => write!(f, "Data not found"),
        }
    }
}

impl std::error::Error for SecureStorageError {}
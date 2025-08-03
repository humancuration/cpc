//! Hashing utilities
//!
//! This module provides common hashing functions using SHA-256 algorithm.

use crypto::sha2::Sha256;
use crypto::digest::Digest;
use crate::error::{CommonError, Result};

/// Hash a string using SHA-256
pub fn hash_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(input);
    hasher.result_str()
}

/// Hash a string with a salt using SHA-256
pub fn hash_sha256_with_salt(input: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(input);
    hasher.input_str(salt);
    hasher.result_str()
}

/// Verify a hash against an input string
pub fn verify_hash(input: &str, hash: &str) -> bool {
    hash_sha256(input) == hash
}

/// Verify a hash against an input string with salt
pub fn verify_hash_with_salt(input: &str, salt: &str, hash: &str) -> bool {
    hash_sha256_with_salt(input, salt) == hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_sha256() {
        let input = "hello world";
        let hash = hash_sha256(input);
        
        assert_eq!(hash.len(), 64); // SHA-256 produces 64 hex characters
        assert_eq!(hash, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
    }
    
    #[test]
    fn test_hash_sha256_with_salt() {
        let input = "hello world";
        let salt = "salt123";
        let hash = hash_sha256_with_salt(input, salt);
        
        assert_eq!(hash.len(), 64);
        assert_eq!(hash, "c1f474c9e36b042927a04923112353320d7d1860e96a7320e418b524044193d7");
    }
    
    #[test]
    fn test_verify_hash() {
        let input = "hello world";
        let hash = hash_sha256(input);
        
        assert!(verify_hash(input, &hash));
        assert!(!verify_hash("different input", &hash));
    }
    
    #[test]
    fn test_verify_hash_with_salt() {
        let input = "hello world";
        let salt = "salt123";
        let hash = hash_sha256_with_salt(input, salt);
        
        assert!(verify_hash_with_salt(input, salt, &hash));
        assert!(!verify_hash_with_salt("different input", salt, &hash));
    }
}
// DEPRECATED: This file will be removed on 2025-10-01
// Migrate to common_utils equivalents instead
// DEPRECATED: This file has been migrated to use common_utils directly.
// Date: 2025-08-03
// All functionality has been replaced with direct calls to common_utils::crypto
//! Crypto compatibility shim for integrating common_utils
//!
//! This module provides compatibility between the wallet's custom crypto functions
//! and the common_utils::crypto functions for backward compatibility
//! during the migration process.

#[cfg(feature = "common-utils-integration")]
use common_utils::crypto::{hashing, encryption};

/// Hash a password using SHA-256 with a salt
/// 
/// This function provides backward compatibility by forwarding calls
/// to the common_utils implementation.
#[cfg(feature = "common-utils-integration")]
#[deprecated(since = "0.2.0", note = "Use common_utils::crypto::hashing::hash_sha256_with_salt instead")]
pub fn hash_password(password: &str, salt: &str) -> String {
    hashing::hash_sha256_with_salt(password, salt)
}

/// Hash a password using SHA-256
/// 
/// This function provides backward compatibility by forwarding calls
/// to the common_utils implementation.
#[cfg(feature = "common-utils-integration")]
#[deprecated(since = "0.2.0", note = "Use common_utils::crypto::hashing::hash_sha256 instead")]
pub fn hash_simple(input: &str) -> String {
    hashing::hash_sha256(input)
}

/// Verify a hash against an input string
/// 
/// This function provides backward compatibility by forwarding calls
/// to the common_utils implementation.
#[cfg(feature = "common-utils-integration")]
#[deprecated(since = "0.2.0", note = "Use common_utils::crypto::hashing::verify_hash instead")]
pub fn verify_hash(input: &str, hash: &str) -> bool {
    hashing::verify_hash(input, hash)
}

/// Verify a hash against an input string with salt
/// 
/// This function provides backward compatibility by forwarding calls
/// to the common_utils implementation.
#[cfg(feature = "common-utils-integration")]
#[deprecated(since = "0.2.0", note = "Use common_utils::crypto::hashing::verify_hash_with_salt instead")]
pub fn verify_hash_with_salt(input: &str, salt: &str, hash: &str) -> bool {
    hashing::verify_hash_with_salt(input, salt, hash)
}

/// Fallback implementations when common-utils-integration feature is disabled
#[cfg(not(feature = "common-utils-integration"))]
mod fallback {
    use crypto::{sha2::Sha256, digest::Digest};

    /// Hash a password using SHA-256 with a salt
    pub fn hash_password(password: &str, salt: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.input_str(password);
        hasher.input_str(salt);
        hasher.result_str()
    }

    /// Hash a password using SHA-256
    pub fn hash_simple(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.input_str(input);
        hasher.result_str()
    }

    /// Verify a hash against an input string
    pub fn verify_hash(input: &str, hash: &str) -> bool {
        hash_simple(input) == hash
    }

    /// Verify a hash against an input string with salt
    pub fn verify_hash_with_salt(input: &str, salt: &str, hash: &str) -> bool {
        hash_password(input, salt) == hash
    }
}

#[cfg(not(feature = "common-utils-integration"))]
pub use fallback::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "test_password";
        let salt = "test_salt";
        let hash = hash_password(password, salt);
        
        assert_eq!(hash.len(), 64); // SHA-256 produces 64 hex characters
        assert!(verify_hash_with_salt(password, salt, &hash));
    }
    
    #[test]
    fn test_hash_simple() {
        let input = "test_input";
        let hash = hash_simple(input);
        
        assert_eq!(hash.len(), 64); // SHA-256 produces 64 hex characters
        assert!(verify_hash(input, &hash));
    }
    
    #[test]
    fn test_verify_hash() {
        let input = "test_input";
        let hash = hash_simple(input);
        
        assert!(verify_hash(input, &hash));
        assert!(!verify_hash("different_input", &hash));
    }
    
    #[test]
    fn test_verify_hash_with_salt() {
        let input = "test_input";
        let salt = "test_salt";
        let hash = hash_password(input, salt);
        
        assert!(verify_hash_with_salt(input, salt, &hash));
        assert!(!verify_hash_with_salt("different_input", salt, &hash));
    }
}
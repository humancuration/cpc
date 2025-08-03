//! Cryptography utilities for the CPC platform
//!
//! This module provides common cryptographic functions for hashing and encryption.

pub mod hashing;
pub mod encryption;

// Re-export the main crypto functions for convenience
pub use hashing::{hash_sha256, hash_sha256_with_salt, verify_hash};
pub use encryption::{encrypt, decrypt, generate_key};
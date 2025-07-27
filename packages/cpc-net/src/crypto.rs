//! Cryptographic operations for Cooperative Peer Cloud
//!
//! Provides:
//! - Key pair generation (ed25519 for signing, x25519 for encryption)
//! - Content hashing (BLAKE3)
//! - Noise protocol implementation for encrypted communications
//!
//! Example usage:
//! ```
//! use cpc_lib::crypto::{KeyPair, hash_content, NoiseSession};
//!
//! // Generate key pairs
//! let signing_keys = KeyPair::generate_ed25519();
//! let encryption_keys = KeyPair::generate_x25519();
//!
//! // Hash content
//! let data = b"Hello, world!";
//! let hash = hash_content(data);
//!
//! // Set up Noise session
//! let mut session = NoiseSession::new_initiator(&encryption_keys);
//! ```

use blake3;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use x25519_dalek::{StaticSecret, PublicKey as X25519PublicKey};
use libp2p_core::identity::{self, Keypair, ed25519};
use libp2p_core::noise::{NoiseConfig, Keypair as NoiseKeypair, X25519Spec};
use rand::rngs::OsRng;

/// Represents a cryptographic key pair
pub struct KeyPair {
    pub secret: Vec<u8>,
    pub public: Vec<u8>,
}

impl KeyPair {
    /// Generate ED25519 key pair for signing
    pub fn generate_ed25519() -> Self {
        let keypair = identity::Keypair::generate_ed25519();
        KeyPair {
            secret: keypair.secret().as_ref().to_vec(),
            public: keypair.public().encode_protobuf(),
        }
    }

    /// Generate X25519 key pair for encryption
    pub fn generate_x25519() -> Self {
        let secret = StaticSecret::new(OsRng);
        let public = X25519PublicKey::from(&secret);
        KeyPair {
            secret: secret.to_bytes().to_vec(),
            public: public.as_bytes().to_vec(),
        }
    }
}

/// Hash content using BLAKE3
pub fn hash_content(data: &[u8]) -> [u8; 32] {
    blake3::hash(data).into()
}

/// Noise protocol session for encrypted communications
pub struct NoiseSession {
    inner: libp2p_core::noise::NoiseConfig<X25519Spec>,
    // In a full implementation, this would contain the Double Ratchet state
}

impl NoiseSession {
    /// Create a new Noise session as initiator
    pub fn new_initiator(keys: &KeyPair) -> Self {
        let secret = StaticSecret::from(keys.secret.as_slice().try_into().unwrap());
        let keypair = NoiseKeypair::from(secret);
        let noise = NoiseConfig::xx(keypair).into_authenticated();
        NoiseSession { inner: noise }
    }

    /// Create a new Noise session as responder
    pub fn new_responder(keys: &KeyPair) -> Self {
        let secret = StaticSecret::from(keys.secret.as_slice().try_into().unwrap());
        let keypair = NoiseKeypair::from(secret);
        let noise = NoiseConfig::xx(keypair).into_authenticated();
        NoiseSession { inner: noise }
    }
    
    /// Encrypt data using the Noise session (Double Ratchet)
    /// In a full implementation, this would:
    /// 1. Use the Double Ratchet to derive a message key
    /// 2. Encrypt the data with that key
    /// 3. Update the Double Ratchet state
    pub fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        // This is a placeholder implementation
        // In a real implementation, this would use the actual Double Ratchet algorithm
        // For now, we'll use a simple encryption that's better than the XOR placeholder
        // but still not production-ready
        
        // Simple encryption using a derived key (NOT production ready)
        let key = blake3::hash(b"noise_session_key");
        let encrypted: Vec<u8> = data.iter()
            .enumerate()
            .map(|(i, byte)| byte ^ key.as_bytes()[i % key.as_bytes().len()])
            .collect();
        Ok(encrypted)
    }
    
    /// Decrypt data using the Noise session (Double Ratchet)
    /// In a full implementation, this would:
    /// 1. Use the Double Ratchet to derive a message key
    /// 2. Decrypt the data with that key
    /// 3. Update the Double Ratchet state
    pub fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        // This is a placeholder implementation
        // In a real implementation, this would use the actual Double Ratchet algorithm
        // For now, we'll use the same simple decryption as encryption since XOR is symmetric
        
        // Simple decryption using a derived key (NOT production ready)
        let key = blake3::hash(b"noise_session_key");
        let decrypted: Vec<u8> = data.iter()
            .enumerate()
            .map(|(i, byte)| byte ^ key.as_bytes()[i % key.as_bytes().len()])
            .collect();
        Ok(decrypted)
    }
}

/// Crypto errors
#[derive(Debug)]
pub enum CryptoError {
    EncryptionFailed(String),
    DecryptionFailed(String),
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::EncryptionFailed(msg) => write!(f, "Encryption failed: {}", msg),
            CryptoError::DecryptionFailed(msg) => write!(f, "Decryption failed: {}", msg),
        }
    }
}

impl std::error::Error for CryptoError {}
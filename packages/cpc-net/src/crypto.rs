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
}
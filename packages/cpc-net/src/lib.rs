//! Shared library for Cooperative Peer Cloud components
//!
//! Provides core functionality for:
//! - Cryptographic operations (key generation, hashing, encryption)
//! - Network abstractions (peer discovery, transport, protocols)
//! - Content-addressable storage with metrics
//!
//! ## Example
//! ```
//! use cpc_lib::{crypto, net, storage};
//!
//! // Generate cryptographic keys
//! let signing_keys = crypto::KeyPair::generate_ed25519();
//! let encryption_keys = crypto::KeyPair::generate_x25519();
//!
//! // Set up network
//! let mut network = net::NetworkBuilder::new()
//!     .with_tcp()
//!     .with_quic()
//!     .build();
//!
//! // Initialize storage
//! let mut storage = storage::LruStorage::new(1024 * 1024 * 100); // 100 MB
//! ```

pub mod crypto;
pub mod net;
pub mod storage;

// Re-export key types from modules
pub use crypto::{KeyPair, NoiseSession, hash_content};
pub use net::{NetworkBuilder, Network, NetworkEvent};
pub use storage::{ContentStorage, LruStorage, StorageMetrics, StorageError};
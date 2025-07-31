//! Shared library for Cooperative Peer Cloud components
//!
//! Provides core functionality for:
//! - Cryptographic operations (key generation, hashing, encryption)
//! - Network abstractions (peer discovery, transport, protocols)
//! - Content-addressable storage with metrics
//! - Distributed file hosting (chunking, DHT, peer management)
//!
//! ## Example
//! ```
//! use cpc_lib::{crypto, chunking, dht, peer_manager};
//! 
//! // Generate cryptographic keys
//! let signing_keys = crypto::KeyPair::generate_ed25519();
//! let encryption_keys = crypto::KeyPair::generate_x25519();
//! 
//! // Split file into chunks
//! let chunks = chunking::ChunkingService::chunk_bytes(b"Hello, distributed world!");
//! 
//! // Announce chunk availability in DHT
//! // let dht = dht::DhtService::new(local_peer_id);
//! // dht.announce_chunk(chunk_hash, chunk_size).await?;
//! ```

pub mod crypto;
pub mod net;
pub mod storage;
pub mod secure_storage;
pub mod circuit_breaker;
pub mod community_repo;

// Distributed file hosting modules
pub mod chunking;
pub mod dht;
pub mod peer_manager;

// Re-export key types from modules
pub use crypto::{KeyPair, NoiseSession, hash_content, CryptoError};
pub use net::{NetworkBuilder, Network, NetworkEvent};
pub use storage::{ContentStorage, LruStorage, StorageMetrics, StorageError};
pub use secure_storage::{SecureStorage, SecureData, SecureStorageError};
pub use circuit_breaker::{NetworkCircuitBreaker, CircuitBreakerConfig};
pub use chunking::{ChunkingService, FileChunk, CHUNK_SIZE};
pub use dht::{DhtService, ChunkLocationRecord, DhtStats};
pub use peer_manager::{PeerManager, PeerManagerEvent, PeerConnection};
pub use community_repo::{CommunityRepo, CommunityRepoError};
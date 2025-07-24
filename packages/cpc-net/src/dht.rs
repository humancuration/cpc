//! Distributed Hash Table (DHT) implementation using Kademlia
//!
//! Provides distributed indexing for chunk locations using libp2p Kademlia DHT.
//! Tracks which peers have which chunks for efficient content discovery.

use libp2p_kad::{Kademlia, KademliaConfig, KademliaEvent, Record, RecordKey};
use libp2p_kad::store::MemoryStore;
use libp2p_core::{PeerId, Multiaddr};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Key prefix for chunk location records in the DHT
const CHUNK_LOCATION_PREFIX: &str = "/cpc/chunk/";

/// DHT record for tracking chunk locations
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChunkLocationRecord {
    /// BLAKE3 hash of the chunk
    pub chunk_hash: blake3::Hash,
    /// List of peer IDs that have this chunk
    pub peer_ids: Vec<PeerId>,
    /// Timestamp of last update
    pub timestamp: Instant,
    /// Chunk size in bytes
    pub size: u64,
}

/// Distributed Hash Table service for chunk location tracking
pub struct DhtService {
    kademlia: Kademlia<MemoryStore>,
    local_peer_id: PeerId,
    chunk_locations: HashMap<blake3::Hash, ChunkLocationRecord>,
}

impl DhtService {
    /// Create a new DHT service
    pub fn new(local_peer_id: PeerId) -> Self {
        let mut config = KademliaConfig::default();
        config.set_query_timeout(Duration::from_secs(30));
        config.set_replication_factor(std::num::NonZeroUsize::new(20).unwrap());
        
        let store = MemoryStore::new(local_peer_id);
        let kademlia = Kademlia::with_config(local_peer_id, store, config);
        
        Self {
            kademlia,
            local_peer_id,
            chunk_locations: HashMap::new(),
        }
    }
    
    /// Bootstrap the DHT with bootstrap nodes
    pub fn bootstrap(&mut self, bootstrap_nodes: Vec<(PeerId, Multiaddr)>) {
        for (peer_id, addr) in bootstrap_nodes {
            self.kademlia.add_address(&peer_id, addr);
        }
        
        // Start bootstrap process
        self.kademlia.bootstrap().ok();
    }
    
    /// Announce that we have a chunk available
    pub async fn announce_chunk(&mut self, chunk_hash: blake3::Hash, size: u64) -> Result<(), DhtError> {
        let record_key = self.chunk_key_to_record_key(&chunk_hash);
        
        // Create or update chunk location record
        let record = self.chunk_locations.entry(chunk_hash)
            .or_insert_with(|| ChunkLocationRecord {
                chunk_hash,
                peer_ids: Vec::new(),
                timestamp: Instant::now(),
                size,
            });
        
        // Add our peer ID if not already present
        if !record.peer_ids.contains(&self.local_peer_id) {
            record.peer_ids.push(self.local_peer_id);
            record.timestamp = Instant::now();
        }
        
        // Serialize and store in DHT
        let serialized = bincode::serialize(&record)
            .map_err(|_| DhtError::SerializationError)?;
        
        let dht_record = Record::new(record_key, serialized);
        self.kademlia.put_record(dht_record, None)
            .map_err(|_| DhtError::PutError)?;
        
        Ok(())
    }
    
    /// Find peers that have a specific chunk
    pub async fn find_chunk_peers(&mut self, chunk_hash: blake3::Hash) -> Result<Vec<PeerId>, DhtError> {
        let record_key = self.chunk_key_to_record_key(&chunk_hash);
        
        // Query the DHT for chunk locations
        let query_id = self.kademlia.get_record(record_key, Default::default());
        
        // Wait for response (simplified - in real implementation use async channels)
        // For now, return from local cache
        if let Some(record) = self.chunk_locations.get(&chunk_hash) {
            Ok(record.peer_ids.clone())
        } else {
            Ok(Vec::new())
        }
    }
    
    /// Get chunk metadata including size and available peers
    pub async fn get_chunk_info(&mut self, chunk_hash: blake3::Hash) -> Result<Option<ChunkLocationRecord>, DhtError> {
        let record_key = self.chunk_key_to_record_key(&chunk_hash);
        
        // Query DHT for the record
        let query_id = self.kademlia.get_record(record_key, Default::default());
        
        // For now, return from local cache
        Ok(self.chunk_locations.get(&chunk_hash).cloned())
    }
    
    /// Remove chunk announcement (when we no longer have the chunk)
    pub async fn remove_chunk_announcement(&mut self, chunk_hash: blake3::Hash) -> Result<(), DhtError> {
        if let Some(record) = self.chunk_locations.get_mut(&chunk_hash) {
            record.peer_ids.retain(|&peer_id| peer_id != self.local_peer_id);
            
            if record.peer_ids.is_empty() {
                self.chunk_locations.remove(&chunk_hash);
            } else {
                // Update the record in DHT
                let record_key = self.chunk_key_to_record_key(&chunk_hash);
                let serialized = bincode::serialize(&record)
                    .map_err(|_| DhtError::SerializationError)?;
                
                let dht_record = Record::new(record_key, serialized);
                self.kademlia.put_record(dht_record, None)
                    .map_err(|_| DhtError::PutError)?;
            }
        }
        
        Ok(())
    }
    
    /// Handle incoming DHT events
    pub fn handle_dht_event(&mut self, event: KademliaEvent) {
        match event {
            KademliaEvent::InboundRequest { request } => {
                // Handle incoming requests for chunk locations
                tracing::debug!("Received DHT inbound request: {:?}", request);
            }
            KademliaEvent::OutboundQueryCompleted { result, .. } => {
                // Handle query results
                tracing::debug!("DHT query completed: {:?}", result);
            }
            _ => {}
        }
    }
    
    /// Convert chunk hash to DHT record key
    fn chunk_key_to_record_key(&self, chunk_hash: &blake3::Hash) -> RecordKey {
        let key_str = format!("{}{}", CHUNK_LOCATION_PREFIX, chunk_hash.to_hex());
        RecordKey::new(&key_str)
    }
    
    /// Get statistics about the DHT
    pub fn get_stats(&self) -> DhtStats {
        DhtStats {
            known_chunks: self.chunk_locations.len(),
            total_peers: self.chunk_locations.values()
                .flat_map(|r| &r.peer_ids)
                .collect::<std::collections::HashSet<_>>()
                .len(),
        }
    }
}

/// DHT service statistics
#[derive(Debug, Clone)]
pub struct DhtStats {
    pub known_chunks: usize,
    pub total_peers: usize,
}

/// DHT-related errors
#[derive(Debug, thiserror::Error)]
pub enum DhtError {
    #[error("Serialization error")]
    SerializationError,
    
    #[error("Failed to put record in DHT")]
    PutError,
    
    #[error("Failed to get record from DHT")]
    GetError,
    
    #[error("Record not found")]
    NotFound,
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p_core::identity;
    
    #[tokio::test]
    async fn test_announce_and_find_chunk() {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        let mut dht = DhtService::new(local_peer_id);
        
        let chunk_hash = blake3::hash(b"test chunk data");
        let size = 1024;
        
        // Announce chunk
        dht.announce_chunk(chunk_hash, size).await.unwrap();
        
        // Find chunk peers
        let peers = dht.find_chunk_peers(chunk_hash).await.unwrap();
        assert!(peers.contains(&local_peer_id));
        
        // Get chunk info
        let info = dht.get_chunk_info(chunk_hash).await.unwrap();
        assert!(info.is_some());
        assert_eq!(info.unwrap().size, size);
    }
}
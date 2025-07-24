//! File hosting service for distributed chunk operations
//!
//! Provides gRPC service for chunk operations including upload, download, and
//! chunk location management in the distributed file hosting system.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};
use cpc_lib::{chunking, dht};
use blake3::Hash as Blake3Hash;
use libp2p_core::PeerId;

// Define protobuf types for gRPC service
tonic::include_proto!("file_hosting");

/// File hosting service implementation
#[derive(Debug)]
pub struct FileHostingService {
    /// Local chunk storage
    chunks: Arc<RwLock<HashMap<Blake3Hash, chunking::FileChunk>>>,
    
    /// DHT service for chunk location tracking
    dht_service: Arc<RwLock<Option<dht::DhtService>>>,
    
    /// Local peer ID
    local_peer_id: PeerId,
}

impl FileHostingService {
    /// Create a new file hosting service
    pub fn new(local_peer_id: PeerId) -> Self {
        Self {
            chunks: Arc::new(RwLock::new(HashMap::new())),
            dht_service: Arc::new(RwLock::new(None)),
            local_peer_id,
        }
    }
    
    /// Initialize the service with DHT
    pub async fn init_dht(&self, dht_service: dht::DhtService) {
        *self.dht_service.write().await = Some(dht_service);
    }
    
    /// Store a chunk locally
    pub async fn store_chunk(&self, chunk: chunking::FileChunk) -> Result<(), Status> {
        let mut chunks = self.chunks.write().await;
        chunks.insert(chunk.hash, chunk.clone());
        
        // Announce chunk availability in DHT
        if let Some(dht) = self.dht_service.read().await.as_ref() {
            dht.announce_chunk(chunk.hash, chunk.data.len() as u64).await
                .map_err(|e| Status::internal(format!("Failed to announce chunk: {}", e)))?;
        }
        
        Ok(())
    }
    
    /// Retrieve a chunk by hash
    pub async fn get_chunk(&self, chunk_hash: Blake3Hash) -> Result<Option<chunking::FileChunk>, Status> {
        let chunks = self.chunks.read().await;
        Ok(chunks.get(&chunk_hash).cloned())
    }
    
    /// Find peers that have a specific chunk
    pub async fn find_chunk_peers(&self, chunk_hash: Blake3Hash) -> Result<Vec<PeerId>, Status> {
        if let Some(dht) = self.dht_service.read().await.as_ref() {
            dht.find_chunk_peers(chunk_hash).await
                .map_err(|e| Status::internal(format!("Failed to find chunk peers: {}", e)))
        } else {
            Err(Status::failed_precondition("DHT service not initialized"))
        }
    }
}

#[tonic::async_trait]
impl file_hosting_server::FileHosting for FileHostingService {
    /// Upload a chunk to the network
    async fn upload_chunk(
        &self,
        request: Request<UploadChunkRequest>,
    ) -> Result<Response<UploadChunkResponse>, Status> {
        let req = request.into_inner();
        
        let chunk = chunking::FileChunk {
            hash: Blake3Hash::from_hex(&req.chunk_hash)
                .map_err(|_| Status::invalid_argument("Invalid chunk hash"))?,
            data: req.data.into(),
            index: req.index,
            total_chunks: req.total_chunks,
        };
        
        self.store_chunk(chunk).await?;
        
        Ok(Response::new(UploadChunkResponse {
            success: true,
            message: "Chunk uploaded successfully".to_string(),
        }))
    }
    
    /// Download a chunk from the network
    async fn download_chunk(
        &self,
        request: Request<DownloadChunkRequest>,
    ) -> Result<Response<DownloadChunkResponse>, Status> {
        let req = request.into_inner();
        
        let chunk_hash = Blake3Hash::from_hex(&req.chunk_hash)
            .map_err(|_| Status::invalid_argument("Invalid chunk hash"))?;
        
        if let Some(chunk) = self.get_chunk(chunk_hash).await? {
            Ok(Response::new(DownloadChunkResponse {
                data: chunk.data.to_vec(),
                index: chunk.index,
                total_chunks: chunk.total_chunks,
                chunk_hash: chunk.hash.to_hex().to_string(),
            }))
        } else {
            Err(Status::not_found("Chunk not found locally"))
        }
    }
    
    /// Find peers that have a specific chunk
    async fn find_chunk_peers(
        &self,
        request: Request<FindChunkPeersRequest>,
    ) -> Result<Response<FindChunkPeersResponse>, Status> {
        let req = request.into_inner();
        
        let chunk_hash = Blake3Hash::from_hex(&req.chunk_hash)
            .map_err(|_| Status::invalid_argument("Invalid chunk hash"))?;
        
        let peers = self.find_chunk_peers(chunk_hash).await?;
        
        let peer_ids = peers.into_iter()
            .map(|peer_id| peer_id.to_string())
            .collect();
        
        Ok(Response::new(FindChunkPeersResponse { peer_ids }))
    }
    
    /// Get chunk metadata
    async fn get_chunk_info(
        &self,
        request: Request<GetChunkInfoRequest>,
    ) -> Result<Response<GetChunkInfoResponse>, Status> {
        let req = request.into_inner();
        
        let chunk_hash = Blake3Hash::from_hex(&req.chunk_hash)
            .map_err(|_| Status::invalid_argument("Invalid chunk hash"))?;
        
        if let Some(dht) = self.dht_service.read().await.as_ref() {
            if let Some(info) = dht.get_chunk_info(chunk_hash).await
                .map_err(|e| Status::internal(format!("Failed to get chunk info: {}", e)))?
            {
                Ok(Response::new(GetChunkInfoResponse {
                    chunk_hash: info.chunk_hash.to_hex().to_string(),
                    size: info.size,
                    peer_count: info.peer_ids.len() as u64,
                }))
            } else {
                Err(Status::not_found("Chunk info not found"))
            }
        } else {
            Err(Status::failed_precondition("DHT service not initialized"))
        }
    }
}

/// File hosting service server
pub struct FileHostingServer {
    service: FileHostingService,
}

impl FileHostingServer {
    pub fn new(service: FileHostingService) -> Self {
        Self { service }
    }
    
    pub fn into_service(self) -> file_hosting_server::FileHostingServer<FileHostingService> {
        file_hosting_server::FileHostingServer::new(self.service)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blake3::Hasher;
    
    #[tokio::test]
    async fn test_file_hosting_service() {
        let local_peer_id = PeerId::random();
        let service = FileHostingService::new(local_peer_id);
        
        // Test chunk storage
        let data = b"Test chunk data";
        let chunk_hash = blake3::hash(data);
        let chunk = chunking::FileChunk {
            hash: chunk_hash,
            data: data.to_vec().into(),
            index: 0,
            total_chunks: 1,
        };
        
        service.store_chunk(chunk.clone()).await.unwrap();
        
        let retrieved = service.get_chunk(chunk_hash).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().hash, chunk_hash);
    }
}
//! File chunking service for distributed file hosting
//!
//! Provides functionality to split files into 5MB chunks and compute BLAKE3 hashes
//! for content-addressable storage in the P2P network.

use std::path::Path;
use std::io::{Read, Write};
use std::fs::File;
use blake3::Hasher;
use bytes::Bytes;

/// Size of each chunk in bytes (5MB)
pub const CHUNK_SIZE: usize = 5 * 1024 * 1024;

/// Represents a file chunk with its metadata
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileChunk {
    /// BLAKE3 hash of the chunk content
    pub hash: blake3::Hash,
    /// Chunk data
    pub data: Bytes,
    /// Chunk index within the original file
    pub index: u64,
    /// Total number of chunks in the original file
    pub total_chunks: u64,
}

/// File chunking service
pub struct ChunkingService;

impl ChunkingService {
    /// Split a file into 5MB chunks
    pub fn chunk_file<P: AsRef<Path>>(path: P) -> Result<Vec<FileChunk>, ChunkingError> {
        let mut file = File::open(path)?;
        let mut chunks = Vec::new();
        let mut buffer = vec![0u8; CHUNK_SIZE];
        let mut index = 0;
        
        // Get file size for total chunks calculation
        let file_size = file.metadata()?.len();
        let total_chunks = ((file_size + CHUNK_SIZE as u64 - 1) / CHUNK_SIZE as u64) as u64;
        
        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            
            let chunk_data = Bytes::from(buffer[..bytes_read].to_vec());
            let hash = blake3::hash(&chunk_data);
            
            chunks.push(FileChunk {
                hash,
                data: chunk_data,
                index,
                total_chunks,
            });
            
            index += 1;
        }
        
        Ok(chunks)
    }
    
    /// Split bytes into chunks
    pub fn chunk_bytes(data: &[u8]) -> Vec<FileChunk> {
        let mut chunks = Vec::new();
        let mut offset = 0;
        let mut index = 0;
        let total_chunks = ((data.len() + CHUNK_SIZE - 1) / CHUNK_SIZE) as u64;
        
        while offset < data.len() {
            let end = std::cmp::min(offset + CHUNK_SIZE, data.len());
            let chunk_data = Bytes::from(data[offset..end].to_vec());
            let hash = blake3::hash(&chunk_data);
            
            chunks.push(FileChunk {
                hash,
                data: chunk_data,
                index,
                total_chunks,
            });
            
            offset = end;
            index += 1;
        }
        
        chunks
    }
    
    /// Reassemble chunks into a single file
    pub fn reassemble_chunks<P: AsRef<Path>>(chunks: &[FileChunk], output_path: P) -> Result<(), ChunkingError> {
        // Sort chunks by index
        let mut sorted_chunks = chunks.to_vec();
        sorted_chunks.sort_by_key(|c| c.index);
        
        let mut file = File::create(output_path)?;
        
        for chunk in sorted_chunks {
            file.write_all(&chunk.data)?;
        }
        
        Ok(())
    }
    
    /// Validate chunk integrity using BLAKE3 hash
    pub fn validate_chunk(chunk: &FileChunk) -> bool {
        let computed_hash = blake3::hash(&chunk.data);
        computed_hash == chunk.hash
    }
}

/// Chunking-related errors
#[derive(Debug, thiserror::Error)]
pub enum ChunkingError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Invalid chunk data")]
    InvalidChunk,
    
    #[error("Missing chunks")]
    MissingChunks,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_chunk_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let data = vec![0u8; 1024 * 1024 * 10]; // 10MB
        temp_file.write_all(&data).unwrap();
        
        let chunks = ChunkingService::chunk_file(temp_file.path()).unwrap();
        assert_eq!(chunks.len(), 2); // 10MB / 5MB = 2 chunks
        
        for chunk in &chunks {
            assert!(ChunkingService::validate_chunk(chunk));
        }
    }
    
    #[test]
    fn test_chunk_bytes() {
        let data = vec![1u8; 1024 * 1024 * 7]; // 7MB
        let chunks = ChunkingService::chunk_bytes(&data);
        assert_eq!(chunks.len(), 2); // 7MB / 5MB = 2 chunks
        
        let mut reassembled = Vec::new();
        for chunk in chunks {
            reassembled.extend_from_slice(&chunk.data);
        }
        
        assert_eq!(reassembled, data);
    }
}
use chrono::Utc;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;

/// Metadata for a stored file
#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    pub content_address: String, // Merkle root hash
    pub original_path: String,
    pub size: u64,
    pub chunk_count: usize,
    pub mime_type: String,
    pub created_at: i64,
    pub version: u64,
}

/// Metadata for an individual chunk
#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMetadata {
    pub file_content_address: String,
    pub index: usize,
    pub hash: String,
    pub size: usize,
    pub nonce: Vec<u8>,
}

/// SQLite-based metadata storage
pub struct MetadataStore {
    conn: Mutex<Connection>,
}

impl MetadataStore {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS file_metadata (
                content_address TEXT PRIMARY KEY,
                original_path TEXT NOT NULL,
                size INTEGER NOT NULL,
                chunk_count INTEGER NOT NULL,
                mime_type TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                version INTEGER NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS chunk_metadata (
                id INTEGER PRIMARY KEY,
                file_content_address TEXT NOT NULL,
                index INTEGER NOT NULL,
                hash TEXT NOT NULL,
                size INTEGER NOT NULL,
                nonce BLOB NOT NULL,
                FOREIGN KEY(file_content_address) REFERENCES file_metadata(content_address)
            )",
            [],
        )?;

        Ok(MetadataStore {
            conn: Mutex::new(conn),
        })
    }

    /// Store file metadata
    pub fn store_file_metadata(&self, metadata: &FileMetadata) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO file_metadata (content_address, original_path, size, chunk_count, mime_type, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                metadata.content_address,
                metadata.original_path,
                metadata.size,
                metadata.chunk_count,
                metadata.mime_type,
                metadata.created_at,
                metadata.version as i64
            ],
        )?;
        Ok(())
    }

    /// Store chunk metadata
    pub fn store_chunk_metadata(&self, metadata: &ChunkMetadata) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO chunk_metadata (file_content_address, index, hash, size, nonce)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                metadata.file_content_address,
                metadata.index,
                metadata.hash,
                metadata.size,
                metadata.nonce
            ],
        )?;
        Ok(())
    }

    /// Get specific version of file metadata
    pub fn get_versioned_metadata(&self, original_path: &str, version: u64) -> Result<FileMetadata> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT content_address, original_path, size, chunk_count, mime_type, created_at, version
             FROM file_metadata WHERE original_path = ?1 AND version = ?2",
        )?;
        
        stmt.query_row(params![original_path, version as i64], |row| {
            Ok(FileMetadata {
                content_address: row.get(0)?,
                original_path: row.get(1)?,
                size: row.get(2)?,
                chunk_count: row.get(3)?,
                mime_type: row.get(4)?,
                created_at: row.get(5)?,
                version: row.get(6)? as u64,
            })
        })
    }

    /// Get file metadata by content address
    pub fn get_file_metadata(&self, content_address: &str) -> Result<FileMetadata> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT content_address, original_path, size, chunk_count, mime_type, created_at, version
             FROM file_metadata WHERE content_address = ?1",
        )?;
        
        stmt.query_row(params![content_address], |row| {
            Ok(FileMetadata {
                content_address: row.get(0)?,
                original_path: row.get(1)?,
                size: row.get(2)?,
                chunk_count: row.get(3)?,
                mime_type: row.get(4)?,
                created_at: row.get(5)?,
                version: row.get(6)? as u64,
            })
        })
    }

    /// Get all chunks for a file
    pub fn get_chunks_for_file(&self, content_address: &str) -> Result<Vec<ChunkMetadata>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT file_content_address, index, hash, size, nonce
             FROM chunk_metadata WHERE file_content_address = ?1 ORDER BY index",
        )?;
        
        let rows = stmt.query_map(params![content_address], |row| {
            Ok(ChunkMetadata {
                file_content_address: row.get(0)?,
                index: row.get(1)?,
                hash: row.get(2)?,
                size: row.get(3)?,
                nonce: row.get(4)?,
            })
        })?;
        
        rows.collect()
    }
}
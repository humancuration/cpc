//! Repository implementation for media

use shared_packages::messenger::models::{MediaReference, MediaType};
use shared_packages::messenger::errors::MessengerError;
use sqlx::PgPool;
use uuid::Uuid;
use std::sync::Arc;

/// Repository for media operations
pub struct MediaRepository {
    db_pool: Arc<PgPool>,
}

impl MediaRepository {
    /// Create a new MediaRepository
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
    
    /// Store a media reference
    pub async fn store_media(&self, media: &MediaReference, owner_id: Uuid, encryption_key: &[u8], iv: &[u8]) -> Result<(), MessengerError> {
        let mut conn = self.db_pool.acquire().await
            .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
            
        let media_type_str = match media.media_type {
            MediaType::Image => "image",
            MediaType::Document => "document",
            MediaType::Audio => "audio",
            MediaType::Video => "video",
        };
        
        sqlx::query!(
            r#"
            INSERT INTO media (id, owner_id, media_type, storage_path, encryption_key, iv, thumbnail_id, original_filename, size_bytes)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            media.id,
            owner_id,
            media_type_str,
            media.storage_location,
            encryption_key,
            iv,
            media.thumbnail.as_ref().map(|t| t.storage_location.clone()),
            media.filename,
            media.size_bytes as i64
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
        
        Ok(())
    }
    
    /// Get a media reference by ID
    pub async fn get_media(&self, media_id: Uuid) -> Result<MediaReference, MessengerError> {
        let mut conn = self.db_pool.acquire().await
            .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
            
        let row = sqlx::query!(
            r#"
            SELECT id, owner_id, media_type, storage_path, thumbnail_id, original_filename, size_bytes, created_at
            FROM media
            WHERE id = $1
            "#,
            media_id
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| MessengerError::StorageError { message: e.to_string() })?
        .ok_or(MessengerError::MediaNotFound { id: media_id })?;
        
        let media_type = match row.media_type.as_str() {
            "image" => MediaType::Image,
            "document" => MediaType::Document,
            "audio" => MediaType::Audio,
            "video" => MediaType::Video,
            _ => return Err(MessengerError::InvalidInput { 
                message: format!("Unknown media type: {}", row.media_type) 
            }),
        };
        
        Ok(MediaReference {
            id: row.id,
            media_type,
            storage_location: row.storage_path,
            thumbnail: row.thumbnail_id.map(|id| crate::models::ThumbnailReference {
                storage_location: id, // This is a simplification, we'd need to look up the actual thumbnail
                width: 0, // Placeholder
                height: 0, // Placeholder
            }),
            size_bytes: row.size_bytes as u64,
            filename: row.original_filename,
        })
    }
    
    /// Delete a media reference
    pub async fn delete_media(&self, media_id: Uuid) -> Result<(), MessengerError> {
        let mut conn = self.db_pool.acquire().await
            .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
            
        sqlx::query!(
            r#"
            DELETE FROM media
            WHERE id = $1
            "#,
            media_id
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
        
        Ok(())
    }
}
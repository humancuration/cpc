//! Media storage for the Messenger application

use std::path::Path;
use tokio::fs;
use uuid::Uuid;
use tracing::{trace, debug, error};

use messenger_domain::{
    models::{MediaReference, MediaType},
    errors::MessengerError,
};

/// Media storage service
pub struct MediaStorage {
    /// Base path for storing media files
    base_path: String,
    
    /// Maximum file size in bytes (100MB default)
    max_file_size: u64,
}

impl MediaStorage {
    /// Create a new media storage service
    pub fn new(base_path: String) -> Self {
        Self {
            base_path,
            max_file_size: 100 * 1024 * 1024, // 100MB
        }
    }
    
    /// Set the maximum file size
    pub fn with_max_file_size(mut self, max_file_size: u64) -> Self {
        self.max_file_size = max_file_size;
        self
    }
    
    /// Store media data
    pub async fn store_media(&self, media_data: Vec<u8>, media_type: MediaType) -> Result<MediaReference, MessengerError> {
        trace!("Storing media data of {} bytes", media_data.len());
        
        // Check file size
        if media_data.len() as u64 > self.max_file_size {
            return Err(MessengerError::MediaUploadFailed {
                message: format!("File too large: {} bytes (max: {} bytes)", media_data.len(), self.max_file_size),
            });
        }
        
        // Generate a unique ID for the media
        let media_id = Uuid::new_v4();
        
        // Determine file extension based on media type
        let extension = match media_type {
            MediaType::Image => "jpg",
            MediaType::Document => "pdf",
            MediaType::Audio => "opus",
            MediaType::Video => "webm",
        };
        
        // Create storage path
        let storage_path = format!("{}/{}.{}", self.base_path, media_id, extension);
        
        // Ensure the directory exists
        if let Some(parent) = Path::new(&storage_path).parent() {
            fs::create_dir_all(parent).await
                .map_err(|e| MessengerError::MediaUploadFailed {
                    message: format!("Failed to create directory: {}", e),
                })?;
        }
        
        // Write the media data to file
        fs::write(&storage_path, media_data).await
            .map_err(|e| MessengerError::MediaUploadFailed {
                message: format!("Failed to write media file: {}", e),
            })?;
        
        // Create media reference
        let media_reference = MediaReference {
            id: media_id,
            media_type,
            storage_location: storage_path,
            thumbnail: None,
            size_bytes: media_data.len() as u64,
            filename: None,
        };
        
        debug!("Stored media {} at {}", media_id, media_reference.storage_location);
        Ok(media_reference)
    }
    
    /// Retrieve media data
    pub async fn retrieve_media(&self, media_reference: &MediaReference) -> Result<Vec<u8>, MessengerError> {
        trace!("Retrieving media {}", media_reference.id);
        
        // Read the media data from file
        let media_data = fs::read(&media_reference.storage_location).await
            .map_err(|e| MessengerError::MediaNotFound {
                id: media_reference.id,
            })?;
        
        debug!("Retrieved media {} ({} bytes)", media_reference.id, media_data.len());
        Ok(media_data)
    }
    
    /// Delete media
    pub async fn delete_media(&self, media_reference: &MediaReference) -> Result<(), MessengerError> {
        trace!("Deleting media {}", media_reference.id);
        
        // Delete the media file
        fs::remove_file(&media_reference.storage_location).await
            .map_err(|e| MessengerError::MediaNotFound {
                id: media_reference.id,
            })?;
        
        // If there's a thumbnail, delete it too
        if let Some(thumbnail) = &media_reference.thumbnail {
            if let Err(e) = fs::remove_file(&thumbnail.storage_location).await {
                error!("Failed to delete thumbnail for media {}: {}", media_reference.id, e);
            }
        }
        
        debug!("Deleted media {}", media_reference.id);
        Ok(())
    }
    
    /// Generate a thumbnail for an image
    pub async fn generate_thumbnail(&self, media_reference: &MediaReference) -> Result<crate::database::ThumbnailReference, MessengerError> {
        // This is a placeholder implementation
        // In a real implementation, we would use an image processing library
        // to generate a thumbnail and store it
        
        // For now, we'll just create a placeholder thumbnail reference
        let thumbnail_reference = crate::database::ThumbnailReference {
            storage_location: format!("{}.thumb.jpg", media_reference.storage_location),
            width: 100,
            height: 100,
        };
        
        // Create a placeholder thumbnail file
        let placeholder_data = vec![0; 1024]; // 1KB of placeholder data
        fs::write(&thumbnail_reference.storage_location, placeholder_data).await
            .map_err(|e| MessengerError::MediaUploadFailed {
                message: format!("Failed to write thumbnail file: {}", e),
            })?;
        
        Ok(thumbnail_reference)
    }
}

/// Server-side encryption for media
pub struct MediaEncryption {
    /// Encryption key
    key: Vec<u8>,
}

impl MediaEncryption {
    /// Create a new media encryption service
    pub fn new(key: Vec<u8>) -> Self {
        Self { key }
    }
    
    /// Encrypt media data
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, MessengerError> {
        // In a real implementation, we would use a proper encryption library
        // For now, we'll just XOR with the key as a placeholder
        let encrypted: Vec<u8> = data.iter()
            .enumerate()
            .map(|(i, byte)| byte ^ self.key[i % self.key.len()])
            .collect();
        
        Ok(encrypted)
    }
    
    /// Decrypt media data
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, MessengerError> {
        // XOR encryption is symmetric, so decryption is the same as encryption
        self.encrypt(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    
    #[tokio::test]
    async fn test_store_and_retrieve_media() {
        let temp_dir = tempfile::tempdir().unwrap();
        let base_path = temp_dir.path().to_str().unwrap().to_string();
        
        let media_storage = MediaStorage::new(base_path);
        
        let media_data = vec![1, 2, 3, 4, 5];
        let media_reference = media_storage
            .store_media(media_data.clone(), MediaType::Image)
            .await
            .unwrap();
        
        let retrieved_data = media_storage
            .retrieve_media(&media_reference)
            .await
            .unwrap();
        
        assert_eq!(media_data, retrieved_data);
    }
    
    #[tokio::test]
    async fn test_delete_media() {
        let temp_dir = tempfile::tempdir().unwrap();
        let base_path = temp_dir.path().to_str().unwrap().to_string();
        
        let media_storage = MediaStorage::new(base_path);
        
        let media_data = vec![1, 2, 3, 4, 5];
        let media_reference = media_storage
            .store_media(media_data.clone(), MediaType::Image)
            .await
            .unwrap();
        
        // Verify the file exists
        assert!(std::path::Path::new(&media_reference.storage_location).exists());
        
        // Delete the media
        media_storage
            .delete_media(&media_reference)
            .await
            .unwrap();
        
        // Verify the file no longer exists
        assert!(!std::path::Path::new(&media_reference.storage_location).exists());
    }
    
    #[test]
    fn test_media_encryption() {
        let key = b"testkey".to_vec();
        let encryption = MediaEncryption::new(key);
        
        let original_data = b"Hello, World!".to_vec();
        let encrypted_data = encryption.encrypt(&original_data).unwrap();
        let decrypted_data = encryption.decrypt(&encrypted_data).unwrap();
        
        assert_eq!(original_data, decrypted_data);
    }
}
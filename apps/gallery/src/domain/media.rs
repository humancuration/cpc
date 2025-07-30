// Gallery domain - Media entity
// This file defines the Media entity and related business logic

use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::path::Path;

/// Represents a media file (image, video, audio) in the gallery system
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Media {
    /// UUID v7 for federation
    pub id: Uuid,
    /// Path to media file
    pub file_path: String,
    /// Media type enum
    pub file_type: MediaType,
    /// UTC timestamp
    pub upload_date: DateTime<Utc>,
    /// Owner UUID
    pub owner_id: Uuid,
    /// SHA-256 of original file
    pub original_hash: String,
    /// Path to transcoded version
    pub transcoded_path: Option<String>,
}

/// Enum representing different media types
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum MediaType {
    Image,
    Video,
    Audio,
    Document,       // For PDFs and other documents
    Other(String),  // For future extension
}

/// Error types for media operations
#[derive(Debug, thiserror::Error)]
pub enum MediaError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Invalid owner")]
    InvalidOwner,
    #[error("Unsupported media type")]
    UnsupportedMediaType,
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Media validator service
pub struct MediaValidator;

impl MediaValidator {
    /// Check if media type is supported for transcoding
    pub fn is_supported_type(media: &Media) -> bool {
        matches!(media.file_type, MediaType::Video | MediaType::Audio)
    }

    /// Validate media for transcoding
    pub fn validate(media: &Media) -> Result<(), MediaError> {
        if !Self::is_supported_type(media) {
            return Err(MediaError::UnsupportedMediaType);
        }
        Ok(())
    }
}

impl Media {
    /// Create a new media entity
    pub fn new(
        file_path: String,
        file_type: MediaType,
        owner_id: Uuid,
        original_hash: String,
    ) -> Result<Self, MediaError> {
        // Validate file exists and is accessible
        if !Path::new(&file_path).exists() {
            return Err(MediaError::FileNotFound(file_path.clone()));
        }

        Ok(Self {
            id: Uuid::now_v7(),
            file_path,
            file_type,
            upload_date: Utc::now(),
            owner_id,
            original_hash,
            transcoded_path: None,
        })
    }

    /// Set the transcoded path for this media
    pub fn set_transcoded_path(&mut self, path: String) {
        self.transcoded_path = Some(path);
    }

    /// Get the media type
    pub fn get_media_type(&self) -> &MediaType {
        &self.file_type
    }

    /// Check if media is viewable in web browser
    pub fn is_viewable(&self) -> bool {
        match self.file_type {
            MediaType::Image => {
                // Check if image is in web-compatible format (WebP, PNG, JPEG)
                self.file_path.ends_with(".webp")
                    || self.file_path.ends_with(".png")
                    || self.file_path.ends_with(".jpg")
                    || self.file_path.ends_with(".jpeg")
            }
            MediaType::Video => {
                // Must be in WebM/AV1 format
                self.file_path.ends_with(".webm") && self.transcoded_path.is_some()
            }
            MediaType::Audio => {
                // Must be in WebM/Opus format
                self.file_path.ends_with(".webm") && self.transcoded_path.is_some()
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_creation() {
        let owner_id = Uuid::new_v4();
        let file_path = "test.jpg".to_string();
        let hash = "test_hash".to_string();

        // Create a mock file for testing
        std::fs::write(&file_path, "test content").unwrap();

        let media = Media::new(
            file_path.clone(),
            MediaType::Image,
            owner_id,
            hash,
        );

        assert!(media.is_ok());
        let media = media.unwrap();
        assert_eq!(media.file_path, file_path);
        assert_eq!(media.file_type, MediaType::Image);
        assert_eq!(media.owner_id, owner_id);

        // Clean up
        std::fs::remove_file("test.jpg").unwrap();
    }

    #[test]
    fn test_media_creation_file_not_found() {
        let owner_id = Uuid::new_v4();
        let file_path = "nonexistent.jpg".to_string();
        let hash = "test_hash".to_string();

        let media = Media::new(
            file_path,
            MediaType::Image,
            owner_id,
            hash,
        );

        assert!(matches!(media, Err(MediaError::FileNotFound(_))));
    }

    #[test]
    fn test_is_viewable() {
        let owner_id = Uuid::new_v4();
        let hash = "test_hash".to_string();

        // Test viewable image
        let image_media = Media {
            id: Uuid::new_v4(),
            file_path: "test.jpg".to_string(),
            file_type: MediaType::Image,
            upload_date: Utc::now(),
            owner_id,
            original_hash: hash.clone(),
            transcoded_path: None,
        };
        assert!(image_media.is_viewable());

        // Test viewable video
        let video_media = Media {
            id: Uuid::new_v4(),
            file_path: "test.webm".to_string(),
            file_type: MediaType::Video,
            upload_date: Utc::now(),
            owner_id,
            original_hash: hash.clone(),
            transcoded_path: None,
        };
        assert!(video_media.is_viewable());

        // Test non-viewable document
        let doc_media = Media {
            id: Uuid::new_v4(),
            file_path: "test.pdf".to_string(),
            file_type: MediaType::Document,
            upload_date: Utc::now(),
            owner_id,
            original_hash: hash,
            transcoded_path: None,
        };
        assert!(!doc_media.is_viewable());
    }
}
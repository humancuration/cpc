// Gallery domain - Album entity
// This file defines the Album entity and related business logic

use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Represents a collection of media items
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Album {
    /// Album UUID
    pub id: Uuid,
    /// Album name
    pub name: String,
    /// Optional description
    pub description: String,
    /// Owner UUID
    pub owner_id: Uuid,
    /// Creation timestamp
    pub created_date: DateTime<Utc>,
    /// Cover media reference
    pub cover_media_id: Option<Uuid>,
}

/// Represents the relationship between an album and a media item
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AlbumMedia {
    /// Album reference
    pub album_id: Uuid,
    /// Media reference
    pub media_id: Uuid,
    /// Order in album
    pub position: u32,
    /// When added to album
    pub added_date: DateTime<Utc>,
}

/// Error types for album operations
#[derive(Debug, thiserror::Error)]
pub enum AlbumError {
    #[error("Media not found: {0}")]
    MediaNotFound(Uuid),
    #[error("Album not found: {0}")]
    AlbumNotFound(Uuid),
    #[error("Media not in album: album_id={0}, media_id={1}")]
    MediaNotInAlbum(Uuid, Uuid),
    #[error("Invalid position: {0}")]
    InvalidPosition(u32),
    #[error("Database error: {0}")]
    DatabaseError(String),
}

impl Album {
    /// Create a new album
    pub fn new(name: String, description: String, owner_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            owner_id,
            created_date: Utc::now(),
            cover_media_id: None,
        }
    }

    /// Add media to the album
    pub fn add_media(&mut self, media_id: Uuid) -> Result<AlbumMedia, AlbumError> {
        // In a real implementation, we would:
        // 1. Validate media exists and belongs to owner
        // 2. Get next position (current max position + 1)
        // 3. Create AlbumMedia entity
        // 4. Save to repository

        // For this implementation, we'll simulate getting the next position
        let position = self.get_next_position();

        let album_media = AlbumMedia {
            album_id: self.id,
            media_id,
            position,
            added_date: Utc::now(),
        };

        Ok(album_media)
    }

    /// Reorder media in the album
    pub fn reorder_media(
        &self,
        media_id: Uuid,
        new_position: u32,
    ) -> Result<AlbumMedia, AlbumError> {
        // In a real implementation, we would:
        // 1. Validate media exists in album
        // 2. Update position
        // 3. Reorder other media if needed

        // For this implementation, we'll just return a placeholder
        Ok(AlbumMedia {
            album_id: self.id,
            media_id,
            position: new_position,
            added_date: Utc::now(),
        })
    }

    /// Set cover media for the album
    pub fn set_cover(&mut self, media_id: Uuid) -> Result<(), AlbumError> {
        // In a real implementation, we would:
        // 1. Validate media exists in album

        self.cover_media_id = Some(media_id);
        Ok(())
    }

    /// Get the next position for a new media item
    fn get_next_position(&self) -> u32 {
        // In a real implementation, we would query the repository to get
        // the current max position for this album and add 1
        // For now, we'll return a placeholder value
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_album_creation() {
        let owner_id = Uuid::new_v4();
        let album = Album::new(
            "Test Album".to_string(),
            "A test album".to_string(),
            owner_id,
        );

        assert_eq!(album.name, "Test Album");
        assert_eq!(album.description, "A test album");
        assert_eq!(album.owner_id, owner_id);
        assert!(album.cover_media_id.is_none());
    }

    #[test]
    fn test_add_media() {
        let owner_id = Uuid::new_v4();
        let mut album = Album::new(
            "Test Album".to_string(),
            "A test album".to_string(),
            owner_id,
        );

        let media_id = Uuid::new_v4();
        let album_media = album.add_media(media_id);

        assert!(album_media.is_ok());
        let album_media = album_media.unwrap();
        assert_eq!(album_media.album_id, album.id);
        assert_eq!(album_media.media_id, media_id);
        assert_eq!(album_media.position, 1);
    }

    #[test]
    fn test_set_cover() {
        let owner_id = Uuid::new_v4();
        let mut album = Album::new(
            "Test Album".to_string(),
            "A test album".to_string(),
            owner_id,
        );

        let media_id = Uuid::new_v4();
        let result = album.set_cover(media_id);

        assert!(result.is_ok());
        assert_eq!(album.cover_media_id, Some(media_id));
    }
}
# Album Entity Design

## Purpose
Represents a collection of media items. Provides organization and presentation capabilities for grouped media.

## Struct Definitions
```rust
pub struct Album {
    pub id: Uuid,                    // Album UUID
    pub name: String,                // Album name
    pub description: String,         // Optional description
    pub owner_id: Uuid,              // Owner UUID
    pub created_date: DateTime<Utc>, // Creation timestamp
    pub cover_media_id: Option<Uuid> // Cover media reference
}

pub struct AlbumMedia {
    pub album_id: Uuid,             // Album reference
    pub media_id: Uuid,             // Media reference
    pub position: u32,              // Order in album
    pub added_date: DateTime<Utc>   // When added to album
}
```

## Methods
### `add_media()`
```rust
impl Album {
    pub fn add_media(&mut self, media_id: Uuid) -> Result<(), AlbumError> {
        // Validate media exists and belongs to owner
        // Get next position (current max position + 1)
        let position = self.get_next_position()?;
        
        let album_media = AlbumMedia {
            album_id: self.id,
            media_id,
            position,
            added_date: Utc::now(),
        };
        
        // Save to repository
        AlbumMediaRepository::save(album_media)?;
        Ok(())
    }
    
    fn get_next_position(&self) -> Result<u32, AlbumError> {
        // Get current max position for this album
        AlbumMediaRepository::get_max_position(self.id)
    }
}
```

### `reorder_media()`
```rust
impl Album {
    pub fn reorder_media(
        &self, 
        media_id: Uuid, 
        new_position: u32
    ) -> Result<(), AlbumError> {
        // Validate media exists in album
        let mut album_media = AlbumMediaRepository::find_by_album_and_media(
            self.id, 
            media_id
        )?;
        
        // Update position
        album_media.position = new_position;
        AlbumMediaRepository::save(album_media)?;
        
        // Reorder other media if needed
        AlbumMediaRepository::reorder_positions(self.id, new_position)?;
        Ok(())
    }
}
```

### `set_cover()`
```rust
impl Album {
    pub fn set_cover(&mut self, media_id: Uuid) -> Result<(), AlbumError> {
        // Validate media exists in album
        AlbumMediaRepository::find_by_album_and_media(self.id, media_id)?;
        
        self.cover_media_id = Some(media_id);
        Ok(())
    }
}
```

## Error Handling
```rust
pub enum AlbumError {
    MediaNotFound(Uuid),
    AlbumNotFound(Uuid),
    MediaNotInAlbum { album_id: Uuid, media_id: Uuid },
    InvalidPosition(u32),
    DatabaseError(sqlx::Error),
    // ... other errors
}
```

## Integration Points
1. **Media Entity**:
   - Media must exist before being added to album
   - Cover media validation requires media exists
2. **Application Layer**:
   - AlbumService handles business logic
   - AlbumQueryService for retrieval
3. **Permissions**:
   - Only album owner can modify album
4. **Infrastructure**:
   - AlbumRepository for album persistence
   - AlbumMediaRepository for album-media relationships
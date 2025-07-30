# Media Entity Design

## Purpose
Represents a media file (image, video, audio) in the gallery system. Handles core media operations and validation.

## Struct Definition
```rust
pub struct Media {
    pub id: Uuid,                   // UUID v7 for federation
    pub file_path: String,          // Path to media file
    pub file_type: MediaType,       // Media type enum
    pub upload_date: DateTime<Utc>, // UTC timestamp
    pub owner_id: Uuid,             // Owner UUID
    pub original_hash: String,      // SHA-256 of original file
    pub transcoded_path: Option<String> // Path to transcoded version
}

pub enum MediaType {
    Image,
    Video,
    Audio,
    Document,       // For PDFs and other documents
    Other(String)   // For future extension
}
```

## Methods
### `new()`
```rust
pub fn new(
    file_path: String,
    file_type: MediaType,
    owner_id: Uuid,
    original_hash: String
) -> Result<Self, MediaError> {
    // Validate file exists and is accessible
    // Validate owner exists in system
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
```

### `get_media_type()`
```rust
pub fn get_media_type(&self) -> &MediaType {
    &self.file_type
}
```

### `is_viewable()`
```rust
pub fn is_viewable(&self) -> bool {
    match self.file_type {
        MediaType::Image => {
            // Check if image is in web-compatible format (WebP, PNG, JPEG)
            self.file_path.ends_with(".webp") || 
            self.file_path.ends_with(".png") || 
            self.file_path.ends_with(".jpg")
        }
        MediaType::Video => {
            // Must be in WebM/AV1 format
            self.file_path.ends_with(".webm") && 
            self.has_codec("av1")
        }
        MediaType::Audio => {
            // Must be in WebM/Opus format
            self.file_path.ends_with(".webm") && 
            self.has_codec("opus")
        }
        _ => false
    }
}

// Helper to check codec using ffmpeg metadata
fn has_codec(&self, codec: &str) -> bool {
    // Implementation will use ffmpeg-next to check codec
    // Placeholder for actual implementation
    true
}
```

## Error Handling
```rust
pub enum MediaError {
    FileNotFound(String),
    InvalidOwner,
    UnsupportedMediaType,
    StorageError(String),
    // ... other errors
}
```

## Integration Points
1. **Infrastructure Layer**: 
   - MediaRepository trait for persistence
   - FileStorageService for physical file handling
2. **Application Layer**:
   - MediaUploadService uses this entity
   - MediaQueryService for retrieval
3. **Metadata Extraction**:
   - Will collaborate with Metadata entity
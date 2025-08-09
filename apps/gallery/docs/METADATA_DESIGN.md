# Metadata Entity Design

## Purpose
Extracts and stores metadata from media files. Provides a unified interface for accessing technical details across different media types.

## Trait Definition
```rust
#[async_trait]
pub trait MetadataExtractor {
    async fn extract_metadata(file_path: &str) -> Result<Metadata, MetadataError>;
}
```

## Metadata Struct
```rust
pub struct Metadata {
    // Common fields
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<f32>, // in seconds
    pub codec: Option<String>,
    pub file_size: u64,
    pub created_date: Option<DateTime<Utc>>,
    
    // Media-specific fields
    pub media_specific: MediaSpecificMetadata,
}

pub enum MediaSpecificMetadata {
    Image(ImageMetadata),
    Video(VideoMetadata),
    Audio(AudioMetadata),
    None,
}

pub struct ImageMetadata {
    pub exif_data: Option<HashMap<String, String>>, // EXIF tags
    pub color_profile: Option<String>,
    pub dpi: Option<(u16, u16)>,
}

pub struct VideoMetadata {
    pub frame_rate: f32,
    pub bitrate: u32,
    pub audio_codec: Option<String>,
    pub color_space: Option<String>,
}

pub struct AudioMetadata {
    pub sample_rate: u32,
    pub bit_depth: u8,
    pub channels: u8,
    pub bitrate: u32,
}
```

## Implementations
### Image Metadata Extractor
```rust
pub struct ImageMetadataExtractor;

#[async_trait]
impl MetadataExtractor for ImageMetadataExtractor {
    async fn extract_metadata(file_path: &str) -> Result<Metadata, MetadataError> {
// were no longer using ffmpeg
       // Placeholder implementation
        Ok(Metadata {
            width: Some(1920),
            height: Some(1080),
            duration: None,
            codec: Some("WebP".to_string()),
            file_size: fs::metadata(file_path)?.len(),
            created_date: None,
            media_specific: MediaSpecificMetadata::Image(ImageMetadata {
                exif_data: None,
                color_profile: Some("sRGB".to_string()),
                dpi: Some((300, 300)),
            }),
        })
    }
}
```

### Video Metadata Extractor (using ffmpeg-next)
```rust
pub struct VideoMetadataExtractor;

#[async_trait]
impl MetadataExtractor for VideoMetadataExtractor {
    async fn extract_metadata(file_path: &str) -> Result<Metadata, MetadataError> {
        let context = ffmpeg_next::format::input(&file_path)?;
        let stream = context.streams().best(ffmpeg_next::media::Type::Video)
            .ok_or(MetadataError::NoVideoStream)?;
        
        let video = stream.codec().decoder().video()?;
        
        Ok(Metadata {
            width: Some(video.width()),
            height: Some(video.height()),
            duration: Some(context.duration() as f32 / f32::from(ffmpeg_next::ffi::AV_TIME_BASE)),
            codec: video.codec().name().map(|s| s.to_string()),
            file_size: fs::metadata(file_path)?.len(),
            created_date: None,
            media_specific: MediaSpecificMetadata::Video(VideoMetadata {
                frame_rate: video.rate().0 as f32 / video.rate().1 as f32,
                bitrate: context.bit_rate() as u32,
                audio_codec: None, // Would need audio stream extraction
                color_space: None,
            }),
        })
    }
}
```

## Error Handling
```rust
pub enum MetadataError {
    FileNotFound(String),
    UnsupportedMediaType,
    ExtractionFailed(String),
    FfmpegError(ffmpeg_next::Error),
    // ... other errors
}

impl From<ffmpeg_next::Error> for MetadataError {
    fn from(err: ffmpeg_next::Error) -> Self {
        MetadataError::FfmpegError(err)
    }
}
```

## Integration Points
1. **Media Entity**:
   - Media creation triggers metadata extraction
   - Metadata stored with media in database
2. **Application Layer**:
   - MediaUploadService uses appropriate extractor
   - Metadata used for search and organization
3. **Transcoding**:
   - Metadata used to determine if transcoding is needed
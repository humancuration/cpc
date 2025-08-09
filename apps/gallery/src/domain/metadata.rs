// Gallery domain - Metadata entity
// This file defines the Metadata entity and related business logic
// were no longer using ffmpeg

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use ffmpeg_next as ffmpeg;
use std::collections::HashMap;
use std::fs;
use uuid::Uuid;

/// Trait for extracting metadata from media files
#[async_trait]
pub trait MetadataExtractor {
    /// Extract metadata from a file
    async fn extract_metadata(file_path: &str) -> Result<Metadata, MetadataError>;
}

/// Metadata structure containing technical details about a media file
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Metadata {
    /// Width in pixels (for images/videos)
    pub width: Option<u32>,
    /// Height in pixels (for images/videos)
    pub height: Option<u32>,
    /// Duration in seconds (for videos/audio)
    pub duration: Option<f32>,
    /// Codec used
    pub codec: Option<String>,
    /// File size in bytes
    pub file_size: u64,
    /// Creation date
    pub created_date: Option<DateTime<Utc>>,
    /// Media-specific metadata
    pub media_specific: MediaSpecificMetadata,
}

/// Enum for media-specific metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum MediaSpecificMetadata {
    Image(ImageMetadata),
    Video(VideoMetadata),
    Audio(AudioMetadata),
    None,
}

/// Image-specific metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageMetadata {
    /// EXIF tags
    pub exif_data: Option<HashMap<String, String>>,
    /// Color profile
    pub color_profile: Option<String>,
    /// DPI values
    pub dpi: Option<(u16, u16)>,
}

/// Video-specific metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VideoMetadata {
    /// Frame rate
    pub frame_rate: f32,
    /// Bitrate
    pub bitrate: u32,
    /// Audio codec
    pub audio_codec: Option<String>,
    /// Color space
    pub color_space: Option<String>,
}

/// Audio-specific metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AudioMetadata {
    /// Sample rate
    pub sample_rate: u32,
    /// Bit depth
    pub bit_depth: u8,
    /// Number of channels
    pub channels: u8,
    /// Bitrate
    pub bitrate: u32,
}

/// Error types for metadata operations
#[derive(Debug, thiserror::Error)]
pub enum MetadataError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Unsupported media type")]
    UnsupportedMediaType,
    #[error("Extraction failed: {0}")]
    ExtractionFailed(String),
    #[error("No video stream found")]
    NoVideoStream,
    #[error("FFmpeg error: {0}")]
    FfmpegError(#[from] ffmpeg::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Image metadata extractor
pub struct ImageMetadataExtractor;

#[async_trait]
impl MetadataExtractor for ImageMetadataExtractor {
    async fn extract_metadata(file_path: &str) -> Result<Metadata, MetadataError> {
        // Check if file exists
        if !std::path::Path::new(file_path).exists() {
            return Err(MetadataError::FileNotFound(file_path.to_string()));
        }

        // In a real implementation, we would use the exif crate to extract image metadata
        // For now, we'll create a placeholder implementation
        let file_size = fs::metadata(file_path)?.len();

        Ok(Metadata {
            width: Some(1920),
            height: Some(1080),
            duration: None,
            codec: Some("WebP".to_string()),
            file_size,
            created_date: None,
            media_specific: MediaSpecificMetadata::Image(ImageMetadata {
                exif_data: None,
                color_profile: Some("sRGB".to_string()),
                dpi: Some((300, 300)),
            }),
        })
    }
}

/// Video metadata extractor using ffmpeg-next
pub struct VideoMetadataExtractor;

#[async_trait]
impl MetadataExtractor for VideoMetadataExtractor {
    async fn extract_metadata(file_path: &str) -> Result<Metadata, MetadataError> {
        // Check if file exists
        if !std::path::Path::new(file_path).exists() {
            return Err(MetadataError::FileNotFound(file_path.to_string()));
        }

        let context = ffmpeg::format::input(&file_path)?;
        let stream = context
            .streams()
            .best(ffmpeg::media::Type::Video)
            .ok_or(MetadataError::NoVideoStream)?;

        let video = stream.codec().decoder().video()?;

        let file_size = fs::metadata(file_path)?.len();

        Ok(Metadata {
            width: Some(video.width()),
            height: Some(video.height()),
            duration: Some(context.duration() as f32 / f32::from(ffmpeg::ffi::AV_TIME_BASE)),
            codec: video.codec().name().map(|s| s.to_string()),
            file_size,
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

/// Audio metadata extractor using ffmpeg-next
pub struct AudioMetadataExtractor;

#[async_trait]
impl MetadataExtractor for AudioMetadataExtractor {
    async fn extract_metadata(file_path: &str) -> Result<Metadata, MetadataError> {
        // Check if file exists
        if !std::path::Path::new(file_path).exists() {
            return Err(MetadataError::FileNotFound(file_path.to_string()));
        }

        let context = ffmpeg::format::input(&file_path)?;
        let stream = context
            .streams()
            .best(ffmpeg::media::Type::Audio)
            .ok_or(MetadataError::NoVideoStream)?; // Using NoVideoStream for now, should be NoAudioStream

        let audio = stream.codec().decoder().audio()?;

        let file_size = fs::metadata(file_path)?.len();

        Ok(Metadata {
            width: None,
            height: None,
            duration: Some(context.duration() as f32 / f32::from(ffmpeg::ffi::AV_TIME_BASE)),
            codec: audio.codec().name().map(|s| s.to_string()),
            file_size,
            created_date: None,
            media_specific: MediaSpecificMetadata::Audio(AudioMetadata {
                sample_rate: audio.rate(),
                bit_depth: audio.format().bits(),
                channels: audio.channels(),
                bitrate: context.bit_rate() as u32,
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_image_metadata_extraction() {
        // Create a mock file for testing
        let file_path = "test_image.webp";
        std::fs::write(file_path, "test content").unwrap();

        let metadata = ImageMetadataExtractor::extract_metadata(file_path).await;
        assert!(metadata.is_ok());

        // Clean up
        std::fs::remove_file(file_path).unwrap();
    }

    #[tokio::test]
    async fn test_image_metadata_extraction_file_not_found() {
        let metadata = ImageMetadataExtractor::extract_metadata("nonexistent.jpg").await;
        assert!(matches!(metadata, Err(MetadataError::FileNotFound(_))));
    }
}
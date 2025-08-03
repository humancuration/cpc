//! Utility functions for media processing

use uuid::Uuid;
use std::collections::HashMap;

/// Media processing utilities
pub struct MediaUtils;

/// Represents a media segment for streaming
#[derive(Debug, Clone)]
pub struct MediaSegment {
    /// Unique identifier for the segment
    pub id: Uuid,
    
    /// Sequence number of the segment
    pub sequence_number: u64,
    
    /// Duration of the segment in seconds
    pub duration: f64,
    
    /// File path or URL to the segment
    pub location: String,
    
    /// Whether this is a keyframe segment
    pub is_keyframe: bool,
}

/// Stream manifest for adaptive bitrate streaming
#[derive(Debug, Clone)]
pub struct StreamManifest {
    /// Stream key
    pub stream_key: String,
    
    /// Available quality levels
    pub quality_levels: Vec<QualityLevel>,
    
    /// Current segments for each quality level
    pub segments: HashMap<u32, Vec<MediaSegment>>,
    
    /// When the manifest was last updated
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Quality level for adaptive streaming
#[derive(Debug, Clone)]
pub struct QualityLevel {
    /// Quality level identifier
    pub level: u32,
    
    /// Width in pixels
    pub width: u32,
    
    /// Height in pixels
    pub height: u32,
    
    /// Bitrate in kbps
    pub bitrate_kbps: u32,
    
    /// Frame rate
    pub fps: f32,
}

impl MediaUtils {
    /// Create a new stream manifest
    pub fn create_stream_manifest(stream_key: String) -> StreamManifest {
        StreamManifest {
            stream_key,
            quality_levels: Vec::new(),
            segments: HashMap::new(),
            last_updated: chrono::Utc::now(),
        }
    }
    
    /// Add a quality level to a stream manifest
    pub fn add_quality_level(manifest: &mut StreamManifest, level: QualityLevel) {
        manifest.quality_levels.push(level);
        manifest.segments.insert(level.level, Vec::new());
        manifest.last_updated = chrono::Utc::now();
    }
    
    /// Add a segment to a stream manifest
    pub fn add_segment(manifest: &mut StreamManifest, level: u32, segment: MediaSegment) {
        if let Some(segments) = manifest.segments.get_mut(&level) {
            segments.push(segment);
            
            // Keep only the most recent segments (e.g., last 10 minutes)
            if segments.len() > 60 {
                segments.drain(0..segments.len()-60);
            }
            
            manifest.last_updated = chrono::Utc::now();
        }
    }
    
    /// Get the current playlist for a quality level
    pub fn get_playlist(manifest: &StreamManifest, level: u32, segment_count: usize) -> Vec<MediaSegment> {
        if let Some(segments) = manifest.segments.get(&level) {
            let start = if segments.len() > segment_count {
                segments.len() - segment_count
            } else {
                0
            };
            segments[start..].to_vec()
        } else {
            Vec::new()
        }
    }
    
    /// Validate that a media format is supported
    pub fn is_supported_format(format: &crate::media_processing::transcoder::MediaFormat) -> bool {
        // Check video codec
        let supported_video = matches!(format.codec.as_str(), "av1" | "vp9" | "h264");
        
        // Check audio codec
        let supported_audio = matches!(format.audio_codec.as_str(), "opus" | "aac" | "mp3");
        
        // Check resolution is valid
        let valid_resolution = format.width > 0 && format.height > 0 && format.width <= 7680 && format.height <= 4320;
        
        // Check bitrate is valid
        let valid_bitrate = format.bitrate_kbps > 0 && format.bitrate_kbps <= 50000;
        
        supported_video && supported_audio && valid_resolution && valid_bitrate
    }
    
    /// Calculate recommended bitrate based on resolution
    pub fn calculate_recommended_bitrate(width: u32, height: u32) -> u32 {
        let megapixels = (width * height) as f64 / 1_000_000.0;
        
        // Simple formula: ~0.1 Mbps per megapixel for 30fps content
        (megapixels * 100.0) as u32
    }
    
    /// Generate a stream key
    pub fn generate_stream_key() -> String {
        Uuid::new_v4().to_string().replace("-", "")
    }
    
    /// Validate a stream key format
    pub fn validate_stream_key(key: &str) -> bool {
        // Stream key should be alphanumeric and 16-64 characters
        key.len() >= 16 && key.len() <= 64 && key.chars().all(|c| c.is_ascii_alphanumeric())
    }
}
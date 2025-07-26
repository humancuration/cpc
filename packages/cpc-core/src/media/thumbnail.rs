use crate::media::types::*;
use anyhow::Result;
use image::{DynamicImage, ImageFormat};
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Thumbnail generator for various media types
pub struct ThumbnailGenerator;

impl ThumbnailGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Generate thumbnail for any media type
    pub async fn generate_thumbnail(
        &self,
        input_path: &Path,
        output_path: &Path,
        config: ThumbnailConfig,
    ) -> Result<PathBuf> {
        // Determine media type from file extension
        let media_type = self.detect_media_type(input_path)?;
        
        match media_type {
            MediaType::Image => self.generate_image_thumbnail(input_path, output_path, &config).await,
            MediaType::Video => self.generate_video_thumbnail(input_path, output_path, &config).await,
            MediaType::Audio => self.generate_audio_thumbnail(input_path, output_path, &config).await,
        }
    }

    /// Generate thumbnail from image
    async fn generate_image_thumbnail(
        &self,
        input_path: &Path,
        output_path: &Path,
        config: &ThumbnailConfig,
    ) -> Result<PathBuf> {
        log::info!("Generating image thumbnail: {:?} -> {:?}", input_path, output_path);
        
        // Load the image
        let img = image::open(input_path)?;
        
        // Resize to thumbnail dimensions while maintaining aspect ratio
        let thumbnail = img.resize(
            config.width,
            config.height,
            image::imageops::FilterType::Lanczos3,
        );
        
        // Save as PNG with quality settings
        self.save_thumbnail_image(thumbnail, output_path, config.quality)?;
        
        Ok(output_path.to_path_buf())
    }

    /// Generate thumbnail from video (extract frame at specified timestamp)
    async fn generate_video_thumbnail(
        &self,
        input_path: &Path,
        output_path: &Path,
        config: &ThumbnailConfig,
    ) -> Result<PathBuf> {
        log::info!("Generating video thumbnail: {:?} -> {:?}", input_path, output_path);
        
        // TODO: Use ffmpeg.wasm to extract frame from video
        // For now, generate a placeholder thumbnail
        
        let timestamp = config.timestamp.unwrap_or(1.0);
        log::info!("Extracting frame at timestamp: {}s", timestamp);
        
        // In a real implementation, this would use ffmpeg.wasm:
        // let frame = ffmpeg_wasm::extract_frame(input_path, timestamp).await?;
        
        // For now, create a placeholder thumbnail
        self.create_placeholder_thumbnail(output_path, config, "VIDEO").await
    }

    /// Generate thumbnail for audio (waveform or placeholder)
    async fn generate_audio_thumbnail(
        &self,
        input_path: &Path,
        output_path: &Path,
        config: &ThumbnailConfig,
    ) -> Result<PathBuf> {
        log::info!("Generating audio thumbnail: {:?} -> {:?}", input_path, output_path);
        
        // TODO: Generate waveform visualization
        // For now, create a placeholder thumbnail
        
        self.create_placeholder_thumbnail(output_path, config, "AUDIO").await
    }

    /// Create a placeholder thumbnail with text
    async fn create_placeholder_thumbnail(
        &self,
        output_path: &Path,
        config: &ThumbnailConfig,
        text: &str,
    ) -> Result<PathBuf> {
        // Create a simple colored rectangle as placeholder
        let mut img = DynamicImage::new_rgb8(config.width, config.height);
        
        // Fill with a gradient or solid color
        let rgb_img = img.as_mut_rgb8().unwrap();
        for (x, y, pixel) in rgb_img.enumerate_pixels_mut() {
            let intensity = ((x + y) % 255) as u8;
            *pixel = image::Rgb([intensity / 3, intensity / 2, intensity]);
        }
        
        // TODO: Add text overlay using a font rendering library
        // For now, just save the colored rectangle
        
        self.save_thumbnail_image(img, output_path, config.quality)?;
        
        Ok(output_path.to_path_buf())
    }

    /// Save thumbnail image with quality settings
    fn save_thumbnail_image(
        &self,
        img: DynamicImage,
        output_path: &Path,
        quality: u8,
    ) -> Result<()> {
        // Ensure output directory exists
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        // Save as PNG (lossless) or JPEG based on extension
        match output_path.extension().and_then(|e| e.to_str()) {
            Some("jpg") | Some("jpeg") => {
                let mut output = std::fs::File::create(output_path)?;
                img.write_to(&mut output, ImageFormat::Jpeg)?;
            }
            _ => {
                // Default to PNG
                img.save(output_path)?;
            }
        }
        
        Ok(())
    }

    /// Detect media type from file extension
    fn detect_media_type(&self, path: &Path) -> Result<MediaType> {
        match path.extension().and_then(|e| e.to_str()) {
            Some("mp4") | Some("webm") | Some("avi") | Some("mov") | Some("mkv") => {
                Ok(MediaType::Video)
            }
            Some("mp3") | Some("wav") | Some("ogg") | Some("opus") | Some("flac") => {
                Ok(MediaType::Audio)
            }
            Some("jpg") | Some("jpeg") | Some("png") | Some("gif") | Some("bmp") | Some("webp") => {
                Ok(MediaType::Image)
            }
            _ => Err(anyhow::anyhow!("Unsupported media type for file: {:?}", path)),
        }
    }

    /// Generate multiple thumbnail sizes
    pub async fn generate_multiple_thumbnails(
        &self,
        input_path: &Path,
        output_dir: &Path,
        sizes: &[(u32, u32)], // (width, height) pairs
    ) -> Result<Vec<PathBuf>> {
        let mut thumbnails = Vec::new();
        
        for (i, &(width, height)) in sizes.iter().enumerate() {
            let config = ThumbnailConfig {
                width,
                height,
                quality: 80,
                timestamp: Some(1.0),
            };
            
            let output_filename = format!(
                "thumb_{}x{}_{}.png",
                width,
                height,
                input_path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
            );
            
            let output_path = output_dir.join(output_filename);
            
            let thumbnail_path = self.generate_thumbnail(input_path, &output_path, config).await?;
            thumbnails.push(thumbnail_path);
        }
        
        Ok(thumbnails)
    }

    /// Generate thumbnail with automatic size detection based on content
    pub async fn generate_adaptive_thumbnail(
        &self,
        input_path: &Path,
        output_path: &Path,
        max_size: (u32, u32),
    ) -> Result<PathBuf> {
        // TODO: Analyze content to determine optimal thumbnail size
        // For now, use standard thumbnail size
        
        let config = ThumbnailConfig {
            width: max_size.0.min(320),
            height: max_size.1.min(240),
            quality: 85,
            timestamp: Some(1.0),
        };
        
        self.generate_thumbnail(input_path, output_path, config).await
    }
}

impl Default for ThumbnailGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for thumbnail management
pub mod utils {
    use super::*;
    
    /// Get standard thumbnail sizes
    pub fn get_standard_thumbnail_sizes() -> Vec<(u32, u32)> {
        vec![
            (64, 64),     // Small icon
            (128, 128),   // Medium icon
            (320, 240),   // Standard thumbnail
            (640, 480),   // Large thumbnail
        ]
    }
    
    /// Generate thumbnail filename based on input and size
    pub fn generate_thumbnail_filename(
        input_path: &Path,
        width: u32,
        height: u32,
        extension: &str,
    ) -> String {
        let stem = input_path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        
        format!("{}_{}x{}.{}", stem, width, height, extension)
    }
    
    /// Check if thumbnail already exists and is newer than source
    pub async fn is_thumbnail_up_to_date(
        source_path: &Path,
        thumbnail_path: &Path,
    ) -> Result<bool> {
        if !thumbnail_path.exists() {
            return Ok(false);
        }
        
        let source_metadata = tokio::fs::metadata(source_path).await?;
        let thumbnail_metadata = tokio::fs::metadata(thumbnail_path).await?;
        
        Ok(thumbnail_metadata.modified()? >= source_metadata.modified()?)
    }
}
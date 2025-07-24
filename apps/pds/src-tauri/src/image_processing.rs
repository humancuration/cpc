//! Image processing service for PDS desktop client
//!
//! Handles image processing for both file uploads and webcam captures
//! with integration into the Bevy engine for AR preparation.

use image::{ImageFormat, ImageOutputFormat, imageops::FilterType};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use thiserror::Error;

/// Image processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageProcessingConfig {
    pub max_width: u32,
    pub max_height: u32,
    pub quality: u8,
    pub format: ImageFormat,
}

impl Default for ImageProcessingConfig {
    fn default() -> Self {
        Self {
            max_width: 1920,
            max_height: 1080,
            quality: 85,
            format: ImageFormat::WebP,
        }
    }
}

/// Processed image data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedImage {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub hash: String,
}

/// Image processing error
#[derive(Debug, Error)]
pub enum ImageProcessingError {
    #[error("Invalid image format")]
    InvalidFormat,
    #[error("Image processing failed: {0}")]
    ProcessingFailed(String),
    #[error("Unsupported format")]
    UnsupportedFormat,
}

/// Process image data for AR preparation
pub async fn process_image(
    data: Vec<u8>,
    config: ImageProcessingConfig,
) -> Result<ProcessedImage, ImageProcessingError> {
    // Load image from memory
    let img = image::load_from_memory(&data)
        .map_err(|e| ImageProcessingError::ProcessingFailed(e.to_string()))?;

    // Calculate aspect ratio
    let (orig_width, orig_height) = img.dimensions();
    let aspect_ratio = orig_width as f32 / orig_height as f32;
    
    // Resize maintaining aspect ratio
    let (new_width, new_height) = if orig_width > orig_height {
        let new_width = config.max_width.min(orig_width);
        let new_height = (new_width as f32 / aspect_ratio) as u32;
        (new_width, new_height.min(config.max_height))
    } else {
        let new_height = config.max_height.min(orig_height);
        let new_width = (new_height as f32 * aspect_ratio) as u32;
        (new_width.min(config.max_width), new_height)
    };

    let resized = img.resize(new_width, new_height, FilterType::Lanczos3);

    // Convert to target format
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    
    let output_format = match config.format {
        ImageFormat::WebP => ImageOutputFormat::WebP,
        ImageFormat::Png => ImageOutputFormat::Png,
        ImageFormat::Jpeg => ImageOutputFormat::Jpeg(config.quality),
        _ => return Err(ImageProcessingError::UnsupportedFormat),
    };

    resized
        .write_to(&mut cursor, output_format)
        .map_err(|e| ImageProcessingError::ProcessingFailed(e.to_string()))?;

    // Generate BLAKE3 hash
    let hash = blake3::hash(&buffer).to_hex().to_string();

    Ok(ProcessedImage {
        data: buffer,
        width: new_width,
        height: new_height,
        format: format!("{:?}", config.format).to_lowercase(),
        hash,
    })
}

/// Process webcam frame data
pub async fn process_webcam_frame(
    frame_data: Vec<u8>,
    width: u32,
    height: u32,
) -> Result<ProcessedImage, ImageProcessingError> {
    // Create image from raw RGB data
    let img = image::RgbImage::from_raw(width, height, frame_data)
        .ok_or_else(|| ImageProcessingError::ProcessingFailed("Invalid frame data".to_string()))?;

    let dynamic_img = image::DynamicImage::ImageRgb8(img);
    
    // Convert to WebP format
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    
    dynamic_img
        .write_to(&mut cursor, ImageOutputFormat::WebP)
        .map_err(|e| ImageProcessingError::ProcessingFailed(e.to_string()))?;

    // Generate BLAKE3 hash
    let hash = blake3::hash(&buffer).to_hex().to_string();

    Ok(ProcessedImage {
        data: buffer,
        width,
        height,
        format: "webp".to_string(),
        hash,
    })
}

/// Validate image format
pub fn validate_image_format(data: &[u8]) -> Result<ImageFormat, ImageProcessingError> {
    image::guess_format(data)
        .map_err(|_| ImageProcessingError::InvalidFormat)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};

    #[tokio::test]
    async fn test_process_image() {
        // Create a test image
        let img = ImageBuffer::from_pixel(100, 100, Rgb([255, 0, 0]));
        let mut buffer = Vec::new();
        image::write_buffer(&mut buffer, &img, 100, 100, image::ColorType::Rgb8).unwrap();
        
        let config = ImageProcessingConfig::default();
        let result = process_image(buffer, config).await;
        
        assert!(result.is_ok());
        let processed = result.unwrap();
        assert_eq!(processed.width, 100);
        assert_eq!(processed.height, 100);
        assert!(!processed.data.is_empty());
    }

    #[tokio::test]
    async fn test_process_webcam_frame() {
        let frame_data = vec![0u8; 1920 * 1080 * 3]; // RGB 1920x1080
        let result = process_webcam_frame(frame_data, 1920, 1080).await;
        
        assert!(result.is_ok());
        let processed = result.unwrap();
        assert_eq!(processed.width, 1920);
        assert_eq!(processed.height, 1080);
        assert!(!processed.data.is_empty());
    }
}
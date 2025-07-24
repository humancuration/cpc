//! Experience preview generation system
//!
//! Generates thumbnails and previews for AR experiences
//! with distributed storage via content hashes.

use bevy::prelude::*;
use bevy::render::{
    camera::{Camera, Camera2d},
    render_asset::RenderAssetUsages,
    render_resource::{Extent3d, TextureDimension, TextureFormat},
    texture::Image,
};
use image::{ImageBuffer, Rgba};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Preview generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewConfig {
    pub thumbnail_width: u32,
    pub thumbnail_height: u32,
    pub quality: u8,
    pub format: PreviewFormat,
}

/// Preview format options
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PreviewFormat {
    WebP,
    Png,
    Jpeg,
}

/// Generated preview data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preview {
    pub content_hash: String,
    pub thumbnail_data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: PreviewFormat,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Preview generation state
#[derive(Resource, Debug, Default)]
pub struct PreviewState {
    pub active_generations: Vec<PreviewGeneration>,
    pub cache: std::collections::HashMap<String, Preview>,
}

/// Preview generation task
#[derive(Debug, Clone)]
pub struct PreviewGeneration {
    pub experience_id: String,
    pub config: PreviewConfig,
    pub status: GenerationStatus,
}

/// Generation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenerationStatus {
    Pending,
    InProgress,
    Completed(String),
    Failed(String),
}

/// Preview generation service
#[derive(Resource)]
pub struct PreviewService {
    pub config: PreviewConfig,
    pub state: Arc<Mutex<PreviewState>>,
}

impl PreviewService {
    pub fn new(config: PreviewConfig) -> Self {
        Self {
            config,
            state: Arc::new(Mutex::new(PreviewState::default())),
        }
    }

    /// Generate preview for an AR experience
    pub async fn generate_preview(
        &self,
        experience_id: String,
        scene_data: Vec<u8>,
    ) -> Result<Preview, PreviewError> {
        let mut state = self.state.lock().await;
        
        // Check cache first
        if let Some(preview) = state.cache.get(&experience_id) {
            return Ok(preview.clone());
        }
        
        // Add to active generations
        let generation = PreviewGeneration {
            experience_id: experience_id.clone(),
            config: self.config.clone(),
            status: GenerationStatus::InProgress,
        };
        
        state.active_generations.push(generation);
        
        // Generate thumbnail
        let preview = self.create_scene_thumbnail(&scene_data).await?;
        
        // Store in cache
        state.cache.insert(experience_id.clone(), preview.clone());
        
        // Remove from active generations
        state.active_generations.retain(|g| g.experience_id != experience_id);
        
        Ok(preview)
    }

    /// Create scene thumbnail from AR experience
    async fn create_scene_thumbnail(
        &self,
        scene_data: &[u8],
    ) -> Result<Preview, PreviewError> {
        // Create a simple thumbnail image
        let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(
            self.config.thumbnail_width,
            self.config.thumbnail_height,
        );
        
        // Fill with gradient background
        for y in 0..self.config.thumbnail_height {
            for x in 0..self.config.thumbnail_width {
                let r = (x as f32 / self.config.thumbnail_width as f32 * 255.0) as u8;
                let g = (y as f32 / self.config.thumbnail_height as f32 * 255.0) as u8;
                let b = 128;
                let a = 255;
                
                img.put_pixel(x, y, Rgba([r, g, b, a]));
            }
        }
        
        // Convert to target format
        let mut buffer = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut buffer);
        
        match self.config.format {
            PreviewFormat::WebP => {
                image::write_buffer(
                    &mut cursor,
                    &img,
                    self.config.thumbnail_width,
                    self.config.thumbnail_height,
                    image::ColorType::Rgba8,
                ).map_err(|e| PreviewError::ImageError(e.to_string()))?;
            }
            PreviewFormat::Png => {
                image::write_buffer(
                    &mut cursor,
                    &img,
                    self.config.thumbnail_width,
                    self.config.thumbnail_height,
                    image::ColorType::Rgba8,
                ).map_err(|e| PreviewError::ImageError(e.to_string()))?;
            }
            PreviewFormat::Jpeg => {
                let rgb_img = image::DynamicImage::ImageRgba8(img).to_rgb8();
                image::write_buffer(
                    &mut cursor,
                    &rgb_img,
                    self.config.thumbnail_width,
                    self.config.thumbnail_height,
                    image::ColorType::Rgb8,
                ).map_err(|e| PreviewError::ImageError(e.to_string()))?;
            }
        }
        
        // Generate content hash
        let hash = blake3::hash(&buffer).to_hex().to_string();
        
        Ok(Preview {
            content_hash: hash,
            thumbnail_data: buffer,
            width: self.config.thumbnail_width,
            height: self.config.thumbnail_height,
            format: self.config.format,
            created_at: chrono::Utc::now(),
        })
    }

    /// Store preview in distributed storage
    pub async fn store_preview(
        &self,
        preview: &Preview,
    ) -> Result<String, PreviewError> {
        // In real implementation, this would use distributed storage
        // For now, return the content hash
        Ok(preview.content_hash.clone())
    }

    /// Get preview from cache or storage
    pub async fn get_preview(
        &self,
        experience_id: &str,
    ) -> Option<Preview> {
        let state = self.state.lock().await;
        state.cache.get(experience_id).cloned()
    }
}

/// Preview generation error
#[derive(Debug, thiserror::Error)]
pub enum PreviewError {
    #[error("Image processing error: {0}")]
    ImageError(String),
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Generation failed: {0}")]
    GenerationFailed(String),
}

/// Bevy system for preview generation
pub fn generate_preview_system(
    mut commands: Commands,
    preview_service: Res<PreviewService>,
    scene_query: Query<Entity, With<bevy::scene::Scene>>,
) {
    // This would be triggered by events or user actions
    // For now, it's a placeholder for integration
}

/// Preview generation plugin
pub struct PreviewPlugin;

impl Plugin for PreviewPlugin {
    fn build(&self, app: &mut App) {
        let config = PreviewConfig {
            thumbnail_width: 512,
            thumbnail_height: 288,
            quality: 85,
            format: PreviewFormat::WebP,
        };
        
        let service = PreviewService::new(config);
        app.insert_resource(service)
            .add_systems(Update, generate_preview_system);
    }
}

impl Default for PreviewConfig {
    fn default() -> Self {
        Self {
            thumbnail_width: 512,
            thumbnail_height: 288,
            quality: 85,
            format: PreviewFormat::WebP,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_preview_generation() {
        let config = PreviewConfig::default();
        let service = PreviewService::new(config);
        
        let scene_data = vec![0u8; 1024];
        let result = service.generate_preview("test_id".to_string(), scene_data).await;
        
        assert!(result.is_ok());
        let preview = result.unwrap();
        assert_eq!(preview.width, 512);
        assert_eq!(preview.height, 288);
    }

    #[test]
    fn test_preview_config_default() {
        let config = PreviewConfig::default();
        assert_eq!(config.thumbnail_width, 512);
        assert_eq!(config.thumbnail_height, 288);
        assert_eq!(config.quality, 85);
    }
}
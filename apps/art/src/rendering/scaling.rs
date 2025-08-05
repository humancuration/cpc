//! Scaling filters for the Art application
//!
//! This module implements various scaling algorithms for high-quality rendering.

use bevy::prelude::*;
use crate::rendering::{RenderSettings, RenderQuality};

/// Scaling filter implementations
#[derive(Resource)]
pub struct ScalingManager {
    pub current_filter: ScalingFilter,
    pub cached_scaled_textures: std::collections::HashMap<Handle<Image>, Handle<Image>>,
}

impl Default for ScalingManager {
    fn default() -> Self {
        Self {
            current_filter: ScalingFilter::Bicubic,
            cached_scaled_textures: std::collections::HashMap::new(),
        }
    }
}

/// Available scaling filters
#[derive(Debug, Clone, PartialEq)]
pub enum ScalingFilter {
    Nearest,
    Bilinear,
    Bicubic,
}

impl ScalingFilter {
    /// Get the shader name for this filter
    pub fn shader_name(&self) -> &'static str {
        match self {
            ScalingFilter::Nearest => "nearest",
            ScalingFilter::Bilinear => "bilinear",
            ScalingFilter::Bicubic => "bicubic",
        }
    }
}

/// Scale a texture using the specified filter
pub fn scale_texture(
    source: &Handle<Image>,
    target_size: (u32, u32),
    filter: &ScalingFilter,
    images: &mut Assets<Image>,
) -> Handle<Image> {
    // In a real implementation, we would use a compute shader or GPU texture scaling
    // For now, we'll create a placeholder scaled texture
    
    let size = Extent3d {
        width: target_size.0,
        height: target_size.1,
        depth_or_array_layers: 1,
    };
    
    // Create a placeholder - in reality this would be the scaled image
    let mut scaled_image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[200, 200, 200, 255], // Light gray placeholder
        TextureFormat::Rgba8UnormSrgb,
    );
    
    scaled_image.texture_descriptor.usage = 
        bevy::render::render_resource::TextureUsages::TEXTURE_BINDING | 
        bevy::render::render_resource::TextureUsages::COPY_DST;
    
    images.add(scaled_image)
}

/// Select the appropriate scaling filter based on quality settings and zoom level
pub fn select_scaling_filter(
    render_settings: &RenderSettings,
    zoom: f32,
) -> ScalingFilter {
    match render_settings.quality {
        RenderQuality::Low => ScalingFilter::Nearest,
        RenderQuality::Medium => ScalingFilter::Bilinear,
        RenderQuality::High => {
            // For high quality, use bicubic for downscaling and bilinear for upscaling
            if zoom < 1.0 {
                ScalingFilter::Bicubic
            } else {
                ScalingFilter::Bilinear
            }
        }
    }
}

/// System to update scaling based on camera and quality settings
pub fn update_scaling(
    mut scaling_manager: ResMut<ScalingManager>,
    render_settings: Res<RenderSettings>,
    camera_controller: Res<crate::rendering::CameraController>,
) {
    let new_filter = select_scaling_filter(&render_settings, camera_controller.zoom);
    
    if scaling_manager.current_filter != new_filter {
        scaling_manager.current_filter = new_filter.clone();
        // Clear cached textures when filter changes
        scaling_manager.cached_scaled_textures.clear();
        info!("Updated scaling filter to: {:?}", new_filter);
    }
}

/// Apply scaling to visible tiles
pub fn apply_scaling_to_tiles(
    scaling_manager: &ScalingManager,
    tile_textures: Vec<Handle<Image>>,
    target_size: (u32, u32),
    images: &mut Assets<Image>,
) -> Vec<Handle<Image>> {
    tile_textures
        .into_iter()
        .map(|texture| {
            // Check if we have a cached scaled version
            if let Some(cached) = scaling_manager.cached_scaled_textures.get(&texture) {
                cached.clone()
            } else {
                // Scale the texture
                let scaled = scale_texture(
                    &texture,
                    target_size,
                    &scaling_manager.current_filter,
                    images,
                );
                
                // Cache the result
                // In a real implementation, we'd want to be more careful about memory usage
                scaling_manager.cached_scaled_textures.insert(texture, scaled.clone());
                scaled
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rendering::{RenderSettings, RenderQuality};
    
    #[test]
    fn test_filter_selection() {
        let mut settings = RenderSettings::default();
        
        // Low quality should always use nearest
        settings.quality = RenderQuality::Low;
        assert_eq!(select_scaling_filter(&settings, 0.5), ScalingFilter::Nearest);
        assert_eq!(select_scaling_filter(&settings, 2.0), ScalingFilter::Nearest);
        
        // Medium quality should always use bilinear
        settings.quality = RenderQuality::Medium;
        assert_eq!(select_scaling_filter(&settings, 0.5), ScalingFilter::Bilinear);
        assert_eq!(select_scaling_filter(&settings, 2.0), ScalingFilter::Bilinear);
        
        // High quality should use bicubic for downscaling, bilinear for upscaling
        settings.quality = RenderQuality::High;
        assert_eq!(select_scaling_filter(&settings, 0.5), ScalingFilter::Bicubic);
        assert_eq!(select_scaling_filter(&settings, 2.0), ScalingFilter::Bilinear);
    }
    
    #[test]
    fn test_shader_names() {
        assert_eq!(ScalingFilter::Nearest.shader_name(), "nearest");
        assert_eq!(ScalingFilter::Bilinear.shader_name(), "bilinear");
        assert_eq!(ScalingFilter::Bicubic.shader_name(), "bicubic");
    }
    
    #[test]
    fn test_scaling_manager() {
        let mut manager = ScalingManager::default();
        assert_eq!(manager.current_filter, ScalingFilter::Bicubic);
        assert!(manager.cached_scaled_textures.is_empty());
    }
}
//! Quality management for the Art application rendering
//!
//! This module handles different quality presets and their configurations.

use bevy::prelude::*;

/// Quality levels for rendering
#[derive(Debug, Clone, PartialEq)]
pub enum RenderQuality {
    Low,
    Medium,
    High,
}

impl RenderQuality {
    /// Get the scaling filter for this quality level
    pub fn scaling_filter(&self) -> ScalingFilter {
        match self {
            RenderQuality::Low => ScalingFilter::Nearest,
            RenderQuality::Medium => ScalingFilter::Bilinear,
            RenderQuality::High => ScalingFilter::Bicubic,
        }
    }
    
    /// Get the number of mipmap levels for this quality level
    pub fn mipmap_levels(&self) -> u32 {
        match self {
            RenderQuality::Low => 0,
            RenderQuality::Medium => 2,
            RenderQuality::High => 4,
        }
    }
    
    /// Get the tile update strategy for this quality level
    pub fn tile_update_strategy(&self) -> TileUpdateStrategy {
        match self {
            RenderQuality::Low => TileUpdateStrategy::Full,
            RenderQuality::Medium => TileUpdateStrategy::Partial,
            RenderQuality::High => TileUpdateStrategy::Partial,
        }
    }
}

/// Scaling filters for texture rendering
#[derive(Debug, Clone, PartialEq)]
pub enum ScalingFilter {
    Nearest,
    Bilinear,
    Bicubic,
}

/// Tile update strategies
#[derive(Debug, Clone, PartialEq)]
pub enum TileUpdateStrategy {
    Full,    // Update all tiles
    Partial, // Update only dirty tiles
}

/// Quality manager resource
#[derive(Resource)]
pub struct QualityManager {
    pub current_quality: RenderQuality,
    pub scaling_filter: ScalingFilter,
    pub mipmap_levels: u32,
    pub tile_update_strategy: TileUpdateStrategy,
}

impl Default for QualityManager {
    fn default() -> Self {
        let quality = RenderQuality::High;
        Self {
            current_quality: quality.clone(),
            scaling_filter: quality.scaling_filter(),
            mipmap_levels: quality.mipmap_levels(),
            tile_update_strategy: quality.tile_update_strategy(),
        }
    }
}

impl QualityManager {
    /// Create a new quality manager with specified quality
    pub fn new(quality: RenderQuality) -> Self {
        Self {
            scaling_filter: quality.scaling_filter(),
            mipmap_levels: quality.mipmap_levels(),
            tile_update_strategy: quality.tile_update_strategy(),
            current_quality: quality,
        }
    }
    
    /// Set the current quality level
    pub fn set_quality(&mut self, quality: RenderQuality) {
        self.current_quality = quality.clone();
        self.scaling_filter = quality.scaling_filter();
        self.mipmap_levels = quality.mipmap_levels();
        self.tile_update_strategy = quality.tile_update_strategy();
    }
    
    /// Check if bicubic scaling should be used
    pub fn use_bicubic_scaling(&self) -> bool {
        matches!(self.scaling_filter, ScalingFilter::Bicubic)
    }
    
    /// Check if mipmaps should be generated
    pub fn use_mipmaps(&self) -> bool {
        self.mipmap_levels > 0
    }
    
    /// Get the appropriate scaling filter based on zoom level
    pub fn get_scaling_filter_for_zoom(&self, zoom: f32) -> ScalingFilter {
        match &self.current_quality {
            RenderQuality::High => {
                // Use bicubic for downscaling, bilinear for upscaling
                if zoom < 1.0 {
                    ScalingFilter::Bicubic
                } else {
                    ScalingFilter::Bilinear
                }
            }
            _ => self.scaling_filter.clone()
        }
    }
}

/// System to apply quality settings to the rendering pipeline
pub fn apply_quality_settings(
    mut quality_manager: ResMut<QualityManager>,
    render_settings: Res<crate::rendering::RenderSettings>,
) {
    // Update quality manager when render settings change
    if quality_manager.current_quality != render_settings.quality {
        quality_manager.set_quality(render_settings.quality.clone());
        info!("Applied quality settings: {:?}", render_settings.quality);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quality_settings() {
        let low = RenderQuality::Low;
        assert_eq!(low.scaling_filter(), ScalingFilter::Nearest);
        assert_eq!(low.mipmap_levels(), 0);
        assert_eq!(low.tile_update_strategy(), TileUpdateStrategy::Full);
        
        let medium = RenderQuality::Medium;
        assert_eq!(medium.scaling_filter(), ScalingFilter::Bilinear);
        assert_eq!(medium.mipmap_levels(), 2);
        assert_eq!(medium.tile_update_strategy(), TileUpdateStrategy::Partial);
        
        let high = RenderQuality::High;
        assert_eq!(high.scaling_filter(), ScalingFilter::Bicubic);
        assert_eq!(high.mipmap_levels(), 4);
        assert_eq!(high.tile_update_strategy(), TileUpdateStrategy::Partial);
    }
    
    #[test]
    fn test_quality_manager() {
        let mut manager = QualityManager::default();
        assert_eq!(manager.current_quality, RenderQuality::High);
        assert!(manager.use_mipmaps());
        assert!(manager.use_bicubic_scaling());
        
        manager.set_quality(RenderQuality::Low);
        assert_eq!(manager.current_quality, RenderQuality::Low);
        assert!(!manager.use_mipmaps());
        assert!(!manager.use_bicubic_scaling());
    }
    
    #[test]
    fn test_zoom_based_scaling() {
        let manager = QualityManager::new(RenderQuality::High);
        
        // Downscaling should use bicubic
        assert_eq!(manager.get_scaling_filter_for_zoom(0.5), ScalingFilter::Bicubic);
        
        // Upscaling should use bilinear
        assert_eq!(manager.get_scaling_filter_for_zoom(2.0), ScalingFilter::Bilinear);
    }
}
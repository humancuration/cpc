//! Fallback renderer for visualization components
//!
//! This module provides a fallback rendering mechanism for visualizations
//! when the primary rendering fails.

use image::DynamicImage;
use crate::domain::errors::VizError;

/// Fallback renderer for visualizations
pub struct FallbackRenderer;

impl FallbackRenderer {
    /// Create a new fallback renderer
    pub fn new() -> Self {
        Self
    }
    
    /// Render a fallback visualization when the primary renderer fails
    pub fn render_fallback(&self, error: &VizError, width: u32, height: u32) -> DynamicImage {
        // Create a simple fallback image with error information
        let mut img = image::RgbImage::new(width, height);
        
        // Fill with a neutral background color
        for pixel in img.pixels_mut() {
            *pixel = image::Rgb([240, 240, 240]);
        }
        
        // Add error text to the image
        let error_text = match error {
            VizError::AccessibilityFailure(_) => "Accessibility Error",
            VizError::CacheVersionConflict { .. } => "Cache Version Conflict",
            VizError::RenderFallbackTriggered => "Render Failed",
            VizError::DataTransformationError => "Data Transformation Error",
        };
        
        // In a real implementation, we would use a proper text rendering library
        // to draw the error message on the image. For now, we'll just log it.
        println!("Rendering fallback visualization: {}", error_text);
        
        DynamicImage::ImageRgb8(img)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fallback_renderer_creation() {
        let renderer = FallbackRenderer::new();
        assert!(true); // Just test that we can create it
    }
    
    #[test]
    fn test_fallback_rendering() {
        let renderer = FallbackRenderer::new();
        let error = VizError::RenderFallbackTriggered;
        let img = renderer.render_fallback(&error, 400, 300);
        
        assert_eq!(img.width(), 400);
        assert_eq!(img.height(), 300);
    }
}
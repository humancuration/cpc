//! Heatmap visualization component for rating distributions

use feedback_analysis::RatingDistribution;
use feedback_core::FeedbackError;
use plotters::prelude::*;
use std::collections::HashMap;

/// Heatmap visualization component
pub struct Heatmap {
    title: String,
    color_gradient: ColorGradient,
}

/// Color gradient for heatmap visualization
pub enum ColorGradient {
    BlueToRed,
    GreenToRed,
    Monochrome,
}

impl Heatmap {
    /// Create a new heatmap with the specified title
    pub fn new(title: String) -> Self {
        Self {
            title,
            color_gradient: ColorGradient::BlueToRed,
        }
    }
    
    /// Set the color gradient for the heatmap
    pub fn with_gradient(mut self, gradient: ColorGradient) -> Self {
        self.color_gradient = gradient;
        self
    }
    
    /// Generate a PNG image of the heatmap
    pub fn render_png(&self, distribution: &RatingDistribution) -> Result<Vec<u8>, FeedbackError> {
        // Create an in-memory image buffer
        let mut buffer = vec![0; 800 * 600 * 3];
        
        {
            let root = BitMapBackend::with_buffer(&mut buffer, (800, 600)).into_drawing_area();
            root.fill(&WHITE).map_err(|e| FeedbackError::Visualization(e.to_string()))?;
            
            let sorted_values = distribution.sorted_values();
            if sorted_values.is_empty() {
                return Ok(buffer);
            }
            
            let max_count = sorted_values.iter().map(|&(_, count)| count).max().unwrap_or(1);
            
            let mut chart = ChartBuilder::on(&root)
                .caption(&self.title, ("sans-serif", 30))
                .margin(10)
                .x_label_area_size(40)
                .y_label_area_size(40)
                .build_cartesian_2d(0u8..=100u8, 0u32..=max_count)
                .map_err(|e| FeedbackError::Visualization(e.to_string()))?;
            
            chart.configure_mesh().draw().map_err(|e| FeedbackError::Visualization(e.to_string()))?;
            
            // Draw bars for each rating value
            for &(value, count) in &sorted_values {
                let color = self.get_color_for_value(value, max_count, count);
                let bar = Rectangle::new(
                    [(value.saturating_sub(2), 0), (value + 2, count)],
                    color.filled(),
                );
                chart.draw_series(std::iter::once(bar))
                    .map_err(|e| FeedbackError::Visualization(e.to_string()))?;
            }
            
            root.present().map_err(|e| FeedbackError::Visualization(e.to_string()))?;
        }
        
        Ok(buffer)
    }
    
    /// Get color based on the color gradient setting
    fn get_color_for_value(&self, value: u8, max_count: u32, count: u32) -> RGBColor {
        match self.color_gradient {
            ColorGradient::BlueToRed => {
                // Simple blue to red gradient based on rating value
                let ratio = value as f32 / 100.0;
                RGBColor((ratio * 255.0) as u8, 0, ((1.0 - ratio) * 255.0) as u8)
            },
            ColorGradient::GreenToRed => {
                // Simple green to red gradient based on rating value
                if value < 50 {
                    // Green to yellow
                    let ratio = value as f32 / 50.0;
                    RGBColor((ratio * 255.0) as u8, 255, 0)
                } else {
                    // Yellow to red
                    let ratio = (value - 50) as f32 / 50.0;
                    RGBColor(255, ((1.0 - ratio) * 255.0) as u8, 0)
                }
            },
            ColorGradient::Monochrome => {
                // Gray scale based on count density
                let ratio = count as f32 / max_count as f32;
                let gray = (255.0 * (1.0 - ratio)) as u8;
                RGBColor(gray, gray, gray)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use feedback_analysis::RatingDistribution;

    #[test]
    fn test_heatmap_creation() {
        let heatmap = Heatmap::new("Test Heatmap".to_string());
        assert_eq!(heatmap.title, "Test Heatmap");
    }

    #[test]
    fn test_heatmap_with_gradient() {
        let heatmap = Heatmap::new("Test Heatmap".to_string())
            .with_gradient(ColorGradient::GreenToRed);
        
        // We can't easily test the gradient without rendering, but we can ensure
        // the struct is created correctly
        assert_eq!(heatmap.title, "Test Heatmap");
    }
}
//! Statistical visualization adapters
//!
//! This module provides adapters for rendering statistical visualization components
//! including confidence intervals and significance indicators.

use crate::domain::{
    confidence_interval::{ConfidenceIntervalConfig, SignificanceIndicator, StatisticalChartConfig},
    chart::ChartConfig,
    data::DataSeries,
    VisualizationError,
};
use image::{ImageBuffer, Rgba};

/// Statistical visualization adapter
pub struct StatisticalVisualizationAdapter;

impl StatisticalVisualizationAdapter {
    /// Render confidence interval area on a chart
    pub fn render_confidence_interval(
        config: &ConfidenceIntervalConfig,
        canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<(), VisualizationError> {
        // In a real implementation, this would render the confidence interval on the canvas
        // For now, we'll just log that rendering is needed
        tracing::debug!(
            "Rendering confidence interval with {}% confidence level",
            config.confidence_level * 100.0
        );
        
        Ok(())
    }
    
    /// Render error bars for confidence intervals
    pub fn render_error_bars(
        config: &ConfidenceIntervalConfig,
        canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<(), VisualizationError> {
        // In a real implementation, this would render error bars on the canvas
        // For now, we'll just log that rendering is needed
        tracing::debug!("Rendering error bars for confidence interval");
        
        Ok(())
    }
    
    /// Render significance indicators on a chart
    pub fn render_significance_indicators(
        indicators: &[SignificanceIndicator],
        canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<(), VisualizationError> {
        // In a real implementation, this would render significance indicators on the canvas
        // For now, we'll just log that rendering is needed
        tracing::debug!("Rendering {} significance indicators", indicators.len());
        
        Ok(())
    }
    
    /// Render statistical explanations on a chart
    pub fn render_statistical_explanations(
        config: &StatisticalChartConfig,
        canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<(), VisualizationError> {
        // In a real implementation, this would render statistical explanations on the canvas
        // For now, we'll just log that rendering is needed
        tracing::debug!(
            "Rendering statistical explanations for {} confidence intervals and {} significance indicators",
            config.confidence_intervals.len(),
            config.significance_indicators.len()
        );
        
        Ok(())
    }
}

/// Error bar renderer
pub struct ErrorBarRenderer;

impl ErrorBarRenderer {
    /// Draw vertical error bars
    pub fn draw_vertical_error_bars(
        x_positions: &[f64],
        y_positions: &[f64],
        upper_errors: &[f64],
        lower_errors: &[f64],
        color: &str,
        canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<(), VisualizationError> {
        // In a real implementation, this would draw vertical error bars on the canvas
        // For now, we'll just validate the inputs
        if x_positions.len() != y_positions.len() || 
           x_positions.len() != upper_errors.len() || 
           x_positions.len() != lower_errors.len() {
            return Err(VisualizationError::DataTransformationError(
                "All error bar arrays must have the same length".to_string()
            ));
        }
        
        tracing::debug!("Drawing {} vertical error bars", x_positions.len());
        Ok(())
    }
    
    /// Draw horizontal error bars
    pub fn draw_horizontal_error_bars(
        x_positions: &[f64],
        y_positions: &[f64],
        right_errors: &[f64],
        left_errors: &[f64],
        color: &str,
        canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<(), VisualizationError> {
        // In a real implementation, this would draw horizontal error bars on the canvas
        // For now, we'll just validate the inputs
        if x_positions.len() != y_positions.len() || 
           x_positions.len() != right_errors.len() || 
           x_positions.len() != left_errors.len() {
            return Err(VisualizationError::DataTransformationError(
                "All error bar arrays must have the same length".to_string()
            ));
        }
        
        tracing::debug!("Drawing {} horizontal error bars", x_positions.len());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageBuffer;
    
    #[test]
    fn test_render_confidence_interval() {
        let config = ConfidenceIntervalConfig {
            main_series: DataSeries::new("Main".to_string(), vec![]),
            upper_bound: DataSeries::new("Upper".to_string(), vec![]),
            lower_bound: DataSeries::new("Lower".to_string(), vec![]),
            confidence_level: 0.95,
            interval_color: "#808080".to_string(),
            show_error_bars: false,
        };
        
        let mut canvas = ImageBuffer::new(100, 100);
        let result = StatisticalVisualizationAdapter::render_confidence_interval(&config, &mut canvas);
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_error_bar_validation() {
        let mut canvas = ImageBuffer::new(100, 100);
        let result = ErrorBarRenderer::draw_vertical_error_bars(
            &[1.0, 2.0],
            &[1.0, 2.0, 3.0], // Different length
            &[0.1, 0.2],
            &[0.1, 0.2],
            "#FF0000",
            &mut canvas,
        );
        
        assert!(result.is_err());
    }
}
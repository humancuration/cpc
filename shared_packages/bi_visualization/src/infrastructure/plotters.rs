//! Plotters integration
//! 
//! This module provides integration with Plotters for chart generation.

use crate::domain::{
    chart::ChartConfig,
    data::DataSeries,
    VisualizationError,
};
use image::{ImageBuffer, Rgba};
use plotters::prelude::*;

/// Plotters-based chart generator
pub struct PlottersChartGenerator;

impl PlottersChartGenerator {
    /// Generate a chart using Plotters
    pub fn generate_chart(
        config: &ChartConfig,
        data: &DataSeries,
    ) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, VisualizationError> {
        // Create an in-memory image buffer
        let (width, height) = config.dimensions;
        let mut buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
        
        // Create a Plotters backend that draws to our image buffer
        {
            let root = BitMapBackend::with_buffer(buffer.as_mut(), (width, height))
                .into_drawing_area();
            
            // Clear the drawing area with the appropriate background color
            let bg_color = match config.theme {
                crate::domain::chart::VisualizationTheme::Light => &WHITE,
                crate::domain::chart::VisualizationTheme::Dark => &BLACK,
                crate::domain::chart::VisualizationTheme::HighContrast => &BLACK,
            };
            root.fill(bg_color)
                .map_err(|e| VisualizationError::ChartGenerationError(e.to_string()))?;
            
            // Create the chart context
            let mut chart = ChartBuilder::on(&root)
                .caption(&config.title, ("sans-serif", 20))
                .margin(10)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(0f64..100f64, 0f64..100f64)
                .map_err(|e| VisualizationError::ChartGenerationError(e.to_string()))?;
            
            // Configure the mesh
            chart
                .configure_mesh()
                .x_labels(5)
                .y_labels(5)
                .draw()
                .map_err(|e| VisualizationError::ChartGenerationError(e.to_string()))?;
            
            // Draw the data series
            for (series_index, series) in data.series.iter().enumerate() {
                let color = match series_index % 3 {
                    0 => &RED,
                    1 => &BLUE,
                    _ => &GREEN,
                };
                
                // Extract data points (simplified for this example)
                let points: Vec<(f64, f64)> = series
                    .points
                    .iter()
                    .enumerate()
                    .map(|(i, _)| (i as f64 * 10.0, (i as f64 * 5.0) % 100.0))
                    .collect();
                
                // Draw the series based on chart type
                match config.chart_type {
                    crate::domain::chart::ChartType::Line => {
                        chart
                            .draw_series(LineSeries::new(points, color.filled()))
                            .map_err(|e| VisualizationError::ChartGenerationError(e.to_string()))?;
                    }
                    crate::domain::chart::ChartType::Bar => {
                        chart
                            .draw_series(points.iter().map(|(x, y)| {
                                Rectangle::new(
                                    [(*x - 2.0, 0.0), (*x + 2.0, *y)],
                                    color.filled(),
                                )
                            }))
                            .map_err(|e| VisualizationError::ChartGenerationError(e.to_string()))?;
                    }
                    _ => {
                        // For other chart types, draw as a simple line
                        chart
                            .draw_series(LineSeries::new(points, color.filled()))
                            .map_err(|e| VisualizationError::ChartGenerationError(e.to_string()))?;
                    }
                }
            }
            
            // Present the chart to ensure all operations are completed
            root.present()
                .map_err(|e| VisualizationError::ChartGenerationError(e.to_string()))?;
        }
        
        Ok(buffer)
    }
}
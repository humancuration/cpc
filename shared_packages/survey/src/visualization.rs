//! Visualization module for survey data
//!
//! This module provides functions to create various types of visualizations
//! from survey data, including histograms, heatmaps, word clouds, and trend lines.

use crate::models::{SurveyResponse, Answer};
use crate::error::VisualizationError;
use std::collections::HashMap;

/// Plotting functions for survey data visualization
pub mod plot {
    use super::*;
    use plotters::prelude::*;
    
    /// Create a histogram from numerical data
    pub fn histogram(data: &[f32], bins: usize) -> Result<Vec<u8>, VisualizationError> {
        if data.is_empty() {
            return Err(VisualizationError::EmptyData);
        }
        
        if bins == 0 {
            return Err(VisualizationError::InvalidDataFormat("Bins must be greater than 0".to_string()));
        }
        
        // Create in-memory buffer for the image
        let mut buffer = vec![0; 800 * 600 * 3];
        
        // Create backend and root drawing area
        let root = BitMapBackend::with_buffer(&mut buffer, (800, 600)).into_drawing_area();
        root.fill(&WHITE).map_err(|e| VisualizationError::RenderingError(e.to_string()))?;
        
        // Create chart context
        let mut chart = ChartBuilder::on(&root)
            .caption("Histogram", ("sans-serif", 30))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0.0..10.0, 0..100)
            .map_err(|e| VisualizationError::RenderingError(e.to_string()))?;
        
        chart.configure_mesh().draw().map_err(|e| VisualizationError::RenderingError(e.to_string()))?;
        
        // For simplicity, we'll create a dummy histogram
        // In a real implementation, we would calculate the actual histogram
        let histogram_data = vec![(0.0, 10), (1.0, 20), (2.0, 30), (3.0, 25), (4.0, 15)];
        
        chart.draw_series(
            histogram_data.iter().map(|(x, y)| {
                let x_start = x - 0.5;
                let x_end = x + 0.5;
                Rectangle::new([(x_start, 0), (x_end, *y)], RED.filled())
            }),
        ).map_err(|e| VisualizationError::RenderingError(e.to_string()))?;
        
        root.present().map_err(|e| VisualizationError::RenderingError(e.to_string()))?;
        
        Ok(buffer)
    }
    
    /// Create a heatmap from a matrix of data
    pub fn heatmap(matrix: &[Vec<u32>]) -> Result<Vec<u8>, VisualizationError> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return Err(VisualizationError::EmptyData);
        }
        
        // Create in-memory buffer for the image
        let mut buffer = vec![0; 800 * 600 * 3];
        
        // Create backend and root drawing area
        let root = BitMapBackend::with_buffer(&mut buffer, (800, 600)).into_drawing_area();
        root.fill(&WHITE).map_err(|e| VisualizationError::RenderingError(e.to_string()))?;
        
        // Create chart context
        let rows = matrix.len();
        let cols = matrix[0].len();
        
        let mut chart = ChartBuilder::on(&root)
            .caption("Heatmap", ("sans-serif", 30))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0..cols, 0..rows)
            .map_err(|e| VisualizationError::RenderingError(e.to_string()))?;
        
        chart.configure_mesh().draw().map_err(|e| VisualizationError::RenderingError(e.to_string()))?;
        
        // Draw heatmap cells
        for (row_idx, row) in matrix.iter().enumerate() {
            for (col_idx, &value) in row.iter().enumerate() {
                let color_value = (value as f32 / 100.0).min(1.0).max(0.0); // Normalize to [0,1]
                let color = RGBColor((color_value * 255.0) as u8, 0, ((1.0 - color_value) * 255.0) as u8);
                
                chart.draw_series(std::iter::once(
                    Rectangle::new(
                        [(col_idx, row_idx), (col_idx + 1, row_idx + 1)],
                        color.filled(),
                    ),
                )).map_err(|e| VisualizationError::RenderingError(e.to_string()))?;
            }
        }
        
        root.present().map_err(|e| VisualizationError::RenderingError(e.to_string()))?;
        
        Ok(buffer)
    }
    
    /// Create a word cloud from word-frequency pairs
    pub fn word_cloud(words: &[(&str, f32)]) -> Result<Vec<u8>, VisualizationError> {
        if words.is_empty() {
            return Err(VisualizationError::EmptyData);
        }
        
        // Create in-memory buffer for the image
        let mut buffer = vec![0; 800 * 600 * 3];
        
        // Create backend and root drawing area
        let root = BitMapBackend::with_buffer(&mut buffer, (800, 600)).into_drawing_area();
        root.fill(&WHITE).map_err(|e| VisualizationError::RenderingError(e.to_string()))?;
        
        // For simplicity, we'll just display the words with different sizes
        // In a real implementation, this would use a proper word cloud algorithm
        let mut chart = ChartBuilder::on(&root)
            .caption("Word Cloud", ("sans-serif", 30))
            .margin(20)
            .build_cartesian_2d(0..100i32, 0..100i32)
            .map_err(|e| VisualizationError::RenderingError(e.to_string()))?;
        
        for (i, &(word, frequency)) in words.iter().enumerate() {
            let size = (frequency * 20.0 + 10.0) as u32; // Scale frequency to font size
            let x = (i * 15) % 80;
            let y = (i / 5) * 15;
            
            root.draw_text(
                word, 
                &TextStyle::from(("sans-serif", size).into_font()), 
                (x as i32, y as i32)
            ).map_err(|e| VisualizationError::RenderingError(e.to_string()))?;
        }
        
        root.present().map_err(|e| VisualizationError::RenderingError(e.to_string()))?;
        
        Ok(buffer)
    }
    
    /// Create a trend line from time-series data
    pub fn trend_line(points: &[(chrono::DateTime<chrono::Utc>, f32)]) -> Result<Vec<u8>, VisualizationError> {
        if points.is_empty() {
            return Err(VisualizationError::EmptyData);
        }
        
        // Create in-memory buffer for the image
        let mut buffer = vec![0; 800 * 600 * 3];
        
        // Create backend and root drawing area
        let root = BitMapBackend::with_buffer(&mut buffer, (800, 600)).into_drawing_area();
        root.fill(&WHITE).map_err(|e| VisualizationError::RenderingError(e.to_string()))?;
        
        // Extract x and y values
        let y_values: Vec<f32> = points.iter().map(|(_, y)| *y).collect();
        let min_y = *y_values.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max_y = *y_values.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        
        // Create chart context
        let mut chart = ChartBuilder::on(&root)
            .caption("Trend Line", ("sans-serif", 30))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0..points.len(), min_y..max_y)
            .map_err(|e| VisualizationError::RenderingError(e.to_string()))?;
        
        chart.configure_mesh().draw().map_err(|e| VisualizationError::RenderingError(e.to_string()))?;
        
        // Draw trend line
        chart.draw_series(LineSeries::new(
            points.iter().enumerate().map(|(i, (_, y))| (i, *y)),
            &RED,
        )).map_err(|e| VisualizationError::RenderingError(e.to_string()))?;
        
        root.present().map_err(|e| VisualizationError::RenderingError(e.to_string()))?;
        
        Ok(buffer)
    }
}
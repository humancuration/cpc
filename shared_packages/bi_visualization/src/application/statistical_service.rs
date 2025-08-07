//! Statistical analysis service for BI visualization
//!
//! This module provides services for integrating statistical analysis
//! with data visualization components.

use crate::domain::{
    data::{DataSeries, ProcessedData, RawData},
    confidence_interval::{ConfidenceIntervalConfig, SignificanceIndicator, StatisticalChartConfig},
    chart::{ChartConfig, ChartType},
    VisualizationError,
};
use image::{ImageBuffer, Rgba};

/// Statistical analysis service
pub struct StatisticalAnalysisService;

impl StatisticalAnalysisService {
    /// Generate a chart with confidence intervals
    pub fn generate_chart_with_confidence(
        config: StatisticalChartConfig,
        data: Vec<DataSeries>,
    ) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, VisualizationError> {
        // In a real implementation, this would generate a chart with confidence intervals
        // For now, we'll create a simple placeholder image
        let (width, height) = config.base_config.dimensions;
        let mut img = ImageBuffer::new(width, height);
        
        // Fill with a simple gradient based on theme
        let (r, g, b) = match config.base_config.theme {
            crate::domain::chart::VisualizationTheme::Light => (240, 240, 240),
            crate::domain::chart::VisualizationTheme::Dark => (40, 40, 40),
            crate::domain::chart::VisualizationTheme::HighContrast => (0, 0, 0),
        };
        
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let intensity = ((x as f32 / width as f32) * 255.0) as u8;
            *pixel = Rgba([r, g, b, intensity]);
        }
        
        Ok(img)
    }
    
    /// Add confidence intervals to existing chart data
    pub fn add_confidence_intervals(
        base_data: DataSeries,
        upper_bound: DataSeries,
        lower_bound: DataSeries,
        confidence_level: f64,
    ) -> ConfidenceIntervalConfig {
        ConfidenceIntervalConfig::new(
            base_data,
            upper_bound,
            lower_bound,
            confidence_level,
            "#808080".to_string(), // Default gray color
            false, // Default to shaded area, not error bars
        )
    }
    
    /// Add significance indicators to chart data
    pub fn add_significance_indicators(
        data_points: &[(f64, f64, f64)], // (x, y, p_value)
        labels: Option<Vec<String>>,
    ) -> Vec<SignificanceIndicator> {
        data_points
            .iter()
            .enumerate()
            .map(|(i, &(x, y, p_value))| {
                let label = labels.as_ref()
                    .and_then(|l| l.get(i))
                    .cloned();
                SignificanceIndicator::new(p_value, (x, y), label)
            })
            .collect()
    }
    
    /// Transform raw data with statistical analysis
    pub fn transform_with_statistics(
        data: RawData,
        analysis_type: StatisticalAnalysisType,
    ) -> Result<ProcessedData, VisualizationError> {
        // In a real implementation, this would perform statistical analysis on the data
        // For now, we'll create a simple processed data structure
        let series = vec![DataSeries::new(
            "Statistical Analysis".to_string(),
            vec![], // In a real implementation, this would contain analyzed data points
        )];
        
        let metadata = serde_json::json!({
            "analysis_type": analysis_type.to_string(),
            "source": data.source,
            "point_count": data.values.len(),
        });
        
        Ok(ProcessedData::new(series, metadata))
    }
}

/// Types of statistical analysis
#[derive(Debug, Clone)]
pub enum StatisticalAnalysisType {
    /// Confidence interval analysis
    ConfidenceInterval,
    
    /// Significance testing
    SignificanceTest,
    
    /// Correlation analysis
    Correlation,
    
    /// Regression analysis
    Regression,
    
    /// Time series analysis
    TimeSeries,
}

impl std::fmt::Display for StatisticalAnalysisType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatisticalAnalysisType::ConfidenceInterval => write!(f, "ConfidenceInterval"),
            StatisticalAnalysisType::SignificanceTest => write!(f, "SignificanceTest"),
            StatisticalAnalysisType::Correlation => write!(f, "Correlation"),
            StatisticalAnalysisType::Regression => write!(f, "Regression"),
            StatisticalAnalysisType::TimeSeries => write!(f, "TimeSeries"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::data::{DataSeries, TimeSeriesPoint};
    use chrono::Utc;
    
    #[test]
    fn test_add_confidence_intervals() {
        let base_series = DataSeries::new("Base".to_string(), vec![]);
        let upper_series = DataSeries::new("Upper".to_string(), vec![]);
        let lower_series = DataSeries::new("Lower".to_string(), vec![]);
        
        let ci_config = StatisticalAnalysisService::add_confidence_intervals(
            base_series,
            upper_series,
            lower_series,
            0.95,
        );
        
        assert_eq!(ci_config.confidence_level, 0.95);
    }
    
    #[test]
    fn test_add_significance_indicators() {
        let data_points = vec![(1.0, 2.0, 0.001), (3.0, 4.0, 0.03), (5.0, 6.0, 0.1)];
        let indicators = StatisticalAnalysisService::add_significance_indicators(&data_points, None);
        
        assert_eq!(indicators.len(), 3);
        assert_eq!(indicators[0].significance_level, crate::domain::confidence_interval::SignificanceLevel::HighlySignificant);
        assert_eq!(indicators[1].significance_level, crate::domain::confidence_interval::SignificanceLevel::ModeratelySignificant);
        assert_eq!(indicators[2].significance_level, crate::domain::confidence_interval::SignificanceLevel::NotSignificant);
    }
}
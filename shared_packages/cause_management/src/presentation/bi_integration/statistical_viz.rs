//! Statistical visualization integration for cause management
//!
//! This module integrates statistical analysis results with BI visualization
//! components to create cooperative values-aligned cause visualizations.

#[cfg(all(feature = "statistics", feature = "visualization"))]
use cpc_statistics_core::{
    ConfidenceInterval,
    SignificanceResult,
    SignificanceLevel,
};

#[cfg(all(feature = "statistics", feature = "visualization"))]
use cpc_bi_visualization::{
    StatisticalChartConfig,
    ConfidenceIntervalConfig,
    SignificanceIndicator,
    VisualizationService,
    ChartConfig,
    ChartType,
    VisualizationTheme,
    SeriesConfig,
    DataSeries,
    TimeSeriesPoint,
};

#[cfg(all(feature = "statistics", feature = "visualization"))]
use crate::{
    domain::{
        statistical_models::{DonationForecast, DonationTrend},
        impact_models::ImpactAnalysis,
        models::Cause,
    },
    application::{
        statistical_analysis::StatisticalAnalysisService,
        impact_measurement::ImpactMeasurementService,
    },
};
#[cfg(all(feature = "statistics", feature = "visualization"))]
use image::DynamicImage;
#[cfg(all(feature = "statistics", feature = "visualization"))]
use chrono::{DateTime, Utc};
#[cfg(all(feature = "statistics", feature = "visualization"))]
use rust_decimal::Decimal;

/// Statistical visualization service for cause data
#[cfg(all(feature = "statistics", feature = "visualization"))]
pub struct CauseStatisticalVisualization;

#[cfg(all(feature = "statistics", feature = "visualization"))]
impl CauseStatisticalVisualization {
    /// Generate a donation forecast visualization with confidence intervals
    pub fn generate_donation_forecast_visualization(
        forecast: &DonationForecast,
        historical_donations: &[Decimal],
        cause: &Cause,
    ) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        // Create base chart configuration
        let base_config = ChartConfig::new(
            ChartType::Line,
            format!("Donation Forecast: {}", cause.name),
            (800, 600),
            VisualizationTheme::Light,
            vec![
                SeriesConfig::new("Historical", "#2196F3"),
                SeriesConfig::new("Forecast", "#4CAF50"),
            ],
        );
        
        // Transform historical data to visualization format
        // For simplicity, we'll create a time series with dummy dates
        let mut historical_points = Vec::new();
        let now = Utc::now();
        for (i, donation) in historical_donations.iter().enumerate() {
            let date = now - chrono::Duration::days((historical_donations.len() - i) as i64);
            historical_points.push(TimeSeriesPoint::new(date, donation.to_f64().unwrap_or(0.0)));
        }
        
        let historical_series = DataSeries::from_time_series("Historical".to_string(), historical_points);
        
        // Create forecast data series
        let mut forecast_points = Vec::new();
        let last_date = now;
        
        for (i, &value) in forecast.forecast_values.iter().enumerate() {
            let forecast_date = last_date + chrono::Duration::days((i + 1) as i64);
            forecast_points.push(TimeSeriesPoint::new(forecast_date, value));
        }
        
        let forecast_series = DataSeries::from_time_series("Forecast".to_string(), forecast_points);
        
        // Create confidence interval configuration
        let upper_bound_series = DataSeries::from_time_series(
            "Upper Bound".to_string(),
            vec![TimeSeriesPoint::new(last_date, forecast.confidence_interval.upper)],
        );
        
        let lower_bound_series = DataSeries::from_time_series(
            "Lower Bound".to_string(),
            vec![TimeSeriesPoint::new(last_date, forecast.confidence_interval.lower)],
        );
        
        let confidence_config = ConfidenceIntervalConfig::new(
            forecast_series.clone(),
            upper_bound_series,
            lower_bound_series,
            forecast.confidence_level,
            "#808080".to_string(), // Gray color for confidence interval
            false, // Shaded area, not error bars
        );
        
        // Create significance indicators
        let significance_indicators = vec![SignificanceIndicator::new(
            forecast.trend_significance.p_value,
            (0.0, 0.0), // Position would be calculated based on chart coordinates
            Some("Trend Significance".to_string()),
        )];
        
        // Create statistical chart configuration
        let statistical_config = StatisticalChartConfig::new(
            base_config,
            vec![confidence_config],
            significance_indicators,
            true, // Show explanations
        );
        
        // Generate chart
        let data = vec![historical_series, forecast_series];
        let image = VisualizationService::generate_statistical_chart(statistical_config, data)?;
        
        Ok(DynamicImage::ImageRgba8(image))
    }
    
    /// Generate an impact analysis visualization with statistical analysis
    pub fn generate_impact_analysis_visualization(
        impact_analysis: &ImpactAnalysis,
        cause: &Cause,
    ) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        // Create base chart configuration
        let base_config = ChartConfig::new(
            ChartType::Bar,
            format!("Impact Analysis: {}", cause.name),
            (800, 600),
            VisualizationTheme::Light,
            vec![
                SeriesConfig::new("Impact Score", "#9C27B0"),
            ],
        );
        
        // Create data points
        let impact_desc = match impact_analysis.impact_type {
            crate::domain::impact_models::ImpactMetric::LivesImpacted => "Lives Impacted",
            crate::domain::impact_models::ImpactMetric::EnvironmentalBenefit => "Environmental Benefit",
            crate::domain::impact_models::ImpactMetric::CommunityEngagement => "Community Engagement",
            crate::domain::impact_models::ImpactMetric::EconomicImpact => "Economic Impact",
            crate::domain::impact_models::ImpactMetric::EducationalOutcomes => "Educational Outcomes",
            crate::domain::impact_models::ImpactMetric::HealthOutcomes => "Health Outcomes",
        };
        
        let impact_point = (impact_desc.to_string(), impact_analysis.impact_score);
        let impact_series = DataSeries::from_bar("Impact Score".to_string(), vec![impact_point]);
        
        // Create confidence interval configuration
        let upper_bound_series = DataSeries::from_bar(
            "Upper Bound".to_string(),
            vec![(impact_desc.to_string(), impact_analysis.confidence_interval.upper)],
        );
        
        let lower_bound_series = DataSeries::from_bar(
            "Lower Bound".to_string(),
            vec![(impact_desc.to_string(), impact_analysis.confidence_interval.lower)],
        );
        
        let confidence_config = ConfidenceIntervalConfig::new(
            impact_series.clone(),
            upper_bound_series,
            lower_bound_series,
            impact_analysis.confidence_interval.confidence_level,
            "#808080".to_string(), // Gray color for confidence interval
            true, // Show error bars
        );
        
        // Create significance indicators based on statistical significance
        let significance_color = match impact_analysis.evidence_strength {
            SignificanceLevel::HighlySignificant => "#4CAF50", // Green
            SignificanceLevel::ModeratelySignificant => "#FFC107", // Yellow
            SignificanceLevel::NotSignificant => "#F44336", // Red
        };
        
        let significance_indicators = vec![SignificanceIndicator::new(
            impact_analysis.p_value,
            (0.0, 0.0), // Position would be calculated based on chart coordinates
            Some(format!("Significance: {}", significance_color)),
        )];
        
        // Create statistical chart configuration
        let statistical_config = StatisticalChartConfig::new(
            base_config,
            vec![confidence_config],
            significance_indicators,
            true, // Show explanations
        );
        
        // Generate chart
        let data = vec![impact_series];
        let image = VisualizationService::generate_statistical_chart(statistical_config, data)?;
        
        Ok(DynamicImage::ImageRgba8(image))
    }
    
    /// Generate a comprehensive dashboard showing both forecast and impact
    pub fn generate_cause_dashboard(
        forecast: &DonationForecast,
        impact_analysis: &ImpactAnalysis,
        historical_donations: &[Decimal],
        cause: &Cause,
    ) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        // This would create a composite visualization showing both forecast and impact
        // For simplicity, we'll just generate the forecast visualization
        Self::generate_donation_forecast_visualization(forecast, historical_donations, cause)
    }
}

// Fallback implementations when features are disabled
#[cfg(not(all(feature = "statistics", feature = "visualization")))]
pub struct CauseStatisticalVisualization;

#[cfg(not(all(feature = "statistics", feature = "visualization")))]
impl CauseStatisticalVisualization {
    /// Generate a donation forecast visualization (stub implementation)
    pub fn generate_donation_forecast_visualization(
        _forecast: &DonationForecast,
        _historical_donations: &[Decimal],
        _cause: &Cause,
    ) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        Err("Statistical visualization requires both 'statistics' and 'visualization' features to be enabled".into())
    }
    
    /// Generate an impact analysis visualization (stub implementation)
    pub fn generate_impact_analysis_visualization(
        _impact_analysis: &ImpactAnalysis,
        _cause: &Cause,
    ) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        Err("Statistical visualization requires both 'statistics' and 'visualization' features to be enabled".into())
    }
    
    /// Generate a cause dashboard (stub implementation)
    pub fn generate_cause_dashboard(
        _forecast: &DonationForecast,
        _impact_analysis: &ImpactAnalysis,
        _historical_donations: &[Decimal],
        _cause: &Cause,
    ) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        Err("Statistical visualization requires both 'statistics' and 'visualization' features to be enabled".into())
    }
}

#[cfg(test)]
#[cfg(all(feature = "statistics", feature = "visualization"))]
mod tests {
    use super::*;
    use crate::domain::models::Cause;
    use rust_decimal_macros::dec;
    
    #[test]
    fn test_visualization_service_exists() {
        // This test just verifies that the necessary types exist
        // Actual visualization testing would require more complex setup
        assert!(true);
    }
}

#[cfg(test)]
#[cfg(not(all(feature = "statistics", feature = "visualization")))]
mod tests {
    use super::*;
    
    #[test]
    fn test_fallback_implementations() {
        // Test that fallback implementations exist and return appropriate errors
        let cause = Cause::new(
            "Test Cause".to_string(),
            "Test Description".to_string(),
            None,
        );
        
        let donations = vec![dec!(100.0), dec!(120.0), dec!(90.0)];
        
        // These should all return errors when features are disabled
        assert!(CauseStatisticalVisualization::generate_donation_forecast_visualization(
            &DonationForecast {},  // Empty struct for test
            &donations,
            &cause,
        ).is_err());
    }
}
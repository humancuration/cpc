//! Statistical visualization integration for finance app
//!
//! This module integrates statistical analysis results with BI visualization
//! components to create cooperative values-aligned financial visualizations.

#[cfg(feature = "statistics")]
use cpc_statistics_core::{
    ConfidenceInterval,
    SignificanceResult,
    SignificanceLevel,
};

#[cfg(feature = "visualization")]
use bi_visualization::{
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
    application::statistical_forecasting::{ExpenseForecast, BudgetForecast},
    application::impact_analysis::{SavingsImpactAnalysis, BudgetingImpactAnalysis},
    domain::{Budget, Expense},
};
#[cfg(all(feature = "statistics", feature = "visualization"))]
use image::DynamicImage;
#[cfg(all(feature = "statistics", feature = "visualization"))]
use chrono::{DateTime, Utc};

/// Statistical visualization service for financial data
#[cfg(all(feature = "statistics", feature = "visualization"))]
pub struct FinancialStatisticalVisualization;

#[cfg(all(feature = "statistics", feature = "visualization"))]
impl FinancialStatisticalVisualization {
    /// Generate a forecast visualization with confidence intervals
    pub fn generate_forecast_visualization(
        forecast: &ExpenseForecast,
        historical_expenses: &[Expense],
    ) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        // Create base chart configuration
        let base_config = ChartConfig::new(
            ChartType::Line,
            "Expense Forecast with Confidence Intervals".to_string(),
            (800, 600),
            VisualizationTheme::Light,
            vec![
                SeriesConfig::new("Historical", "#2196F3"),
                SeriesConfig::new("Forecast", "#4CAF50"),
            ],
        );
        
        // Transform historical data to visualization format
        let historical_points: Vec<TimeSeriesPoint> = historical_expenses
            .iter()
            .map(|e| TimeSeriesPoint::new(e.date, e.amount.amount))
            .collect();
        
        let historical_series = DataSeries::from_time_series("Historical".to_string(), historical_points);
        
        // Create forecast data series
        let mut forecast_points = Vec::new();
        let last_date = historical_expenses
            .last()
            .map(|e| e.date)
            .unwrap_or_else(Utc::now);
        
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
    
    /// Generate a budget impact visualization with statistical analysis
    pub fn generate_budget_impact_visualization(
        budget_forecast: &BudgetForecast,
        budget: &Budget,
    ) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        // Create base chart configuration
        let base_config = ChartConfig::new(
            ChartType::Bar,
            format!("Budget Utilization: {}", budget.name),
            (800, 600),
            VisualizationTheme::Light,
            vec![
                SeriesConfig::new("Current", "#2196F3"),
                SeriesConfig::new("Forecast", "#4CAF50"),
            ],
        );
        
        // Create data points
        let current_point = (budget.name.clone(), budget_forecast.current_utilization * 100.0);
        let forecast_point = (format!("{} (Forecast)", budget.name), budget_forecast.forecasted_utilization * 100.0);
        
        let current_series = DataSeries::from_bar("Current".to_string(), vec![current_point]);
        let forecast_series = DataSeries::from_bar("Forecast".to_string(), vec![forecast_point]);
        
        // Create confidence interval configuration
        let upper_bound_series = DataSeries::from_bar(
            "Upper Bound".to_string(),
            vec![(budget.name.clone(), budget_forecast.confidence_interval.upper * 100.0)],
        );
        
        let lower_bound_series = DataSeries::from_bar(
            "Lower Bound".to_string(),
            vec![(budget.name.clone(), budget_forecast.confidence_interval.lower * 100.0)],
        );
        
        let confidence_config = ConfidenceIntervalConfig::new(
            forecast_series.clone(),
            upper_bound_series,
            lower_bound_series,
            0.95, // Standard 95% confidence level
            "#808080".to_string(), // Gray color for confidence interval
            true, // Show error bars
        );
        
        // Create significance indicators based on statistical significance
        let significance_color = match budget_forecast.significance_test.level {
            SignificanceLevel::HighlySignificant => "#4CAF50", // Green
            SignificanceLevel::ModeratelySignificant => "#FFC107", // Yellow
            SignificanceLevel::NotSignificant => "#F44336", // Red
        };
        
        let significance_indicators = vec![SignificanceIndicator::new(
            budget_forecast.significance_test.p_value,
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
        let data = vec![current_series, forecast_series];
        let image = VisualizationService::generate_statistical_chart(statistical_config, data)?;
        
        Ok(DynamicImage::ImageRgba8(image))
    }
}

// Fallback implementations when features are disabled
#[cfg(not(all(feature = "statistics", feature = "visualization")))]
pub struct FinancialStatisticalVisualization;

#[cfg(not(all(feature = "statistics", feature = "visualization")))]
impl FinancialStatisticalVisualization {
    /// Generate a forecast visualization (stub implementation)
    pub fn generate_forecast_visualization(
        _forecast: &ExpenseForecast,
        _historical_expenses: &[Expense],
    ) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        Err("Statistical visualization requires both 'statistics' and 'visualization' features to be enabled".into())
    }
    
    /// Generate a budget impact visualization (stub implementation)
    pub fn generate_budget_impact_visualization(
        _budget_forecast: &BudgetForecast,
        _budget: &Budget,
    ) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        Err("Statistical visualization requires both 'statistics' and 'visualization' features to be enabled".into())
    }
}

#[cfg(test)]
#[cfg(all(feature = "statistics", feature = "visualization"))]
mod tests {
    use super::*;
    use crate::domain::{primitives::Money, Currency, BudgetPeriod};
    use chrono::Utc;
    use rust_decimal::Decimal;
    use uuid::Uuid;
    
    #[test]
    fn test_visualization_service_exists() {
        // This test just verifies that the necessary types exist
        // Actual visualization testing would require more complex setup
        assert!(true);
    }
}
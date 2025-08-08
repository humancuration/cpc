//! Statistical models for cause management
//!
//! This module defines the core data structures used for statistical analysis
//! of donation patterns and cause impact measurement.

#[cfg(feature = "statistics")]
use cpc_statistics_core::{
    ConfidenceInterval,
    SignificanceResult,
    SignificanceLevel,
};

#[cfg(feature = "statistics")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "statistics")]
use chrono::{DateTime, Utc};

/// Statistical trend types for donation analysis
#[cfg(feature = "statistics")]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrendType {
    /// Linear trend (constant rate of change)
    Linear,
    /// Exponential trend (compounding growth/decline)
    Exponential,
    /// Seasonal trend (repeating patterns)
    Seasonal,
    /// No clear trend
    Flat,
}

/// Forecast results for donation predictions
#[cfg(feature = "statistics")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonationForecast {
    /// Forecasted values for each period
    pub forecast_values: Vec<f64>,
    
    /// Confidence interval for the forecast
    pub confidence_interval: ConfidenceInterval,
    
    /// Significance test results for trend analysis
    pub trend_significance: SignificanceResult,
    
    /// Confidence level used for the forecast
    pub confidence_level: f64,
    
    /// Timestamp when the forecast was generated
    pub generated_at: DateTime<Utc>,
}

#[cfg(feature = "statistics")]
impl DonationForecast {
    /// Create a new donation forecast
    pub fn new(
        forecast_values: Vec<f64>,
        confidence_interval: ConfidenceInterval,
        trend_significance: SignificanceResult,
        confidence_level: f64,
    ) -> Self {
        Self {
            forecast_values,
            confidence_interval,
            trend_significance,
            confidence_level,
            generated_at: Utc::now(),
        }
    }
    
    /// Generate a plain-language explanation of the forecast
    pub fn explanation(&self) -> String {
        format!(
            "Based on historical donation data, future donations are forecasted to be {:.2} with a {:.0}% confidence interval of [{:.2}, {:.2}]. \
            The trend analysis shows {} evidence for a significant change in donation patterns (p = {:.4}). \
            This forecast was generated on {}.",
            self.forecast_values.iter().sum::<f64>() / self.forecast_values.len() as f64,
            self.confidence_level * 100.0,
            self.confidence_interval.lower,
            self.confidence_interval.upper,
            self.trend_significance.level.description(),
            self.trend_significance.p_value,
            self.generated_at.format("%Y-%m-%d")
        )
    }
    
    /// Generate a cooperative values-aligned explanation
    pub fn cooperative_explanation(&self) -> String {
        let forecast_trend = if self.forecast_values.iter().sum::<f64>() > 0.0 {
            "growth in donations"
        } else {
            "decline in donations"
        };
        
        format!(
            "Our analysis shows {} for this cause. \
            With {:.0}% confidence, we expect donations to range from {:.2} to {:.2}. \
            Statistical analysis shows {} evidence for significant donation patterns (p = {:.4}). \
            This information helps the cooperative better support causes that are making a positive impact in our community.",
            forecast_trend,
            self.confidence_level * 100.0,
            self.confidence_interval.lower,
            self.confidence_interval.upper,
            self.trend_significance.level.description(),
            self.trend_significance.p_value
        )
    }
}

/// Donation trend analysis results
#[cfg(feature = "statistics")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonationTrend {
    /// Type of trend identified
    pub trend_type: TrendType,
    
    /// Strength of the trend (0.0 to 1.0)
    pub strength: f64,
    
    /// P-value for trend significance
    pub p_value: f64,
    
    /// Number of data points used in analysis
    pub data_points: usize,
    
    /// Timestamp when the trend was analyzed
    pub analyzed_at: DateTime<Utc>,
}

#[cfg(feature = "statistics")]
impl DonationTrend {
    /// Create a new donation trend analysis
    pub fn new(
        trend_type: TrendType,
        strength: f64,
        p_value: f64,
        data_points: usize,
    ) -> Self {
        Self {
            trend_type,
            strength,
            p_value,
            data_points,
            analyzed_at: Utc::now(),
        }
    }
    
    /// Check if the trend is statistically significant
    pub fn is_significant(&self) -> bool {
        self.p_value < 0.05
    }
    
    /// Generate a plain-language explanation of the trend
    pub fn explanation(&self) -> String {
        let trend_desc = match self.trend_type {
            TrendType::Linear => "a linear trend",
            TrendType::Exponential => "an exponential trend",
            TrendType::Seasonal => "a seasonal pattern",
            TrendType::Flat => "no clear trend",
        };
        
        let significance = if self.is_significant() {
            "statistically significant"
        } else {
            "not statistically significant"
        };
        
        format!(
            "Donation analysis shows {} with a strength of {:.2} ({}). \
            This trend was identified using {} data points and is {} (p = {:.4}).",
            trend_desc,
            self.strength,
            trend_desc,
            self.data_points,
            significance,
            self.p_value
        )
    }
}

/// Fallback struct when statistics feature is disabled
#[cfg(not(feature = "statistics"))]
#[derive(Debug, Clone)]
pub struct DonationForecast;

/// Fallback struct when statistics feature is disabled
#[cfg(not(feature = "statistics"))]
#[derive(Debug, Clone)]
pub struct DonationTrend;

/// Fallback enum when statistics feature is disabled
#[cfg(not(feature = "statistics"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TrendType {
    Linear,
    Exponential,
    Seasonal,
    Flat,
}

#[cfg(test)]
#[cfg(feature = "statistics")]
mod tests {
    use super::*;
    use cpc_statistics_core::{ConfidenceMethod, SignificanceResult};
    
    #[test]
    fn test_donation_forecast_creation() {
        let ci = ConfidenceInterval::new(
            100.0,
            200.0,
            0.95,
            ConfidenceMethod::Parametric,
            50,
        );
        
        let significance = SignificanceResult::new(
            0.01,
            2.5,
            Some(49.0),
        );
        
        let forecast = DonationForecast::new(
            vec![150.0, 160.0, 170.0],
            ci,
            significance,
            0.95,
        );
        
        assert_eq!(forecast.forecast_values.len(), 3);
        assert_eq!(forecast.confidence_level, 0.95);
    }
    
    #[test]
    fn test_donation_trend_creation() {
        let trend = DonationTrend::new(
            TrendType::Linear,
            0.85,
            0.001,
            100,
        );
        
        assert_eq!(trend.trend_type, TrendType::Linear);
        assert_eq!(trend.strength, 0.85);
        assert!(trend.is_significant());
    }
}
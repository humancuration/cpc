//! Confidence interval visualization components
//!
//! This module provides visualization components for statistical confidence intervals
//! and significance indicators.

use serde::{Deserialize, Serialize};
use crate::domain::{
    chart::{ChartType, SeriesConfig},
    data::DataSeries,
};

/// Configuration for confidence interval visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceIntervalConfig {
    /// The main series to display
    pub main_series: DataSeries,
    
    /// Upper bound of confidence interval
    pub upper_bound: DataSeries,
    
    /// Lower bound of confidence interval
    pub lower_bound: DataSeries,
    
    /// Confidence level (e.g., 0.95 for 95% confidence)
    pub confidence_level: f64,
    
    /// Color for the confidence interval area
    pub interval_color: String,
    
    /// Whether to show error bars instead of shaded area
    pub show_error_bars: bool,
}

impl ConfidenceIntervalConfig {
    /// Create a new confidence interval configuration
    pub fn new(
        main_series: DataSeries,
        upper_bound: DataSeries,
        lower_bound: DataSeries,
        confidence_level: f64,
        interval_color: String,
        show_error_bars: bool,
    ) -> Self {
        Self {
            main_series,
            upper_bound,
            lower_bound,
            confidence_level,
            interval_color,
            show_error_bars,
        }
    }
}

/// Statistical significance indicator for chart elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignificanceIndicator {
    /// The p-value for this indicator
    pub p_value: f64,
    
    /// Significance level classification
    pub significance_level: SignificanceLevel,
    
    /// Position of the indicator on the chart
    pub position: (f64, f64),
    
    /// Optional label for the indicator
    pub label: Option<String>,
}

impl SignificanceIndicator {
    /// Create a new significance indicator
    pub fn new(
        p_value: f64,
        position: (f64, f64),
        label: Option<String>,
    ) -> Self {
        let significance_level = SignificanceLevel::from_p_value(p_value);
        Self {
            p_value,
            significance_level,
            position,
            label,
        }
    }
}

/// Statistical significance levels with color coding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignificanceLevel {
    /// Highly significant (p < 0.01) - Green
    HighlySignificant,
    /// Moderately significant (0.01 ≤ p < 0.05) - Yellow
    ModeratelySignificant,
    /// Not significant (p ≥ 0.05) - Red
    NotSignificant,
}

impl SignificanceLevel {
    /// Create a significance level from a p-value
    pub fn from_p_value(p_value: f64) -> Self {
        if p_value < 0.01 {
            SignificanceLevel::HighlySignificant
        } else if p_value < 0.05 {
            SignificanceLevel::ModeratelySignificant
        } else {
            SignificanceLevel::NotSignificant
        }
    }
    
    /// Get the color associated with this significance level
    pub fn color(&self) -> &'static str {
        match self {
            SignificanceLevel::HighlySignificant => "#4CAF50", // Green
            SignificanceLevel::ModeratelySignificant => "#FFC107", // Yellow
            SignificanceLevel::NotSignificant => "#F44336", // Red
        }
    }
    
    /// Get a plain-language description of this significance level
    pub fn description(&self) -> &'static str {
        match self {
            SignificanceLevel::HighlySignificant => "Highly Significant",
            SignificanceLevel::ModeratelySignificant => "Moderately Significant",
            SignificanceLevel::NotSignificant => "Not Significant",
        }
    }
}

/// Extended chart configuration with statistical components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalChartConfig {
    /// Base chart configuration
    pub base_config: crate::domain::chart::ChartConfig,
    
    /// Confidence interval configurations
    pub confidence_intervals: Vec<ConfidenceIntervalConfig>,
    
    /// Significance indicators
    pub significance_indicators: Vec<SignificanceIndicator>,
    
    /// Whether to show statistical explanations
    pub show_explanations: bool,
}

impl StatisticalChartConfig {
    /// Create a new statistical chart configuration
    pub fn new(
        base_config: crate::domain::chart::ChartConfig,
        confidence_intervals: Vec<ConfidenceIntervalConfig>,
        significance_indicators: Vec<SignificanceIndicator>,
        show_explanations: bool,
    ) -> Self {
        Self {
            base_config,
            confidence_intervals,
            significance_indicators,
            show_explanations,
        }
    }
}
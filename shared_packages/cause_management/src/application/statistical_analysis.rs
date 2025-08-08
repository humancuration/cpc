//! Statistical analysis for cause management
//!
//! This module provides statistical analysis capabilities for donation patterns
//! and cause forecasting, mirroring the functionality in the finance app but
//! tailored for cause management needs.

#[cfg(feature = "statistics")]
use cpc_statistics_core::{
    ConfidenceCalculator,
    SignificanceTester,
    StatisticalError,
    ConfidenceInterval,
    SignificanceResult,
};

#[cfg(feature = "statistics")]
use crate::domain::{
    statistical_models::{DonationForecast, DonationTrend, TrendType},
    models::Cause,
};
#[cfg(feature = "statistics")]
use crate::models::CauseError;
#[cfg(feature = "statistics")]
use chrono::{DateTime, Utc};
#[cfg(feature = "statistics")]
use rust_decimal::Decimal;
#[cfg(feature = "statistics")]
use std::collections::HashMap;

/// Statistical analysis service for cause management
#[cfg(feature = "statistics")]
pub struct StatisticalAnalysisService;

#[cfg(feature = "statistics")]
impl StatisticalAnalysisService {
    /// Forecast future donations for a cause based on historical data with confidence intervals
    pub fn forecast_donations(
        donations: &[Decimal],
        forecast_periods: usize,
        confidence_level: f64,
    ) -> Result<DonationForecast, CauseError> {
        if donations.is_empty() {
            return Err(CauseError::InvalidInput("No donation data provided".to_string()));
        }
        
        if forecast_periods == 0 {
            return Err(CauseError::InvalidInput("Forecast periods must be greater than 0".to_string()));
        }
        
        if confidence_level <= 0.0 || confidence_level >= 1.0 {
            return Err(CauseError::InvalidInput("Confidence level must be between 0 and 1".to_string()));
        }
        
        // Convert Decimal donations to f64 for statistical analysis
        let amounts: Vec<f64> = donations
            .iter()
            .map(|d| d.to_f64().unwrap_or(0.0))
            .collect();
        
        // Calculate basic statistics
        let mean = amounts.iter().sum::<f64>() / amounts.len() as f64;
        let std_dev = if amounts.len() > 1 {
            amounts.iter().map(|&x| (x - mean).powi(2)).sum::<f64>().sqrt() / (amounts.len() - 1) as f64
        } else {
            0.0
        };
        
        // Generate forecast periods
        let mut forecast_values = Vec::with_capacity(forecast_periods);
        let mut rng = rand::thread_rng();
        
        for i in 0..forecast_periods {
            // Simple random walk forecast with trend
            let trend = mean * 0.01 * i as f64; // 1% growth per period
            let random_component = rand_distr::Normal::new(0.0, std_dev)
                .map_err(|_| CauseError::InvalidInput("Invalid distribution parameters".to_string()))?
                .sample(&mut rng);
            
            let forecast_value = mean + trend + random_component;
            forecast_values.push(forecast_value.max(0.0)); // Ensure non-negative values
        }
        
        // Calculate confidence intervals
        let ci = ConfidenceCalculator::parametric_interval(&amounts, confidence_level)
            .map_err(|e| CauseError::General(format!("Failed to calculate confidence interval: {}", e)))?;
        
        // Perform significance test for trend
        let trend_test = SignificanceTester::one_sample_t_test(&amounts, 0.0)
            .map_err(|e| CauseError::General(format!("Failed to perform significance test: {}", e)))?;
        
        Ok(DonationForecast::new(
            forecast_values,
            ci,
            trend_test,
            confidence_level,
        ))
    }
    
    /// Analyze donation trends for a cause
    pub fn analyze_donation_trends(
        donations: &[Decimal],
    ) -> Result<Vec<DonationTrend>, CauseError> {
        if donations.is_empty() {
            return Err(CauseError::InvalidInput("No donation data provided".to_string()));
        }
        
        // Convert Decimal donations to f64 for statistical analysis
        let amounts: Vec<f64> = donations
            .iter()
            .map(|d| d.to_f64().unwrap_or(0.0))
            .collect();
        
        // For simplicity, we'll identify one primary trend
        // In a real implementation, this could identify multiple trends
        
        // Calculate basic trend using linear regression approach
        let n = amounts.len() as f64;
        let indices: Vec<f64> = (0..amounts.len()).map(|i| i as f64).collect();
        
        // Calculate correlation between time and donations
        let mean_x = indices.iter().sum::<f64> / n;
        let mean_y = amounts.iter().sum::<f64> / n;
        
        let numerator: f64 = indices.iter().zip(amounts.iter())
            .map(|(&x, &y)| (x - mean_x) * (y - mean_y))
            .sum();
        
        let denominator_x: f64 = indices.iter()
            .map(|&x| (x - mean_x).powi(2))
            .sum();
        
        let denominator_y: f64 = amounts.iter()
            .map(|&y| (y - mean_y).powi(2))
            .sum();
        
        let correlation = if denominator_x > 0.0 && denominator_y > 0.0 {
            numerator / (denominator_x * denominator_y).sqrt()
        } else {
            0.0
        };
        
        // Determine trend type based on correlation
        let trend_type = if correlation > 0.7 {
            TrendType::Linear
        } else if correlation > 0.3 {
            TrendType::Exponential
        } else if correlation < -0.3 {
            TrendType::Seasonal // For simplicity, using seasonal for negative trends
        } else {
            TrendType::Flat
        };
        
        // Calculate trend strength
        let strength = correlation.abs();
        
        // Perform significance test
        let significance_test = SignificanceTester::correlation_test(&indices, &amounts)
            .map_err(|e| CauseError::General(format!("Failed to perform significance test: {}", e)))?;
        
        // Create trend analysis
        let trend = DonationTrend::new(
            trend_type,
            strength,
            significance_test.p_value,
            amounts.len(),
        );
        
        Ok(vec![trend])
    }
    
    /// Calculate donation velocity (rate of donations over time)
    pub fn calculate_donation_velocity(
        donations: &[Decimal],
        time_periods: &[DateTime<Utc>],
    ) -> Result<f64, CauseError> {
        if donations.is_empty() {
            return Err(CauseError::InvalidInput("No donation data provided".to_string()));
        }
        
        if donations.len() != time_periods.len() {
            return Err(CauseError::InvalidInput("Donations and time periods must have the same length".to_string()));
        }
        
        if donations.len() < 2 {
            return Err(CauseError::InvalidInput("At least 2 data points required for velocity calculation".to_string()));
        }
        
        // Convert Decimal donations to f64
        let amounts: Vec<f64> = donations
            .iter()
            .map(|d| d.to_f64().unwrap_or(0.0))
            .collect();
        
        // Calculate total donations
        let total_donations: f64 = amounts.iter().sum();
        
        // Calculate time span in days
        let min_time = time_periods.iter().min().unwrap();
        let max_time = time_periods.iter().max().unwrap();
        let time_span_days = (*max_time - *min_time).num_days() as f64;
        
        // Calculate velocity (donations per day)
        if time_span_days > 0.0 {
            Ok(total_donations / time_span_days)
        } else {
            Ok(total_donations)
        }
    }
    
    /// Analyze donation frequency patterns
    pub fn analyze_donation_frequency(
        time_periods: &[DateTime<Utc>],
    ) -> Result<HashMap<String, f64>, CauseError> {
        if time_periods.is_empty() {
            return Err(CauseError::InvalidInput("No time data provided".to_string()));
        }
        
        let mut frequency_map = HashMap::new();
        
        // Count donations by day of week
        for time in time_periods {
            let day = time.weekday().to_string();
            *frequency_map.entry(day).or_insert(0.0) += 1.0;
        }
        
        // Count donations by month
        for time in time_periods {
            let month = time.month().to_string();
            *frequency_map.entry(format!("month_{}", month)).or_insert(0.0) += 1.0;
        }
        
        // Normalize frequencies
        let total: f64 = frequency_map.values().sum();
        if total > 0.0 {
            for value in frequency_map.values_mut() {
                *value /= total;
            }
        }
        
        Ok(frequency_map)
    }
}

// Fallback implementation when statistics feature is disabled
#[cfg(not(feature = "statistics"))]
pub struct StatisticalAnalysisService;

#[cfg(not(feature = "statistics"))]
impl StatisticalAnalysisService {
    /// Forecast future donations (stub implementation)
    pub fn forecast_donations(
        _donations: &[Decimal],
        _forecast_periods: usize,
        _confidence_level: f64,
    ) -> Result<DonationForecast, CauseError> {
        Err(CauseError::General("Statistical analysis requires the 'statistics' feature to be enabled".to_string()))
    }
    
    /// Analyze donation trends (stub implementation)
    pub fn analyze_donation_trends(
        _donations: &[Decimal],
    ) -> Result<Vec<DonationTrend>, CauseError> {
        Err(CauseError::General("Statistical analysis requires the 'statistics' feature to be enabled".to_string()))
    }
    
    /// Calculate donation velocity (stub implementation)
    pub fn calculate_donation_velocity(
        _donations: &[Decimal],
        _time_periods: &[DateTime<Utc>],
    ) -> Result<f64, CauseError> {
        Err(CauseError::General("Statistical analysis requires the 'statistics' feature to be enabled".to_string()))
    }
    
    /// Analyze donation frequency patterns (stub implementation)
    pub fn analyze_donation_frequency(
        _time_periods: &[DateTime<Utc>],
    ) -> Result<HashMap<String, f64>, CauseError> {
        Err(CauseError::General("Statistical analysis requires the 'statistics' feature to be enabled".to_string()))
    }
}

#[cfg(test)]
#[cfg(feature = "statistics")]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use chrono::Duration;
    
    #[test]
    fn test_donation_forecast() {
        let donations = vec![
            dec!(100.0),
            dec!(120.0),
            dec!(90.0),
            dec!(110.0),
            dec!(130.0),
        ];
        
        let forecast = StatisticalAnalysisService::forecast_donations(&donations, 5, 0.95);
        assert!(forecast.is_ok());
        
        let forecast = forecast.unwrap();
        assert_eq!(forecast.forecast_values.len(), 5);
        assert_eq!(forecast.confidence_level, 0.95);
    }
    
    #[test]
    fn test_donation_trend_analysis() {
        let donations = vec![
            dec!(100.0),
            dec!(110.0),
            dec!(120.0),
            dec!(130.0),
            dec!(140.0),
        ];
        
        let trends = StatisticalAnalysisService::analyze_donation_trends(&donations);
        assert!(trends.is_ok());
        
        let trends = trends.unwrap();
        assert!(!trends.is_empty());
    }
    
    #[test]
    fn test_donation_velocity() {
        let donations = vec![
            dec!(100.0),
            dec!(150.0),
            dec!(200.0),
        ];
        
        let now = Utc::now();
        let time_periods = vec![
            now - Duration::days(2),
            now - Duration::days(1),
            now,
        ];
        
        let velocity = StatisticalAnalysisService::calculate_donation_velocity(&donations, &time_periods);
        assert!(velocity.is_ok());
    }
    
    #[test]
    fn test_empty_donations_error() {
        let donations = vec![];
        
        let forecast = StatisticalAnalysisService::forecast_donations(&donations, 5, 0.95);
        assert!(forecast.is_err());
        
        let trends = StatisticalAnalysisService::analyze_donation_trends(&donations);
        assert!(trends.is_err());
    }
}
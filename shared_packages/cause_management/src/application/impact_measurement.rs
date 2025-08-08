//! Impact measurement for cause management
//!
//! This module provides tools for measuring the cooperative impact of causes
//! using Bayesian statistical methods aligned with cooperative values.

#[cfg(feature = "statistics")]
use cpc_statistics_core::{
    ConfidenceCalculator,
    SignificanceTester,
    StatisticalError,
    ConfidenceInterval,
    SignificanceResult,
    SignificanceLevel,
};

#[cfg(feature = "statistics")]
use crate::domain::{
    impact_models::{ImpactAnalysis, ImpactMetric, ValidationResult},
    statistical_models::DonationTrend,
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

/// Impact measurement service for cause management
#[cfg(feature = "statistics")]
pub struct ImpactMeasurementService;

#[cfg(feature = "statistics")]
impl ImpactMeasurementService {
    /// Measure the impact of a cause based on donations and outcomes
    pub fn measure_impact(
        cause: &Cause,
        donations: &[Decimal],
        outcomes: &[ImpactOutcome],
        impact_type: ImpactMetric,
    ) -> Result<ImpactAnalysis, CauseError> {
        if donations.is_empty() {
            return Err(CauseError::InvalidInput("No donation data provided".to_string()));
        }
        
        if outcomes.is_empty() {
            return Err(CauseError::InvalidInput("No outcome data provided".to_string()));
        }
        
        // Convert Decimal donations to f64 for statistical analysis
        let donation_amounts: Vec<f64> = donations
            .iter()
            .map(|d| d.to_f64().unwrap_or(0.0))
            .collect();
        
        // Calculate total donations
        let total_donations: f64 = donation_amounts.iter().sum();
        
        // Extract outcome values
        let outcome_values: Vec<f64> = outcomes
            .iter()
            .map(|o| o.value)
            .collect();
        
        // Calculate impact correlation between donations and outcomes
        let correlation_result = SignificanceTester::correlation_test(&donation_amounts, &outcome_values)
            .map_err(|e| CauseError::General(format!("Failed to calculate impact correlation: {}", e)))?;
        
        // Calculate impact score based on correlation and total donations
        let impact_score = (correlation_result.statistic.abs() * 10.0).min(10.0);
        
        // Calculate confidence interval using Bayesian approach
        let ci = ConfidenceCalculator::bayesian_interval(&outcome_values, 0.95, impact_score, 1.0)
            .map_err(|e| CauseError::General(format!("Failed to calculate confidence interval: {}", e)))?;
        
        // Determine evidence strength
        let evidence_strength = correlation_result.level.clone();
        
        Ok(ImpactAnalysis::new(
            impact_score,
            evidence_strength,
            correlation_result.p_value,
            ci,
            impact_type,
        ))
    }
    
    /// Validate impact metrics for consistency and reasonableness
    pub fn validate_impact_metrics(
        metrics: &[ImpactMetric],
        values: &[f64],
    ) -> Result<ValidationResult, CauseError> {
        if metrics.len() != values.len() {
            return Err(CauseError::InvalidInput("Metrics and values must have the same length".to_string()));
        }
        
        let mut errors = Vec::new();
        
        // Validate value ranges
        for (i, &value) in values.iter().enumerate() {
            match metrics[i] {
                ImpactMetric::LivesImpacted => {
                    if value < 0.0 {
                        errors.push(format!("Lives impacted cannot be negative (index {})", i));
                    }
                },
                ImpactMetric::EnvironmentalBenefit => {
                    if value < -10.0 || value > 10.0 {
                        errors.push(format!("Environmental benefit should be between -10 and 10 (index {})", i));
                    }
                },
                ImpactMetric::CommunityEngagement => {
                    if value < 0.0 || value > 1.0 {
                        errors.push(format!("Community engagement should be between 0 and 1 (index {})", i));
                    }
                },
                ImpactMetric::EconomicImpact => {
                    // Economic impact can be negative (costs) or positive (benefits)
                    if value < -1000000.0 || value > 1000000.0 {
                        errors.push(format!("Economic impact value out of reasonable range (index {})", i));
                    }
                },
                ImpactMetric::EducationalOutcomes => {
                    if value < 0.0 || value > 1.0 {
                        errors.push(format!("Educational outcomes should be between 0 and 1 (index {})", i));
                    }
                },
                ImpactMetric::HealthOutcomes => {
                    if value < 0.0 || value > 1.0 {
                        errors.push(format!("Health outcomes should be between 0 and 1 (index {})", i));
                    }
                },
            }
        }
        
        if errors.is_empty() {
            Ok(ValidationResult::success())
        } else {
            Ok(ValidationResult::failure(errors))
        }
    }
    
    /// Calculate composite impact score from multiple metrics
    pub fn calculate_composite_impact(
        analyses: &[ImpactAnalysis],
    ) -> Result<f64, CauseError> {
        if analyses.is_empty() {
            return Err(CauseError::InvalidInput("No impact analyses provided".to_string()));
        }
        
        // Weighted average based on evidence strength
        let mut total_weighted_score = 0.0;
        let mut total_weight = 0.0;
        
        for analysis in analyses {
            // Weight based on evidence strength (p-value)
            let weight = match analysis.evidence_strength {
                SignificanceLevel::HighlySignificant => 3.0,
                SignificanceLevel::ModeratelySignificant => 2.0,
                SignificanceLevel::NotSignificant => 1.0,
            };
            
            total_weighted_score += analysis.impact_score * weight;
            total_weight += weight;
        }
        
        if total_weight > 0.0 {
            Ok(total_weighted_score / total_weight)
        } else {
            Ok(0.0)
        }
    }
    
    /// Generate impact report with trends and forecasts
    pub fn generate_impact_report(
        cause: &Cause,
        donations: &[Decimal],
        time_periods: &[DateTime<Utc>],
        outcomes: &[ImpactOutcome],
    ) -> Result<ImpactReport, CauseError> {
        // Calculate donation trends
        let donation_trends = super::statistical_analysis::StatisticalAnalysisService::analyze_donation_trends(donations)?;
        
        // Calculate donation velocity
        let donation_velocity = super::statistical_analysis::StatisticalAnalysisService::calculate_donation_velocity(donations, time_periods)?;
        
        // Create impact report
        Ok(ImpactReport {
            cause_id: cause.id,
            cause_name: cause.name.clone(),
            total_donations: cause.total_donations,
            donation_trends,
            donation_velocity,
            report_generated_at: Utc::now(),
        })
    }
}

/// Outcome data for impact measurement
#[cfg(feature = "statistics")]
#[derive(Debug, Clone)]
pub struct ImpactOutcome {
    /// The outcome value
    pub value: f64,
    /// Timestamp when the outcome was measured
    pub measured_at: DateTime<Utc>,
    /// Description of the outcome
    pub description: String,
}

#[cfg(feature = "statistics")]
impl ImpactOutcome {
    /// Create a new impact outcome
    pub fn new(value: f64, measured_at: DateTime<Utc>, description: String) -> Self {
        Self {
            value,
            measured_at,
            description,
        }
    }
}

/// Impact report combining multiple analyses
#[cfg(feature = "statistics")]
#[derive(Debug, Clone)]
pub struct ImpactReport {
    /// ID of the cause
    pub cause_id: uuid::Uuid,
    /// Name of the cause
    pub cause_name: String,
    /// Total donations received
    pub total_donations: Decimal,
    /// Donation trends analysis
    pub donation_trends: Vec<DonationTrend>,
    /// Donation velocity (donations per time unit)
    pub donation_velocity: f64,
    /// When the report was generated
    pub report_generated_at: DateTime<Utc>,
}

// Fallback implementation when statistics feature is disabled
#[cfg(not(feature = "statistics"))]
pub struct ImpactMeasurementService;

#[cfg(not(feature = "statistics"))]
impl ImpactMeasurementService {
    /// Measure the impact of a cause (stub implementation)
    pub fn measure_impact(
        _cause: &Cause,
        _donations: &[Decimal],
        _outcomes: &[ImpactOutcome],
        _impact_type: ImpactMetric,
    ) -> Result<ImpactAnalysis, CauseError> {
        Err(CauseError::General("Impact measurement requires the 'statistics' feature to be enabled".to_string()))
    }
    
    /// Validate impact metrics (stub implementation)
    pub fn validate_impact_metrics(
        _metrics: &[ImpactMetric],
        _values: &[f64],
    ) -> Result<ValidationResult, CauseError> {
        Err(CauseError::General("Impact measurement requires the 'statistics' feature to be enabled".to_string()))
    }
    
    /// Calculate composite impact score (stub implementation)
    pub fn calculate_composite_impact(
        _analyses: &[ImpactAnalysis],
    ) -> Result<f64, CauseError> {
        Err(CauseError::General("Impact measurement requires the 'statistics' feature to be enabled".to_string()))
    }
    
    /// Generate impact report (stub implementation)
    pub fn generate_impact_report(
        _cause: &Cause,
        _donations: &[Decimal],
        _time_periods: &[DateTime<Utc>],
        _outcomes: &[ImpactOutcome],
    ) -> Result<ImpactReport, CauseError> {
        Err(CauseError::General("Impact measurement requires the 'statistics' feature to be enabled".to_string()))
    }
}

#[cfg(not(feature = "statistics"))]
#[derive(Debug, Clone)]
pub struct ImpactOutcome {
    pub value: f64,
    pub measured_at: DateTime<Utc>,
    pub description: String,
}

#[cfg(not(feature = "statistics"))]
#[derive(Debug, Clone)]
pub struct ImpactReport {
    pub cause_id: uuid::Uuid,
    pub cause_name: String,
    pub total_donations: Decimal,
    pub donation_trends: Vec<DonationTrend>,
    pub donation_velocity: f64,
    pub report_generated_at: DateTime<Utc>,
}

#[cfg(test)]
#[cfg(feature = "statistics")]
mod tests {
    use super::*;
    use crate::domain::models::Cause;
    use rust_decimal_macros::dec;
    use chrono::Duration;
    
    #[test]
    fn test_impact_measurement() {
        let cause = Cause::new(
            "Test Cause".to_string(),
            "Test Description".to_string(),
            None,
        );
        
        let donations = vec![
            dec!(100.0),
            dec!(150.0),
            dec!(200.0),
        ];
        
        let outcomes = vec![
            ImpactOutcome::new(5.0, Utc::now(), "Outcome 1".to_string()),
            ImpactOutcome::new(7.0, Utc::now(), "Outcome 2".to_string()),
            ImpactOutcome::new(9.0, Utc::now(), "Outcome 3".to_string()),
        ];
        
        let analysis = ImpactMeasurementService::measure_impact(
            &cause,
            &donations,
            &outcomes,
            ImpactMetric::LivesImpacted,
        );
        
        assert!(analysis.is_ok());
    }
    
    #[test]
    fn test_impact_validation() {
        let metrics = vec![
            ImpactMetric::LivesImpacted,
            ImpactMetric::CommunityEngagement,
        ];
        
        let values = vec![10.0, 0.8];
        
        let validation = ImpactMeasurementService::validate_impact_metrics(&metrics, &values);
        assert!(validation.is_ok());
        
        let result = validation.unwrap();
        assert!(result.is_valid);
    }
    
    #[test]
    fn test_invalid_impact_validation() {
        let metrics = vec![
            ImpactMetric::CommunityEngagement,
        ];
        
        let values = vec![1.5]; // Invalid value (> 1.0)
        
        let validation = ImpactMeasurementService::validate_impact_metrics(&metrics, &values);
        assert!(validation.is_ok());
        
        let result = validation.unwrap();
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }
}
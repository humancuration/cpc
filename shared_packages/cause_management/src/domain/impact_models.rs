//! Impact models for cause management
//!
//! This module defines the core data structures used for measuring
//! the impact of causes and donations within the cooperative.

#[cfg(feature = "statistics")]
use cpc_statistics_core::{
    ConfidenceInterval,
    SignificanceLevel,
    SignificanceResult,
};

#[cfg(feature = "statistics")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "statistics")]
use chrono::{DateTime, Utc};

/// Types of impact metrics for causes
#[cfg(feature = "statistics")]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImpactMetric {
    /// Number of lives directly impacted
    LivesImpacted,
    /// Environmental benefit measurement
    EnvironmentalBenefit,
    /// Community engagement level
    CommunityEngagement,
    /// Economic impact on local communities
    EconomicImpact,
    /// Educational outcomes improvement
    EducationalOutcomes,
    /// Health outcomes improvement
    HealthOutcomes,
}

/// Impact analysis results for causes
#[cfg(feature = "statistics")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAnalysis {
    /// Overall impact score (0.0 to 10.0)
    pub impact_score: f64,
    
    /// Strength of evidence for the impact
    pub evidence_strength: SignificanceLevel,
    
    /// P-value for statistical significance
    pub p_value: f64,
    
    /// Confidence interval for the impact score
    pub confidence_interval: ConfidenceInterval,
    
    /// Type of impact being measured
    pub impact_type: ImpactMetric,
    
    /// Timestamp when the analysis was performed
    pub analyzed_at: DateTime<Utc>,
}

#[cfg(feature = "statistics")]
impl ImpactAnalysis {
    /// Create a new impact analysis
    pub fn new(
        impact_score: f64,
        evidence_strength: SignificanceLevel,
        p_value: f64,
        confidence_interval: ConfidenceInterval,
        impact_type: ImpactMetric,
    ) -> Self {
        Self {
            impact_score,
            evidence_strength,
            p_value,
            confidence_interval,
            impact_type,
            analyzed_at: Utc::now(),
        }
    }
    
    /// Generate a plain-language explanation of the impact
    pub fn explanation(&self) -> String {
        let impact_desc = match self.impact_type {
            ImpactMetric::LivesImpacted => "lives impacted",
            ImpactMetric::EnvironmentalBenefit => "environmental benefit",
            ImpactMetric::CommunityEngagement => "community engagement",
            ImpactMetric::EconomicImpact => "economic impact",
            ImpactMetric::EducationalOutcomes => "educational outcomes",
            ImpactMetric::HealthOutcomes => "health outcomes",
        };
        
        format!(
            "This cause has an impact score of {:.1}/10 for {} with a {:.0}% confidence interval of [{:.2}, {:.2}]. \
            The evidence strength is {} with a p-value of {:.4}. \
            This analysis was performed on {}.",
            self.impact_score,
            impact_desc,
            self.confidence_interval.confidence_level * 100.0,
            self.confidence_interval.lower,
            self.confidence_interval.upper,
            self.evidence_strength.description(),
            self.p_value,
            self.analyzed_at.format("%Y-%m-%d")
        )
    }
    
    /// Generate a cooperative values-aligned explanation
    pub fn cooperative_explanation(&self) -> String {
        let impact_desc = match self.impact_type {
            ImpactMetric::LivesImpacted => "lives impacted",
            ImpactMetric::EnvironmentalBenefit => "environmental benefit",
            ImpactMetric::CommunityEngagement => "community engagement",
            ImpactMetric::EconomicImpact => "economic impact",
            ImpactMetric::EducationalOutcomes => "educational outcomes",
            ImpactMetric::HealthOutcomes => "health outcomes",
        };
        
        let impact_strength = if self.impact_score > 8.0 {
            "exceptional"
        } else if self.impact_score > 6.0 {
            "strong"
        } else if self.impact_score > 4.0 {
            "moderate"
        } else {
            "limited"
        };
        
        format!(
            "This cause demonstrates {} {} in our community. \
            With an impact score of {:.1}/10 and {} evidence (p = {:.4}), \
            your donations are making a meaningful difference. \
            The cooperative values of mutual aid and community support are clearly reflected in these results. \
            This analysis shows that {} is effectively contributing to positive change.",
            impact_strength,
            impact_desc,
            self.impact_score,
            self.evidence_strength.description(),
            self.p_value,
            impact_desc
        )
    }
}

/// Validation result for impact metrics
#[cfg(feature = "statistics")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether the validation passed
    pub is_valid: bool,
    
    /// List of validation errors, if any
    pub errors: Vec<String>,
    
    /// Timestamp when validation was performed
    pub validated_at: DateTime<Utc>,
}

#[cfg(feature = "statistics")]
impl ValidationResult {
    /// Create a new validation result
    pub fn new(is_valid: bool, errors: Vec<String>) -> Self {
        Self {
            is_valid,
            errors,
            validated_at: Utc::now(),
        }
    }
    
    /// Create a successful validation result
    pub fn success() -> Self {
        Self::new(true, vec![])
    }
    
    /// Create a failed validation result
    pub fn failure(errors: Vec<String>) -> Self {
        Self::new(false, errors)
    }
}

/// Fallback struct when statistics feature is disabled
#[cfg(not(feature = "statistics"))]
#[derive(Debug, Clone)]
pub struct ImpactAnalysis;

/// Fallback enum when statistics feature is disabled
#[cfg(not(feature = "statistics"))]
#[derive(Debug, Clone, PartialEq)]
pub enum ImpactMetric {
    LivesImpacted,
    EnvironmentalBenefit,
    CommunityEngagement,
    EconomicImpact,
    EducationalOutcomes,
    HealthOutcomes,
}

/// Fallback struct when statistics feature is disabled
#[cfg(not(feature = "statistics"))]
#[derive(Debug, Clone)]
pub struct ValidationResult;

#[cfg(test)]
#[cfg(feature = "statistics")]
mod tests {
    use super::*;
    use cpc_statistics_core::{ConfidenceMethod, SignificanceResult};
    
    #[test]
    fn test_impact_analysis_creation() {
        let ci = ConfidenceInterval::new(
            7.5,
            9.2,
            0.95,
            ConfidenceMethod::Parametric,
            75,
        );
        
        let analysis = ImpactAnalysis::new(
            8.3,
            SignificanceLevel::HighlySignificant,
            0.001,
            ci,
            ImpactMetric::LivesImpacted,
        );
        
        assert_eq!(analysis.impact_score, 8.3);
        assert_eq!(analysis.impact_type, ImpactMetric::LivesImpacted);
        assert_eq!(analysis.evidence_strength, SignificanceLevel::HighlySignificant);
    }
    
    #[test]
    fn test_validation_result() {
        let success = ValidationResult::success();
        assert!(success.is_valid);
        assert!(success.errors.is_empty());
        
        let failure = ValidationResult::failure(vec!["Invalid metric".to_string()]);
        assert!(!failure.is_valid);
        assert_eq!(failure.errors.len(), 1);
    }
}
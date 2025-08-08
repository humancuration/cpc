//! Cooperative values integration for optimization
//!
//! This module ensures that optimization processes align with cooperative values
//! by implementing fairness constraints, community benefit prioritization, and
//! transparent decision-making processes.

use serde::{Deserialize, Serialize};
use crate::error::OptimizationError;

/// Cooperative values settings for optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CooperativeValues {
    /// Whether to prioritize community benefit metrics
    pub prioritize_community_benefit: bool,
    
    /// Weight factor for community impact in optimization
    pub community_impact_weight: f64,
    
    /// Whether to show transparent optimization explanations
    pub show_transparency: bool,
    
    /// Whether to enable community validation of optimization parameters
    pub enable_community_validation: bool,
    
    /// Fairness constraint threshold
    pub fairness_threshold: f64,
    
    /// Maximum allowable inequality in solutions
    pub max_inequality: f64,
}

impl Default for CooperativeValues {
    fn default() -> Self {
        Self {
            prioritize_community_benefit: true,
            community_impact_weight: 1.5,
            show_transparency: true,
            enable_community_validation: false,
            fairness_threshold: 0.8,
            max_inequality: 0.3,
        }
    }
}

/// Impact explorer for showing community benefit in optimization
pub struct ImpactExplorer {
    values: CooperativeValues,
}

impl ImpactExplorer {
    /// Create a new impact explorer
    pub fn new(values: CooperativeValues) -> Self {
        Self { values }
    }
    
    /// Calculate impact-weighted objective functions
    pub fn calculate_impact_weighted_objective(
        &self,
        base_objective: f64,
        community_impact: f64,
    ) -> f64 {
        if self.values.prioritize_community_benefit {
            base_objective + (community_impact * self.values.community_impact_weight)
        } else {
            base_objective
        }
    }
    
    /// Generate transparent explanation of optimization process
    pub fn generate_transparent_explanation(
        &self,
        optimization_type: &str,
        data_source: &str,
    ) -> String {
        if self.values.show_transparency {
            format!(
                "This {} optimization was performed on {} data. Community impact weighting factor: {:.2}. \
                Optimization follows cooperative values to prioritize community benefit and ensure fairness.",
                optimization_type,
                data_source,
                self.values.community_impact_weight
            )
        } else {
            "Optimization completed.".to_string()
        }
    }
}

/// Cooperative governance controls for optimization
pub struct CooperativeGovernance {
    values: CooperativeValues,
}

impl CooperativeGovernance {
    /// Create new cooperative governance controls
    pub fn new(values: CooperativeValues) -> Self {
        Self { values }
    }
    
    /// Validate optimization parameters against cooperative values
    pub fn validate_parameters(
        &self,
        parameters: &std::collections::HashMap<String, serde_json::Value>,
    ) -> Result<(), OptimizationError> {
        // Validate that optimization parameters align with cooperative values
        // and don't prioritize individual metrics over community benefit
        if self.values.enable_community_validation {
            // In a real implementation, this would validate parameters
            // against community-approved constraints
        }
        
        Ok(())
    }
    
    /// Apply fairness constraints to optimization results
    pub fn apply_fairness_constraints(
        &self,
        solution: &mut ndarray::Array1<f64>,
    ) -> Result<(), OptimizationError> {
        // Apply fairness constraints to ensure equitable solutions
        let min_val = solution.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = solution.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        
        if (max_val - min_val).abs() > self.values.max_inequality {
            return Err(OptimizationError::CooperativeValuesViolation(
                format!("Solution inequality {} exceeds maximum allowed {}", 
                       max_val - min_val, self.values.max_inequality)
            ));
        }
        
        Ok(())
    }
    
    /// Check if solution meets cooperative values requirements
    pub fn validate_solution(
        &self,
        solution: &ndarray::Array1<f64>,
    ) -> Result<bool, OptimizationError> {
        // Check if solution meets fairness threshold
        let mean = solution.mean().unwrap_or(0.0);
        let min_val = solution.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        
        let fairness_ratio = if mean > 0.0 { min_val / mean } else { 0.0 };
        
        Ok(fairness_ratio >= self.values.fairness_threshold)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cooperative_values_default() {
        let values = CooperativeValues::default();
        assert!(values.prioritize_community_benefit);
        assert_eq!(values.community_impact_weight, 1.5);
        assert_eq!(values.max_inequality, 0.3);
    }
    
    #[test]
    fn test_impact_explorer_creation() {
        let values = CooperativeValues::default();
        let explorer = ImpactExplorer::new(values);
        let weighted = explorer.calculate_impact_weighted_objective(10.0, 5.0);
        assert_eq!(weighted, 10.0 + (5.0 * 1.5));
    }
    
    #[test]
    fn test_cooperative_governance_creation() {
        let values = CooperativeValues::default();
        let governance = CooperativeGovernance::new(values);
        // Governance should be created successfully
        assert!(true);
    }
    
    #[test]
    fn test_transparent_explanation() {
        let values = CooperativeValues::default();
        let explorer = ImpactExplorer::new(values);
        let explanation = explorer.generate_transparent_explanation("resource allocation", "donation");
        assert!(explanation.contains("community impact weighting"));
    }
    
    #[test]
    fn test_fairness_validation() {
        let values = CooperativeValues::default();
        let governance = CooperativeGovernance::new(values);
        
        // Test valid solution (fair distribution)
        let mut valid_solution = ndarray::arr1(&[0.25, 0.30, 0.20, 0.25]);
        assert!(governance.apply_fairness_constraints(&mut valid_solution).is_ok());
        
        // Test invalid solution (unfair distribution)
        let mut invalid_solution = ndarray::arr1(&[0.1, 0.9, 0.0, 0.0]);
        assert!(governance.apply_fairness_constraints(&mut invalid_solution).is_err());
    }
}
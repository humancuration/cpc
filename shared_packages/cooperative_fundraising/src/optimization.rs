//! Resource allocation optimization for cooperative fundraising
//!
//! This module provides optimization capabilities for distributing resources
//! across causes and projects to maximize community impact.

use optimization_core::problem::ResourceAllocationObjective;
use optimization_core::engine::OptimizationEngine;
use optimization_core::error::OptimizationError;
use ndarray::Array1;

/// Resource allocation optimizer
pub struct ResourceAllocationOptimizer {
    engine: OptimizationEngine,
}

impl ResourceAllocationOptimizer {
    /// Create a new resource allocation optimizer
    pub fn new() -> Self {
        Self {
            engine: OptimizationEngine::new(),
        }
    }
    
    /// Optimize resource distribution across causes
    pub fn optimize_resource_distribution(
        &mut self,
        demand: Array1<f64>, // resource demand for each cause
        impact: Array1<f64>, // expected impact of each cause
        total_resources: f64, // total available resources
        community_votes: Array1<f64>, // community voting results
    ) -> Result<Array1<f64>, OptimizationError> {
        // Create optimization problem
        let problem = self.engine.create_resource_allocation_problem(
            demand,
            impact,
            total_resources,
            community_votes,
        );
        
        // Solve the optimization problem
        let result = self.engine.solve(&problem, Some("resource_allocation".to_string()))?;
        
        Ok(result.solution)
    }
    
    /// Balance immediate needs with long-term sustainability
    pub fn balance_immediate_and_long_term(
        &mut self,
        immediate_needs: Array1<f64>, // immediate resource needs
        long_term_investments: Array1<f64>, // long-term investment opportunities
        total_resources: f64, // total available resources
        sustainability_weight: f64, // weight for long-term sustainability (0.0 to 1.0)
    ) -> Result<(Array1<f64>, Array1<f64>), OptimizationError> {
        // This would create an optimization problem that balances immediate needs
        // with long-term sustainability based on the sustainability weight
        
        // For now, we'll do a simple proportional allocation
        let immediate_allocation = &immediate_needs * (1.0 - sustainability_weight) * total_resources / immediate_needs.sum();
        let long_term_allocation = &long_term_investments * sustainability_weight * total_resources / long_term_investments.sum();
        
        Ok((immediate_allocation, long_term_allocation))
    }
    
    /// Incorporate community voting results into allocation
    pub fn allocate_by_community_votes(
        &mut self,
        cause_demands: Array1<f64>, // resource demand for each cause
        cause_impacts: Array1<f64>, // expected impact of each cause
        total_resources: f64, // total available resources
        community_votes: Array1<f64>, // community voting results (normalized)
    ) -> Result<Array1<f64>, OptimizationError> {
        // Normalize community votes to ensure they sum to 1.0
        let vote_sum: f64 = community_votes.sum();
        let normalized_votes = if vote_sum > 0.0 {
            &community_votes / vote_sum
        } else {
            // If no votes, distribute evenly
            Array1::from_elem(community_votes.len(), 1.0 / community_votes.len() as f64)
        };
        
        // Use community votes as a weighting factor in the optimization
        self.optimize_resource_distribution(
            cause_demands,
            cause_impacts,
            total_resources,
            normalized_votes,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_resource_allocation_optimizer_creation() {
        let optimizer = ResourceAllocationOptimizer::new();
        assert!(true); // Creation should succeed
    }
    
    #[test]
    fn test_optimize_resource_distribution() {
        let mut optimizer = ResourceAllocationOptimizer::new();
        
        // Simple test case: 3 causes
        let demand = ndarray::arr1(&[100.0, 200.0, 150.0]);
        let impact = ndarray::arr1(&[0.8, 0.9, 0.7]);
        let total_resources = 300.0;
        let community_votes = ndarray::arr1(&[50.0, 80.0, 30.0]);
        
        let result = optimizer.optimize_resource_distribution(
            demand,
            impact,
            total_resources,
            community_votes,
        );
        
        assert!(result.is_ok());
        let allocation = result.unwrap();
        assert_eq!(allocation.len(), 3);
        
        // Check that total allocation doesn't exceed resources
        let total_allocation: f64 = allocation.sum();
        assert!(total_allocation <= total_resources);
    }
    
    #[test]
    fn test_balance_immediate_and_long_term() {
        let mut optimizer = ResourceAllocationOptimizer::new();
        
        let immediate_needs = ndarray::arr1(&[100.0, 50.0]);
        let long_term_investments = ndarray::arr1(&[200.0, 300.0]);
        let total_resources = 400.0;
        let sustainability_weight = 0.6;
        
        let result = optimizer.balance_immediate_and_long_term(
            immediate_needs,
            long_term_investments,
            total_resources,
            sustainability_weight,
        );
        
        assert!(result.is_ok());
        let (immediate, long_term) = result.unwrap();
        
        // Check that allocations match expected sizes
        assert_eq!(immediate.len(), 2);
        assert_eq!(long_term.len(), 2);
    }
}
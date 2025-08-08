//! Comprehensive optimization framework using argmin for the CPC ecosystem
//!
//! This module provides a comprehensive optimization engine that:
//! - Uses argmin as the underlying optimization framework
//! - Implements multiple optimization algorithms (gradient descent, evolutionary algorithms, etc.)
//! - Provides cooperative values-aware objective functions
//! - Supports both single-objective and multi-objective optimization
//! - Includes robust convergence detection and error handling
//!
//! The framework is designed to work across multiple domains in the CPC ecosystem,
//! including volunteer coordination, resource allocation, route optimization, and skill development.

pub mod engine;
pub mod problem;
pub mod cooperative_values;
pub mod solvers;
pub mod progressive;
pub mod error;

// Re-export key types
pub use engine::OptimizationEngine;
pub use problem::{OptimizationProblem, ObjectiveFunction};
pub use error::OptimizationError;

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array1;
    
    #[test]
    fn test_optimization_framework_compilation() {
        // Test that we can create the main components
        let engine = engine::OptimizationEngine::new();
        assert!(true); // If we get here, compilation worked
        
        // Test that we can create cooperative values
        let values = cooperative_values::CooperativeValues::default();
        assert!(values.prioritize_community_benefit);
        
        // Test that we can create solvers
        let config = solvers::SolverConfig::default();
        let solver = solvers::OptimizationSolver::with_config(config);
        assert!(true); // If we get here, compilation worked
    }
    
    #[test]
    fn test_problem_creation() {
        // Test that we can create optimization problems
        let initial_param = Array1::zeros(5);
        // We can't easily test the objective function without implementing one
        // but we can at least verify the structure compiles
        assert_eq!(initial_param.len(), 5);
    }
}
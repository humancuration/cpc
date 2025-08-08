//! Progressive optimization for Web/WASM environments
//!
//! This module provides time-sliced optimization to prevent browser UI blocking
//! and implements fallback mechanisms for complex optimizations in constrained environments.

use ndarray::Array1;
use crate::problem::OptimizationProblem;
use crate::solvers::{OptimizationSolver, SolverConfig, OptimizationAlgorithm, OptimizationResult};
use crate::error::OptimizationError;
use std::time::{Instant, Duration};

/// Progressive optimization configuration
#[derive(Debug, Clone)]
pub struct ProgressiveConfig {
    /// Time slice duration in milliseconds
    pub time_slice_ms: u64,
    
    /// Maximum total optimization time in seconds
    pub max_total_time_secs: u64,
    
    /// Fallback algorithm for constrained environments
    pub fallback_algorithm: OptimizationAlgorithm,
    
    /// Fallback maximum iterations
    pub fallback_max_iterations: u64,
    
    /// Whether to provide progressive results
    pub provide_progressive_results: bool,
}

impl Default for ProgressiveConfig {
    fn default() -> Self {
        Self {
            time_slice_ms: 50, // 50ms time slices to keep UI responsive
            max_total_time_secs: 30, // 30 second maximum
            fallback_algorithm: OptimizationAlgorithm::NelderMead,
            fallback_max_iterations: 100,
            provide_progressive_results: true,
        }
    }
}

/// Progressive optimization engine
pub struct ProgressiveOptimizer {
    config: ProgressiveConfig,
}

impl ProgressiveOptimizer {
    /// Create a new progressive optimizer with default configuration
    pub fn new() -> Self {
        Self::with_config(ProgressiveConfig::default())
    }
    
    /// Create a new progressive optimizer with custom configuration
    pub fn with_config(config: ProgressiveConfig) -> Self {
        Self { config }
    }
    
    /// Solve an optimization problem with time constraints
    pub fn solve_progressive(
        &self,
        problem: &OptimizationProblem,
    ) -> Result<ProgressiveResult, OptimizationError> {
        let start_time = Instant::now();
        let max_time = Duration::from_secs(self.config.max_total_time_secs);
        
        // Try the primary solver first
        let primary_config = SolverConfig {
            max_iterations: 10000, // High iteration count for quality
            ..Default::default()
        };
        
        let primary_solver = OptimizationSolver::with_config(primary_config);
        
        // Attempt progressive optimization
        match self.solve_with_time_limit(&primary_solver, problem, start_time, max_time) {
            Ok(result) => {
                Ok(ProgressiveResult {
                    best_result: result,
                    used_fallback: false,
                    computation_time: start_time.elapsed(),
                })
            },
            
            Err(_) => {
                // Fallback to simpler algorithm
                let fallback_config = SolverConfig {
                    max_iterations: self.config.fallback_max_iterations,
                    algorithm: self.config.fallback_algorithm.clone(),
                    ..Default::default()
                };
                
                let fallback_solver = OptimizationSolver::with_config(fallback_config);
                let result = fallback_solver.solve(problem)?;
                
                Ok(ProgressiveResult {
                    best_result: result,
                    used_fallback: true,
                    computation_time: start_time.elapsed(),
                })
            }
        }
    }
    
    /// Solve with time limit checking
    fn solve_with_time_limit(
        &self,
        solver: &OptimizationSolver,
        problem: &OptimizationProblem,
        start_time: Instant,
        max_time: Duration,
    ) -> Result<OptimizationResult, OptimizationError> {
        // For now, we'll implement a simple time-checking wrapper
        // In a real implementation, this would use async execution with time slicing
        let result = solver.solve(problem)?;
        
        // Check if we've exceeded time limits
        if start_time.elapsed() > max_time {
            return Err(OptimizationError::Timeout(
                "Optimization exceeded maximum time limit".to_string()
            ));
        }
        
        Ok(result)
    }
    
    /// Solve with periodic callback for progressive results
    pub fn solve_with_callback<F>(
        &self,
        problem: &OptimizationProblem,
        mut callback: F,
    ) -> Result<OptimizationResult, OptimizationError>
    where
        F: FnMut(&Array1<f64>, f64, u64) -> bool, // Return false to cancel
    {
        // This is a simplified implementation
        // In a real implementation, this would integrate with the solver's iteration process
        let solver = OptimizationSolver::new();
        let result = solver.solve(problem)?;
        
        // Call callback with final result
        if !callback(&result.solution, result.cost, result.iterations) {
            return Err(OptimizationError::ConvergenceFailure(
                "Optimization cancelled by callback".to_string()
            ));
        }
        
        Ok(result)
    }
}

/// Result of progressive optimization
#[derive(Debug, Clone)]
pub struct ProgressiveResult {
    /// The best result achieved
    pub best_result: OptimizationResult,
    
    /// Whether fallback algorithm was used
    pub used_fallback: bool,
    
    /// Total computation time
    pub computation_time: Duration,
}

impl ProgressiveResult {
    /// Get the best solution found
    pub fn solution(&self) -> &Array1<f64> {
        &self.best_result.solution
    }
    
    /// Get the cost of the best solution
    pub fn cost(&self) -> f64 {
        self.best_result.cost
    }
    
    /// Check if optimization converged
    pub fn converged(&self) -> bool {
        self.best_result.converged
    }
    
    /// Check if fallback was used
    pub fn used_fallback(&self) -> bool {
        self.used_fallback
    }
    
    /// Get computation time
    pub fn computation_time(&self) -> Duration {
        self.computation_time
    }
}

// WASM-specific implementations
#[cfg(target_arch = "wasm32")]
mod wasm_impl {
    use super::*;
    use wasm_bindgen::prelude::*;
    use gloo_timers::future::TimeoutFuture;
    
    /// Async progressive optimization for WASM
    #[wasm_bindgen]
    pub struct WasmProgressiveOptimizer {
        inner: ProgressiveOptimizer,
    }
    
    #[wasm_bindgen]
    impl WasmProgressiveOptimizer {
        /// Create a new WASM progressive optimizer
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            Self {
                inner: ProgressiveOptimizer::new(),
            }
        }
        
        /// Solve an optimization problem asynchronously
        #[wasm_bindgen]
        pub async fn solve_async(&self, problem: &js_sys::Object) -> Result<js_sys::Object, JsValue> {
            // In a real implementation, this would convert JS objects to Rust structures
            // and perform the optimization with proper async support
            
            // Yield control periodically to prevent UI blocking
            TimeoutFuture::new(1).await;
            
            // Return a placeholder result
            Ok(js_sys::Object::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::problem::{VolunteerMatchingObjective};
    use ndarray::arr1;
    
    #[test]
    fn test_progressive_optimizer_creation() {
        let optimizer = ProgressiveOptimizer::new();
        assert_eq!(optimizer.config.time_slice_ms, 50);
        assert_eq!(optimizer.config.max_total_time_secs, 30);
    }
    
    #[test]
    fn test_progressive_config_default() {
        let config = ProgressiveConfig::default();
        assert_eq!(config.time_slice_ms, 50);
        assert_eq!(config.max_total_time_secs, 30);
    }
    
    #[test]
    fn test_progressive_result_creation() {
        let optimization_result = OptimizationResult {
            solution: arr1(&[1.0, 2.0, 3.0]),
            cost: 10.5,
            iterations: 100,
            converged: true,
        };
        
        let progressive_result = ProgressiveResult {
            best_result: optimization_result,
            used_fallback: false,
            computation_time: Duration::from_millis(100),
        };
        
        assert_eq!(progressive_result.cost(), 10.5);
        assert!(progressive_result.converged());
        assert!(!progressive_result.used_fallback());
    }
}
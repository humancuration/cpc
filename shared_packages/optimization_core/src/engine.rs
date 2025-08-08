//! Core optimization engine using argmin for high-performance optimization
//!
//! This module provides the main optimization engine that orchestrates all
//! optimization processes, manages caching, and provides a unified interface
//! for different optimization domains in the CPC ecosystem.

use ndarray::Array1;
use std::collections::HashMap;
use tracing::{info, debug, warn};
use uuid::Uuid;
use crate::error::OptimizationError;
use crate::problem::{OptimizationProblem, ObjectiveFunction};
use crate::solvers::{OptimizationSolver, SolverConfig, OptimizationResult};
use crate::progressive::{ProgressiveOptimizer, ProgressiveResult};
use crate::cooperative_values::CooperativeValues;

/// Configuration for the optimization engine
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// Maximum cache size for optimization results
    pub max_cache_size: usize,
    
    /// Enable progressive optimization for Web/WASM
    pub enable_progressive: bool,
    
    /// Default solver configuration
    pub default_solver_config: SolverConfig,
    
    /// Cooperative values settings
    pub cooperative_values: CooperativeValues,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            max_cache_size: 100,
            enable_progressive: true,
            default_solver_config: SolverConfig::default(),
            cooperative_values: CooperativeValues::default(),
        }
    }
}

/// Core optimization engine using argmin for high-performance optimization
pub struct OptimizationEngine {
    config: EngineConfig,
    /// Cache for frequently used optimization results
    result_cache: HashMap<String, OptimizationResult>,
    /// Progressive optimizer for Web/WASM environments
    progressive_optimizer: ProgressiveOptimizer,
}

impl OptimizationEngine {
    /// Create a new optimization engine with default configuration
    pub fn new() -> Self {
        Self::with_config(EngineConfig::default())
    }
    
    /// Create a new optimization engine with custom configuration
    pub fn with_config(config: EngineConfig) -> Self {
        info!("Initializing OptimizationEngine with config: {:?}", config);
        
        let progressive_optimizer = if config.enable_progressive {
            ProgressiveOptimizer::new()
        } else {
            ProgressiveOptimizer::with_config(
                crate::progressive::ProgressiveConfig {
                    provide_progressive_results: false,
                    ..Default::default()
                }
            )
        };
        
        Self {
            config,
            result_cache: HashMap::new(),
            progressive_optimizer,
        }
    }
    
    /// Solve an optimization problem
    pub fn solve(
        &mut self,
        problem: &OptimizationProblem,
        cache_key: Option<String>,
    ) -> Result<OptimizationResult, OptimizationError> {
        // Check cache first
        if let Some(key) = &cache_key {
            if let Some(cached_result) = self.result_cache.get(key) {
                info!("Using cached optimization result for key: {}", key);
                return Ok(cached_result.clone());
            }
        }
        
        info!("Solving optimization problem");
        
        // Create solver with configuration
        let solver = OptimizationSolver::with_config(self.config.default_solver_config.clone());
        let result = solver.solve(problem)?;
        
        // Cache the result if requested
        if let Some(key) = cache_key {
            if self.result_cache.len() < self.config.max_cache_size {
                debug!("Caching optimization result for key: {}", key);
                self.result_cache.insert(key, result.clone());
            } else {
                // Remove oldest entry if cache is full
                if let Some(oldest_key) = self.result_cache.keys().next().cloned() {
                    self.result_cache.remove(&oldest_key);
                    self.result_cache.insert(key, result.clone());
                }
            }
        }
        
        Ok(result)
    }
    
    /// Solve an optimization problem with progressive/fallback behavior
    pub fn solve_progressive(
        &mut self,
        problem: &OptimizationProblem,
        cache_key: Option<String>,
    ) -> Result<ProgressiveResult, OptimizationError> {
        // Check cache first for progressive results
        // (In a real implementation, we might want to cache progressive results separately)
        if let Some(key) = &cache_key {
            if let Some(cached_result) = self.result_cache.get(key) {
                info!("Using cached optimization result for key: {}", key);
                let progressive_result = ProgressiveResult {
                    best_result: cached_result.clone(),
                    used_fallback: false,
                    computation_time: std::time::Duration::from_millis(0),
                };
                return Ok(progressive_result);
            }
        }
        
        info!("Solving optimization problem progressively");
        
        let result = self.progressive_optimizer.solve_progressive(problem)?;
        
        // Cache the result if requested
        if let Some(key) = cache_key {
            if let Some(best_result) = self.result_cache.get_mut(&key) {
                *best_result = result.best_result.clone();
            } else if self.result_cache.len() < self.config.max_cache_size {
                debug!("Caching optimization result for key: {}", key);
                self.result_cache.insert(key, result.best_result.clone());
            }
        }
        
        Ok(result)
    }
    
    /// Create a volunteer-task matching optimization problem
    pub fn create_volunteer_matching_problem(
        &self,
        skill_matrix: ndarray::Array2<f64>,
        availability: Array1<f64>,
        task_priority: Array1<f64>,
        community_impact: Array1<f64>,
    ) -> OptimizationProblem {
        let initial_param = Array1::zeros(skill_matrix.shape()[0] * skill_matrix.shape()[1]);
        
        let objective = Box::new(
            crate::problem::VolunteerMatchingObjective::new(
                skill_matrix,
                availability,
                task_priority,
                community_impact,
            )
        );
        
        OptimizationProblem::new(objective, initial_param)
            .with_cooperative_values(self.config.cooperative_values.clone())
    }
    
    /// Create a resource allocation optimization problem
    pub fn create_resource_allocation_problem(
        &self,
        demand: Array1<f64>,
        impact: Array1<f64>,
        total_resources: f64,
        community_votes: Array1<f64>,
    ) -> OptimizationProblem {
        let initial_param = Array1::zeros(demand.len());
        
        let objective = Box::new(
            crate::problem::ResourceAllocationObjective::new(
                demand,
                impact,
                total_resources,
                community_votes,
            )
        );
        
        OptimizationProblem::new(objective, initial_param)
            .with_cooperative_values(self.config.cooperative_values.clone())
    }
    
    /// Clear the result cache
    pub fn clear_cache(&mut self) {
        self.result_cache.clear();
        debug!("Optimization result cache cleared");
    }
    
    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        (self.result_cache.len(), self.config.max_cache_size)
    }
}

impl Default for OptimizationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr1;
    
    #[test]
    fn test_optimization_engine_creation() {
        let engine = OptimizationEngine::new();
        assert_eq!(engine.config.max_cache_size, 100);
        assert!(engine.config.enable_progressive);
    }
    
    #[test]
    fn test_engine_config_default() {
        let config = EngineConfig::default();
        assert_eq!(config.max_cache_size, 100);
        assert!(config.enable_progressive);
    }
    
    #[test]
    fn test_cache_functionality() {
        let mut engine = OptimizationEngine::new();
        
        // Cache should be empty initially
        assert_eq!(engine.cache_stats().0, 0);
        
        // Clearing empty cache should work
        engine.clear_cache();
        assert_eq!(engine.cache_stats().0, 0);
    }
    
    #[test]
    fn test_problem_creation() {
        let engine = OptimizationEngine::new();
        
        // Test volunteer matching problem creation
        let skill_matrix = ndarray::arr2(&[[0.8, 0.2], [0.3, 0.9]]);
        let availability = arr1(&[1.0, 1.0]);
        let task_priority = arr1(&[1.0, 1.0]);
        let community_impact = arr1(&[0.5, 0.7]);
        
        let problem = engine.create_volunteer_matching_problem(
            skill_matrix,
            availability,
            task_priority,
            community_impact,
        );
        
        assert_eq!(problem.initial_param.len(), 4); // 2 volunteers * 2 tasks
    }
}
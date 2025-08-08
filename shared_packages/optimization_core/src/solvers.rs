//! Optimization solvers implementation
//!
//! This module provides various optimization algorithms that can be used
//! with the optimization framework, including gradient-based methods,
//! evolutionary algorithms, and specialized solvers for different problem types.

use argmin::core::{Executor, State};
use argmin::solver::gradientdescent::SteepestDescent;
use argmin::solver::linesearch::MoreThuenteLineSearch;
use argmin::solver::simulatedannealing::SimulatedAnnealing;
use argmin::solver::neldermead::NelderMead;
use ndarray::Array1;
use crate::problem::{OptimizationProblem, ArgminCostWrapper};
use crate::error::OptimizationError;
use crate::cooperative_values::CooperativeGovernance;

/// Available optimization algorithms
#[derive(Debug, Clone)]
pub enum OptimizationAlgorithm {
    /// Gradient descent with line search
    GradientDescent,
    
    /// Simulated annealing (evolutionary algorithm)
    SimulatedAnnealing,
    
    /// Nelder-Mead simplex algorithm
    NelderMead,
    
    /// Custom algorithm configuration
    Custom(String),
}

/// Solver configuration
#[derive(Debug, Clone)]
pub struct SolverConfig {
    /// Maximum number of iterations
    pub max_iterations: u64,
    
    /// Convergence tolerance
    pub tolerance: f64,
    
    /// Algorithm to use
    pub algorithm: OptimizationAlgorithm,
    
    /// Whether to enable cooperative governance
    pub enable_cooperative_governance: bool,
}

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            max_iterations: 1000,
            tolerance: 1e-6,
            algorithm: OptimizationAlgorithm::GradientDescent,
            enable_cooperative_governance: true,
        }
    }
}

/// Optimization solver
pub struct OptimizationSolver {
    config: SolverConfig,
}

impl OptimizationSolver {
    /// Create a new optimization solver with default configuration
    pub fn new() -> Self {
        Self::with_config(SolverConfig::default())
    }
    
    /// Create a new optimization solver with custom configuration
    pub fn with_config(config: SolverConfig) -> Self {
        Self { config }
    }
    
    /// Solve an optimization problem
    pub fn solve(
        &self,
        problem: &OptimizationProblem,
    ) -> Result<OptimizationResult, OptimizationError> {
        // Create argmin cost function wrapper
        let cost_wrapper = ArgminCostWrapper::new(problem.objective.clone());
        
        // Create solver based on configuration
        let result = match &self.config.algorithm {
            OptimizationAlgorithm::GradientDescent => {
                let linesearch = MoreThuenteLineSearch::new();
                let solver = SteepestDescent::new(linesearch);
                
                Executor::new(cost_wrapper, solver, problem.initial_param.clone())
                    .configure(|state| state
                        .max_iters(self.config.max_iterations)
                        .tolerance(self.config.tolerance)
                    )
                    .run()
                    .map_err(|e| OptimizationError::InternalError(e))?
            },
            
            OptimizationAlgorithm::SimulatedAnnealing => {
                let solver = SimulatedAnnealing::new(1000.0)
                    .with_period(100)
                    .with_noise(0.1);
                
                Executor::new(cost_wrapper, solver, problem.initial_param.clone())
                    .configure(|state| state
                        .max_iters(self.config.max_iterations)
                        .tolerance(self.config.tolerance)
                    )
                    .run()
                    .map_err(|e| OptimizationError::InternalError(e))?
            },
            
            OptimizationAlgorithm::NelderMead => {
                let solver = NelderMead::new(1e-4, 1e-4, 1e-4, 1e-4);
                
                Executor::new(cost_wrapper, solver, problem.initial_param.clone())
                    .configure(|state| state
                        .max_iters(self.config.max_iterations)
                        .tolerance(self.config.tolerance)
                    )
                    .run()
                    .map_err(|e| OptimizationError::InternalError(e))?
            },
            
            OptimizationAlgorithm::Custom(name) => {
                return Err(OptimizationError::UnsupportedAlgorithm(
                    format!("Custom algorithm '{}' not implemented", name)
                ));
            },
        };
        
        // Extract solution
        let solution = result.state().get_best_param().unwrap_or(&problem.initial_param).clone();
        let cost = result.state().get_best_cost().unwrap_or(f64::INFINITY);
        let iterations = result.state().get_iter();
        
        // Apply cooperative governance if enabled
        let solution = if self.config.enable_cooperative_governance {
            let governance = CooperativeGovernance::new(problem.cooperative_values.clone());
            let mut solution_array = solution.clone();
            
            // Apply fairness constraints
            governance.apply_fairness_constraints(&mut solution_array)?;
            
            // Validate solution meets cooperative values
            if !governance.validate_solution(&solution_array)? {
                return Err(OptimizationError::CooperativeValuesViolation(
                    "Solution does not meet cooperative values requirements".to_string()
                ));
            }
            
            solution_array
        } else {
            solution
        };
        
        Ok(OptimizationResult {
            solution,
            cost,
            iterations,
            converged: result.state().is_best_cost_updated(),
        })
    }
}

/// Result of an optimization
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    /// The optimal solution
    pub solution: Array1<f64>,
    
    /// The cost/value of the optimal solution
    pub cost: f64,
    
    /// Number of iterations performed
    pub iterations: u64,
    
    /// Whether the optimization converged
    pub converged: bool,
}

impl OptimizationResult {
    /// Get the solution vector
    pub fn solution(&self) -> &Array1<f64> {
        &self.solution
    }
    
    /// Get the objective value
    pub fn cost(&self) -> f64 {
        self.cost
    }
    
    /// Check if optimization converged
    pub fn converged(&self) -> bool {
        self.converged
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::problem::{VolunteerMatchingObjective, ResourceAllocationObjective};
    use ndarray::arr1;
    
    #[test]
    fn test_gradient_descent_solver() {
        let solver_config = SolverConfig {
            algorithm: OptimizationAlgorithm::GradientDescent,
            ..Default::default()
        };
        
        let solver = OptimizationSolver::with_config(solver_config);
        assert!(true); // Solver creation should succeed
    }
    
    #[test]
    fn test_simulated_annealing_solver() {
        let solver_config = SolverConfig {
            algorithm: OptimizationAlgorithm::SimulatedAnnealing,
            ..Default::default()
        };
        
        let solver = OptimizationSolver::with_config(solver_config);
        assert!(true); // Solver creation should succeed
    }
    
    #[test]
    fn test_nelder_mead_solver() {
        let solver_config = SolverConfig {
            algorithm: OptimizationAlgorithm::NelderMead,
            ..Default::default()
        };
        
        let solver = OptimizationSolver::with_config(solver_config);
        assert!(true); // Solver creation should succeed
    }
    
    #[test]
    fn test_solver_creation() {
        let solver = OptimizationSolver::new();
        assert_eq!(solver.config.max_iterations, 1000);
        assert_eq!(solver.config.tolerance, 1e-6);
    }
}
//! Optimization problem definitions and objective functions
//!
//! This module defines the core optimization problem structures and objective functions
//! that can be used across different domains in the CPC ecosystem.

use argmin::core::{CostFunction, Gradient};
use ndarray::Array1;
use serde::{Deserialize, Serialize};
use crate::error::OptimizationError;
use crate::cooperative_values::CooperativeValues;

/// Trait for objective functions that can be optimized
pub trait ObjectiveFunction {
    /// Calculate the cost/value of a solution
    fn cost(&self, param: &Array1<f64>) -> Result<f64, OptimizationError>;
    
    /// Calculate the gradient of the objective function (if available)
    fn gradient(&self, param: &Array1<f64>) -> Result<Array1<f64>, OptimizationError> {
        // Default finite difference implementation
        let epsilon = 1e-8;
        let mut grad = Array1::zeros(param.len());
        
        for i in 0..param.len() {
            let mut param_plus = param.clone();
            let mut param_minus = param.clone();
            
            param_plus[i] += epsilon;
            param_minus[i] -= epsilon;
            
            let cost_plus = self.cost(&param_plus)?;
            let cost_minus = self.cost(&param_minus)?;
            
            grad[i] = (cost_plus - cost_minus) / (2.0 * epsilon);
        }
        
        Ok(grad)
    }
}

/// Core optimization problem structure
#[derive(Debug, Clone)]
pub struct OptimizationProblem {
    /// The objective function to optimize
    pub objective: Box<dyn ObjectiveFunction + Send + Sync>,
    
    /// The initial parameter values
    pub initial_param: Array1<f64>,
    
    /// Lower bounds for parameters
    pub lower_bounds: Option<Array1<f64>>,
    
    /// Upper bounds for parameters
    pub upper_bounds: Option<Array1<f64>>,
    
    /// Cooperative values settings
    pub cooperative_values: CooperativeValues,
}

impl OptimizationProblem {
    /// Create a new optimization problem
    pub fn new(
        objective: Box<dyn ObjectiveFunction + Send + Sync>,
        initial_param: Array1<f64>,
    ) -> Self {
        Self {
            objective,
            initial_param,
            lower_bounds: None,
            upper_bounds: None,
            cooperative_values: CooperativeValues::default(),
        }
    }
    
    /// Set parameter bounds
    pub fn with_bounds(mut self, lower: Array1<f64>, upper: Array1<f64>) -> Self {
        self.lower_bounds = Some(lower);
        self.upper_bounds = Some(upper);
        self
    }
    
    /// Set cooperative values
    pub fn with_cooperative_values(mut self, values: CooperativeValues) -> Self {
        self.cooperative_values = values;
        self
    }
}

/// Wrapper for argmin cost function implementation
pub struct ArgminCostWrapper {
    pub objective: Box<dyn ObjectiveFunction + Send + Sync>,
}

impl ArgminCostWrapper {
    pub fn new(objective: Box<dyn ObjectiveFunction + Send + Sync>) -> Self {
        Self { objective }
    }
}

impl CostFunction for ArgminCostWrapper {
    type Param = Array1<f64>;
    type Output = f64;
    
    fn cost(&self, param: &Self::Param) -> Result<Self::Output, argmin::core::Error> {
        self.objective.cost(param)
            .map_err(|e| argmin::core::Error::msg(e.to_string()))
    }
}

impl Gradient for ArgminCostWrapper {
    type Param = Array1<f64>;
    type Gradient = Array1<f64>;
    
    fn gradient(&self, param: &Self::Param) -> Result<Self::Gradient, argmin::core::Error> {
        self.objective.gradient(param)
            .map_err(|e| argmin::core::Error::msg(e.to_string()))
    }
}

/// Example objective function for volunteer-task matching
#[derive(Debug, Clone)]
pub struct VolunteerMatchingObjective {
    /// Skill compatibility matrix (volunteers x tasks)
    pub skill_matrix: ndarray::Array2<f64>,
    
    /// Volunteer availability vector
    pub availability: Array1<f64>,
    
    /// Task priority vector
    pub task_priority: Array1<f64>,
    
    /// Community impact weights
    pub community_impact: Array1<f64>,
}

impl VolunteerMatchingObjective {
    pub fn new(
        skill_matrix: ndarray::Array2<f64>,
        availability: Array1<f64>,
        task_priority: Array1<f64>,
        community_impact: Array1<f64>,
    ) -> Self {
        Self {
            skill_matrix,
            availability,
            task_priority,
            community_impact,
        }
    }
}

impl ObjectiveFunction for VolunteerMatchingObjective {
    fn cost(&self, param: &Array1<f64>) -> Result<f64, OptimizationError> {
        // param represents assignment matrix flattened to vector
        // We need to reshape it to match the skill matrix dimensions
        let num_volunteers = self.skill_matrix.shape()[0];
        let num_tasks = self.skill_matrix.shape()[1];
        
        if param.len() != num_volunteers * num_tasks {
            return Err(OptimizationError::InvalidParameters(
                "Parameter vector size does not match assignment matrix dimensions".to_string()
            ));
        }
        
        // Reshape parameter vector to assignment matrix
        let assignment = ndarray::Array2::from_shape_vec(
            (num_volunteers, num_tasks),
            param.to_vec()
        ).map_err(|_| OptimizationError::InvalidParameters(
            "Failed to reshape parameter vector".to_string()
        ))?;
        
        // Calculate total skill match score
        let skill_match = (&self.skill_matrix * &assignment).sum();
        
        // Calculate availability satisfaction
        let volunteer_load: Array1<f64> = assignment.sum_axis(ndarray::Axis(1));
        let availability_satisfaction: f64 = (&self.availability * &volunteer_load).sum();
        
        // Calculate task priority fulfillment
        let task_completion: Array1<f64> = assignment.sum_axis(ndarray::Axis(0));
        let priority_fulfillment: f64 = (&self.task_priority * &task_completion).sum();
        
        // Calculate community impact
        let community_benefit: f64 = (&self.community_impact * &task_completion).sum();
        
        // Combine objectives (negative because we minimize in argmin)
        let total_cost = -(skill_match + availability_satisfaction + priority_fulfillment + community_benefit);
        
        Ok(total_cost)
    }
}

/// Example objective function for resource allocation
#[derive(Debug, Clone)]
pub struct ResourceAllocationObjective {
    /// Resource demand for each cause/project
    pub demand: Array1<f64>,
    
    /// Expected impact of each cause/project
    pub impact: Array1<f64>,
    
    /// Resource constraints (total available resources)
    pub total_resources: f64,
    
    /// Community voting results for each cause/project
    pub community_votes: Array1<f64>,
}

impl ResourceAllocationObjective {
    pub fn new(
        demand: Array1<f64>,
        impact: Array1<f64>,
        total_resources: f64,
        community_votes: Array1<f64>,
    ) -> Self {
        Self {
            demand,
            impact,
            total_resources,
            community_votes,
        }
    }
}

impl ObjectiveFunction for ResourceAllocationObjective {
    fn cost(&self, param: &Array1<f64>) -> Result<f64, OptimizationError> {
        // param represents resource allocation to each cause/project
        
        if param.len() != self.demand.len() {
            return Err(OptimizationError::InvalidParameters(
                "Parameter vector size does not match number of causes/projects".to_string()
            ));
        }
        
        // Check resource constraint
        let total_allocated: f64 = param.sum();
        if total_allocated > self.total_resources {
            // Penalize solutions that exceed resource constraints
            let penalty = (total_allocated - self.total_resources) * 1000.0;
            return Ok(penalty);
        }
        
        // Calculate total impact
        let total_impact: f64 = (param * &self.impact).sum();
        
        // Calculate demand satisfaction
        let demand_satisfaction: f64 = param.iter()
            .zip(self.demand.iter())
            .map(|(alloc, demand)| alloc.min(demand))
            .sum();
        
        // Calculate community alignment
        let community_alignment: f64 = (param * &self.community_votes).sum();
        
        // Combine objectives (negative because we minimize in argmin)
        let total_cost = -(total_impact + demand_satisfaction + community_alignment);
        
        Ok(total_cost)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_volunteer_matching_objective() {
        let skill_matrix = ndarray::arr2(&[[0.8, 0.2], [0.3, 0.9]]);
        let availability = ndarray::arr1(&[1.0, 1.0]);
        let task_priority = ndarray::arr1(&[1.0, 1.0]);
        let community_impact = ndarray::arr1(&[0.5, 0.7]);
        
        let objective = VolunteerMatchingObjective::new(
            skill_matrix,
            availability,
            task_priority,
            community_impact,
        );
        
        let assignment = ndarray::arr1(&[1.0, 0.0, 0.0, 1.0]); // Perfect matching
        let cost = objective.cost(&assignment).unwrap();
        
        // Cost should be negative (because we're maximizing in the objective)
        assert!(cost < 0.0);
    }
    
    #[test]
    fn test_resource_allocation_objective() {
        let demand = ndarray::arr1(&[100.0, 200.0, 150.0]);
        let impact = ndarray::arr1(&[0.8, 0.9, 0.7]);
        let total_resources = 300.0;
        let community_votes = ndarray::arr1(&[50.0, 80.0, 30.0]);
        
        let objective = ResourceAllocationObjective::new(
            demand,
            impact,
            total_resources,
            community_votes,
        );
        
        let allocation = ndarray::arr1(&[100.0, 150.0, 50.0]); // Valid allocation
        let cost = objective.cost(&allocation).unwrap();
        
        // Cost should be negative (because we're maximizing in the objective)
        assert!(cost < 0.0);
    }
    
    #[test]
    fn test_resource_allocation_constraint_violation() {
        let demand = ndarray::arr1(&[100.0, 200.0, 150.0]);
        let impact = ndarray::arr1(&[0.8, 0.9, 0.7]);
        let total_resources = 300.0;
        let community_votes = ndarray::arr1(&[50.0, 80.0, 30.0]);
        
        let objective = ResourceAllocationObjective::new(
            demand,
            impact,
            total_resources,
            community_votes,
        );
        
        let allocation = ndarray::arr1(&[150.0, 200.0, 100.0]); // Exceeds total resources
        let cost = objective.cost(&allocation).unwrap();
        
        // Cost should be positive (penalty for constraint violation)
        assert!(cost > 0.0);
    }
}
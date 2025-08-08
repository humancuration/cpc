//! Volunteer-task matching optimization
//!
//! This module provides optimization capabilities for matching volunteers to tasks
//! based on skills, availability, and community impact.

use optimization_core::problem::{VolunteerMatchingObjective};
use optimization_core::engine::OptimizationEngine;
use optimization_core::error::OptimizationError;
use ndarray::Array1;

/// Volunteer matching optimizer
pub struct VolunteerMatchingOptimizer {
    engine: OptimizationEngine,
}

impl VolunteerMatchingOptimizer {
    /// Create a new volunteer matching optimizer
    pub fn new() -> Self {
        Self {
            engine: OptimizationEngine::new(),
        }
    }
    
    /// Match volunteers to tasks based on skills and availability
    pub fn match_volunteers_to_tasks(
        &mut self,
        skill_matrix: ndarray::Array2<f64>, // volunteers x tasks
        availability: Array1<f64>, // volunteer availability
        task_priority: Array1<f64>, // task priority
        community_impact: Array1<f64>, // community impact of tasks
    ) -> Result<ndarray::Array2<f64>, OptimizationError> {
        // Create optimization problem
        let problem = self.engine.create_volunteer_matching_problem(
            skill_matrix.clone(),
            availability,
            task_priority,
            community_impact,
        );
        
        // Solve the optimization problem
        let result = self.engine.solve(&problem, Some("volunteer_matching".to_string()))?;
        
        // Reshape solution to assignment matrix
        let num_volunteers = skill_matrix.shape()[0];
        let num_tasks = skill_matrix.shape()[1];
        
        let assignment = ndarray::Array2::from_shape_vec(
            (num_volunteers, num_tasks),
            result.solution.to_vec(),
        ).map_err(|_| OptimizationError::InvalidParameters(
            "Failed to reshape solution to assignment matrix".to_string()
        ))?;
        
        Ok(assignment)
    }
    
    /// Balance workload across volunteers
    pub fn balance_workload(
        &mut self,
        current_assignments: ndarray::Array2<f64>, // volunteers x tasks
        volunteer_capacity: Array1<f64>, // maximum workload per volunteer
    ) -> Result<ndarray::Array2<f64>, OptimizationError> {
        // This would create an optimization problem to redistribute workload
        // while maintaining task completion
        
        // For now, we'll just return the current assignments
        // A full implementation would create a load balancing optimization problem
        Ok(current_assignments)
    }
    
    /// Optimize for skill development opportunities
    pub fn optimize_for_skill_development(
        &mut self,
        skill_matrix: ndarray::Array2<f64>, // volunteers x tasks
        volunteer_skill_goals: ndarray::Array2<f64>, // volunteers x skills (development goals)
        availability: Array1<f64>, // volunteer availability
        task_priority: Array1<f64>, // task priority
    ) -> Result<ndarray::Array2<f64>, OptimizationError> {
        // This would create an optimization problem that prioritizes tasks
        // that help volunteers develop their desired skills
        
        // For now, we'll just use the basic matching
        let community_impact = Array1::zeros(task_priority.len());
        self.match_volunteers_to_tasks(
            skill_matrix,
            availability,
            task_priority,
            community_impact,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_volunteer_matching_optimizer_creation() {
        let optimizer = VolunteerMatchingOptimizer::new();
        assert!(true); // Creation should succeed
    }
    
    #[test]
    fn test_match_volunteers_to_tasks() {
        let mut optimizer = VolunteerMatchingOptimizer::new();
        
        // Simple test case: 2 volunteers, 2 tasks
        let skill_matrix = ndarray::arr2(&[[0.8, 0.2], [0.3, 0.9]]);
        let availability = ndarray::arr1(&[1.0, 1.0]);
        let task_priority = ndarray::arr1(&[1.0, 1.0]);
        let community_impact = ndarray::arr1(&[0.5, 0.7]);
        
        let result = optimizer.match_volunteers_to_tasks(
            skill_matrix,
            availability,
            task_priority,
            community_impact,
        );
        
        assert!(result.is_ok());
        let assignment = result.unwrap();
        assert_eq!(assignment.shape(), &[2, 2]);
    }
}
//! Skill development pathway optimization
//!
//! This module provides optimization capabilities for creating personalized learning paths
//! and optimizing skill acquisition sequences for community members.

use optimization_core::engine::OptimizationEngine;
use optimization_core::error::OptimizationError;
use ndarray::Array1;

/// Skill development optimization problem
#[derive(Debug, Clone)]
pub struct SkillDevelopmentProblem {
    /// Prerequisite relationships between skills (skill x skill matrix)
    pub prerequisites: ndarray::Array2<f64>,
    
    /// Time required to learn each skill
    pub learning_time: Array1<f64>,
    
    /// Individual's current skill levels
    pub current_skills: Array1<f64>,
    
    /// Target skill levels
    pub target_skills: Array1<f464>,
    
    /// Community need for each skill
    pub community_need: Array1<f64>,
}

/// Skill development objective function
pub struct SkillDevelopmentObjective {
    /// Prerequisite relationships between skills
    pub prerequisites: ndarray::Array2<f64>,
    
    /// Time required to learn each skill
    pub learning_time: Array1<f64>,
    
    /// Individual's current skill levels
    pub current_skills: Array1<f64>,
    
    /// Target skill levels
    pub target_skills: Array1<f64>,
    
    /// Community need for each skill
    pub community_need: Array1<f64>,
    
    /// Weight for learning time minimization
    pub time_weight: f64,
    
    /// Weight for prerequisite satisfaction
    pub prerequisite_weight: f64,
    
    /// Weight for community need maximization
    pub community_need_weight: f64,
}

impl SkillDevelopmentObjective {
    pub fn new(
        prerequisites: ndarray::Array2<f64>,
        learning_time: Array1<f64>,
        current_skills: Array1<f64>,
        target_skills: Array1<f64>,
        community_need: Array1<f64>,
        time_weight: f64,
        prerequisite_weight: f64,
        community_need_weight: f64,
    ) -> Self {
        Self {
            prerequisites,
            learning_time,
            current_skills,
            target_skills,
            community_need,
            time_weight,
            prerequisite_weight,
            community_need_weight,
        }
    }
}

impl optimization_core::problem::ObjectiveFunction for SkillDevelopmentObjective {
    fn cost(&self, param: &Array1<f64>) -> Result<f64, OptimizationError> {
        // param represents learning path (sequence of skills to learn)
        
        if param.len() != self.learning_time.len() {
            return Err(OptimizationError::InvalidParameters(
                "Parameter vector size does not match number of skills".to_string()
            ));
        }
        
        // Calculate total learning time
        let total_time: f64 = param.iter()
            .enumerate()
            .map(|(i, &learn)| learn * self.learning_time[i])
            .sum();
        
        // Calculate prerequisite satisfaction
        let mut prerequisite_violations = 0.0;
        for i in 0..param.len() {
            if param[i] > 0.0 { // If we're learning this skill
                for j in 0..param.len() {
                    // If skill j is a prerequisite for skill i but we're not learning it
                    // or learning it after skill i
                    if self.prerequisites[[j, i]] > 0.0 && param[j] < param[i] {
                        prerequisite_violations += self.prerequisites[[j, i]];
                    }
                }
            }
        }
        
        // Calculate community need fulfillment
        let community_need_fulfilled: f64 = param.iter()
            .enumerate()
            .map(|(i, &learn)| learn * self.community_need[i])
            .sum();
        
        // Combine objectives (negative because we minimize in argmin)
        // We want to minimize time and prerequisite violations, maximize community need
        let total_cost = (self.time_weight * total_time) +
            (self.prerequisite_weight * prerequisite_violations) -
            (self.community_need_weight * community_need_fulfilled);
        
        Ok(total_cost)
    }
}

/// Skill development optimizer
pub struct SkillDevelopmentOptimizer {
    engine: OptimizationEngine,
}

impl SkillDevelopmentOptimizer {
    /// Create a new skill development optimizer
    pub fn new() -> Self {
        Self {
            engine: OptimizationEngine::new(),
        }
    }
    
    /// Create personalized learning paths
    pub fn create_learning_path(
        &mut self,
        prerequisites: ndarray::Array2<f64>, // skill x skill matrix
        learning_time: Array1<f64>, // time required for each skill
        current_skills: Array1<f64>, // current skill levels (0.0 to 1.0)
        target_skills: Array1<f64>, // target skill levels (0.0 to 1.0)
        community_need: Array1<f64>, // community need for each skill
        weights: Option<(f64, f64, f64)>, // (time, prerequisite, community_need)
    ) -> Result<Array1<f64>, OptimizationError> {
        let (time_weight, prerequisite_weight, community_need_weight) = 
            weights.unwrap_or((1.0, 1.0, 1.0));
        
        // Create objective function
        let objective = Box::new(SkillDevelopmentObjective::new(
            prerequisites.clone(),
            learning_time.clone(),
            current_skills.clone(),
            target_skills.clone(),
            community_need.clone(),
            time_weight,
            prerequisite_weight,
            community_need_weight,
        ));
        
        // Create initial parameter (learn all target skills equally)
        let initial_param = &target_skills - &current_skills;
        
        // Create optimization problem
        let problem = optimization_core::problem::OptimizationProblem::new(objective, initial_param);
        
        // Solve the optimization problem
        let result = self.engine.solve(&problem, Some("skill_development".to_string()))?;
        
        Ok(result.solution)
    }
    
    /// Optimize skill acquisition sequences
    pub fn optimize_skill_acquisition_sequence(
        &mut self,
        prerequisites: ndarray::Array2<f64>,
        learning_time: Array1<f64>,
        current_skills: Array1<f64>,
        target_skills: Array1<f64>,
        community_need: Array1<f64>,
    ) -> Result<Array1<f64>, OptimizationError> {
        // Use higher weight for prerequisite satisfaction
        self.create_learning_path(
            prerequisites,
            learning_time,
            current_skills,
            target_skills,
            community_need,
            Some((1.0, 3.0, 1.0)), // Triple weight for prerequisite satisfaction
        )
    }
    
    /// Balance individual growth with community needs
    pub fn balance_individual_and_community_needs(
        &mut self,
        prerequisites: ndarray::Array2<f64>,
        learning_time: Array1<f64>,
        current_skills: Array1<f64>,
        target_skills: Array1<f64>,
        community_need: Array1<f64>,
        community_weight: f64, // weight for community needs (0.0 to 1.0)
    ) -> Result<Array1<f64>, OptimizationError> {
        // Adjust weights based on community_weight parameter
        self.create_learning_path(
            prerequisites,
            learning_time,
            current_skills,
            target_skills,
            community_need,
            Some((1.0, 1.0, community_weight * 3.0)), // Scale community need weight
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_skill_development_optimizer_creation() {
        let optimizer = SkillDevelopmentOptimizer::new();
        assert!(true); // Creation should succeed
    }
    
    #[test]
    fn test_create_learning_path() {
        let mut optimizer = SkillDevelopmentOptimizer::new();
        
        // Simple test case: 3 skills
        let prerequisites = ndarray::arr2(&[
            [0.0, 1.0, 0.0], // Skill 1 is prerequisite for skill 2
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0]
        ]);
        
        let learning_time = ndarray::arr1(&[10.0, 20.0, 15.0]);
        let current_skills = ndarray::arr1(&[0.0, 0.0, 0.0]);
        let target_skills = ndarray::arr1(&[1.0, 1.0, 1.0]);
        let community_need = ndarray::arr1(&[0.8, 0.9, 0.7]);
        
        let result = optimizer.create_learning_path(
            prerequisites,
            learning_time,
            current_skills,
            target_skills,
            community_need,
            None,
        );
        
        assert!(result.is_ok());
        let learning_path = result.unwrap();
        assert_eq!(learning_path.len(), 3);
    }
}
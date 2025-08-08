//! Integration tests for the optimization framework

use optimization_core::engine::OptimizationEngine;
use optimization_core::problem::{VolunteerMatchingObjective, ResourceAllocationObjective};
use optimization_core::error::OptimizationError;
use ndarray::Array1;

#[test]
fn test_volunteer_matching_integration() -> Result<(), OptimizationError> {
    // Create optimization engine
    let mut engine = OptimizationEngine::new();
    
    // Define a simple volunteer matching problem
    let skill_matrix = ndarray::arr2(&[
        [0.8, 0.2],
        [0.3, 0.9],
    ]);
    
    let availability = ndarray::arr1(&[1.0, 1.0]);
    let task_priority = ndarray::arr1(&[1.0, 1.0]);
    let community_impact = ndarray::arr1(&[0.5, 0.7]);
    
    // Create optimization problem
    let problem = engine.create_volunteer_matching_problem(
        skill_matrix,
        availability,
        task_priority,
        community_impact,
    );
    
    // Solve the problem
    let result = engine.solve(&problem, Some("integration_test_volunteer".to_string()))?;
    
    // Verify the result
    assert!(result.solution.len() > 0);
    assert!(!result.solution.iter().all(|&x| x == 0.0)); // Should not be all zeros
    
    Ok(())
}

#[test]
fn test_resource_allocation_integration() -> Result<(), OptimizationError> {
    // Create optimization engine
    let mut engine = OptimizationEngine::new();
    
    // Define a simple resource allocation problem
    let demand = ndarray::arr1(&[100.0, 150.0]);
    let impact = ndarray::arr1(&[0.8, 0.9]);
    let total_resources = 200.0;
    let community_votes = ndarray::arr1(&[50.0, 75.0]);
    
    // Create optimization problem
    let problem = engine.create_resource_allocation_problem(
        demand,
        impact,
        total_resources,
        community_votes,
    );
    
    // Solve the problem
    let result = engine.solve(&problem, Some("integration_test_resource".to_string()))?;
    
    // Verify the result
    assert_eq!(result.solution.len(), 2);
    
    // Check that total allocation doesn't exceed resources
    let total_allocation: f64 = result.solution.sum();
    assert!(total_allocation <= total_resources);
    
    Ok(())
}

#[test]
fn test_cache_functionality() -> Result<(), OptimizationError> {
    // Create optimization engine
    let mut engine = OptimizationEngine::new();
    
    // Solve a problem
    let skill_matrix = ndarray::arr2(&[[0.8, 0.2], [0.3, 0.9]]);
    let availability = ndarray::arr1(&[1.0, 1.0]);
    let task_priority = ndarray::arr1(&[1.0, 1.0]);
    let community_impact = ndarray::arr1(&[0.5, 0.7]);
    
    let problem = engine.create_volunteer_matching_problem(
        skill_matrix,
        availability,
        task_priority,
        community_impact,
    );
    
    let result1 = engine.solve(&problem, Some("cache_test".to_string()))?;
    let (cache_size, max_cache) = engine.cache_stats();
    assert_eq!(cache_size, 1);
    
    // Solve the same problem again (should use cache)
    let result2 = engine.solve(&problem, Some("cache_test".to_string()))?;
    
    // Results should be the same (from cache)
    assert_eq!(result1.solution, result2.solution);
    
    Ok(())
}
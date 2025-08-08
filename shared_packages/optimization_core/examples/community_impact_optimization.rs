//! Comprehensive example demonstrating community impact optimization
//!
//! This example shows how to use the optimization framework to solve real-world
//! cooperative problems like volunteer matching, resource allocation, and route planning.

use optimization_core::engine::OptimizationEngine;
use optimization_core::problem::{VolunteerMatchingObjective, ResourceAllocationObjective};
use optimization_core::error::OptimizationError;
use optimization_core::cooperative_values::CooperativeValues;
use ndarray::Array1;

/// Example: Volunteer-Task Matching
fn volunteer_task_matching_example() -> Result<(), OptimizationError> {
    println!("=== Volunteer-Task Matching Example ===");
    
    // Create optimization engine
    let mut engine = OptimizationEngine::new();
    
    // Define volunteer skills (2 volunteers, 3 tasks)
    let skill_matrix = ndarray::arr2(&[
        [0.9, 0.2, 0.7], // Volunteer 1 skills
        [0.3, 0.8, 0.6], // Volunteer 2 skills
    ]);
    
    // Volunteer availability
    let availability = ndarray::arr1(&[1.0, 1.0]);
    
    // Task priorities
    let task_priority = ndarray::arr1(&[1.0, 1.0, 1.0]);
    
    // Community impact of tasks
    let community_impact = ndarray::arr1(&[0.8, 0.9, 0.7]);
    
    // Create optimization problem
    let problem = engine.create_volunteer_matching_problem(
        skill_matrix,
        availability,
        task_priority,
        community_impact,
    );
    
    // Solve the problem
    let result = engine.solve(&problem, Some("volunteer_matching_example".to_string()))?;
    
    println!("Volunteer-Task Assignment Solution:");
    println!("Cost: {:.2}", result.cost);
    println!("Iterations: {}", result.iterations);
    println!("Converged: {}", result.converged);
    
    // Reshape solution to assignment matrix
    let num_volunteers = 2;
    let num_tasks = 3;
    let assignment = ndarray::Array2::from_shape_vec(
        (num_volunteers, num_tasks),
        result.solution.to_vec(),
    )?;
    
    println!("Assignment Matrix:");
    for (i, row) in assignment.rows().into_iter().enumerate() {
        println!("  Volunteer {}: {:?}", i + 1, row.to_vec());
    }
    
    Ok(())
}

/// Example: Resource Allocation
fn resource_allocation_example() -> Result<(), OptimizationError> {
    println!("\n=== Resource Allocation Example ===");
    
    // Create optimization engine
    let mut engine = OptimizationEngine::new();
    
    // Define causes/projects (3 causes)
    let demand = ndarray::arr1(&[100.0, 200.0, 150.0]); // Resource demand
    let impact = ndarray::arr1(&[0.8, 0.9, 0.7]); // Expected impact
    let total_resources = 300.0; // Total available resources
    let community_votes = ndarray::arr1(&[50.0, 80.0, 30.0]); // Community votes
    
    // Create optimization problem
    let problem = engine.create_resource_allocation_problem(
        demand,
        impact,
        total_resources,
        community_votes,
    );
    
    // Solve the problem
    let result = engine.solve(&problem, Some("resource_allocation_example".to_string()))?;
    
    println!("Resource Allocation Solution:");
    println!("Cost: {:.2}", result.cost);
    println!("Iterations: {}", result.iterations);
    println!("Converged: {}", result.converged);
    
    println!("Resource Allocation:");
    for (i, &allocation) in result.solution.iter().enumerate() {
        println!("  Cause {}: {:.2} resources", i + 1, allocation);
    }
    
    // Check total allocation
    let total_allocation: f64 = result.solution.sum();
    println!("Total Allocation: {:.2} (Available: {:.2})", total_allocation, total_resources);
    
    Ok(())
}

/// Example: Cooperative Values Integration
fn cooperative_values_example() -> Result<(), OptimizationError> {
    println!("\n=== Cooperative Values Integration Example ===");
    
    // Create optimization engine with cooperative values
    let cooperative_values = CooperativeValues {
        prioritize_community_benefit: true,
        community_impact_weight: 2.0, // Higher weight for community impact
        show_transparency: true,
        enable_community_validation: true,
        fairness_threshold: 0.7,
        max_inequality: 0.4,
    };
    
    let config = optimization_core::engine::EngineConfig {
        cooperative_values,
        ..Default::default()
    };
    
    let mut engine = OptimizationEngine::with_config(config);
    
    // Define a simple resource allocation problem
    let demand = ndarray::arr1(&[50.0, 100.0, 75.0]);
    let impact = ndarray::arr1(&[0.7, 0.9, 0.8]);
    let total_resources = 150.0;
    let community_votes = ndarray::arr1(&[30.0, 60.0, 20.0]);
    
    // Create optimization problem
    let problem = engine.create_resource_allocation_problem(
        demand,
        impact,
        total_resources,
        community_votes,
    );
    
    // Solve the problem
    let result = engine.solve(&problem, Some("cooperative_values_example".to_string()))?;
    
    println!("Cooperative Values-Aware Solution:");
    println!("Cost: {:.2}", result.cost);
    println!("Iterations: {}", result.iterations);
    println!("Converged: {}", result.converged);
    
    println!("Resource Allocation:");
    for (i, &allocation) in result.solution.iter().enumerate() {
        println!("  Project {}: {:.2} resources", i + 1, allocation);
    }
    
    // Show transparency information
    let explorer = optimization_core::cooperative_values::ImpactExplorer::new(
        engine.config.cooperative_values.clone()
    );
    
    let explanation = explorer.generate_transparent_explanation(
        "resource allocation",
        "community project funding"
    );
    
    println!("\nTransparent Explanation:");
    println!("{}", explanation);
    
    Ok(())
}

/// Example: Progressive Optimization for Web/WASM
#[cfg(target_arch = "wasm32")]
async fn progressive_optimization_example() -> Result<(), OptimizationError> {
    use optimization_core::progressive::ProgressiveOptimizer;
    
    println!("\n=== Progressive Optimization Example ===");
    
    // Create progressive optimizer
    let optimizer = ProgressiveOptimizer::new();
    
    // Define a volunteer matching problem
    let skill_matrix = ndarray::arr2(&[
        [0.8, 0.3, 0.6],
        [0.4, 0.9, 0.5],
        [0.7, 0.2, 0.8],
    ]);
    
    let availability = ndarray::arr1(&[1.0, 1.0, 1.0]);
    let task_priority = ndarray::arr1(&[1.0, 1.0, 1.0]);
    let community_impact = ndarray::arr1(&[0.7, 0.8, 0.6]);
    
    let initial_param = Array1::zeros(9); // 3 volunteers * 3 tasks
    let objective = Box::new(VolunteerMatchingObjective::new(
        skill_matrix,
        availability,
        task_priority,
        community_impact,
    ));
    
    let problem = optimization_core::problem::OptimizationProblem::new(objective, initial_param);
    
    // Solve with progressive optimization
    let result = optimizer.solve_progressive(&problem)?;
    
    println!("Progressive Optimization Result:");
    println!("Cost: {:.2}", result.best_result.cost);
    println!("Used Fallback: {}", result.used_fallback);
    println!("Computation Time: {:?}", result.computation_time);
    
    Ok(())
}

/// Example: Progressive Optimization for Web/WASM (synchronous version for non-WASM)
#[cfg(not(target_arch = "wasm32"))]
fn progressive_optimization_example() -> Result<(), OptimizationError> {
    use optimization_core::progressive::ProgressiveOptimizer;
    
    println!("\n=== Progressive Optimization Example ===");
    
    // Create progressive optimizer
    let optimizer = ProgressiveOptimizer::new();
    
    // Define a volunteer matching problem
    let skill_matrix = ndarray::arr2(&[
        [0.8, 0.3, 0.6],
        [0.4, 0.9, 0.5],
        [0.7, 0.2, 0.8],
    ]);
    
    let availability = ndarray::arr1(&[1.0, 1.0, 1.0]);
    let task_priority = ndarray::arr1(&[1.0, 1.0, 1.0]);
    let community_impact = ndarray::arr1(&[0.7, 0.8, 0.6]);
    
    let initial_param = Array1::zeros(9); // 3 volunteers * 3 tasks
    let objective = Box::new(VolunteerMatchingObjective::new(
        skill_matrix,
        availability,
        task_priority,
        community_impact,
    ));
    
    let problem = optimization_core::problem::OptimizationProblem::new(objective, initial_param);
    
    // Solve with progressive optimization
    let result = optimizer.solve_progressive(&problem)?;
    
    println!("Progressive Optimization Result:");
    println!("Cost: {:.2}", result.best_result.cost);
    println!("Used Fallback: {}", result.used_fallback);
    println!("Computation Time: {:?}", result.computation_time);
    
    Ok(())
}

fn main() -> Result<(), OptimizationError> {
    println!("Community Impact Optimization Examples");
    println!("=====================================");
    
    // Run all examples
    volunteer_task_matching_example()?;
    resource_allocation_example()?;
    cooperative_values_example()?;
    progressive_optimization_example()?;
    
    println!("\nAll examples completed successfully!");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_volunteer_matching_example() {
        assert!(volunteer_task_matching_example().is_ok());
    }
    
    #[test]
    fn test_resource_allocation_example() {
        assert!(resource_allocation_example().is_ok());
    }
    
    #[test]
    fn test_cooperative_values_example() {
        assert!(cooperative_values_example().is_ok());
    }
    
    #[test]
    fn test_progressive_optimization_example() {
        assert!(progressive_optimization_example().is_ok());
    }
}
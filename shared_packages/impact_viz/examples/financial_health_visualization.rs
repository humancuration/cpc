//! Example: Financial Health Visualization
//!
//! This example demonstrates how to create financial health visualizations using the impact visualization framework.

use impact_viz::core::{ImpactVizCore, ImpactVisualization};
use impact_viz::financial::{FinancialHealthViz, AllocationScenario};
use cpay_core::ml::{FinancialData, ResourceData};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Financial Health Visualization Example ===\n");
    
    // Create the core visualization engine
    let core_viz = ImpactVizCore::new();
    let financial_viz = FinancialHealthViz::new(Box::new(core_viz));
    
    // Example 1: Community wellbeing visualization
    println!("1. Community Wellbeing Visualization");
    let financial_data = create_sample_financial_data();
    let wellbeing_viz = financial_viz.visualize_community_wellbeing(&financial_data);
    println!("   Generated visualization type: {:?}", wellbeing_viz.viz_type);
    println!("   Data size: {} characters\n", wellbeing_viz.data.json_data.len());
    
    // Example 2: Resource flows visualization
    println!("2. Resource Flows Visualization");
    let resource_data = create_sample_resource_data();
    let flows_viz = financial_viz.visualize_resource_flows(&resource_data);
    println!("   Generated visualization type: {:?}", flows_viz.viz_type);
    println!("   Data size: {} characters\n", flows_viz.data.json_data.len());
    
    // Example 3: Sustainability metrics visualization
    println!("3. Sustainability Metrics Visualization");
    let sustainability_viz = financial_viz.visualize_sustainability(&financial_data);
    println!("   Generated visualization type: {:?}", sustainability_viz.viz_type);
    println!("   Data size: {} characters\n", sustainability_viz.data.json_data.len());
    
    // Example 4: Allocation scenarios visualization
    println!("4. Allocation Scenarios Visualization");
    let scenarios = create_sample_scenarios();
    let scenarios_viz = financial_viz.visualize_allocation_scenarios(&scenarios);
    println!("   Generated visualization type: {:?}", scenarios_viz.viz_type);
    println!("   Data size: {} characters\n", scenarios_viz.data.json_data.len());
    
    println!("\n=== Example Completed Successfully ===");
    Ok(())
}

/// Create sample financial data
fn create_sample_financial_data() -> FinancialData {
    let mut expense_patterns = HashMap::new();
    expense_patterns.insert("Operational".to_string(), vec![5000.0, 5200.0, 5100.0, 5300.0]);
    expense_patterns.insert("Community Programs".to_string(), vec![3000.0, 3200.0, 3100.0, 3300.0]);
    expense_patterns.insert("Infrastructure".to_string(), vec![2000.0, 2100.0, 2050.0, 2150.0]);
    
    FinancialData {
        revenue_trends: vec![12000.0, 12500.0, 12300.0, 12800.0],
        expense_patterns,
        reserve_levels: vec![50000.0, 51000.0, 50500.0, 52000.0],
        contribution_rates: vec![0.75, 0.78, 0.76, 0.80],
        investment_returns: vec![0.05, 0.06, 0.055, 0.065],
        debt_levels: vec![15000.0, 14500.0, 14000.0, 13500.0],
    }
}

/// Create sample resource data
fn create_sample_resource_data() -> ResourceData {
    let mut available_resources = HashMap::new();
    available_resources.insert("Funding".to_string(), 75000.0);
    available_resources.insert("Volunteer Hours".to_string(), 12000.0);
    available_resources.insert("Equipment".to_string(), 30000.0);
    
    let mut demand_forecasts = HashMap::new();
    demand_forecasts.insert("Education Programs".to_string(), 30000.0);
    demand_forecasts.insert("Healthcare Initiatives".to_string(), 25000.0);
    demand_forecasts.insert("Environmental Projects".to_string(), 20000.0);
    
    let mut impact_potential = HashMap::new();
    impact_potential.insert("Education Programs".to_string(), 8.5);
    impact_potential.insert("Healthcare Initiatives".to_string(), 9.2);
    impact_potential.insert("Environmental Projects".to_string(), 7.8);
    
    let mut constraints = HashMap::new();
    constraints.insert("Funding".to_string(), 100000.0);
    constraints.insert("Volunteer Hours".to_string(), 15000.0);
    
    let mut priority_scores = HashMap::new();
    priority_scores.insert("Education Programs".to_string(), 0.9);
    priority_scores.insert("Healthcare Initiatives".to_string(), 0.95);
    priority_scores.insert("Environmental Projects".to_string(), 0.8);
    
    ResourceData {
        available_resources,
        demand_forecasts,
        impact_potential,
        constraints,
        priority_scores,
    }
}

/// Create sample allocation scenarios
fn create_sample_scenarios() -> Vec<AllocationScenario> {
    vec![
        AllocationScenario {
            name: "Community Focus".to_string(),
            description: "Prioritize community programs and education".to_string(),
            allocations: vec![], // In a real implementation, this would contain actual allocations
            projected_impact: 8.7,
            sustainability_score: 0.85,
        },
        AllocationScenario {
            name: "Growth Focus".to_string(),
            description: "Invest in infrastructure and expansion".to_string(),
            allocations: vec![], // In a real implementation, this would contain actual allocations
            projected_impact: 7.9,
            sustainability_score: 0.92,
        },
        AllocationScenario {
            name: "Balanced Approach".to_string(),
            description: "Equal distribution across all priorities".to_string(),
            allocations: vec![], // In a real implementation, this would contain actual allocations
            projected_impact: 8.3,
            sustainability_score: 0.88,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_financial_health_visualization_example() {
        // This test ensures the example code compiles and runs without panicking
        assert!(main().is_ok());
    }
    
    #[test]
    fn test_create_sample_data() {
        let financial_data = create_sample_financial_data();
        assert!(!financial_data.revenue_trends.is_empty());
        assert!(!financial_data.expense_patterns.is_empty());
        
        let resource_data = create_sample_resource_data();
        assert!(!resource_data.available_resources.is_empty());
        assert!(!resource_data.demand_forecasts.is_empty());
        
        let scenarios = create_sample_scenarios();
        assert_eq!(scenarios.len(), 3);
    }
}
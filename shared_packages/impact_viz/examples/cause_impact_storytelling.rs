//! Example: Cause Impact Storytelling
//!
//! This example demonstrates how to create cause impact visualizations using the impact visualization framework.

use impact_viz::core::{ImpactVizCore, ImpactVisualization};
use impact_viz::cause::CauseImpactStorytelling;
use cause_management::ml::{CauseData, ImpactMeasurement, ResourceAllocationRecord, 
                           EngagementMetric, OutcomeMeasurement};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Cause Impact Storytelling Example ===\n");
    
    // Create the core visualization engine
    let core_viz = ImpactVizCore::new();
    let cause_viz = CauseImpactStorytelling::new(Box::new(core_viz));
    
    // Example 1: Cause effectiveness visualization
    println!("1. Cause Effectiveness Visualization");
    let cause_data = create_sample_cause_data();
    let effectiveness_viz = cause_viz.visualize_cause_effectiveness(&cause_data);
    println!("   Generated visualization type: {:?}", effectiveness_viz.viz_type);
    println!("   Data size: {} characters\n", effectiveness_viz.data.json_data.len());
    
    // Example 2: Resource impact visualization
    println!("2. Resource Impact Visualization");
    let resource_viz = cause_viz.visualize_resource_impact(&cause_data);
    println!("   Generated visualization type: {:?}", resource_viz.viz_type);
    println!("   Data size: {} characters\n", resource_viz.data.json_data.len());
    
    // Example 3: Prediction confidence visualization
    println!("3. Prediction Confidence Visualization");
    let confidence_viz = cause_viz.visualize_prediction_confidence(&cause_data);
    println!("   Generated visualization type: {:?}", confidence_viz.viz_type);
    println!("   Data size: {} characters\n", confidence_viz.data.json_data.len());
    
    // Example 4: Cause comparison visualization
    println!("4. Cause Comparison Visualization");
    let causes = vec![cause_data.clone(), create_sample_cause_data_2()];
    let comparison_viz = cause_viz.visualize_cause_comparison(&causes);
    println!("   Generated visualization type: {:?}", comparison_viz.viz_type);
    println!("   Data size: {} characters\n", comparison_viz.data.json_data.len());
    
    println!("\n=== Example Completed Successfully ===");
    Ok(())
}

/// Create sample cause data
fn create_sample_cause_data() -> CauseData {
    let historical_impact = vec![
        ImpactMeasurement {
            date: chrono::Utc::now() - chrono::Duration::days(90),
            impact_score: 0.75,
            people_affected: 1500,
            geographic_scope: "Local Community".to_string(),
        },
        ImpactMeasurement {
            date: chrono::Utc::now() - chrono::Duration::days(60),
            impact_score: 0.82,
            people_affected: 1800,
            geographic_scope: "Local Community".to_string(),
        },
        ImpactMeasurement {
            date: chrono::Utc::now() - chrono::Duration::days(30),
            impact_score: 0.88,
            people_affected: 2100,
            geographic_scope: "Local Community".to_string(),
        },
    ];
    
    let resource_allocation = vec![
        ResourceAllocationRecord {
            date: chrono::Utc::now() - chrono::Duration::days(90),
            resource_type: "Funding".to_string(),
            amount: 15000.0,
            source: "Community Fund".to_string(),
        },
        ResourceAllocationRecord {
            date: chrono::Utc::now() - chrono::Duration::days(60),
            resource_type: "Volunteer Hours".to_string(),
            amount: 800.0,
            source: "Volunteer Program".to_string(),
        },
        ResourceAllocationRecord {
            date: chrono::Utc::now() - chrono::Duration::days(30),
            resource_type: "Equipment".to_string(),
            amount: 5000.0,
            source: "Equipment Grant".to_string(),
        },
    ];
    
    let engagement_metrics = vec![
        EngagementMetric {
            date: chrono::Utc::now() - chrono::Duration::days(90),
            engagement_type: "Volunteer Participation".to_string(),
            participants: 45,
            quality_score: 0.85,
        },
        EngagementMetric {
            date: chrono::Utc::now() - chrono::Duration::days(60),
            engagement_type: "Community Feedback".to_string(),
            participants: 120,
            quality_score: 0.90,
        },
        EngagementMetric {
            date: chrono::Utc::now() - chrono::Duration::days(30),
            engagement_type: "Media Coverage".to_string(),
            participants: 8,
            quality_score: 0.95,
        },
    ];
    
    let outcomes = vec![
        OutcomeMeasurement {
            date: chrono::Utc::now() - chrono::Duration::days(90),
            outcome_type: "Education Access".to_string(),
            value: 75.0,
            method: "Standardized Testing".to_string(),
        },
        OutcomeMeasurement {
            date: chrono::Utc::now() - chrono::Duration::days(60),
            outcome_type: "Community Engagement".to_string(),
            value: 82.0,
            method: "Survey Results".to_string(),
        },
        OutcomeMeasurement {
            date: chrono::Utc::now() - chrono::Duration::days(30),
            outcome_type: "Sustainability".to_string(),
            value: 88.0,
            method: "Long-term Tracking".to_string(),
        },
    ];
    
    CauseData {
        id: "cause_001".to_string(),
        name: "Community Education Program".to_string(),
        category: "Education".to_string(),
        historical_impact,
        resource_allocation,
        engagement_metrics,
        outcomes,
    }
}

/// Create sample cause data 2
fn create_sample_cause_data_2() -> CauseData {
    let historical_impact = vec![
        ImpactMeasurement {
            date: chrono::Utc::now() - chrono::Duration::days(90),
            impact_score: 0.65,
            people_affected: 2000,
            geographic_scope: "Regional Area".to_string(),
        },
        ImpactMeasurement {
            date: chrono::Utc::now() - chrono::Duration::days(60),
            impact_score: 0.72,
            people_affected: 2400,
            geographic_scope: "Regional Area".to_string(),
        },
        ImpactMeasurement {
            date: chrono::Utc::now() - chrono::Duration::days(30),
            impact_score: 0.78,
            people_affected: 2800,
            geographic_scope: "Regional Area".to_string(),
        },
    ];
    
    let resource_allocation = vec![
        ResourceAllocationRecord {
            date: chrono::Utc::now() - chrono::Duration::days(90),
            resource_type: "Funding".to_string(),
            amount: 25000.0,
            source: "Grant Foundation".to_string(),
        },
        ResourceAllocationRecord {
            date: chrono::Utc::now() - chrono::Duration::days(60),
            resource_type: "Volunteer Hours".to_string(),
            amount: 1200.0,
            source: "Volunteer Program".to_string(),
        },
    ];
    
    let engagement_metrics = vec![
        EngagementMetric {
            date: chrono::Utc::now() - chrono::Duration::days(90),
            engagement_type: "Volunteer Participation".to_string(),
            participants: 65,
            quality_score: 0.75,
        },
        EngagementMetric {
            date: chrono::Utc::now() - chrono::Duration::days(60),
            engagement_type: "Community Feedback".to_string(),
            participants: 180,
            quality_score: 0.80,
        },
    ];
    
    let outcomes = vec![
        OutcomeMeasurement {
            date: chrono::Utc::now() - chrono::Duration::days(90),
            outcome_type: "Healthcare Access".to_string(),
            value: 65.0,
            method: "Medical Records Analysis".to_string(),
        },
        OutcomeMeasurement {
            date: chrono::Utc::now() - chrono::Duration::days(60),
            outcome_type: "Community Health".to_string(),
            value: 72.0,
            method: "Health Survey Results".to_string(),
        },
    ];
    
    CauseData {
        id: "cause_002".to_string(),
        name: "Healthcare Access Initiative".to_string(),
        category: "Healthcare".to_string(),
        historical_impact,
        resource_allocation,
        engagement_metrics,
        outcomes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cause_impact_storytelling_example() {
        // This test ensures the example code compiles and runs without panicking
        assert!(main().is_ok());
    }
    
    #[test]
    fn test_create_sample_data() {
        let cause_data = create_sample_cause_data();
        assert_eq!(cause_data.id, "cause_001");
        assert_eq!(cause_data.name, "Community Education Program");
        assert!(!cause_data.historical_impact.is_empty());
        assert!(!cause_data.resource_allocation.is_empty());
        assert!(!cause_data.engagement_metrics.is_empty());
        assert!(!cause_data.outcomes.is_empty());
        
        let cause_data_2 = create_sample_cause_data_2();
        assert_eq!(cause_data_2.id, "cause_002");
        assert_eq!(cause_data_2.name, "Healthcare Access Initiative");
    }
}
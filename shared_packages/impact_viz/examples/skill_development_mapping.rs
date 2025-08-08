//! Example: Skill Development Mapping
//!
//! This example demonstrates how to create skill development visualizations using the impact visualization framework.

use impact_viz::core::{ImpactVizCore, ImpactVisualization};
use impact_viz::skill::SkillDevelopmentViz;
use skill_development::ml::{LearnerProfile, CommunityData, LearningExperience};
use ml_core::models::LearningPathway;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Skill Development Mapping Example ===\n");
    
    // Create the core visualization engine
    let core_viz = ImpactVizCore::new();
    let skill_viz = SkillDevelopmentViz::new(Box::new(core_viz));
    
    // Example 1: Individual skill growth visualization
    println!("1. Individual Skill Growth Visualization");
    let profile = create_sample_learner_profile();
    let growth_viz = skill_viz.visualize_individual_growth(&profile);
    println!("   Generated visualization type: {:?}", growth_viz.viz_type);
    println!("   Data size: {} characters\n", growth_viz.data.json_data.len());
    
    // Example 2: Community skill landscape visualization
    println!("2. Community Skill Landscape Visualization");
    let community_data = create_sample_community_data();
    let landscape_viz = skill_viz.visualize_skill_landscape(&community_data);
    println!("   Generated visualization type: {:?}", landscape_viz.viz_type);
    println!("   Data size: {} characters\n", landscape_viz.data.json_data.len());
    
    // Example 3: Skill needs mapping visualization
    println!("3. Skill Needs Mapping Visualization");
    let needs_viz = skill_viz.visualize_skill_needs_mapping(&community_data);
    println!("   Generated visualization type: {:?}", needs_viz.viz_type);
    println!("   Data size: {} characters\n", needs_viz.data.json_data.len());
    
    // Example 4: Learning pathways visualization
    println!("4. Learning Pathways Visualization");
    let pathways = create_sample_learning_pathways();
    let pathways_viz = skill_viz.visualize_learning_pathways(&pathways);
    println!("   Generated visualization type: {:?}", pathways_viz.viz_type);
    println!("   Data size: {} characters\n", pathways_viz.data.json_data.len());
    
    println!("\n=== Example Completed Successfully ===");
    Ok(())
}

/// Create sample learner profile
fn create_sample_learner_profile() -> LearnerProfile {
    let mut current_skills = HashMap::new();
    current_skills.insert("Rust Programming".to_string(), 0.75);
    current_skills.insert("Data Analysis".to_string(), 0.65);
    current_skills.insert("Project Management".to_string(), 0.80);
    current_skills.insert("Public Speaking".to_string(), 0.55);
    
    let learning_history = vec![
        LearningExperience {
            skill: "Rust Programming".to_string(),
            time_taken: 120.0,
            satisfaction: 9,
            completion_date: chrono::Utc::now() - chrono::Duration::days(60),
        },
        LearningExperience {
            skill: "Data Analysis".to_string(),
            time_taken: 80.0,
            satisfaction: 8,
            completion_date: chrono::Utc::now() - chrono::Duration::days(30),
        },
    ];
    
    LearnerProfile {
        current_skills,
        learning_pace: 8,
        learning_styles: vec!["Visual".to_string(), "Hands-on".to_string()],
        available_time: 15.0,
        learning_goals: vec![
            "Master Systems Programming".to_string(),
            "Become Data Science Expert".to_string(),
            "Lead Community Projects".to_string(),
        ],
        learning_history,
    }
}

/// Create sample community data
fn create_sample_community_data() -> CommunityData {
    let mut skill_distribution = HashMap::new();
    skill_distribution.insert("Programming".to_string(), vec![0.2, 0.3, 0.25, 0.15, 0.1]);
    skill_distribution.insert("Design".to_string(), vec![0.15, 0.25, 0.3, 0.2, 0.1]);
    skill_distribution.insert("Education".to_string(), vec![0.3, 0.25, 0.2, 0.15, 0.1]);
    
    let mut projected_needs = HashMap::new();
    projected_needs.insert("Healthcare".to_string(), 0.8);
    projected_needs.insert("Technology".to_string(), 0.9);
    projected_needs.insert("Education".to_string(), 0.75);
    
    let mut learning_resources = HashMap::new();
    learning_resources.insert("Online Courses".to_string(), vec!["Coursera".to_string(), "edX".to_string()]);
    learning_resources.insert("Mentorship".to_string(), vec!["Peer Mentoring".to_string(), "Expert Guidance".to_string()]);
    
    let mut demographics = HashMap::new();
    demographics.insert("Age 18-30".to_string(), 0.4);
    demographics.insert("Age 31-50".to_string(), 0.35);
    demographics.insert("Age 51+".to_string(), 0.25);
    
    let mut historical_trends = HashMap::new();
    historical_trends.insert("Programming Skills".to_string(), vec![0.1, 0.15, 0.25, 0.35, 0.45]);
    historical_trends.insert("Leadership Skills".to_string(), vec![0.2, 0.25, 0.3, 0.35, 0.4]);
    
    CommunityData {
        skill_distribution,
        projected_needs,
        learning_resources,
        demographics,
        historical_trends,
    }
}

/// Create sample learning pathways
fn create_sample_learning_pathways() -> Vec<LearningPathway> {
    vec![
        LearningPathway {
            skills: vec!["Rust Programming".to_string(), "Systems Design".to_string(), "Performance Optimization".to_string()],
            estimated_time: 200.0,
            resources_needed: vec!["The Rust Programming Language Book".to_string(), "Online Compiler".to_string()],
        },
        LearningPathway {
            skills: vec!["Data Analysis".to_string(), "Machine Learning".to_string(), "Data Visualization".to_string()],
            estimated_time: 180.0,
            resources_needed: vec!["Python Course".to_string(), "Jupyter Notebooks".to_string(), "Kaggle Datasets".to_string()],
        },
        LearningPathway {
            skills: vec!["Project Management".to_string(), "Team Leadership".to_string(), "Community Organization".to_string()],
            estimated_time: 150.0,
            resources_needed: vec!["PMP Study Guide".to_string(), "Leadership Workshop".to_string()],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_skill_development_mapping_example() {
        // This test ensures the example code compiles and runs without panicking
        assert!(main().is_ok());
    }
    
    #[test]
    fn test_create_sample_data() {
        let profile = create_sample_learner_profile();
        assert!(!profile.current_skills.is_empty());
        assert_eq!(profile.learning_pace, 8);
        
        let community_data = create_sample_community_data();
        assert!(!community_data.skill_distribution.is_empty());
        assert!(!community_data.projected_needs.is_empty());
        
        let pathways = create_sample_learning_pathways();
        assert_eq!(pathways.len(), 3);
    }
}
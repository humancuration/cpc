//! Example: Volunteer Impact Dashboard
//!
//! This example demonstrates how to create a volunteer impact dashboard using the impact visualization framework.

use impact_viz::core::{ImpactVizCore, ImpactVisualization};
use impact_viz::volunteer::VolunteerImpactDashboard;
use volunteer_coordination::ml::{VolunteerEngagementData, VolunteerProfile, VolunteerActivity};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Volunteer Impact Dashboard Example ===\n");
    
    // Create the core visualization engine
    let core_viz = ImpactVizCore::new();
    let dashboard = VolunteerImpactDashboard::new(Box::new(core_viz));
    
    // Example 1: Individual volunteer impact
    println!("1. Individual Volunteer Impact Visualization");
    let engagement_data = create_sample_engagement_data();
    let individual_viz = dashboard.visualize_individual_impact(&engagement_data);
    println!("   Generated visualization type: {:?}", individual_viz.viz_type);
    println!("   Data size: {} characters\n", individual_viz.data.json_data.len());
    
    // Example 2: Collective volunteer impact
    println!("2. Collective Volunteer Impact Visualization");
    let activities = create_sample_activities();
    let collective_viz = dashboard.visualize_collective_impact(&activities);
    println!("   Generated visualization type: {:?}", collective_viz.viz_type);
    println!("   Data size: {} characters\n", collective_viz.data.json_data.len());
    
    // Example 3: Skill development progression
    println!("3. Skill Development Progression Visualization");
    let profile = create_sample_profile();
    let skill_viz = dashboard.visualize_skill_progression(&profile);
    println!("   Generated visualization type: {:?}", skill_viz.viz_type);
    println!("   Data size: {} characters\n", skill_viz.data.json_data.len());
    
    // Example 4: Impact stories
    println!("4. Community Impact Stories");
    let stories = dashboard.create_impact_stories(&activities);
    println!("   Generated {} impact stories", stories.len());
    if !stories.is_empty() {
        println!("   First story: {}", stories[0].title);
    }
    
    println!("\n=== Example Completed Successfully ===");
    Ok(())
}

/// Create sample volunteer engagement data
fn create_sample_engagement_data() -> VolunteerEngagementData {
    let mut skill_progress = HashMap::new();
    skill_progress.insert("Communication".to_string(), 0.8);
    skill_progress.insert("Leadership".to_string(), 0.7);
    skill_progress.insert("Organization".to_string(), 0.9);
    
    VolunteerEngagementData {
        hours_per_week: 15.0,
        completion_rate: 0.95,
        feedback_scores: vec![4.8, 4.9, 4.7, 5.0],
        skill_progress,
        social_connections: 12,
        tenure_months: 18.0,
    }
}

/// Create sample volunteer activities
fn create_sample_activities() -> Vec<VolunteerActivity> {
    vec![
        VolunteerActivity {
            id: "activity_001".to_string(),
            activity_type: "Community Garden Maintenance".to_string(),
            date: Utc::now() - Duration::days(30),
            duration: Duration::hours(4),
            participants: 25,
            measured_impact: 8.5,
            feedback: vec![
                "Great teamwork!".to_string(),
                "Garden looks amazing!".to_string(),
            ],
        },
        VolunteerActivity {
            id: "activity_002".to_string(),
            activity_type: "Youth Mentorship Program".to_string(),
            date: Utc::now() - Duration::days(15),
            duration: Duration::hours(3),
            participants: 18,
            measured_impact: 9.2,
            feedback: vec![
                "Kids were so engaged!".to_string(),
                "Wonderful mentors!".to_string(),
            ],
        },
        VolunteerActivity {
            id: "activity_003".to_string(),
            activity_type: "Senior Center Visit".to_string(),
            date: Utc::now() - Duration::days(7),
            duration: Duration::hours(2),
            participants: 12,
            measured_impact: 7.8,
            feedback: vec![
                "Seniors were delighted!".to_string(),
                "Heartwarming experience!".to_string(),
            ],
        },
    ]
}

/// Create sample volunteer profile
fn create_sample_profile() -> VolunteerProfile {
    let mut skills = HashMap::new();
    skills.insert("Gardening".to_string(), 0.85);
    skills.insert("Teaching".to_string(), 0.75);
    skills.insert("Event Planning".to_string(), 0.70);
    skills.insert("Public Speaking".to_string(), 0.65);
    
    VolunteerProfile {
        skills,
        interests: vec![
            "Community Development".to_string(),
            "Youth Education".to_string(),
            "Environmental Sustainability".to_string(),
        ],
        preferred_activities: vec![
            "Outdoor Work".to_string(),
            "Mentoring".to_string(),
            "Organizing Events".to_string(),
        ],
        availability: {
            let mut availability = HashMap::new();
            availability.insert("Weekends".to_string(), true);
            availability.insert("Evenings".to_string(), true);
            availability.insert("Weekdays".to_string(), false);
            availability
        },
        learning_goals: vec![
            "Advanced Project Management".to_string(),
            "Nonprofit Leadership".to_string(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_volunteer_impact_dashboard_example() {
        // This test ensures the example code compiles and runs without panicking
        assert!(main().is_ok());
    }
    
    #[test]
    fn test_create_sample_data() {
        let engagement_data = create_sample_engagement_data();
        assert_eq!(engagement_data.hours_per_week, 15.0);
        assert_eq!(engagement_data.completion_rate, 0.95);
        
        let activities = create_sample_activities();
        assert_eq!(activities.len(), 3);
        
        let profile = create_sample_profile();
        assert!(!profile.skills.is_empty());
    }
}
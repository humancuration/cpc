//! Main entry point for the volunteer impact tracker
//!
//! This is a demonstration and testing entry point for the volunteer impact tracker crate.

use volunteer_impact_tracker::{
    VolunteerImpactTracker, 
    ImpactAnalyticsDashboard,
    FeedbackCollector,
    ImprovementEngine,
    EcosystemIntegrator
};
use consent_manager::domain::consent::DataSharingLevel;
use impact_viz::core::VisualizationType;
use skill_development::ml::CommunityData;
use std::collections::HashMap;

fn main() {
    println!("Volunteer Impact Tracker Demo");
    
    // Create a new volunteer impact tracker
    let mut tracker = VolunteerImpactTracker::new(DataSharingLevel::Standard);
    
    // Simulate tracking some visualization engagement
    let _ = tracker.track_visualization_engagement(
        "volunteer_123",
        "individual_impact_dashboard",
        VisualizationType::Narrative,
        180.5, // 3 minutes interaction time
        25,    // 25 interactions
        0.85,  // Quality score
    );
    
    let _ = tracker.track_visualization_engagement(
        "volunteer_456",
        "collective_impact_visualization",
        VisualizationType::Comparative,
        120.0, // 2 minutes interaction time
        15,    // 15 interactions
        0.75,  // Quality score
    );
    
    // Simulate tracking retention correlation
    let _ = tracker.track_retention_correlation(
        "volunteer_123",
        vec!["individual_impact_dashboard".to_string(), "skill_progression".to_string()],
        true,   // Retained
        Some(12.5), // 12.5 months retention
        Some(8),    // Satisfaction rating 8/10
    );
    
    let _ = tracker.track_retention_correlation(
        "volunteer_789",
        vec![], // No visualization usage
        false,  // Not retained
        Some(2.0), // 2 months retention
        Some(4),   // Satisfaction rating 4/10
    );
    
    // Simulate tracking task completion
    let _ = tracker.track_task_completion(
        "volunteer_123",
        "community_cleanup_001",
        Some("individual_impact_dashboard".to_string()),
        true,   // Completed
        Some(9), // Quality rating 9/10
        Some(3.5), // 3.5 hours to completion
    );
    
    // Simulate recording feedback
    let _ = tracker.record_feedback(
        "volunteer_123",
        "individual_impact_dashboard",
        5, // 5-star rating
        Some("This visualization really helped me understand my impact!".to_string()),
        true, // Helpful
        Some("Made me want to continue volunteering".to_string()), // Decision impact
    );
    
    // Create analytics dashboard
    let dashboard = ImpactAnalyticsDashboard::new(tracker.get_metrics().clone());
    
    // Create mock community data
    let community_data = CommunityData {
        skill_distribution: HashMap::new(),
        projected_needs: HashMap::new(),
        learning_resources: HashMap::new(),
        demographics: HashMap::new(),
        historical_trends: HashMap::new(),
    };
    
    // Generate dashboard summary
    let summary = dashboard.generate_summary(&community_data);
    
    println!("Dashboard Summary:");
    println!("  Total Views: {}", summary.engagement.total_views);
    println!("  Avg Interaction Time: {:.2} seconds", summary.engagement.avg_interaction_time);
    println!("  Quality Score: {:.2}", summary.engagement.quality_score);
    println!("  Retention Rate with Viz: {:.2}%", summary.volunteer_effectiveness.retention_rate_with_viz * 100.0);
    println!("  Task Completion Rate with Viz: {:.2}%", summary.volunteer_effectiveness.completion_rate_with_viz * 100.0);
    println!("  Avg Feedback Rating: {:.2}", summary.feedback.avg_rating);
    println!("  Helpful Percentage: {:.2}%", summary.feedback.helpful_percentage);
    println!("  Recommendations: {}", summary.recommendations.len());
    
    // Create feedback collector
    let mut feedback_collector = FeedbackCollector::new();
    
    // Process feedback for a visualization
    if let Ok(result) = feedback_collector.process_feedback_for_viz("individual_impact_dashboard") {
        println!("Feedback Processing Result for 'individual_impact_dashboard':");
        println!("  Helpfulness Score: {:.2}", result.helpfulness_score);
        println!("  Themes Found: {}", result.themes.len());
        println!("  Insights Generated: {}", result.insights.len());
    }
    
    // Create improvement engine
    let mut improvement_engine = ImprovementEngine::new();
    
    // Generate improvement suggestions
    let suggestions = improvement_engine.generate_improvement_suggestions(
        &summary, 
        &vec![] // Empty feedback results for demo
    );
    
    println!("Improvement Suggestions: {}", suggestions.len());
    for (i, suggestion) in suggestions.iter().enumerate() {
        println!("  {}. {}", i + 1, suggestion.description);
    }
    
    // Create ecosystem integrator
    let integrator = EcosystemIntegrator::new();
    
    // Connect a component
    let component = volunteer_impact_tracker::integration::EcosystemComponent {
        id: "learning_platform".to_string(),
        name: "Learning Platform".to_string(),
        component_type: volunteer_impact_tracker::integration::ComponentType::LearningPlatform,
        status: volunteer_impact_tracker::integration::ConnectionStatus::Connected,
        last_sync: chrono::Utc::now(),
    };
    
    if let Ok(_) = integrator.connect_component(component) {
        println!("Connected to Learning Platform component");
    }
    
    println!("Demo completed successfully!");
}
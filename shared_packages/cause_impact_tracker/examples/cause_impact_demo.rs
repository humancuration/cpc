//! Cause Impact Tracker Demo
//!
//! This example demonstrates how to use the Cause Impact Tracker to measure
//! the effectiveness of cause impact visualizations.

use cause_impact_tracker::{
    CauseImpactTracker,
    ImpactAnalyticsDashboard,
    FeedbackCollector,
    tracker::{VisualizationType, ValidationType},
    analytics::DashboardSummary,
};
use consent_manager::domain::consent::DataSharingLevel;
use std::collections::HashMap;
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Cause Impact Tracker Demo ===\n");
    
    // Create a new cause impact tracker with standard consent level
    let mut tracker = CauseImpactTracker::new(DataSharingLevel::Standard);
    
    // Simulate some user interactions with cause visualizations
    println!("1. Tracking visualization engagement...");
    
    tracker.track_visualization_engagement(
        "user_123",
        "cause_impact_storytelling_v1",
        VisualizationType::Narrative,
        180.5, // 3 minutes interaction time
        25,    // 25 interactions
        0.85,  // Quality score
        Some(0.92), // Decision confidence
    )?;
    
    tracker.track_visualization_engagement(
        "user_456",
        "resource_allocation_tracker",
        VisualizationType::Comparative,
        120.0, // 2 minutes interaction time
        18,    // 18 interactions
        0.78,  // Quality score
        Some(0.85), // Decision confidence
    )?;
    
    println!("   ✓ Tracked 2 visualization engagements\n");
    
    // Track cause engagement correlation
    println!("2. Tracking cause engagement correlation...");
    
    tracker.track_engagement_correlation(
        "user_123",
        vec!["cause_impact_storytelling_v1".to_string()],
        true,   // Engaged
        Some(4.5), // 4.5 months engaged
        Some(9),   // Satisfaction rating
        Some(75.50), // Contribution amount
    )?;
    
    tracker.track_engagement_correlation(
        "user_789",
        vec![], // No visualization usage
        false,  // Not engaged
        None,   // N/A
        None,   // N/A
        None,   // N/A
    )?;
    
    println!("   ✓ Tracked cause engagement correlations\n");
    
    // Track contribution effectiveness
    println!("3. Tracking contribution effectiveness...");
    
    tracker.track_contribution_effectiveness(
        "user_123",
        Some("cause_impact_storytelling_v1".to_string()),
        "Increased monthly contribution",
        Some(9), // Quality rating
        Some(0.85), // Impact score
    )?;
    
    println!("   ✓ Tracked contribution effectiveness\n");
    
    // Record community validation
    println!("4. Recording community validation...");
    
    tracker.record_community_validation(
        "user_456",
        "cause_impact_storytelling_v1",
        ValidationType::Endorsement,
        "This visualization really helped me understand how my contributions make a difference!",
        Some("Education cause".to_string()),
    )?;
    
    tracker.record_community_validation(
        "user_789",
        "resource_allocation_tracker",
        ValidationType::Suggestion,
        "Would be helpful to see historical data going back further",
        Some("Healthcare cause".to_string()),
    )?;
    
    println!("   ✓ Recorded community validation\n");
    
    // Collect feedback
    println!("5. Collecting feedback...");
    
    let mut feedback_collector = FeedbackCollector::new(DataSharingLevel::Standard);
    
    feedback_collector.collect_quick_feedback(
        "user_123",
        "cause_impact_storytelling_v1",
        true, // Helpful
    )?;
    
    feedback_collector.collect_detailed_feedback(
        "user_456",
        "resource_allocation_tracker",
        4, // 4-star rating
        Some("Very useful for understanding where my donations go".to_string()),
        true, // Helpful
        Some("Decided to increase my contribution amount".to_string()),
        Some(8), // Understanding improvement
        Some(9), // Confidence improvement
    )?;
    
    println!("   ✓ Collected feedback\n");
    
    // Generate dashboard summary
    println!("6. Generating dashboard summary...");
    
    let metrics = tracker.get_metrics();
    let dashboard = ImpactAnalyticsDashboard::new(metrics.clone());
    
    // Create mock community data
    let community_data = skill_development::ml::CommunityData {
        member_count: 1250,
        skill_distribution: HashMap::new(),
        activity_levels: HashMap::new(),
        resource_availability: HashMap::new(),
    };
    
    let summary = dashboard.generate_summary(&community_data);
    
    println!("   Dashboard Summary:");
    println!("   - Total Visualization Views: {}", summary.engagement.total_views);
    println!("   - Average Interaction Time: {:.1} seconds", summary.engagement.avg_interaction_time);
    println!("   - Engagement Quality Score: {:.2}", summary.engagement.quality_score);
    println!("   - Decision Confidence: {:.2}", summary.engagement.avg_decision_confidence);
    println!("   - Cause Engagement Rate (with viz): {:.1}%", summary.cause_effectiveness.engagement_rate_with_viz * 100.0);
    println!("   - Cause Engagement Rate (without viz): {:.1}%", summary.cause_effectiveness.engagement_rate_without_viz * 100.0);
    println!("   - Helpful Feedback: {:.1}%", summary.feedback.helpful_percentage);
    println!("   - Recommendations Generated: {}", summary.recommendations.len());
    
    println!("\n   ✓ Dashboard summary generated\n");
    
    // Show recommendations
    if !summary.recommendations.is_empty() {
        println!("7. Recommendations:");
        for (i, recommendation) in summary.recommendations.iter().enumerate() {
            println!("   {}. {}", i + 1, recommendation.description);
        }
    }
    
    println!("\n=== Demo Complete ===");
    
    Ok(())
}
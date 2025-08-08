//! Example: Volunteer impact prediction and pathway recommendation

use ml_core::MLEngine;
use ml_core::models::VolunteerImpactModel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Volunteer Impact Prediction Example");
    println!("==================================");
    
    // Create ML engine
    let engine = MLEngine::new();
    
    // Create volunteer impact model
    let model = engine.create_volunteer_impact_model();
    
    // Example: Predict volunteer retention
    println!("\n1. Predicting volunteer retention...");
    let engagement_data = vec![0.8, 0.9, 0.7, 0.85, 0.92]; // Placeholder data
    let retention_score = model.predict_volunteer_retention(&engagement_data)?;
    println!("   Predicted retention score: {:.2}", retention_score);
    
    // Example: Identify skill opportunities
    println!("\n2. Identifying skill development opportunities...");
    let volunteer_profile = vec![0.7, 0.6, 0.8, 0.5, 0.9]; // Placeholder data
    let available_tasks = vec![0.8, 0.7, 0.6, 0.9, 0.5]; // Placeholder data
    let opportunities = model.identify_skill_opportunities(&volunteer_profile, &available_tasks)?;
    println!("   Identified {} opportunities:", opportunities.len());
    for (i, opportunity) in opportunities.iter().enumerate() {
        println!("     {}. {}", i + 1, opportunity);
    }
    
    // Example: Forecast community impact
    println!("\n3. Forecasting community impact...");
    let volunteer_activities = vec![0.8, 0.7, 0.9, 0.6, 0.85]; // Placeholder data
    let impact_forecast = model.forecast_community_impact(&volunteer_activities)?;
    println!("   Forecasted community impact: {:.2}", impact_forecast);
    
    // Example: Recommend volunteer pathways
    println!("\n4. Recommending volunteer pathways...");
    let profile_data = vec![0.7, 0.8, 0.6, 0.9, 0.75]; // Placeholder data
    let pathways = model.recommend_volunteer_pathways(&profile_data)?;
    println!("   Generated {} pathway recommendations:", pathways.len());
    for (i, pathway) in pathways.iter().enumerate() {
        println!("     {}. {}", i + 1, pathway.name);
    }
    
    println!("\nExample completed successfully!");
    Ok(())
}
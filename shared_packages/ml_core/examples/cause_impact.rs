//! Example: Cause impact modeling and resource allocation optimization

use ml_core::MLEngine;
use ml_core::models::CauseImpactModel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Cause Impact Modeling Example");
    println!("============================");
    
    // Create ML engine
    let engine = MLEngine::new();
    
    // Create cause impact model
    let model = engine.create_cause_impact_model();
    
    // Example: Predict cause effectiveness
    println!("\n1. Predicting cause effectiveness...");
    let cause_data = vec![0.8, 0.7, 0.9, 0.6, 0.85]; // Placeholder data
    let effectiveness = model.predict_cause_effectiveness(&cause_data)?;
    println!("   Predicted cause effectiveness: {:.2}", effectiveness);
    
    // Example: Identify success factors
    println!("\n2. Identifying key success factors...");
    let factors = model.identify_success_factors(&cause_data)?;
    println!("   Identified {} key success factors:", factors.len());
    for (i, factor) in factors.iter().enumerate() {
        println!("     {}. {}: {:.2} importance", i + 1, factor.factor, factor.importance);
    }
    
    // Example: Forecast resource needs
    println!("\n3. Forecasting resource needs...");
    let cause_profiles = vec![0.75, 0.8, 0.7, 0.85, 0.78]; // Placeholder data
    let resource_needs = model.forecast_resource_needs(&cause_profiles)?;
    println!("   Forecasted resource needs for {} categories:", resource_needs.len());
    for (category, need) in &resource_needs {
        println!("     {}: {:.2} units", category, need);
    }
    
    // Example: Recommend cause prioritization
    println!("\n4. Recommending cause prioritization...");
    let community_needs = vec![0.8, 0.7, 0.9, 0.6, 0.85]; // Placeholder data
    let cause_data_list = vec![0.75, 0.8, 0.7, 0.85, 0.78]; // Placeholder data
    let priorities = model.recommend_cause_prioritization(&community_needs, &cause_data_list)?;
    println!("   Generated {} prioritization recommendations:", priorities.len());
    for (i, priority) in priorities.iter().enumerate() {
        println!("     {}. {}: {:.2} priority", i + 1, priority.cause_id, priority.priority_score);
    }
    
    println!("\nExample completed successfully!");
    Ok(())
}
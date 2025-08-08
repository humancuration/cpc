//! Cooperative values integration example

use bi_analytics::{
    AnalyticsEngine, 
    cooperative_values::{CooperativeValues, ImpactExplorer, CooperativeGovernance},
    pipeline::VolunteerCoordinationAdapter
};
use polars::df;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Cooperative Values Integration Example");
    
    // Create cooperative values settings
    let coop_values = CooperativeValues {
        prioritize_community_benefit: true,
        community_impact_weight: 1.5,
        show_transparency: true,
        enable_community_validation: true,
    };
    
    println!("✓ Created cooperative values settings");
    
    // Create impact explorer
    let explorer = ImpactExplorer::new(coop_values.clone());
    println!("✓ Created impact explorer");
    
    // Create cooperative governance
    let governance = CooperativeGovernance::new(coop_values.clone());
    println!("✓ Created cooperative governance");
    
    // Create analytics engine
    let engine = AnalyticsEngine::new();
    println!("✓ Created analytics engine");
    
    // Create sample volunteer impact data
    let volunteer_data = df![
        "volunteer_id" => ["vol_001", "vol_002", "vol_003", "vol_004"],
        "hours_contributed" => [10.5, 25.0, 15.75, 30.0],
        "community_benefit" => [8.2, 15.5, 12.3, 22.1],
        "individual_benefit" => [2.3, 9.5, 3.4, 7.9]
    ]?;
    
    println!("✓ Created volunteer data with {} rows", volunteer_data.height());
    
    // Generate transparent explanation
    let explanation = explorer.generate_transparent_explanation("volunteer impact", "volunteer coordination");
    println!("Transparent explanation: {}", explanation);
    
    // Calculate impact-weighted metrics
    let weighted_metrics = explorer.calculate_impact_weighted_metrics(&volunteer_data)?;
    println!("✓ Calculated impact-weighted metrics with {} rows", weighted_metrics.height());
    
    // Apply cooperative governance
    let governed_data = governance.apply_governance(weighted_metrics)?;
    println!("✓ Applied cooperative governance");
    
    // Validate parameters (empty for this example)
    let parameters = std::collections::HashMap::new();
    governance.validate_parameters(&parameters)?;
    println!("✓ Validated analytics parameters");
    
    // Process with analytics engine
    let normalized_data = engine.normalize_data(&governed_data)?;
    println!("✓ Normalized data with {} rows", normalized_data.height());
    
    // Generate descriptive statistics
    let stats = engine.generate_descriptive_stats(&normalized_data)?;
    println!("✓ Generated descriptive statistics");
    println!("Statistics shape: {} rows × {} columns", stats.height(), stats.width());
    
    // Check if community validation is enabled
    if explorer.enable_community_validation() {
        println!("✓ Community validation is enabled for insights");
    }
    
    println!("Cooperative values integration completed successfully!");
    Ok(())
}
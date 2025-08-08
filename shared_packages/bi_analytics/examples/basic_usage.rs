//! Basic usage example of the BI analytics framework

use bi_analytics::{
    AnalyticsEngine, 
    DataPipeline, 
    CooperativeValues, 
    pipeline::{CauseManagementAdapter, SkillDevelopmentAdapter}
};
use polars::df;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("BI Analytics Basic Usage Example");
    
    // Create analytics engine
    let engine = AnalyticsEngine::new();
    println!("✓ Created analytics engine");
    
    // Create data pipeline
    let mut pipeline = DataPipeline::new(engine);
    println!("✓ Created data pipeline");
    
    // Add data source adapters
    pipeline.add_adapter("cause_management".to_string(), Box::new(CauseManagementAdapter {}));
    pipeline.add_adapter("skill_development".to_string(), Box::new(SkillDevelopmentAdapter {}));
    println!("✓ Added data source adapters");
    
    // List available sources
    let sources = pipeline.list_sources();
    println!("Available data sources: {:?}", sources);
    
    // Create a sample DataFrame for demonstration
    let sample_data = df![
        "category" => ["Donation", "Volunteer", "Fundraising", "Community"],
        "amount" => [1000.0, 500.0, 750.0, 1200.0],
        "count" => [50, 25, 30, 40]
    ]?;
    
    println!("✓ Created sample data with {} rows", sample_data.height());
    
    // Apply cooperative values
    let values = CooperativeValues::default();
    println!("✓ Applied cooperative values: {:?}", values);
    
    // Generate descriptive statistics
    let stats = pipeline.engine.generate_descriptive_stats(&sample_data)?;
    println!("✓ Generated descriptive statistics");
    println!("Statistics shape: {} rows × {} columns", stats.height(), stats.width());
    
    println!("Example completed successfully!");
    Ok(())
}
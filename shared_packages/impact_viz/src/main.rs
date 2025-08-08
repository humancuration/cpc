//! Main entry point for the Impact Visualization Framework examples and testing

use impact_viz::core::{ImpactVizCore, ImpactVisualization, VisualizationStyle};
use impact_viz::volunteer::VolunteerImpactDashboard;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Impact Visualization Framework ===");
    println!("Demonstrating core functionality...\n");
    
    // Create the core visualization engine
    let core_viz = ImpactVizCore::new();
    
    // Create a simple mathematical output
    let mut metadata = HashMap::new();
    metadata.insert("demonstration".to_string(), serde_json::Value::String("core_functionality".to_string()));
    
    let math_output = impact_viz::core::MathematicalOutput {
        value: 75.5,
        confidence_interval: None,
        significance: None,
        metadata,
    };
    
    // Translate to impact metric
    let impact_metric = core_viz.translate_impact(&math_output);
    println!("✓ Translated mathematical output to impact metric");
    println!("  Metric name: {}", impact_metric.name);
    println!("  Metric value: {}", impact_metric.value);
    
    // Generate visualization
    let viz_result = core_viz.visualize(&impact_metric, VisualizationStyle::Narrative);
    println!("✓ Generated visualization");
    println!("  Visualization type: {:?}", viz_result.viz_type);
    println!("  Data size: {} characters", viz_result.data.json_data.len());
    
    // Translate values
    let values_metric = core_viz.translate_values(&impact_metric);
    println!("✓ Translated to values-aligned metric");
    println!("  Values connections: {}", values_metric.values_connection.len());
    
    // Create volunteer dashboard as an example
    let volunteer_dashboard = VolunteerImpactDashboard::new(Box::new(core_viz));
    println!("✓ Created volunteer impact dashboard");
    
    println!("\n=== Framework Ready for Use ===");
    println!("Explore the examples in the examples/ directory to see specific use cases.");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_main_execution() {
        // This test ensures the main function executes without panicking
        assert!(main().is_ok());
    }
}
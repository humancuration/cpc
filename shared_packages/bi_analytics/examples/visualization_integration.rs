//! Visualization integration example

use bi_analytics::{
    AnalyticsEngine, 
    visualization::{
        VisualizationIntegration, 
        VisualizationChartType, 
        FilterSpecification, 
        ColumnFilter
    },
    pipeline::FeedbackAnalysisAdapter
};
use polars::df;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Visualization Integration Example");
    
    // Create analytics engine
    let engine = AnalyticsEngine::new();
    println!("✓ Created analytics engine");
    
    // Create visualization integration
    let visualization = VisualizationIntegration::new(engine);
    println!("✓ Created visualization integration");
    
    // Create sample feedback data
    let feedback_data = df![
        "feedback_id" => ["fb_001", "fb_002", "fb_003", "fb_004", "fb_005"],
        "rating" => [4.5, 3.2, 5.0, 2.8, 4.1],
        "category" => ["usability", "features", "performance", "usability", "support"],
        "sentiment" => ["positive", "negative", "positive", "negative", "neutral"]
    ]?;
    
    println!("✓ Created feedback data with {} rows", feedback_data.height());
    
    // Convert to visualization data
    let viz_data = visualization.to_visualization_data(&feedback_data, VisualizationChartType::BarChart)?;
    println!("✓ Converted to visualization data ({} bytes)", viz_data.data_json.len());
    
    // Create interactive filter
    let mut filter_spec = FilterSpecification {
        column_filters: std::collections::HashMap::new(),
        global_search: None,
    };
    
    filter_spec.column_filters.insert(
        "rating".to_string(), 
        ColumnFilter::NumericRange { min: Some(3.0), max: Some(5.0) }
    );
    
    filter_spec.column_filters.insert(
        "category".to_string(),
        ColumnFilter::Categorical { values: vec!["usability".to_string(), "performance".to_string()] }
    );
    
    let interactive_filter = visualization.create_interactive_filter(&feedback_data, &filter_spec)?;
    println!("✓ Created interactive filter for {} columns", interactive_filter.available_columns.len());
    
    // Enable drill-down capability
    let hierarchy_columns = vec!["category".to_string(), "sentiment".to_string()];
    let drill_down = visualization.enable_drill_down(&feedback_data, &hierarchy_columns)?;
    println!("✓ Enabled drill-down capability with depth {}", drill_down.max_depth);
    
    // Stream data for web environments
    let mut data_stream = visualization.stream_data_for_web(&feedback_data, 2)?;
    println!("✓ Created data stream: {} rows in {} chunks of size {}", 
             data_stream.total_rows, data_stream.total_chunks, data_stream.chunk_size);
    
    // Process chunks
    let mut chunk_count = 0;
    data_stream.reset();
    while let Some(chunk) = data_stream.next_chunk(&feedback_data)? {
        chunk_count += 1;
        println!("  Processed chunk {}: {} rows", chunk_count, chunk.height());
    }
    
    println!("Visualization integration completed successfully!");
    Ok(())
}
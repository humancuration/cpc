//! Main binary for the real-time data processing pipeline demo

use anyhow::Result;
use shtairir_demos_data_processing::{pipeline, metrics};
use shtairir_registry::Registry;
use tracing::{info, warn};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let test_mode = args.contains(&"--test-mode".to_string());
    
    // Initialize logging
    if !test_mode {
        tracing_subscriber::fmt::init();
    }
    
    info!("Starting Shtairir Data Processing Pipeline Demo");
    
    // Create a metrics collector
    let mut metrics_collector = metrics::MetricsCollector::new();
    
    // Create registry and load modules
    info!("Loading Shtairir modules");
    let start = metrics_collector.start_operation();
    
    let registry = Registry::load(&[".".into()])?;
    
    metrics_collector.end_operation(start);
    
    // Get available blocks and graphs
    let modules = registry.modules();
    info!("Discovered modules: {:?}", modules);
    
    for module in modules {
        let blocks = registry.blocks_in_module(module)?;
        info!("Blocks in {}: {:?}", module, blocks);
        
        let graphs = registry.graphs_in_module(module)?;
        info!("Graphs in {}: {:?}", module, graphs);
    }
    
    // Execute the pipeline
    info!("Executing sensor data processing pipeline");
    let start = metrics_collector.start_operation();
    
    // Use smaller dataset for test mode
    let reading_count = if test_mode { 10i64 } else { 100i64 };
    match pipeline::execute_pipeline(&registry, reading_count).await {
        Ok(report) => {
            metrics_collector.end_operation(start);
            info!("Pipeline execution successful");
            if !test_mode {
                println!("\n=== Sensor Data Processing Report ===");
                println!("{}", report);
                println!("=====================================\n");
            }
        }
        Err(e) => {
            warn!("Pipeline execution failed: {}", e);
            return Err(e);
        }
    }
    
    // Print metrics
    if !test_mode {
        let (min, avg, max) = metrics_collector.processing_time_stats();
        let total_time = metrics_collector.total_execution_time();
        
        println!("=== Performance Metrics ===");
        println!("Total execution time: {:.2}ms", total_time);
        println!("Processing time (min/avg/max): {:.2}ms / {:.2}ms / {:.2}ms", min, avg, max);
        println!("===========================\n");
    }
    
    info!("Demo completed successfully");
    Ok(())
}
//! Main binary for the machine learning feature pipeline demo

use anyhow::Result;
use shtairir_demos_ml_features::{pipeline, features};
use shtairir_registry::Registry;
use tracing::{info, warn};
use tracing_subscriber;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let test_mode = args.contains(&"--test-mode".to_string());
    
    // Initialize logging
    if !test_mode {
        tracing_subscriber::fmt::init();
    }
    
    info!("Starting Shtairir ML Feature Pipeline Demo");
    
    // Measure execution time
    let start_time = Instant::now();
    
    // Create registry and load modules
    info!("Loading Shtairir modules");
    let registry = Registry::load(&[".".into()])?;
    
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
    info!("Executing ML feature pipeline");
    // Use smaller dataset for test mode
    let sample_count = if test_mode { 50i64 } else { 1000i64 };
    let feature_count = if test_mode { 5i64 } else { 10i64 };
    
    match pipeline::execute_pipeline(&registry, sample_count, feature_count).await {
        Ok((report, normalized_data)) => {
            let duration = start_time.elapsed();
            info!("Pipeline execution successful");
            if !test_mode {
                println!("\n=== ML Feature Pipeline Report ===");
                println!("{}", report);
                println!("Execution time: {:?}", duration);
                println!("==================================\n");
                
                // Show some statistics about the processed data
                if !normalized_data.is_empty() {
                    println!("Processed dataset statistics:");
                    println!("  - Samples: {}", normalized_data.len());
                    println!("  - Features per sample: {}", normalized_data[0].len());
                    
                    // Compute overall statistics
                    let stats = pipeline::compute_dataset_stats(&normalized_data);
                    println!("  - Overall mean: {:.4}", stats.mean);
                    println!("  - Overall std dev: {:.4}", stats.std_dev);
                    println!("  - Value range: {:.4} to {:.4}", stats.min, stats.max);
                    
                    // Show first few samples
                    println!("\nFirst 3 processed samples:");
                    for (i, sample) in normalized_data.iter().take(3).enumerate() {
                        let display_sample: Vec<String> = sample.iter().take(5).map(|x| format!("{:.3}", x)).collect();
                        println!("  {}. [{}{}]", i+1, display_sample.join(", "),
                                 if sample.len() > 5 { ", ..." } else { "" });
                    }
                }
                println!();
            }
        }
        Err(e) => {
            warn!("Pipeline execution failed: {}", e);
            return Err(e);
        }
    }
    
    info!("Demo completed successfully");
    Ok(())
}
//! Main binary for the user profile management workflow demo

use anyhow::Result;
use shtairir_demos_user_profiles::{workflow, validation};
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
    
    info!("Starting Shtairir User Profile Management Workflow Demo");
    
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
    
    // Execute the workflow
    info!("Executing user profile workflow");
    // Use smaller dataset for test mode
    let profile_count = if test_mode { 5i64 } else { 10i64 };
    
    match workflow::execute_workflow(&registry, profile_count).await {
        Ok((summary, profiles)) => {
            let duration = start_time.elapsed();
            info!("Workflow execution successful");
            if !test_mode {
                println!("\n=== User Profile Management Report ===");
                println!("{}", summary);
                println!("Execution time: {:?}", duration);
                println!("=====================================\n");
                
                // Show some processed profiles
                println!("Sample processed profiles:");
                for (i, profile) in profiles.iter().take(3).enumerate() {
                    println!("  {}. {} - {} ({})", i+1, profile.display_name.as_ref().unwrap_or(&profile.name), profile.email, profile.age);
                }
                
                if profiles.len() > 3 {
                    println!("  ... and {} more", profiles.len() - 3);
                }
                println!();
            }
        }
        Err(e) => {
            warn!("Workflow execution failed: {}", e);
            return Err(e);
        }
    }
    
    info!("Demo completed successfully");
    Ok(())
}
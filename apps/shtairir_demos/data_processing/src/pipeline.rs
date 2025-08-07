//! Data processing pipeline implementation
//!
//! This module implements the data processing pipeline using the Shtairir execution engine.

use anyhow::Result;
use shtairir_core::{Graph, Runtime};
use shtairir_registry::Registry;
use tracing::info;

/// Represents a sensor reading
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SensorReading {
    pub id: String,
    pub timestamp: i64,
    pub temperature: f64,
    pub humidity: f64,
    pub location: String,
}

/// Represents processing metrics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessingMetrics {
    pub latency_ms: f64,
    pub processed_count: usize,
    pub filtered_count: usize,
}

/// Execute the sensor data processing pipeline
pub async fn execute_pipeline(registry: &Registry, reading_count: i64) -> Result<String> {
    info!("Loading sensor data processing pipeline graph");
    
    // Load the graph from the registry
    let graph = registry.find_graph("demos.shtairir.data_processing/sensor_data_pipeline")?
        .ok_or_else(|| anyhow::anyhow!("Graph not found"))?;
    
    info!("Creating runtime for pipeline execution");
    
    // Create a runtime
    let mut runtime = Runtime::new(registry.clone());
    
    // Set input values
    let inputs = serde_json::json!({
        "mock_data.count": reading_count
    });
    
    info!("Executing pipeline with {} mock readings", reading_count);
    
    // Execute the graph
    let outputs = runtime.execute_graph(&graph, inputs).await?;
    
    // Extract the report
    let report = outputs.get("report")
        .ok_or_else(|| anyhow::anyhow!("Report not found in outputs"))?
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Report is not a string"))?
        .to_string();
    
    info!("Pipeline execution completed successfully");
    
    Ok(report)
}

/// Generate mock sensor data
pub fn generate_mock_data(count: usize) -> Vec<SensorReading> {
    use rand::Rng;
    
    let mut rng = rand::thread_rng();
    let mut readings = Vec::with_capacity(count);
    
    for i in 0..count {
        readings.push(SensorReading {
            id: format!("sensor-{}", i),
            timestamp: chrono::Utc::now().timestamp(),
            temperature: rng.gen_range(-10.0..40.0), // Realistic temperature range
            humidity: rng.gen_range(0.0..100.0),    // Percentage
            location: format!("room-{}", rng.gen_range(1..=10)),
        });
    }
    
    readings
}
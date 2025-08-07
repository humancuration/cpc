//! ML feature pipeline implementation
//!
//! This module implements the machine learning feature engineering pipeline using the Shtairir execution engine.

use anyhow::Result;
use shtairir_core::{Graph, Runtime};
use shtairir_registry::Registry;
use tracing::info;

/// Represents ML pipeline metrics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MLPipelineMetrics {
    pub samples: usize,
    pub features: usize,
    pub processing_time_ms: f64,
    pub data_quality: DataQuality,
}

/// Represents data quality metrics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataQuality {
    pub mean: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
}

/// Execute the ML feature pipeline
pub async fn execute_pipeline(registry: &Registry, sample_count: i64, feature_count: i64) -> Result<(String, Vec<Vec<f64>>)> {
    info!("Loading ML feature pipeline graph");
    
    // Load the graph from the registry
    let graph = registry.find_graph("demos.shtairir.ml_features/ml_feature_pipeline")?
        .ok_or_else(|| anyhow::anyhow!("Graph not found"))?;
    
    info!("Creating runtime for pipeline execution");
    
    // Create a runtime
    let mut runtime = Runtime::new(registry.clone());
    
    // Set input values
    let inputs = serde_json::json!({
        "mock_data.samples": sample_count,
        "mock_data.features": feature_count
    });
    
    info!("Executing pipeline with {} samples and {} features", sample_count, feature_count);
    
    // Execute the graph
    let outputs = runtime.execute_graph(&graph, inputs).await?;
    
    // Extract the report
    let report = outputs.get("report")
        .ok_or_else(|| anyhow::anyhow!("Report not found in outputs"))?
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Report is not a string"))?
        .to_string();
    
    // Extract the normalized data
    let normalized_data: Vec<Vec<f64>> = if let Some(data_value) = outputs.get("normalized_data") {
        serde_json::from_value(data_value.clone())?
    } else {
        Vec::new()
    };
    
    info!("Pipeline execution completed successfully");
    
    Ok((report, normalized_data))
}

/// Generate mock ML data
pub fn generate_mock_data(samples: usize, features: usize) -> Vec<Vec<f64>> {
    use rand::Rng;
    use rand_distr::{Normal, Distribution};
    
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, 1.0).unwrap();
    let mut dataset = Vec::with_capacity(samples);
    
    for _ in 0..samples {
        let mut sample = Vec::with_capacity(features);
        for _ in 0..features {
            // Generate normally distributed features
            sample.push(normal.sample(&mut rng));
        }
        dataset.push(sample);
    }
    
    dataset
}

/// Compute basic statistics for a dataset
pub fn compute_dataset_stats(dataset: &[Vec<f64>]) -> DataQuality {
    if dataset.is_empty() || dataset[0].is_empty() {
        return DataQuality {
            mean: 0.0,
            std_dev: 0.0,
            min: 0.0,
            max: 0.0,
        };
    }
    
    let all_values: Vec<f64> = dataset.iter().flatten().copied().collect();
    
    if all_values.is_empty() {
        return DataQuality {
            mean: 0.0,
            std_dev: 0.0,
            min: 0.0,
            max: 0.0,
        };
    }
    
    let mean = all_values.iter().sum::<f64>() / all_values.len() as f64;
    let variance = all_values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / all_values.len() as f64;
    let std_dev = variance.sqrt();
    let min = all_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = all_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    
    DataQuality {
        mean,
        std_dev,
        min,
        max,
    }
}
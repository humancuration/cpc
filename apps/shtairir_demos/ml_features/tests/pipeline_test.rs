//! Tests for the ML feature pipeline

use anyhow::Result;
use shtairir_demos_ml_features::{pipeline, features};
use shtairir_registry::Registry;

#[tokio::test]
async fn test_pipeline_execution() -> Result<()> {
    // Create registry and load modules
    let registry = Registry::load(&["../".into()])?;
    
    // Execute the pipeline with a small dataset
    let (report, normalized_data) = pipeline::execute_pipeline(&registry, 50, 5).await?;
    
    // Check that we got a report and data
    assert!(!report.is_empty());
    assert!(report.contains("ML Pipeline"));
    assert!(report.contains("samples"));
    assert!(report.contains("features"));
    
    // Check that we got normalized data
    assert!(!normalized_data.is_empty());
    
    // Check data structure
    assert_eq!(normalized_data.len(), 50);
    for sample in &normalized_data {
        assert_eq!(sample.len(), 5); // Should match input features
    }
    
    Ok(())
}

#[test]
fn test_mock_data_generation() {
    let samples = 10;
    let features = 3;
    let data = pipeline::generate_mock_data(samples, features);
    
    assert_eq!(data.len(), samples);
    for sample in data {
        assert_eq!(sample.len(), features);
        // All values should be finite numbers
        for value in sample {
            assert!(value.is_finite());
        }
    }
}

#[test]
fn test_feature_engineering() {
    let sample = vec![1.0, 2.0, 3.0];
    
    // Test polynomial features
    let extended = features::add_polynomial_features(&sample, 2);
    assert_eq!(extended.len(), 6); // Original 3 + squared 3
    assert_eq!(extended[3], 1.0);  // 1^2
    assert_eq!(extended[4], 4.0);  // 2^2
    assert_eq!(extended[5], 9.0);  // 3^2
    
    // Test interaction features
    let extended = features::add_interaction_features(&sample);
    assert_eq!(extended.len(), 5); // Original 3 + interactions 2
    assert_eq!(extended[3], 2.0);  // 1*2
    assert_eq!(extended[4], 6.0);  // 2*3
}

#[test]
fn test_normalization() {
    let dataset = vec![
        vec![1.0, 2.0, 3.0],
        vec![2.0, 3.0, 4.0],
        vec![3.0, 4.0, 5.0],
    ];
    
    let means = features::compute_feature_means(&dataset);
    let stds = features::compute_feature_stds(&dataset, &means);
    
    assert_eq!(means, vec![2.0, 3.0, 4.0]);
    // All std devs should be the same for this dataset
    assert!((stds[0] - 1.0).abs() < 0.001);
    assert!((stds[1] - 1.0).abs() < 0.001);
    assert!((stds[2] - 1.0).abs() < 0.001);
    
    // Test normalization
    let sample = vec![1.0, 2.0, 3.0];
    let normalized = features::normalize_features(&sample, &means, &stds);
    assert_eq!(normalized, vec![-1.0, -1.0, -1.0]);
}
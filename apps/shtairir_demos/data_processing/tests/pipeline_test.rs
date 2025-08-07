//! Tests for the data processing pipeline

use anyhow::Result;
use shtairir_demos_data_processing::pipeline;
use shtairir_registry::Registry;

#[tokio::test]
async fn test_pipeline_execution() -> Result<()> {
    // Create registry and load modules
    let registry = Registry::load(&["../".into()])?;
    
    // Execute the pipeline with a small dataset
    let report = pipeline::execute_pipeline(&registry, 10).await?;
    
    // Check that we got a report
    assert!(!report.is_empty());
    assert!(report.contains("Average temperature"));
    assert!(report.contains("Processed"));
    assert!(report.contains("readings"));
    
    Ok(())
}

#[test]
fn test_mock_data_generation() {
    let count = 5;
    let data = pipeline::generate_mock_data(count);
    
    assert_eq!(data.len(), count);
    
    for reading in data {
        assert!(!reading.id.is_empty());
        assert!(reading.timestamp > 0);
        // Temperature should be in a reasonable range
        assert!(reading.temperature >= -50.0 && reading.temperature <= 100.0);
        // Humidity should be a percentage
        assert!(reading.humidity >= 0.0 && reading.humidity <= 100.0);
        assert!(!reading.location.is_empty());
    }
}
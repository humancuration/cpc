//! Integration tests for the BI analytics framework

use bi_analytics::{
    AnalyticsEngine, 
    DataPipeline, 
    cooperative_values::{CooperativeValues, ImpactExplorer},
    privacy::{PrivacySettings, ConsentAwareProcessor},
    visualization::VisualizationIntegration,
    pipeline::{CauseManagementAdapter, SkillDevelopmentAdapter, VolunteerCoordinationAdapter},
    error::AnalyticsError
};
use consent_manager::domain::consent::DataSharingLevel;
use polars::df;

#[tokio::test]
async fn test_complete_analytics_pipeline() -> Result<(), AnalyticsError> {
    // Create analytics engine
    let engine = AnalyticsEngine::new();
    
    // Create data pipeline
    let mut pipeline = DataPipeline::new(engine.clone());
    
    // Add data source adapters
    pipeline.add_adapter("cause_management".to_string(), Box::new(CauseManagementAdapter {}));
    pipeline.add_adapter("skill_development".to_string(), Box::new(SkillDevelopmentAdapter {}));
    pipeline.add_adapter("volunteer_coordination".to_string(), Box::new(VolunteerCoordinationAdapter {}));
    
    // Verify sources were added
    let sources = pipeline.list_sources();
    assert_eq!(sources.len(), 3);
    assert!(sources.contains(&"cause_management".to_string()));
    assert!(sources.contains(&"skill_development".to_string()));
    assert!(sources.contains(&"volunteer_coordination".to_string()));
    
    // Create sample data for testing
    let sample_data = df![
        "project" => ["Project A", "Project B", "Project C"],
        "funding" => [10000.0, 15000.0, 12000.0],
        "volunteers" => [25, 30, 20],
        "impact_score" => [8.5, 9.2, 7.8]
    ]?;
    
    // Test engine functionality
    let normalized_data = engine.normalize_data(&sample_data)?;
    assert_eq!(normalized_data.height(), 3);
    assert_eq!(normalized_data.width(), 4);
    
    // Test descriptive statistics
    let stats = engine.generate_descriptive_stats(&sample_data)?;
    assert!(stats.height() > 0);
    assert!(stats.width() > 0);
    
    // Test cooperative values integration
    let coop_values = CooperativeValues::default();
    let explorer = ImpactExplorer::new(coop_values);
    let weighted_metrics = explorer.calculate_impact_weighted_metrics(&sample_data)?;
    assert_eq!(weighted_metrics.height(), 3);
    
    // Test privacy integration
    let privacy_settings = PrivacySettings::default();
    let processor = ConsentAwareProcessor::new(privacy_settings);
    let anonymized_data = processor.apply_anonymization(sample_data.clone(), DataSharingLevel::Standard)?;
    assert_eq!(anonymized_data.height(), 3);
    
    // Test visualization integration
    let visualization = VisualizationIntegration::new(engine);
    let viz_data = visualization.to_visualization_data(&sample_data, bi_analytics::visualization::VisualizationChartType::BarChart)?;
    assert!(!viz_data.data_json.is_empty());
    
    // Test data streaming
    let mut data_stream = visualization.stream_data_for_web(&sample_data, 2)?;
    assert_eq!(data_stream.total_rows, 3);
    assert_eq!(data_stream.chunk_size, 2);
    assert_eq!(data_stream.total_chunks, 2);
    
    // Process first chunk
    let chunk1 = data_stream.next_chunk(&sample_data)?;
    assert!(chunk1.is_some());
    assert_eq!(chunk1.unwrap().height(), 2);
    
    // Process second chunk
    let chunk2 = data_stream.next_chunk(&sample_data)?;
    assert!(chunk2.is_some());
    assert_eq!(chunk2.unwrap().height(), 1);
    
    // No more chunks
    let no_chunk = data_stream.next_chunk(&sample_data)?;
    assert!(no_chunk.is_none());
    
    Ok(())
}

#[test]
fn test_engine_configurations() {
    // Test default configuration
    let engine = AnalyticsEngine::new();
    assert_eq!(engine.config.max_memory, 1024 * 1024 * 1024);
    assert!(engine.config.enable_streaming);
    
    // Test custom configuration
    let custom_config = bi_analytics::engine::EngineConfig {
        max_memory: 512 * 1024 * 1024, // 512MB
        enable_streaming: false,
        privacy_settings: bi_analytics::privacy::PrivacySettings::default(),
        cooperative_values: bi_analytics::cooperative_values::CooperativeValues::default(),
    };
    
    let custom_engine = AnalyticsEngine::with_config(custom_config);
    assert_eq!(custom_engine.config.max_memory, 512 * 1024 * 1024);
    assert!(!custom_engine.config.enable_streaming);
}

#[tokio::test]
async fn test_consent_aware_processing() -> Result<(), Box<dyn std::error::Error>> {
    let privacy_settings = PrivacySettings {
        minimum_consent_level: DataSharingLevel::Standard,
        apply_differential_privacy: true,
        differential_privacy_epsilon: 1.0,
        anonymize_by_default: true,
    };
    
    let processor = ConsentAwareProcessor::new(privacy_settings);
    
    // Test consent checking (simulated)
    let has_consent = processor.check_consent("user_123", consent_manager::domain::consent::Domain::FinancialData).await?;
    assert!(has_consent); // Simulated result
    
    // Test anonymization with different consent levels
    let test_data = df![
        "name" => ["Alice", "Bob", "Charlie"],
        "value" => [100, 200, 300]
    ]?;
    
    // Test with None level (should remove PII)
    let anonymized_none = processor.apply_anonymization(test_data.clone(), DataSharingLevel::None)?;
    assert_eq!(anonymized_none.width(), 0); // No columns should remain
    
    // Test with Minimal level (should remove PII columns)
    let anonymized_minimal = processor.apply_anonymization(test_data.clone(), DataSharingLevel::Minimal)?;
    assert!(!anonymized_minimal.get_column_names().contains(&"name")); // PII should be removed
    assert!(anonymized_minimal.get_column_names().contains(&"value")); // Non-PII should remain
    
    Ok(())
}

#[test]
fn test_cooperative_values_functionality() {
    let coop_values = CooperativeValues {
        prioritize_community_benefit: true,
        community_impact_weight: 2.0,
        show_transparency: true,
        enable_community_validation: true,
    };
    
    let explorer = ImpactExplorer::new(coop_values.clone());
    let governance = bi_analytics::cooperative_values::CooperativeGovernance::new(coop_values);
    
    // Test impact explorer
    assert!(explorer.enable_community_validation());
    
    let explanation = explorer.generate_transparent_explanation("test", "sample");
    assert!(explanation.contains("community impact weighting"));
    assert!(explanation.contains("2.00")); // Weight factor
    
    // Test governance
    let empty_params = std::collections::HashMap::new();
    let validation_result = governance.validate_parameters(&empty_params);
    assert!(validation_result.is_ok());
}

#[tokio::test]
async fn test_data_pipeline_operations() -> Result<(), AnalyticsError> {
    let engine = AnalyticsEngine::new();
    let mut pipeline = DataPipeline::new(engine);
    
    // Add adapters
    pipeline.add_adapter("test_source".to_string(), Box::new(CauseManagementAdapter {}));
    
    // Test list sources
    let sources = pipeline.list_sources();
    assert_eq!(sources, vec!["test_source"]);
    
    // Test transformations
    let test_data = df![
        "category" => ["A", "B", "C", "A", "B"],
        "value" => [10, 20, 30, 40, 50]
    ]?;
    
    // Test select transformation
    let select_transform = bi_analytics::pipeline::Transformation::Select(vec!["category".to_string()]);
    let selected_data = pipeline.transform_data(&test_data, vec![select_transform])?;
    assert_eq!(selected_data.width(), 1);
    assert_eq!(selected_data.get_column_names(), &["category"]);
    
    // Test sort transformation
    let sort_transform = bi_analytics::pipeline::Transformation::Sort {
        columns: vec!["value".to_string()],
        descending: true,
    };
    let sorted_data = pipeline.transform_data(&test_data, vec![sort_transform])?;
    assert_eq!(sorted_data.height(), 5);
    
    Ok(())
}
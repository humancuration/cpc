//! Integration tests for the BI visualization toolkit

use bi_visualization::{
    VisualizationService,
    ChartConfig,
    ChartType,
    VisualizationTheme,
    SeriesConfig,
    DataSeries,
    TimeSeriesPoint,
};

#[test]
fn test_chart_generation() {
    // Create chart configuration
    let config = ChartConfig::new(
        ChartType::Line,
        "Test Chart".to_string(),
        (800, 600),
        VisualizationTheme::Light,
        vec![SeriesConfig::new("Test Series", "red")],
    );
    
    // Create test data
    let points = vec![
        TimeSeriesPoint::new(chrono::Utc::now(), 10.0),
        TimeSeriesPoint::new(chrono::Utc::now(), 20.0),
        TimeSeriesPoint::new(chrono::Utc::now(), 15.0),
    ];
    let data = DataSeries::from_time_series("Test Data".to_string(), points);
    
    // Generate chart
    let result = VisualizationService::generate_chart(config, data);
    
    // Verify result
    assert!(result.is_ok());
    let image = result.unwrap();
    assert_eq!(image.width(), 800);
    assert_eq!(image.height(), 600);
}

#[test]
fn test_data_transformation() {
    // Create raw data
    let values = vec![
        serde_json::json!({"x": 1, "y": 10}),
        serde_json::json!({"x": 2, "y": 20}),
        serde_json::json!({"x": 3, "y": 15}),
    ];
    let raw_data = bi_visualization::domain::data::RawData::new(values, "test_source".to_string());
    
    // Transform data
    let transformation = bi_visualization::application::service::DataTransformation::Normalization;
    let result = VisualizationService::transform_data(transformation, raw_data);
    
    // Verify result
    assert!(result.is_ok());
    let processed_data = result.unwrap();
    assert_eq!(processed_data.series.len(), 1);
    assert_eq!(processed_data.metadata["source"], "test_source");
}
//! Basic usage example for the BI visualization toolkit

use bi_visualization::{
    VisualizationService,
    ChartConfig,
    ChartType,
    VisualizationTheme,
    SeriesConfig,
    DataSeries,
    TimeSeriesPoint,
    DataTransformation,
    RawData,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create chart configuration
    let config = ChartConfig::new(
        ChartType::Line,
        "Monthly Spending Trend".to_string(),
        (800, 600),
        VisualizationTheme::Dark,
        vec![
            SeriesConfig::new("Groceries", "red"),
            SeriesConfig::new("Transport", "blue"),
        ],
    );
    
    // Create test data
    let points = vec![
        TimeSeriesPoint::new(chrono::Utc::now() - chrono::Duration::days(30), 150.0),
        TimeSeriesPoint::new(chrono::Utc::now() - chrono::Duration::days(20), 120.0),
        TimeSeriesPoint::new(chrono::Utc::now() - chrono::Duration::days(10), 180.0),
        TimeSeriesPoint::new(chrono::Utc::now(), 160.0),
    ];
    let data = DataSeries::from_time_series("Spending".to_string(), points);
    
    // Generate chart
    let chart_image = VisualizationService::generate_chart(config, data)?;
    println!("Generated chart with dimensions: {}x{}", chart_image.width(), chart_image.height());
    
    // Create raw data for transformation
    let values = vec![
        serde_json::json!({"timestamp": "2023-01-01T00:00:00Z", "amount": 150.0}),
        serde_json::json!({"timestamp": "2023-01-02T00:00:00Z", "amount": 120.0}),
        serde_json::json!({"timestamp": "2023-01-03T00:00:00Z", "amount": 180.0}),
    ];
    let raw_data = RawData::new(values, "finance_module".to_string());
    
    // Transform data
    let transformation = DataTransformation::TimeAggregation(
        bi_visualization::application::service::TimeAggregation::Daily,
    );
    let processed_data = VisualizationService::transform_data(transformation, raw_data)?;
    println!("Transformed {} data points", processed_data.metadata["point_count"]);
    
    println!("BI visualization example completed successfully!");
    Ok(())
}
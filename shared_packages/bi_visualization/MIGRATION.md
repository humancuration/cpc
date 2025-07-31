# Migration Guide for BI Visualization Toolkit

This document provides guidance on migrating existing modules to use the new BI Visualization Toolkit.

## Overview

The BI Visualization Toolkit provides standardized data visualization capabilities across applications using Plotters, with Bevy integration for interactive components.

## Migration Steps

### 1. Update Cargo.toml

Add the BI visualization dependency to your module's Cargo.toml:

```toml
[dependencies]
bi_visualization = { path = "../bi_visualization" }
```

### 2. Replace Direct Visualization Implementations

#### Before (Finance Module Example)
```rust
// Direct chart generation using a custom implementation
use image::{ImageBuffer, Rgba};

struct FinanceChartGenerator;

impl FinanceChartGenerator {
    fn generate_spending_chart(&self, spending_data: Vec<(String, f64)>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        // Custom chart generation logic
        // This would typically involve:
        // 1. Creating a canvas
        // 2. Drawing axes
        // 3. Plotting data points
        // 4. Adding labels and titles
        // 5. Returning the image buffer
        
        let width = 800;
        let height = 600;
        let mut img = ImageBuffer::new(width, height);
        
        // Simplified drawing logic
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = Rgba([255, 255, 255, 255]); // White background
        }
        
        img
    }
}
```

#### After (Using BI Visualization Toolkit)
```rust
// Using BI Visualization Toolkit
use bi_visualization::{VisualizationService, ChartConfig, ChartType, VisualizationTheme, SeriesConfig, DataSeries, TimeSeriesPoint};

struct FinanceService;

impl FinanceService {
    fn generate_spending_chart(&self, spending_data: Vec<(String, f64)>) -> Result<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
        // Create chart configuration
        let config = ChartConfig::new(
            ChartType::Line,
            "Monthly Spending".to_string(),
            (800, 600),
            VisualizationTheme::Dark,
            vec![
                SeriesConfig::new("Groceries", "red"),
                SeriesConfig::new("Transport", "blue"),
            ],
        );
        
        // Convert data to time series format
        let points: Vec<TimeSeriesPoint> = spending_data
            .into_iter()
            .enumerate()
            .map(|(i, (_, value))| {
                TimeSeriesPoint::new(
                    chrono::Utc::now() - chrono::Duration::days((spending_data.len() - i) as i64),
                    value,
                )
            })
            .collect();
        
        let data = DataSeries::from_time_series("Spending".to_string(), points);
        
        // Generate chart using BI Visualization Toolkit
        let chart_image = VisualizationService::generate_chart(config, data)?;
        
        Ok(chart_image)
    }
}
```

### 3. Update Service Initialization

#### Before (Finance Module Example)
```rust
// Direct visualization initialization
let chart_generator = FinanceChartGenerator;
```

#### After (Using BI Visualization Toolkit)
```rust
// Using BI Visualization Toolkit
// No specific initialization needed - VisualizationService is stateless
```

## Finance Module Integration Example

```rust
// Finance service using BI Visualization Toolkit
use bi_visualization::{
    VisualizationService,
    ChartConfig,
    ChartType,
    VisualizationTheme,
    SeriesConfig,
    DataSeries,
    TimeSeriesPoint,
    infrastructure::export::{VisualizationExporter, ExportFormat},
};

struct FinanceService;

impl FinanceService {
    fn generate_monthly_spending_chart(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Generate monthly spending chart
        let chart_config = ChartConfig {
            chart_type: ChartType::Line,
            title: "Monthly Spending".to_string(),
            dimensions: (800, 600),
            theme: VisualizationTheme::Dark,
            series: vec![
                SeriesConfig::new("Groceries", "#FF0000"),
                SeriesConfig::new("Transport", "#0000FF"),
                SeriesConfig::new("Entertainment", "#00FF00"),
            ],
        };
        
        // Create sample spending data (in a real app, this would come from actual data)
        let mut groceries_points = Vec::new();
        let mut transport_points = Vec::new();
        let mut entertainment_points = Vec::new();
        
        for i in 0..30 {
            let timestamp = chrono::Utc::now() - chrono::Duration::days(30 - i);
            groceries_points.push(TimeSeriesPoint::new(timestamp, 50.0 + (i as f64 * 2.0) % 100.0));
            transport_points.push(TimeSeriesPoint::new(timestamp, 30.0 + (i as f64 * 1.5) % 80.0));
            entertainment_points.push(TimeSeriesPoint::new(timestamp, 20.0 + (i as f64 * 1.0) % 60.0));
        }
        
        let groceries_series = DataSeries::from_time_series("Groceries".to_string(), groceries_points);
        let transport_series = DataSeries::from_time_series("Transport".to_string(), transport_points);
        let entertainment_series = DataSeries::from_time_series("Entertainment".to_string(), entertainment_points);
        
        // For this example, we'll just use the first series
        // In a real implementation, we would combine multiple series or create a multi-series chart
        let chart_image = VisualizationService::generate_chart(chart_config, groceries_series)?;
        
        // Export to PNG
        VisualizationExporter::export_image(
            &chart_image,
            ExportFormat::Png,
            "spending_trend.png",
        )?;
        
        Ok(())
    }
}
```

## Health Module Integration Example

```rust
// Health service using BI Visualization Toolkit with Bevy integration
use bi_visualization::{
    VisualizationService,
    InteractiveConfig,
    ChartType,
    DataSeries,
    HeatmapPoint,
    infrastructure::bevy::{BiVisualizationPlugin, InteractiveChart},
};
use bevy::prelude::*;

struct HealthService;

impl HealthService {
    fn create_sleep_analysis_chart(&self) -> Result<impl Bundle, Box<dyn std::error::Error>> {
        // Create interactive sleep quality visualization
        let interactive_config = InteractiveConfig {
            chart_type: ChartType::Heatmap,
            title: "Sleep Quality Over Time".to_string(),
            dimensions: (1200, 800),
            interactive_elements: vec![
                bi_visualization::domain::chart::InteractiveElement::Tooltip,
                bi_visualization::domain::chart::InteractiveElement::Zoom,
            ],
        };
        
        // Create sample sleep data
        let mut sleep_points = Vec::new();
        
        for day in 0..30 {
            for hour in 0..24 {
                // Generate some sample sleep quality data (0.0 to 1.0)
                let quality = (day as f64 * 0.1 + hour as f64 * 0.05) % 1.0;
                sleep_points.push(HeatmapPoint::new(day, hour, quality));
            }
        }
        
        let sleep_series = DataSeries::from_heatmap("Sleep Quality".to_string(), sleep_points);
        
        // Create interactive chart using BI Visualization Toolkit
        let chart_bundle = VisualizationService::create_interactive_chart(interactive_config, sleep_series)?;
        
        Ok(chart_bundle)
    }
}

// Bevy system to add the interactive chart
fn add_sleep_analysis_chart(mut commands: Commands) {
    let health_service = HealthService;
    
    match health_service.create_sleep_analysis_chart() {
        Ok(bundle) => {
            commands.spawn(bundle);
        }
        Err(e) => {
            eprintln!("Failed to create sleep analysis chart: {}", e);
        }
    }
}

// Bevy app setup
fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(BiVisualizationPlugin)
        .add_systems(Startup, add_sleep_analysis_chart)
        .run();
}
```

## Data Transformation

The BI Visualization Toolkit provides data transformation capabilities:

```rust
use bi_visualization::{
    VisualizationService,
    DataTransformation,
    TimeAggregation,
    RawData,
};

struct DataService;

impl DataService {
    async fn transform_financial_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Get raw financial data (in a real app, this would come from storage)
        let values = vec![
            serde_json::json!({"timestamp": "2023-01-01T00:00:00Z", "amount": 150.0}),
            serde_json::json!({"timestamp": "2023-01-02T00:00:00Z", "amount": 120.0}),
            serde_json::json!({"timestamp": "2023-01-03T00:00:00Z", "amount": 180.0}),
        ];
        let raw_data = RawData::new(values, "finance_module".to_string());
        
        // Transform data
        let transformation = DataTransformation::TimeAggregation(TimeAggregation::Daily);
        let processed_data = VisualizationService::transform_data(transformation, raw_data)?;
        
        println!("Transformed {} data points", processed_data.metadata["point_count"]);
        
        Ok(())
    }
}
```

## Testing During Migration

Use mock data for testing:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use bi_visualization::{VisualizationService, ChartConfig, ChartType, VisualizationTheme, SeriesConfig, DataSeries, TimeSeriesPoint};

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
}
```

## Performance Considerations

1. The BI Visualization Toolkit uses efficient rendering through Plotters.

2. Data transformation is optimized for common use cases.

3. Interactive visualizations in Bevy are designed to be lightweight.

4. Export operations are asynchronous to avoid blocking the main thread.

## Troubleshooting

### Common Issues

1. **Chart Generation Errors**: Ensure chart configurations are valid and data series contain data points.

2. **Export Failures**: Verify file paths are writable and export format is supported.

3. **Bevy Integration Issues**: Check that the BiVisualizationPlugin is properly added to the Bevy app.

### Logging and Monitoring

The BI Visualization Toolkit uses tracing for logging. Enable tracing in your application to monitor visualization operations:

```rust
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // ... rest of your application
}
```

This will provide detailed logs of visualization operations, including chart generation, data transformation, and any errors that occur.
# BI Visualization Toolkit

Provides standardized data visualization capabilities across applications using Plotters, with Bevy integration for interactive components.

## Overview

The BI Visualization Toolkit offers a comprehensive solution for creating data visualizations across all CPC applications. It supports various chart types, interactive visualizations, data transformation, and export capabilities.

## Features

- **Multiple Chart Types**: Line, bar, pie, scatter, histogram, heatmap, and area charts
- **Interactive Visualizations**: Real-time charts with zoom, pan, tooltips, and selection
- **Data Transformation**: Convert raw data to visualization-ready formats
- **Multiple Themes**: Light, dark, and high-contrast themes
- **Export Capabilities**: Export to PNG, JPEG, SVG, and PDF formats
- **Bevy Integration**: Interactive components for real-time applications
- **Plotters Integration**: High-quality static chart generation
- **Extensible Architecture**: Easy to add new chart types and features

## Architecture

The module follows hexagonal architecture principles with clear separation of concerns:

```
Domain Layer
├── chart.rs        # Chart types & configurations
├── data.rs         # Data models
└── errors.rs       # Error types

Application Layer
├── service.rs      # VisualizationService
└── transformer.rs  # Data transformation

Infrastructure Layer
├── plotters.rs     # Plotters integration
├── bevy.rs         # Bevy interactive components
└── export.rs       # Image/PDF export
```

## Usage

### Basic Usage

```rust
use bi_visualization::{
    VisualizationService,
    ChartConfig,
    ChartType,
    VisualizationTheme,
    SeriesConfig,
    DataSeries,
    TimeSeriesPoint,
};

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

// Create sample data
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
```

### Finance Module Integration

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
            ],
        };
        
        // Create sample spending data
        let mut groceries_points = Vec::new();
        let mut transport_points = Vec::new();
        
        for i in 0..30 {
            let timestamp = chrono::Utc::now() - chrono::Duration::days(30 - i);
            groceries_points.push(TimeSeriesPoint::new(timestamp, 50.0 + (i as f64 * 2.0) % 100.0));
            transport_points.push(TimeSeriesPoint::new(timestamp, 30.0 + (i as f64 * 1.5) % 80.0));
        }
        
        let groceries_series = DataSeries::from_time_series("Groceries".to_string(), groceries_points);
        let transport_series = DataSeries::from_time_series("Transport".to_string(), transport_points);
        
        // Generate chart
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

### Health Module Integration

```rust
// Health service using BI Visualization Toolkit with Bevy integration
use bi_visualization::{
    VisualizationService,
    InteractiveConfig,
    ChartType,
    DataSeries,
    HeatmapPoint,
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
        
        // Create interactive chart
        let chart_bundle = VisualizationService::create_interactive_chart(interactive_config, sleep_series)?;
        
        Ok(chart_bundle)
    }
}
```

## Integration Examples

See the examples directory for integration examples with:
- Basic usage
- Finance module integration
- Health module integration

Run examples with:
```bash
cargo run --example basic_usage
cargo run --example finance_integration
cargo run --example health_integration
```

## Testing

Run tests with:
```bash
cargo test
```

## Migration

See [MIGRATION.md](MIGRATION.md) for detailed migration guidance for existing modules.

## Dependencies

- **tokio**: Async runtime
- **serde**: Serialization framework
- **plotters**: Chart generation library
- **image**: Image processing library
- **bevy**: Game engine for interactive components
- **storage_abstraction**: Storage abstraction layer for data retrieval
- **tracing**: Logging and monitoring

## License

This module is part of the CPC software ecosystem and is licensed under the CPC license.
# Statistical Visualization Components

Documentation for statistical visualization components in the BI Visualization Toolkit.

## Overview

This document describes the statistical visualization components that extend the BI Visualization Toolkit with confidence intervals, significance indicators, and cooperative values-aligned presentation.

## Domain Components

### ConfidenceIntervalConfig (`domain/confidence_interval.rs`)

Configuration for visualizing confidence intervals:

```rust
pub struct ConfidenceIntervalConfig {
    pub main_series: DataSeries,
    pub upper_bound: DataSeries,
    pub lower_bound: DataSeries,
    pub confidence_level: f64,
    pub interval_color: String,
    pub show_error_bars: bool,
}
```

### SignificanceIndicator (`domain/confidence_interval.rs`)

Visual indicators for statistical significance:

```rust
pub struct SignificanceIndicator {
    pub p_value: f64,
    pub significance_level: SignificanceLevel,
    pub position: (f64, f64),
    pub label: Option<String>,
}
```

### StatisticalChartConfig (`domain/confidence_interval.rs`)

Extended chart configuration with statistical components:

```rust
pub struct StatisticalChartConfig {
    pub base_config: ChartConfig,
    pub confidence_intervals: Vec<ConfidenceIntervalConfig>,
    pub significance_indicators: Vec<SignificanceIndicator>,
    pub show_explanations: bool,
}
```

## Application Components

### StatisticalAnalysisService (`application/statistical_service.rs`)

Service for integrating statistical analysis with visualization:

```rust
pub struct StatisticalAnalysisService;

impl StatisticalAnalysisService {
    pub fn generate_chart_with_confidence(
        config: StatisticalChartConfig,
        data: Vec<DataSeries>,
    ) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, VisualizationError>;
    
    pub fn add_confidence_intervals(
        base_data: DataSeries,
        upper_bound: DataSeries,
        lower_bound: DataSeries,
        confidence_level: f64,
    ) -> ConfidenceIntervalConfig;
}
```

## Infrastructure Components

### StatisticalVisualizationAdapter (`infrastructure/statistical_adapters.rs`)

Adapters for rendering statistical visualization components:

```rust
pub struct StatisticalVisualizationAdapter;

impl StatisticalVisualizationAdapter {
    pub fn render_confidence_interval(
        config: &ConfidenceIntervalConfig,
        canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<(), VisualizationError>;
    
    pub fn render_significance_indicators(
        indicators: &[SignificanceIndicator],
        canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<(), VisualizationError>;
}
```

## Usage Example

```rust
use bi_visualization::{
    StatisticalChartConfig,
    ConfidenceIntervalConfig,
    SignificanceIndicator,
    ChartConfig,
    ChartType,
    VisualizationTheme,
    DataSeries,
};

// Create base chart configuration
let base_config = ChartConfig::new(
    ChartType::Line,
    "Financial Forecast".to_string(),
    (800, 600),
    VisualizationTheme::Light,
    vec![],
);

// Create confidence interval
let ci_config = ConfidenceIntervalConfig::new(
    main_series,
    upper_bound_series,
    lower_bound_series,
    0.95,
    "#808080".to_string(),
    false,
);

// Create significance indicators
let indicators = vec![SignificanceIndicator::new(
    0.001, // p-value
    (100.0, 200.0), // position
    Some("Highly Significant Trend".to_string()),
)];

// Create statistical chart configuration
let statistical_config = StatisticalChartConfig::new(
    base_config,
    vec![ci_config],
    indicators,
    true, // show explanations
);

// Generate chart
let chart = VisualizationService::generate_statistical_chart(statistical_config, data)?;
```

## Cooperative Values Integration

All statistical visualizations include:
- **"Explain This"** buttons for detailed methodology
- **Color-coded significance** indicators (Green/Yellow/Red)
- **Confidence interval** visualization with clear labeling
- **Plain-language explanations** integrated into visualizations

## Platform-Specific Considerations

### Web Platform
- Implements progressive loading for statistical computations
- Uses Web Workers for heavy statistical calculations
- Respects 5MB memory limit for web context

### Win64 Platform
- Leverages rayon for parallel processing of statistical operations
- Uses memory-mapped files for large datasets
- Implements incremental computation for live data

## Performance Optimization

### Memory Management
- Automatic down-sampling for datasets >5k rows in web context
- Chunked processing for large statistical computations
- Caching of intermediate statistical results

### Progressive Loading
- Initial rendering with basic data
- Asynchronous computation of confidence intervals
- Real-time updates as statistical analysis completes

## Testing

The statistical visualization components are tested for:
- Visual correctness of confidence interval rendering
- Proper color coding of significance indicators
- Cross-platform consistency (Web vs Win64)
- Performance within memory and time constraints
- User validation of statistical explanations

Run visualization tests with:

```bash
cargo test -p cpc-bi-visualization
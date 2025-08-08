# BI Analytics Framework

A comprehensive business intelligence and analytics framework designed for the Cooperative Software Ecosystem, with built-in cooperative values integration, privacy-aware processing, and visualization capabilities.

## Overview

The BI Analytics Framework provides a complete solution for data analysis within cooperative software ecosystems. It emphasizes community benefit, privacy protection, and transparent data processing while delivering powerful analytical capabilities.

## Features

### Core Analytics Engine
- Data normalization and cleaning
- Descriptive statistics generation
- Data transformation capabilities
- Memory-efficient processing with streaming support
- Integration with Polars for high-performance data analysis

### Cooperative Values Integration
- Community impact weighting
- Transparent algorithmic explanations
- Cooperative governance validation
- Impact explorer for community benefit metrics

### Privacy-Aware Processing
- Consent-aware data processing
- Differential privacy implementation
- PII anonymization based on consent levels
- Integration with consent management systems

### Data Pipeline
- Extensible adapter system for various data sources
- Built-in adapters for:
  - Cause Management
  - Skill Development
  - Volunteer Coordination
- Data transformation capabilities
- Streaming data processing

### Visualization Integration
- Data preparation for visualization
- Web-compatible data streaming
- Chart type support (bar, line, pie, scatter)
- Integration-ready JSON output

## Architecture

```
bi_analytics/
├── src/
│   ├── lib.rs              # Public API exports
│   ├── engine.rs           # Core analytics engine
│   ├── error.rs            # Error types
│   ├── privacy.rs          # Privacy-aware processing
│   ├── cooperative_values.rs # Cooperative values integration
│   ├── pipeline/           # Data pipeline components
│   │   ├── mod.rs          # Pipeline module
│   │   ├── adapter.rs      # Adapter traits
│   │   └── sources/        # Built-in adapters
│   └── visualization.rs    # Visualization integration
├── examples/               # Usage examples
├── tests/                  # Integration tests
└── Cargo.toml             # Package configuration
```

## Usage

### Basic Analytics Engine

```rust
use bi_analytics::AnalyticsEngine;
use polars::df;

// Create analytics engine
let engine = AnalyticsEngine::new();

// Create sample data
let data = df![
    "project" => ["Project A", "Project B"],
    "funding" => [10000.0, 15000.0]
]?;

// Normalize data
let normalized = engine.normalize_data(&data)?;

// Generate statistics
let stats = engine.generate_descriptive_stats(&data)?;
```

### Data Pipeline

```rust
use bi_analytics::{AnalyticsEngine, DataPipeline};
use bi_analytics::pipeline::CauseManagementAdapter;

// Create pipeline
let engine = AnalyticsEngine::new();
let mut pipeline = DataPipeline::new(engine);

// Add data source
pipeline.add_adapter("causes".to_string(), Box::new(CauseManagementAdapter {}));

// Process data
let processed = pipeline.process_all()?;
```

### Privacy-Aware Processing

```rust
use bi_analytics::privacy::{PrivacySettings, ConsentAwareProcessor};
use consent_manager::domain::consent::DataSharingLevel;

// Configure privacy settings
let privacy_settings = PrivacySettings {
    minimum_consent_level: DataSharingLevel::Standard,
    apply_differential_privacy: true,
    differential_privacy_epsilon: 1.0,
    anonymize_by_default: true,
};

let processor = ConsentAwareProcessor::new(privacy_settings);

// Process with consent awareness
let anonymized = processor.apply_anonymization(data, DataSharingLevel::Standard)?;
```

### Cooperative Values Integration

```rust
use bi_analytics::cooperative_values::{CooperativeValues, ImpactExplorer};

// Configure cooperative values
let coop_values = CooperativeValues {
    prioritize_community_benefit: true,
    community_impact_weight: 2.0,
    show_transparency: true,
    enable_community_validation: true,
};

let explorer = ImpactExplorer::new(coop_values);

// Calculate impact-weighted metrics
let weighted_metrics = explorer.calculate_impact_weighted_metrics(data)?;
```

### Visualization Integration

```rust
use bi_analytics::visualization::VisualizationIntegration;

// Create visualization integration
let visualization = VisualizationIntegration::new(engine);

// Prepare data for visualization
let viz_data = visualization.to_visualization_data(&data, VisualizationChartType::BarChart)?;

// Stream data for web
let mut data_stream = visualization.stream_data_for_web(&data, 100)?;
```

## Integration with Other Packages

This package integrates with several other components of the CPC ecosystem:

- **consent_manager**: For privacy-aware processing based on user consent
- **cause_management**: Through built-in adapters
- **skill_development**: Through built-in adapters
- **volunteer_coordination**: Through built-in adapters
- **plotters**: For advanced visualization capabilities

## Testing

Run the integration tests:

```bash
cargo test --package bi_analytics
```

## Examples

See the `examples/` directory for complete usage examples:

- `basic_usage.rs`: Core analytics engine usage
- `pipeline_example.rs`: Data pipeline implementation
- `privacy_integration.rs`: Privacy-aware processing
- `cooperative_values.rs`: Cooperative values integration
- `visualization_integration.rs`: Visualization preparation

## License

This package is part of the Cooperative Software Ecosystem and is licensed under the CPC License.

## Contributing

Contributions are welcome! Please ensure all code follows the cooperative values principles and includes appropriate tests.
# Basic Chart Visualization Example

This example demonstrates how to create a simple bar chart visualization in a CPC application.

## Overview

This example shows:
- Setting up the visualization client
- Creating a basic bar chart request
- Handling visualization responses
- Implementing basic accessibility features

## Prerequisites

- CPC development environment
- Rust programming knowledge
- Access to API Gateway service

## Running the Example

1. Ensure the API Gateway is running on `http://localhost:3001`
2. Navigate to this directory
3. Run the example:

```bash
cargo run
```

## Key Components

### Chart Data Structure

The example uses a simple data structure for chart data:

```rust
struct ChartData {
    labels: Vec<String>,
    values: Vec<f64>,
}
```

### Visualization Request

Creates a visualization request with appropriate context:

```rust
let request = VisualizationRequest {
    visualization_id: "bar-chart-example".to_string(),
    parameters: VisualizationParameters {
        width: 800,
        height: 600,
        lod_level: 2,
        accessibility_mode: "standard".to_string(),
    },
    context: VisualizationContext {
        originating_app: "example_app".to_string(),
        user_id: "user-123".to_string(),
        sharing_scope: SharingScope::Private("user-123".parse().unwrap()),
        accessibility_mode: AccessibilityMode::Standard,
        lod_level: 2,
    },
}
```

## Expected Output

When running successfully, you should see output similar to:

```
Starting basic chart example...
Visualization generated successfully!
Visualization type: image
Cache TTL: 300 seconds
Alt text: Bar chart showing 5 data points. Values range from 100 to 250. Highest value is 250 in May.
```

## Customization

You can customize this example by:

1. Modifying the chart data in `main.rs`
2. Changing visualization parameters
3. Implementing different chart types
4. Adding more advanced accessibility features

## Related Documentation

- [Basic Chart Implementation Tutorial](../../../docs/tutorials/basic_chart_implementation.md)
- [Visualization Setup Guide](../../../docs/developer/visualization_setup.md)
- [Visualization Architecture](../../../docs/developer/visualization_architecture.md)
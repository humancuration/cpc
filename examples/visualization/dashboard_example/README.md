# Complex Dashboard Visualization Example

This example demonstrates how to create a complex dashboard with multiple visualizations, real-time updates, and advanced features.

## Overview

This example shows:
- Creating a dashboard with multiple visualization widgets
- Implementing real-time data updates using WebSockets
- Managing visualization state and caching
- Handling complex user interactions
- Implementing advanced accessibility features

## Prerequisites

- CPC development environment
- Rust programming knowledge
- Understanding of WebSocket connections
- Access to API Gateway service

## Running the Example

1. Ensure the API Gateway is running on `http://localhost:3001`
2. Navigate to this directory
3. Run the example:

```bash
cargo run
```

## Key Components

### Dashboard Structure

The example implements a dashboard with multiple widgets:

```rust
struct DashboardWidget {
    id: String,
    title: String,
    visualization_id: String,
    position: (u32, u32),
    size: (u32, u32),
    parameters: HashMap<String, serde_json::Value>,
}
```

### WebSocket Integration

Real-time updates are handled through WebSocket connections:

```rust
let ws_client = WebSocketClient::new(
    "ws://localhost:3001/visualizations/realtime/ws".to_string(),
    ws_callback,
);
```

### Caching Manager

Visualization caching improves performance:

```rust
let cache_manager = CacheManager::new("./dashboard_cache", 300)?;
```

## Expected Output

When running successfully, you should see output similar to:

```
Starting complex dashboard example...
Dashboard: Sales Dashboard
Widgets: 3
Theme: dark
WebSocket connection established
Created visualization request for 'Sales Overview': sales-bar-chart
Subscribed to real-time updates for Real-time Data Stream
Dashboard example completed successfully!
```

## Dashboard Layout

The example dashboard includes:

1. **Sales Overview** - Bar chart showing sales data
2. **Performance Metrics** - Gauge visualization for system metrics
3. **Real-time Data Stream** - Line chart with live updates

## Customization

You can customize this example by:

1. Adding new widget types
2. Modifying the dashboard layout
3. Implementing custom themes
4. Adding more sophisticated caching strategies
5. Extending accessibility features

## Related Documentation

- [Complex Dashboard Tutorial](../../../docs/tutorials/complex_dashboard.md)
- [Visualization Architecture Guide](../../../docs/developer/visualization_architecture.md)
- [Basic Chart Implementation Tutorial](../../../docs/tutorials/basic_chart_implementation.md)
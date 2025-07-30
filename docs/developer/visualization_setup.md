# Visualization Setup Guide

This guide provides instructions for setting up the BI Visualization framework in CPC applications.

## Prerequisites

Before integrating visualization capabilities, ensure you have:

- CPC development environment set up
- Access to the API Gateway service
- Required dependencies installed:
  - `packages/visualization_context` crate
  - Bevy engine (v0.16)
  - Plotters library
  - Sled database for caching

## Installation

Add the following dependencies to your application's `Cargo.toml`:

```toml
[dependencies]
visualization_context = { path = "../../packages/visualization_context" }
bevy = "0.16"
plotters = "0.3"
sled = "0.34"
```

## Configuration

### API Gateway Integration

Configure your application to communicate with the visualization API gateway:

```rust
use visualization_context::{VisualizationContext, SharingScope};

// Create a visualization context
let context = VisualizationContext {
    originating_app: "dashboard".to_string(),
    user_id: "user-uuid-here".to_string(),
    sharing_scope: SharingScope::Team(team_id),
    accessibility_mode: AccessibilityMode::ScreenReader,
    lod_level: 2,
};
```

### Caching Setup

The visualization framework uses a multi-tier caching strategy:

1. **Edge Cache** (Sled) - Local to each node
2. **Regional Cache** (Redis) - Shared within region
3. **Origin Cache** - Service-level caching

Configure Sled caching in your application:

```toml
# config/cache.toml
[visualization_cache]
type = "sled"
path = "./data/visualization_cache"
ttl_seconds = 300
```

## Basic Integration

### 1. Initialize the Visualization Client

```rust
use visualization_context::VisualizationClient;

let client = VisualizationClient::new("http://api-gateway:3001");
```

### 2. Request a Visualization

```rust
use visualization_context::{VisualizationRequest, VisualizationResponse};

let request = VisualizationRequest {
    visualization_id: "chart-uuid-here".to_string(),
    parameters: VisualizationParameters {
        width: 800,
        height: 600,
        lod_level: 2,
        accessibility_mode: "screen_reader".to_string(),
    },
    context: context,
};

let response: VisualizationResponse = client.get_visualization(request).await?;
```

## Environment Variables

Set the following environment variables for your application:

```bash
# API Gateway endpoint
VISUALIZATION_API_URL=http://api-gateway:3001

# Caching configuration
CACHE_TTL_SECONDS=300
CACHE_MAX_SIZE_MB=100

# Accessibility settings
DEFAULT_ACCESSIBILITY_MODE=standard
```

## Testing Integration

To verify your setup, run the integration tests:

```bash
cargo test --package your-app --lib visualization
```

### Sample Test

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_visualization_request() {
        let client = VisualizationClient::new("http://localhost:3001");
        let context = create_test_context();
        
        let request = VisualizationRequest {
            visualization_id: "test-chart".to_string(),
            parameters: VisualizationParameters {
                width: 400,
                height: 300,
                lod_level: 1,
                accessibility_mode: "standard".to_string(),
            },
            context,
        };
        
        let response = client.get_visualization(request).await;
        assert!(response.is_ok());
    }
}
```

## Troubleshooting

Common setup issues and solutions:

1. **Connection Refused to API Gateway**
   - Verify the API gateway is running
   - Check network connectivity
   - Confirm the gateway URL is correct

2. **Missing Dependencies**
   - Ensure all required crates are in Cargo.toml
   - Run `cargo build` to fetch dependencies

3. **Caching Issues**
   - Check Sled database permissions
   - Verify cache path exists and is writable

## Next Steps

After successful setup, proceed to:

- [Visualization Architecture Guide](./visualization_architecture.md) for deeper understanding
- [Basic Chart Implementation Tutorial](../tutorials/basic_chart_implementation.md) for hands-on examples
- [User Guide](../user/visualization_guide.md) for end-user documentation

## Compliance Considerations

When implementing visualization features, ensure:

- All data sharing respects CPC's cooperative principles
- User-controlled data sharing preferences are respected
- Accessibility compliance is maintained
- PII detection and redaction is properly implemented
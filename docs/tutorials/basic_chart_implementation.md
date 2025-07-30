# Basic Chart Implementation Tutorial

This tutorial walks you through creating a basic chart visualization in a CPC application.

## Prerequisites

Before starting this tutorial, ensure you have:

- Completed the [Visualization Setup Guide](../developer/visualization_setup.md)
- Basic knowledge of Rust programming
- Familiarity with the Bevy engine concepts
- Access to a CPC development environment

## Learning Objectives

By the end of this tutorial, you will be able to:

1. Create a simple bar chart visualization
2. Configure visualization parameters
3. Handle visualization responses
4. Display visualizations in your application
5. Implement basic accessibility features

## Step 1: Project Setup

First, create a new Rust project for our tutorial:

```bash
cargo new basic_chart_tutorial
cd basic_chart_tutorial
```

Add the required dependencies to `Cargo.toml`:

```toml
[dependencies]
visualization_context = { path = "../../packages/visualization_context" }
bevy = "0.16"
plotters = "0.3"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Step 2: Create the Visualization Request

Create a new file `src/chart.rs`:

```rust
use visualization_context::{
    VisualizationContext, VisualizationRequest, VisualizationParameters, 
    SharingScope, AccessibilityMode
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub values: Vec<f64>,
}

pub fn create_bar_chart_request(
    chart_data: ChartData,
    width: u32,
    height: u32,
) -> VisualizationRequest {
    // Create visualization context
    let context = VisualizationContext {
        originating_app: "tutorial_app".to_string(),
        user_id: "user-123".to_string(),
        sharing_scope: SharingScope::Private("user-123".parse().unwrap()),
        accessibility_mode: AccessibilityMode::Standard,
        lod_level: 2,
    };
    
    // Create visualization parameters
    let parameters = VisualizationParameters {
        width,
        height,
        lod_level: 2,
        accessibility_mode: "standard".to_string(),
    };
    
    // Create request
    VisualizationRequest {
        visualization_id: "bar-chart-tutorial".to_string(),
        parameters,
        context,
    }
}
```

## Step 3: Implement the Visualization Client

Create `src/client.rs`:

```rust
use visualization_context::{VisualizationRequest, VisualizationResponse};
use reqwest;
use serde_json;

pub struct VisualizationClient {
    base_url: String,
    client: reqwest::Client,
}

impl VisualizationClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: reqwest::Client::new(),
        }
    }
    
    pub async fn get_visualization(
        &self,
        request: VisualizationRequest,
    ) -> Result<VisualizationResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/visualizations/{}", self.base_url, request.visualization_id);
        
        let response = self.client
            .get(&url)
            .json(&request)
            .send()
            .await?;
            
        let visualization_response: VisualizationResponse = response.json().await?;
        Ok(visualization_response)
    }
}
```

## Step 4: Create the Main Application

Update `src/main.rs`:

```rust
mod chart;
mod client;

use chart::{ChartData, create_bar_chart_request};
use client::VisualizationClient;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting basic chart tutorial...");
    
    // Sample data for our chart
    let chart_data = ChartData {
        labels: vec![
            "January".to_string(),
            "February".to_string(),
            "March".to_string(),
            "April".to_string(),
            "May".to_string(),
        ],
        values: vec![100.0, 150.0, 200.0, 175.0, 250.0],
    };
    
    // Create visualization request
    let request = create_bar_chart_request(chart_data, 800, 600);
    
    // Initialize client
    let client = VisualizationClient::new("http://localhost:3001");
    
    // Request visualization
    match client.get_visualization(request).await {
        Ok(response) => {
            println!("Visualization generated successfully!");
            println!("Visualization type: {}", response.visualization_data.type_field);
            println!("Cache TTL: {} seconds", response.metadata.cache_ttl);
            
            // In a real application, you would now render the visualization
            // For this tutorial, we'll just print the accessibility information
            if let Some(accessibility) = response.visualization_data.accessibility {
                println!("Alt text: {}", accessibility.alt_text);
            }
        }
        Err(e) => {
            eprintln!("Error generating visualization: {}", e);
        }
    }
    
    Ok(())
}
```

## Step 5: Add Data Structures

Add the missing data structures to `src/chart.rs`:

```rust
use visualization_context::*;
use serde::{Deserialize, Serialize};

// Add these at the top of the file
#[derive(Debug, Serialize, Deserialize)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub values: Vec<f64>,
}

// Add these implementations at the end of the file
impl From<ChartData> for serde_json::Value {
    fn from(data: ChartData) -> Self {
        serde_json::json!({
            "labels": data.labels,
            "values": data.values,
        })
    }
}

pub fn create_bar_chart_request(
    chart_data: ChartData,
    width: u32,
    height: u32,
) -> VisualizationRequest {
    // Create visualization context
    let context = VisualizationContext {
        originating_app: "tutorial_app".to_string(),
        user_id: "user-123".to_string(),
        sharing_scope: SharingScope::Private("user-123".parse().unwrap()),
        accessibility_mode: AccessibilityMode::Standard,
        lod_level: 2,
    };
    
    // Create visualization parameters
    let parameters = VisualizationParameters {
        width,
        height,
        lod_level: 2,
        accessibility_mode: "standard".to_string(),
    };
    
    // Create request
    VisualizationRequest {
        visualization_id: "bar-chart-tutorial".to_string(),
        parameters,
        context,
    }
}
```

## Step 6: Implement Accessibility Features

Create `src/accessibility.rs`:

```rust
use visualization_context::AccessibilityMetadata;
use serde_json::Value;

pub fn generate_alt_text(chart_type: &str, data_labels: &[String], data_values: &[f64]) -> String {
    match chart_type {
        "bar" => {
            let max_value = data_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let min_value = data_values.iter().cloned().fold(f64::INFINITY, f64::min);
            
            format!(
                "Bar chart showing {} data points. Values range from {} to {}. Highest value is {} in {}.",
                data_labels.len(),
                min_value,
                max_value,
                max_value,
                data_labels[data_values.iter().position(|&x| x == max_value).unwrap_or(0)]
            )
        }
        _ => "Chart visualization".to_string(),
    }
}

pub fn create_navigation_map(data_labels: &[String]) -> std::collections::HashMap<String, Value> {
    let mut map = std::collections::HashMap::new();
    
    // Add title navigation
    map.insert("T".to_string(), serde_json::json!({
        "label": "Chart Title",
        "position": [0, 3, 0]
    }));
    
    // Add legend navigation
    map.insert("L".to_string(), serde_json::json!({
        "label": "Legend",
        "position": [-3, 0, 0]
    }));
    
    // Add data navigation
    map.insert("D".to_string(), serde_json::json!({
        "label": "Data Points",
        "position": [0, 0, 0]
    }));
    
    map
}

pub fn create_accessibility_metadata(
    chart_type: &str,
    data_labels: &[String],
    data_values: &[f64],
) -> AccessibilityMetadata {
    AccessibilityMetadata {
        alt_text: generate_alt_text(chart_type, data_labels, data_values),
        navigation_map: create_navigation_map(data_labels),
        live_region: "polite".to_string(),
    }
}
```

## Step 7: Update Main Application with Accessibility

Update `src/main.rs` to include accessibility:

```rust
mod chart;
mod client;
mod accessibility;

use chart::{ChartData, create_bar_chart_request};
use client::VisualizationClient;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting basic chart tutorial...");
    
    // Sample data for our chart
    let chart_data = ChartData {
        labels: vec![
            "January".to_string(),
            "February".to_string(),
            "March".to_string(),
            "April".to_string(),
            "May".to_string(),
        ],
        values: vec![100.0, 150.0, 200.0, 175.0, 250.0],
    };
    
    // Create visualization request
    let request = create_bar_chart_request(chart_data, 800, 600);
    
    // Initialize client
    let client = VisualizationClient::new("http://localhost:3001");
    
    // Request visualization
    match client.get_visualization(request).await {
        Ok(response) => {
            println!("Visualization generated successfully!");
            println!("Visualization type: {}", response.visualization_data.type_field);
            println!("Cache TTL: {} seconds", response.metadata.cache_ttl);
            
            // Display accessibility information
            if let Some(accessibility) = response.visualization_data.accessibility {
                println!("Alt text: {}", accessibility.alt_text);
                println!("Keyboard shortcuts: T=Title, L=Legend, D=Data Points");
                println!("Live region: {}", accessibility.live_region);
            }
        }
        Err(e) => {
            eprintln!("Error generating visualization: {}", e);
        }
    }
    
    Ok(())
}
```

## Step 8: Testing the Implementation

To test your implementation:

1. Ensure the API Gateway is running on `http://localhost:3001`
2. Run the tutorial application:

```bash
cargo run
```

You should see output similar to:

```
Starting basic chart tutorial...
Visualization generated successfully!
Visualization type: image
Cache TTL: 300 seconds
Alt text: Bar chart showing 5 data points. Values range from 100 to 250. Highest value is 250 in May.
Keyboard shortcuts: T=Title, L=Legend, D=Data Points
Live region: polite
```

## Step 9: Extending the Tutorial

### Adding Error Handling

Improve error handling in your client:

```rust
impl VisualizationClient {
    // ... existing code ...
    
    pub async fn get_visualization(
        &self,
        request: VisualizationRequest,
    ) -> Result<VisualizationResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/visualizations/{}", self.base_url, request.visualization_id);
        
        let response = self.client
            .get(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;
            
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("API error {}: {}", status, error_text).into());
        }
            
        let visualization_response: VisualizationResponse = response
            .json()
            .await
            .map_err(|e| format!("JSON parsing error: {}", e))?;
            
        Ok(visualization_response)
    }
}
```

### Adding Configuration

Create `config/settings.toml`:

```toml
[visualization]
api_url = "http://localhost:3001"
default_width = 800
default_height = 600
cache_enabled = true

[accessibility]
default_mode = "standard"
screen_reader_support = true
```

## Troubleshooting

### Common Issues

1. **Connection Refused**
   - Ensure the API Gateway is running
   - Check the URL in your client initialization
   - Verify network connectivity

2. **Serialization Errors**
   - Check that all structs derive Serialize and Deserialize
   - Ensure field names match between Rust and JSON

3. **Missing Dependencies**
   - Run `cargo build` to fetch all dependencies
   - Check that paths to local crates are correct

### Debugging Tips

1. **Enable Logging**
   Add logging to see detailed request/response information:

   ```rust
   use tracing::{info, error};
   
   // In your functions:
   info!("Sending visualization request: {:?}", request);
   ```

2. **Test with curl**
   You can also test the API directly with curl:

   ```bash
   curl -X GET http://localhost:3001/visualizations/bar-chart-tutorial \
        -H "Content-Type: application/json" \
        -d '{"visualization_id":"bar-chart-tutorial","parameters":{"width":800,"height":600,"lod_level":2,"accessibility_mode":"standard"},"context":{"originating_app":"tutorial_app","user_id":"user-123","sharing_scope":"private","accessibility_mode":"standard","lod_level":2}}'
   ```

## Next Steps

After completing this tutorial, consider exploring:

1. [Complex Dashboard Tutorial](./complex_dashboard.md) - Learn to create advanced dashboard layouts
2. [Accessibility Demo](./accessibility_demo.md) - Deep dive into accessibility features
3. [Visualization Architecture Guide](../developer/visualization_architecture.md) - Understand the system architecture

## Conclusion

You've successfully created a basic chart visualization in a CPC application! You've learned how to:

- Set up the visualization client
- Create visualization requests with appropriate context
- Handle visualization responses
- Implement basic accessibility features
- Test and debug your implementation

This foundation will serve you well as you build more complex visualization applications within the CPC ecosystem.
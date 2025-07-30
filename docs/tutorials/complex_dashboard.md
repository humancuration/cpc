# Complex Dashboard Implementation Tutorial

This tutorial walks you through creating a complex dashboard with multiple visualizations, real-time updates, and advanced features.

## Prerequisites

Before starting this tutorial, ensure you have:

- Completed the [Basic Chart Implementation Tutorial](./basic_chart_implementation.md)
- Understanding of WebSocket connections
- Familiarity with asynchronous programming in Rust
- Access to a CPC development environment

## Learning Objectives

By the end of this tutorial, you will be able to:

1. Create a dashboard with multiple visualization widgets
2. Implement real-time data updates using WebSockets
3. Manage visualization state and caching
4. Handle complex user interactions
5. Implement advanced accessibility features

## Step 1: Project Setup

Create a new Rust project for our dashboard:

```bash
cargo new complex_dashboard_tutorial
cd complex_dashboard_tutorial
```

Add the required dependencies to `Cargo.toml`:

```toml
[dependencies]
visualization_context = { path = "../../packages/visualization_context" }
bevy = "0.16"
plotters = "0.3"
tokio = { version = "1.0", features = ["full"] }
tokio-tungstenite = "0.17"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
futures-util = "0.3"
uuid = { version = "1.0", features = ["v4"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

## Step 2: Define Dashboard Structure

Create `src/dashboard.rs`:

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardWidget {
    pub id: String,
    pub title: String,
    pub visualization_id: String,
    pub position: (u32, u32), // x, y coordinates
    pub size: (u32, u32),     // width, height
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardLayout {
    pub id: String,
    pub name: String,
    pub widgets: Vec<DashboardWidget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardState {
    pub layout: DashboardLayout,
    pub user_preferences: UserPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub theme: String,
    pub accessibility_mode: String,
    pub auto_refresh: bool,
    pub refresh_interval: u32, // seconds
}

impl Default for DashboardState {
    fn default() -> Self {
        Self {
            layout: DashboardLayout {
                id: Uuid::new_v4().to_string(),
                name: "Default Dashboard".to_string(),
                widgets: vec![],
            },
            user_preferences: UserPreferences {
                theme: "light".to_string(),
                accessibility_mode: "standard".to_string(),
                auto_refresh: true,
                refresh_interval: 60,
            },
        }
    }
}

impl DashboardState {
    pub fn add_widget(&mut self, widget: DashboardWidget) {
        self.layout.widgets.push(widget);
    }
    
    pub fn remove_widget(&mut self, widget_id: &str) {
        self.layout.widgets.retain(|w| w.id != widget_id);
    }
    
    pub fn update_widget_parameters(&mut self, widget_id: &str, parameters: HashMap<String, serde_json::Value>) {
        if let Some(widget) = self.layout.widgets.iter_mut().find(|w| w.id == widget_id) {
            widget.parameters = parameters;
        }
    }
}
```

## Step 3: Create Visualization Manager

Create `src/visualization_manager.rs`:

```rust
use crate::dashboard::{DashboardWidget, DashboardState};
use visualization_context::{
    VisualizationContext, VisualizationRequest, VisualizationParameters,
    SharingScope, AccessibilityMode
};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

pub struct VisualizationManager {
    base_url: String,
}

impl VisualizationManager {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }
    
    pub fn create_visualization_request(
        &self,
        widget: &DashboardWidget,
        state: &DashboardState,
    ) -> VisualizationRequest {
        // Create visualization context
        let context = VisualizationContext {
            originating_app: "dashboard".to_string(),
            user_id: "current-user-id".to_string(), // In real app, get from session
            sharing_scope: SharingScope::Private("current-user-id".parse().unwrap()),
            accessibility_mode: match state.user_preferences.accessibility_mode.as_str() {
                "screen_reader" => AccessibilityMode::ScreenReader,
                "high_contrast" => AccessibilityMode::HighContrast,
                _ => AccessibilityMode::Standard,
            },
            lod_level: 2, // Could be configurable per widget
        };
        
        // Create visualization parameters
        let parameters = VisualizationParameters {
            width: widget.size.0,
            height: widget.size.1,
            lod_level: 2,
            accessibility_mode: state.user_preferences.accessibility_mode.clone(),
        };
        
        // Create request
        VisualizationRequest {
            visualization_id: widget.visualization_id.clone(),
            parameters,
            context,
        }
    }
    
    pub fn create_dashboard_layout(
        &self,
        name: &str,
        widgets_config: Vec<WidgetConfig>,
    ) -> DashboardState {
        let mut state = DashboardState::default();
        state.layout.name = name.to_string();
        
        for config in widgets_config {
            let widget = DashboardWidget {
                id: Uuid::new_v4().to_string(),
                title: config.title,
                visualization_id: config.visualization_id,
                position: config.position,
                size: config.size,
                parameters: config.parameters,
            };
            state.add_widget(widget);
        }
        
        state
    }
}

#[derive(Debug)]
pub struct WidgetConfig {
    pub title: String,
    pub visualization_id: String,
    pub position: (u32, u32),
    pub size: (u32, u32),
    pub parameters: HashMap<String, Value>,
}
```

## Step 4: Implement WebSocket Client

Create `src/websocket_client.rs`:

```rust
use tokio_tungstenite::{
    connect_async,
    tungstenite::protocol::Message,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationUpdate {
    pub visualization_id: String,
    pub data: serde_json::Value,
    pub timestamp: u64,
}

pub type UpdateCallback = Arc<dyn Fn(VisualizationUpdate) + Send + Sync>;

pub struct WebSocketClient {
    url: String,
    update_callback: UpdateCallback,
    is_connected: Arc<Mutex<bool>>,
}

impl WebSocketClient {
    pub fn new(url: String, callback: UpdateCallback) -> Self {
        Self {
            url,
            update_callback: callback,
            is_connected: Arc::new(Mutex::new(false)),
        }
    }
    
    pub async fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Connecting to WebSocket: {}", self.url);
        
        let (ws_stream, _) = connect_async(&self.url).await?;
        let (mut write, mut read) = ws_stream.split();
        
        // Mark as connected
        {
            let mut connected = self.is_connected.lock().await;
            *connected = true;
        }
        
        info!("WebSocket connection established");
        
        // Clone callback for use in async block
        let callback = self.update_callback.clone();
        
        // Spawn task to handle incoming messages
        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        match serde_json::from_str::<VisualizationUpdate>(&text) {
                            Ok(update) => {
                                info!("Received visualization update for: {}", update.visualization_id);
                                callback(update);
                            }
                            Err(e) => {
                                error!("Failed to parse visualization update: {}", e);
                            }
                        }
                    }
                    Ok(Message::Close(_)) => {
                        info!("WebSocket connection closed");
                        break;
                    }
                    Err(e) => {
                        error!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });
        
        Ok(())
    }
    
    pub async fn subscribe_to_visualization(&self, visualization_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let connected = {
            let connected = self.is_connected.lock().await;
            *connected
        };
        
        if !connected {
            return Err("Not connected to WebSocket".into());
        }
        
        // In a real implementation, you would send a subscription message
        info!("Subscribed to visualization updates for: {}", visualization_id);
        Ok(())
    }
    
    pub async fn is_connected(&self) -> bool {
        let connected = self.is_connected.lock().await;
        *connected
    }
}
```

## Step 5: Create Main Dashboard Application

Create `src/main.rs`:

```rust
mod dashboard;
mod visualization_manager;
mod websocket_client;

use dashboard::{DashboardState, UserPreferences};
use visualization_manager::{VisualizationManager, WidgetConfig};
use websocket_client::{WebSocketClient, VisualizationUpdate};
use std::collections::HashMap;
use serde_json::Value;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("Starting complex dashboard tutorial...");
    
    // Initialize visualization manager
    let viz_manager = VisualizationManager::new("http://localhost:3001");
    
    // Create dashboard layout
    let widgets_config = vec![
        WidgetConfig {
            title: "Sales Overview".to_string(),
            visualization_id: "sales-bar-chart".to_string(),
            position: (0, 0),
            size: (400, 300),
            parameters: {
                let mut params = HashMap::new();
                params.insert("chart_type".to_string(), Value::String("bar".to_string()));
                params.insert("data_source".to_string(), Value::String("sales_db".to_string()));
                params
            },
        },
        WidgetConfig {
            title: "Performance Metrics".to_string(),
            visualization_id: "performance-gauge".to_string(),
            position: (400, 0),
            size: (200, 200),
            parameters: {
                let mut params = HashMap::new();
                params.insert("chart_type".to_string(), Value::String("gauge".to_string()));
                params.insert("metric".to_string(), Value::String("cpu_usage".to_string()));
                params
            },
        },
        WidgetConfig {
            title: "Real-time Data Stream".to_string(),
            visualization_id: "realtime-line-chart".to_string(),
            position: (0, 300),
            size: (600, 200),
            parameters: {
                let mut params = HashMap::new();
                params.insert("chart_type".to_string(), Value::String("line".to_string()));
                params.insert("refresh_rate".to_string(), Value::Number(serde_json::Number::from(1000)));
                params
            },
        },
    ];
    
    let mut dashboard_state = viz_manager.create_dashboard_layout("Sales Dashboard", widgets_config);
    
    // Update user preferences
    dashboard_state.user_preferences = UserPreferences {
        theme: "dark".to_string(),
        accessibility_mode: "screen_reader".to_string(),
        auto_refresh: true,
        refresh_interval: 30,
    };
    
    // Print dashboard information
    println!("Dashboard: {}", dashboard_state.layout.name);
    println!("Widgets: {}", dashboard_state.layout.widgets.len());
    println!("Theme: {}", dashboard_state.user_preferences.theme);
    
    // Initialize WebSocket client for real-time updates
    let ws_callback = Arc::new(|update: VisualizationUpdate| {
        println!("Real-time update received for {}: {:?}", update.visualization_id, update.data);
    });
    
    let ws_client = WebSocketClient::new(
        "ws://localhost:3001/visualizations/realtime/ws".to_string(),
        ws_callback,
    );
    
    // Attempt to connect to WebSocket
    match ws_client.connect().await {
        Ok(_) => println!("WebSocket connection established"),
        Err(e) => println!("Failed to connect to WebSocket: {}", e),
    }
    
    // Create visualization requests for each widget
    for widget in &dashboard_state.layout.widgets {
        let request = viz_manager.create_visualization_request(widget, &dashboard_state);
        println!("Created visualization request for '{}': {}", widget.title, request.visualization_id);
        
        // In a real application, you would send this request to the API
        // and handle the response to render the visualization
    }
    
    // Subscribe to real-time updates for the line chart
    if let Some(realtime_widget) = dashboard_state.layout.widgets.iter().find(|w| w.visualization_id == "realtime-line-chart") {
        match ws_client.subscribe_to_visualization(&realtime_widget.visualization_id).await {
            Ok(_) => println!("Subscribed to real-time updates for {}", realtime_widget.title),
            Err(e) => println!("Failed to subscribe to updates: {}", e),
        }
    }
    
    // Simulate dashboard interaction
    println!("\nSimulating dashboard interaction...");
    
    // Update a widget's parameters
    if let Some(first_widget) = dashboard_state.layout.widgets.first() {
        let mut new_params = HashMap::new();
        new_params.insert("time_range".to_string(), Value::String("last_30_days".to_string()));
        new_params.insert("aggregation".to_string(), Value::String("average".to_string()));
        
        dashboard_state.update_widget_parameters(&first_widget.id, new_params);
        println!("Updated parameters for widget: {}", first_widget.title);
    }
    
    println!("Dashboard tutorial completed successfully!");
    
    // In a real application, you would keep the event loop running
    // to handle real-time updates and user interactions
    // For this tutorial, we'll just exit
    
    Ok(())
}
```

## Step 6: Add Advanced Accessibility Features

Create `src/accessibility.rs`:

```rust
use crate::dashboard::{DashboardState, DashboardWidget};
use serde_json::Value;
use std::collections::HashMap;

pub struct AccessibilityManager;

impl AccessibilityManager {
    pub fn generate_dashboard_alt_text(state: &DashboardState) -> String {
        format!(
            "Dashboard '{}' containing {} widgets. Navigation: Use Tab to move between widgets, Enter to activate.",
            state.layout.name,
            state.layout.widgets.len()
        )
    }
    
    pub fn generate_widget_navigation_map(widgets: &[DashboardWidget]) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        
        // Add dashboard-level navigation
        map.insert("D".to_string(), serde_json::json!({
            "label": "Dashboard Title",
            "position": [0, 5, 0]
        }));
        
        // Add widget-specific navigation
        for (index, widget) in widgets.iter().enumerate() {
            let key = format!("W{}", index + 1);
            map.insert(key, serde_json::json!({
                "label": widget.title,
                "position": [widget.position.0 as i32, widget.position.1 as i32, 0]
            }));
        }
        
        map
    }
    
    pub fn generate_keyboard_shortcuts(widgets: &[DashboardWidget]) -> String {
        let mut shortcuts = vec![
            "D: Dashboard Title".to_string(),
            "R: Refresh Dashboard".to_string(),
            "T: Toggle Theme".to_string(),
        ];
        
        for (index, widget) in widgets.iter().enumerate() {
            shortcuts.push(format!("W{}: {}", index + 1, widget.title));
        }
        
        shortcuts.join(", ")
    }
    
    pub fn apply_accessibility_preferences(state: &DashboardState, html_content: &mut String) {
        match state.user_preferences.accessibility_mode.as_str() {
            "screen_reader" => {
                // Add screen reader specific attributes
                html_content.push_str(" aria-live=\"polite\" role=\"application\"");
            }
            "high_contrast" => {
                // Add high contrast CSS classes
                html_content.push_str(" class=\"high-contrast\"");
            }
            _ => {
                // Standard mode
                html_content.push_str(" class=\"standard\"");
            }
        }
    }
}
```

## Step 7: Implement Caching Manager

Create `src/cache_manager.rs`:

```rust
use sled::Db;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, warn};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CachedVisualization {
    pub data: Vec<u8>,
    pub metadata: VisualizationMetadata,
    pub timestamp: u64,
    pub expires_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VisualizationMetadata {
    pub visualization_id: String,
    pub content_type: String,
    pub size: usize,
}

pub struct CacheManager {
    db: Db,
    default_ttl_seconds: u64,
}

impl CacheManager {
    pub fn new(db_path: &str, default_ttl_seconds: u64) -> Result<Self, Box<dyn std::error::Error>> {
        let db = sled::open(db_path)?;
        Ok(Self {
            db,
            default_ttl_seconds,
        })
    }
    
    pub fn store_visualization(
        &self,
        visualization_id: &str,
        data: Vec<u8>,
        content_type: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        let metadata = VisualizationMetadata {
            visualization_id: visualization_id.to_string(),
            content_type: content_type.to_string(),
            size: data.len(),
        };
        
        let cached = CachedVisualization {
            data,
            metadata,
            timestamp,
            expires_at: timestamp + self.default_ttl_seconds,
        };
        
        let serialized = bincode::serialize(&cached)?;
        self.db.insert(visualization_id.as_bytes(), serialized)?;
        
        info!("Cached visualization: {}", visualization_id);
        Ok(())
    }
    
    pub fn get_visualization(&self, visualization_id: &str) -> Result<Option<CachedVisualization>, Box<dyn std::error::Error>> {
        match self.db.get(visualization_id.as_bytes())? {
            Some(value) => {
                let cached: CachedVisualization = bincode::deserialize(&value)?;
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                
                if cached.expires_at > now {
                    info!("Cache hit for visualization: {}", visualization_id);
                    Ok(Some(cached))
                } else {
                    info!("Cache expired for visualization: {}", visualization_id);
                    self.db.remove(visualization_id.as_bytes())?;
                    Ok(None)
                }
            }
            None => {
                info!("Cache miss for visualization: {}", visualization_id);
                Ok(None)
            }
        }
    }
    
    pub fn invalidate_visualization(&self, visualization_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.db.remove(visualization_id.as_bytes())?;
        info!("Invalidated cache for visualization: {}", visualization_id);
        Ok(())
    }
    
    pub fn cleanup_expired(&self) -> Result<usize, Box<dyn std::error::Error>> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut removed_count = 0;
        
        for result in self.db.iter() {
            let (key, value) = result?;
            
            let cached: CachedVisualization = match bincode::deserialize(&value) {
                Ok(cached) => cached,
                Err(e) => {
                    warn!("Failed to deserialize cache entry: {}", e);
                    continue;
                }
            };
            
            if cached.expires_at <= now {
                self.db.remove(key)?;
                removed_count += 1;
            }
        }
        
        info!("Cleaned up {} expired cache entries", removed_count);
        Ok(removed_count)
    }
}
```

## Step 8: Update Cargo.toml with Additional Dependencies

Update `Cargo.toml`:

```toml
[dependencies]
visualization_context = { path = "../../packages/visualization_context" }
bevy = "0.16"
plotters = "0.3"
tokio = { version = "1.0", features = ["full"] }
tokio-tungstenite = "0.17"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
futures-util = "0.3"
uuid = { version = "1.0", features = ["v4"] }
tracing = "0.1"
tracing-subscriber = "0.3"
sled = "0.34"
bincode = "1.3"
```

## Step 9: Update Main Application with Advanced Features

Update `src/main.rs`:

```rust
mod dashboard;
mod visualization_manager;
mod websocket_client;
mod accessibility;
mod cache_manager;

use dashboard::{DashboardState, UserPreferences};
use visualization_manager::{VisualizationManager, WidgetConfig};
use websocket_client::{WebSocketClient, VisualizationUpdate};
use accessibility::AccessibilityManager;
use cache_manager::CacheManager;
use std::collections::HashMap;
use serde_json::Value;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("Starting complex dashboard tutorial...");
    
    // Initialize cache manager
    let cache_manager = CacheManager::new("./dashboard_cache", 300)?; // 5 minute TTL
    
    // Clean up expired cache entries
    match cache_manager.cleanup_expired() {
        Ok(count) => println!("Cleaned up {} expired cache entries", count),
        Err(e) => println!("Cache cleanup failed: {}", e),
    }
    
    // Initialize visualization manager
    let viz_manager = VisualizationManager::new("http://localhost:3001");
    
    // Create dashboard layout
    let widgets_config = vec![
        WidgetConfig {
            title: "Sales Overview".to_string(),
            visualization_id: "sales-bar-chart".to_string(),
            position: (0, 0),
            size: (400, 300),
            parameters: {
                let mut params = HashMap::new();
                params.insert("chart_type".to_string(), Value::String("bar".to_string()));
                params.insert("data_source".to_string(), Value::String("sales_db".to_string()));
                params
            },
        },
        WidgetConfig {
            title: "Performance Metrics".to_string(),
            visualization_id: "performance-gauge".to_string(),
            position: (400, 0),
            size: (200, 200),
            parameters: {
                let mut params = HashMap::new();
                params.insert("chart_type".to_string(), Value::String("gauge".to_string()));
                params.insert("metric".to_string(), Value::String("cpu_usage".to_string()));
                params
            },
        },
        WidgetConfig {
            title: "Real-time Data Stream".to_string(),
            visualization_id: "realtime-line-chart".to_string(),
            position: (0, 300),
            size: (600, 200),
            parameters: {
                let mut params = HashMap::new();
                params.insert("chart_type".to_string(), Value::String("line".to_string()));
                params.insert("refresh_rate".to_string(), Value::Number(serde_json::Number::from(1000)));
                params
            },
        },
    ];
    
    let mut dashboard_state = viz_manager.create_dashboard_layout("Sales Dashboard", widgets_config);
    
    // Update user preferences
    dashboard_state.user_preferences = UserPreferences {
        theme: "dark".to_string(),
        accessibility_mode: "screen_reader".to_string(),
        auto_refresh: true,
        refresh_interval: 30,
    };
    
    // Generate accessibility information
    let dashboard_alt_text = AccessibilityManager::generate_dashboard_alt_text(&dashboard_state);
    let navigation_map = AccessibilityManager::generate_widget_navigation_map(&dashboard_state.layout.widgets);
    let keyboard_shortcuts = AccessibilityManager::generate_keyboard_shortcuts(&dashboard_state.layout.widgets);
    
    println!("Dashboard Alt Text: {}", dashboard_alt_text);
    println!("Navigation Map Keys: {:?}", navigation_map.keys().collect::<Vec<_>>());
    println!("Keyboard Shortcuts: {}", keyboard_shortcuts);
    
    // Initialize WebSocket client for real-time updates
    let ws_callback = Arc::new(|update: VisualizationUpdate| {
        println!("Real-time update received for {}: {:?}", update.visualization_id, update.data);
    });
    
    let ws_client = WebSocketClient::new(
        "ws://localhost:3001/visualizations/realtime/ws".to_string(),
        ws_callback,
    );
    
    // Attempt to connect to WebSocket
    match ws_client.connect().await {
        Ok(_) => println!("WebSocket connection established"),
        Err(e) => println!("Failed to connect to WebSocket: {}", e),
    }
    
    // Create visualization requests for each widget
    for widget in &dashboard_state.layout.widgets {
        let request = viz_manager.create_visualization_request(widget, &dashboard_state);
        println!("Created visualization request for '{}': {}", widget.title, request.visualization_id);
        
        // Check cache first
        match cache_manager.get_visualization(&request.visualization_id) {
            Ok(Some(cached)) => {
                println!("Using cached visualization for: {}", widget.title);
                println!("Cached data size: {} bytes", cached.metadata.size);
            }
            Ok(None) => {
                println!("No cached visualization found for: {}", widget.title);
                // In a real application, you would make the API request here
            }
            Err(e) => {
                println!("Cache error for {}: {}", widget.title, e);
            }
        }
    }
    
    // Subscribe to real-time updates for the line chart
    if let Some(realtime_widget) = dashboard_state.layout.widgets.iter().find(|w| w.visualization_id == "realtime-line-chart") {
        match ws_client.subscribe_to_visualization(&realtime_widget.visualization_id).await {
            Ok(_) => println!("Subscribed to real-time updates for {}", realtime_widget.title),
            Err(e) => println!("Failed to subscribe to updates: {}", e),
        }
    }
    
    // Simulate dashboard interaction
    println!("\nSimulating dashboard interaction...");
    
    // Update a widget's parameters
    if let Some(first_widget) = dashboard_state.layout.widgets.first() {
        let mut new_params = HashMap::new();
        new_params.insert("time_range".to_string(), Value::String("last_30_days".to_string()));
        new_params.insert("aggregation".to_string(), Value::String("average".to_string()));
        
        dashboard_state.update_widget_parameters(&first_widget.id, new_params);
        println!("Updated parameters for widget: {}", first_widget.title);
    }
    
    println!("Dashboard tutorial completed successfully!");
    
    Ok(())
}
```

## Step 10: Testing the Implementation

To test your implementation:

1. Ensure the API Gateway is running on `http://localhost:3001`
2. Run the tutorial application:

```bash
cargo run
```

You should see output similar to:

```
Starting complex dashboard tutorial...
Cleaned up 0 expired cache entries
Dashboard Alt Text: Dashboard 'Sales Dashboard' containing 3 widgets. Navigation: Use Tab to move between widgets, Enter to activate.
Navigation Map Keys: ["D", "W1", "W2", "W3"]
Keyboard Shortcuts: D: Dashboard Title, R: Refresh Dashboard, T: Toggle Theme, W1: Sales Overview, W2: Performance Metrics, W3: Real-time Data Stream
WebSocket connection established
Created visualization request for 'Sales Overview': sales-bar-chart
No cached visualization found for: Sales Overview
Created visualization request for 'Performance Metrics': performance-gauge
No cached visualization found for: Performance Metrics
Created visualization request for 'Real-time Data Stream': realtime-line-chart
No cached visualization found for: Real-time Data Stream
Subscribed to real-time updates for Real-time Data Stream

Simulating dashboard interaction...
Updated parameters for widget: Sales Overview
Dashboard tutorial completed successfully!
```

## Advanced Features

### 1. Custom Themes

Implement theme switching:

```rust
pub enum Theme {
    Light,
    Dark,
    HighContrast,
    Custom(String), // Custom CSS URL
}

impl Theme {
    pub fn css_class(&self) -> &'static str {
        match self {
            Theme::Light => "theme-light",
            Theme::Dark => "theme-dark",
            Theme::HighContrast => "theme-high-contrast",
            Theme::Custom(_) => "theme-custom",
        }
    }
    
    pub fn css_url(&self) -> Option<&str> {
        match self {
            Theme::Custom(url) => Some(url),
            _ => None,
        }
    }
}
```

### 2. Dashboard Persistence

Save and load dashboard layouts:

```rust
use std::fs;
use std::path::Path;

impl DashboardState {
    pub fn save_to_file(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
    
    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(path)?;
        let state = serde_json::from_str(&json)?;
        Ok(state)
    }
}
```

### 3. Performance Monitoring

Add performance metrics collection:

```rust
use std::time::Instant;

pub struct PerformanceMetrics {
    render_times: Vec<u64>, // milliseconds
    cache_hits: u32,
    cache_misses: u32,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            render_times: Vec::new(),
            cache_hits: 0,
            cache_misses: 0,
        }
    }
    
    pub fn record_render_time(&mut self, duration: std::time::Duration) {
        self.render_times.push(duration.as_millis() as u64);
    }
    
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }
    
    pub fn record_cache_miss(&mut self) {
        self.cache_misses += 1;
    }
    
    pub fn cache_hit_ratio(&self) -> f64 {
        if self.cache_hits + self.cache_misses == 0 {
            0.0
        } else {
            self.cache_hits as f64 / (self.cache_hits + self.cache_misses) as f64
        }
    }
    
    pub fn average_render_time(&self) -> f64 {
        if self.render_times.is_empty() {
            0.0
        } else {
            self.render_times.iter().sum::<u64>() as f64 / self.render_times.len() as f64
        }
    }
}
```

## Troubleshooting

### Common Issues

1. **WebSocket Connection Failures**
   - Verify the WebSocket endpoint is correct
   - Check that the server supports WebSocket upgrades
   - Ensure network connectivity

2. **Cache Database Permissions**
   - Check that the application has write permissions to the cache directory
   - Verify the sled database path is accessible

3. **Serialization Errors**
   - Ensure all structs derive the necessary serde traits
   - Check for type mismatches in JSON data

### Debugging Tips

1. **Enable Detailed Logging**
   ```bash
   RUST_LOG=debug cargo run
   ```

2. **Test WebSocket Connection Separately**
   ```bash
   websocat ws://localhost:3001/visualizations/realtime/ws
   ```

3. **Inspect Cache Database**
   Use sled's built-in tools or a database viewer to inspect cache contents.

## Next Steps

After completing this tutorial, consider exploring:

1. [Accessibility Demo](./accessibility_demo.md) - Deep dive into accessibility features
2. [Visualization Architecture Guide](../developer/visualization_architecture.md) - Understand the system architecture
3. [Visualization Setup Guide](../developer/visualization_setup.md) - Review setup procedures

## Conclusion

You've successfully created a complex dashboard with multiple visualizations, real-time updates, and advanced features! You've learned how to:

- Structure a dashboard with multiple widgets
- Implement real-time data updates using WebSockets
- Manage visualization caching for performance
- Implement advanced accessibility features
- Handle complex user interactions and state management

This foundation will serve you well as you build sophisticated visualization applications within the CPC ecosystem.
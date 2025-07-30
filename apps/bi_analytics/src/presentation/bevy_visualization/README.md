# Bevy Visualization System for BI Analytics

This module provides a complete 3D visualization system for BI analytics using the Bevy game engine. It transforms JSON data into interactive 3D charts with realistic rendering, lighting, and materials.

## Features

### 🎯 Chart Types
- **3D Bar Charts**: Cuboid bars with height proportional to data values
- **3D Line Charts**: Connected data points with cylindrical line segments
- **3D Pie Charts**: Circular segments with angles proportional to values

### 🎨 Visual Features
- **Physically Based Rendering (PBR)** with metallic and roughness properties
- **Dynamic Color Schemes** based on data values and categories
- **3D Text Labels** for values, categories, and percentages
- **Professional Lighting** with shadows and ambient illumination

### 🖱️ Interactivity
- **Hover Effects**: Brighten elements on mouse hover
- **Click Handlers**: Ready for drill-down functionality
- **Value Display**: Show exact values on interaction
- **Responsive Design**: Adapts to different data ranges

## Architecture

### Core Components

```rust
// Chart entity with metadata
#[derive(Component)]
pub struct ChartEntity {
    pub chart_type: VisualizationType,
    pub title: String,
}

// Interactive elements
#[derive(Component)]
pub struct InteractiveElement {
    pub value: f64,
    pub label: String,
    pub original_color: Color,
    pub is_hovered: bool,
}
```

### Systems
- `update_visualizations`: Handles data updates
- `handle_chart_interactions`: Manages click events
- `update_hover_effects`: Updates visual feedback on hover

## Usage

### Basic Setup

```rust
use cpc_bi_analytics::presentation::bevy_visualization::BiVisualizationApp;

let mut app = BiVisualizationApp::new();
app.add_report_visualization(&report);
app.run();
```

### Report Structure

Reports should contain JSON data in the following formats:

#### Bar Chart Data
```json
[
    {"x": "Jan", "y": 45000},
    {"x": "Feb", "y": 52000},
    {"x": "Mar", "y": 48000}
]
```

#### Line Chart Data
```json
[
    {"x": 1, "y": 100},
    {"x": 2, "y": 150},
    {"x": 3, "y": 200}
]
```

#### Pie Chart Data
```json
[
    {"label": "Category A", "value": 35},
    {"label": "Category B", "value": 25},
    {"label": "Category C", "value": 40}
]
```

## Running the Demo

```bash
# Run the visualization demo
cargo run --example bevy_visualization_demo --features bevy-ui

# Run with visualization features enabled
cargo run --features visualization
```

## Technical Details

### 3D Rendering Pipeline
- Uses Bevy's PBR (Physically Based Rendering) system
- Custom mesh generation for pie slices and line segments
- Optimized material management with shared resources

### Performance Considerations
- Efficient mesh generation and caching
- Material reuse across similar elements
- Scalable to large datasets with proper LOD management

### Customization

#### Color Schemes
Colors are dynamically generated based on:
- Data values (normalized to color range)
- Category indices (cyclic hue rotation)
- Hover states (brightness adjustment)

#### Scaling
- Automatic scaling based on data ranges
- Configurable axis limits
- Responsive positioning within 3D space

## Integration

### Web Backend Integration
The visualization system can be integrated with the web backend:

```rust
use cpc_bi_analytics::presentation::bevy_visualization::create_visualization_window;

// Create visualization from web endpoint
async fn create_chart_endpoint(report: Report) {
    tokio::task::spawn_blocking(move || {
        create_visualization_window(&report);
    });
}
```

### Future Enhancements
- Real-time data streaming
- Animation transitions
- Export to 3D formats (GLTF, OBJ)
- VR/AR support
- Advanced interactivity (filtering, zooming)

## Dependencies

```toml
[dependencies]
bevy = { version = "0.16", features = [
    "bevy_pbr",
    "bevy_text", 
    "bevy_ui",
    "bevy_render",
    "bevy_core_pipeline",
    "bevy_asset",
    "bevy_scene",
    "bevy_winit",
] }
```

## Testing

Run the test suite:
```bash
cargo test --features bevy-ui
```

## License

This module is part of the CPC platform and uses the CPC license to promote sharing within the federation.
# Feedback Showcase Application

This application demonstrates the integration of all feedback system components:
- Sample product reviews with attributes
- Survey responses mapped to reviews
- Federation metadata simulation
- Visualization rendering examples
- Web-based UI for data generation

## Features

### Console-Based Showcase
The original console-based showcase demonstrates:
- Data generation for product reviews, surveys, and federated reviews
- Statistical analysis of feedback data
- Visualization component rendering (heatmaps, trend comparisons, correlation matrices)
- Federation consent rules demonstration

### Web UI Data Generator
The new web-based UI allows users to:
- Configure data generation parameters through an intuitive interface
- Generate feedback data with real-time progress monitoring
- View generation metrics (items processed, rate, memory usage)
- Export generated data (placeholder)

## Components

### Data Generator
Generates realistic sample data for product reviews, survey responses, and federated reviews based on configurable parameters.

### Analysis
Performs statistical analysis on feedback data, including rating distributions and metric correlations.

### Visualization
Renders visual representations of feedback data, including heatmaps, trend comparisons, and correlation matrices.

## Visualization Components

The app includes several visualization tools for feedback analysis:

### Summary View
- Displays key metrics: average rating, review count
- Shows sentiment distribution

### Ratings Chart
- Bar chart of star rating distribution
- Color-coded from red (1-star) to green (5-star)

### Word Cloud
- Visualizes common terms in feedback
- Size indicates frequency
- Click to filter reviews

### Sentiment Analysis
- Radial chart showing sentiment distribution
- Segments: Positive, Neutral, Negative

### Federation
Simulates federated review sharing with consent rules and metadata tracking.

### Social Sharing
Enables sharing of visualizations through multiple channels:
- Federation network (p2panda-based)
- Social media platforms (Twitter, Facebook, LinkedIn)
- Embed codes for websites and blogs
- Image export for presentations and reports

## UI Components

The web UI is built with Yew components:
- **DataGeneratorUI**: Main application component
- **ConfigPanel**: Configuration interface
- **ActionBar**: Action buttons (Generate, Export, Reset)
- **MetricsPanel**: Real-time metrics display
- **ShareButtonGroup**: Social sharing options for visualizations
- **SocialSharingDialog**: Dialog for sharing to social media platforms
- **EmbedCodeDialog**: Dialog for generating customizable embed codes

## Services

- **GeneratorService**: Bridge between UI and core generation logic

## Building and Running

### Console Version
```bash
cargo run
```

### Web Version
```bash
# Build for web
wasm-pack build --target web

# Serve the static files
# (Use any static file server, e.g., Python's http.server)
python -m http.server 8000
```

Then open `http://localhost:8000/static/index.html` in your browser.

## Dependencies

- feedback_core: Core feedback system types and error handling
- feedback_analysis: Statistical analysis of feedback data
- feedback_visualization: Visualization components for feedback data
- reviews: Product review data structures and validation
- survey: Survey data structures and response handling
- Yew: Web framework for building UI components
- Plotters: Visualization rendering
- uuid: For generating unique identifiers
- chrono: For timestamp handling

## Architecture

The application follows a hexagonal architecture pattern, with clear separation between:
- Core business logic (data generation, analysis, visualization)
- UI components (Yew-based web interface)
- Services (bridging components with core logic)
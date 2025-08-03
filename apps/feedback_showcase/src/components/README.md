# Feedback Showcase UI Components

This directory contains Yew-based UI components for the Feedback Data Generator application.

## Components

### DataGeneratorUI
The main application component that orchestrates the other components.

### ConfigPanel
Allows users to configure the data generation parameters:
- Review count
- Survey response rate
- Rating distribution parameters
- Product types

### ActionBar
Provides action buttons:
- Generate Data
- Export Data (placeholder)
- Reset to Default

### MetricsPanel
Displays real-time metrics during data generation:
- Progress bar
- Items processed
- Items per second
- Memory usage

### Visualization
Collection of data visualization components for analyzing feedback data:
- Summary: Key metrics display
- RatingsChart: Rating distribution bar chart
- WordCloud: Common words visualization
- Sentiment: Sentiment analysis radial chart
See visualization/README.md for details.

## Usage

To use these components in your Yew application:

```rust
use yew::prelude::*;
use crate::components::DataGeneratorUI;

#[function_component(App)]
fn app() -> Html {
    html! {
        <DataGeneratorUI />
    }
}
```

## Styling

The components are styled using the CSS in `static/styles.css`. Make sure to include this stylesheet in your HTML file.
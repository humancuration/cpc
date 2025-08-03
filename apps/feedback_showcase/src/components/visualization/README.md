# Visualization Components

This module contains visualization components for displaying feedback data analysis.

## Components

### Summary
Displays key metrics including average rating, total reviews, and sentiment distribution.

### RatingsChart
Shows a bar chart of rating distributions with color coding from red (1-star) to green (5-star).

### WordCloud
Visualizes common words in review content using a spiral placement algorithm.

### Sentiment
Displays sentiment analysis results in a radial chart with positive, neutral, and negative segments.

## Usage

Each component takes `VisualizationProps` which includes:
- `reviews`: Vector of Review objects
- `loading`: Boolean indicating if data is loading

Example:
```rust
<Summary reviews={reviews.clone()} loading={false} />
```

## Styling

Components are styled using CSS classes defined in `static/styles.css`:
- `.visualization-summary`
- `.visualization-ratings-chart`
- `.visualization-word-cloud`
- `.visualization-sentiment`
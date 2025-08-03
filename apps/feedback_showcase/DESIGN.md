# Feedback System Showcase Design

## Overview

This showcase application demonstrates the integration of all feedback system components in the CPC ecosystem. It shows how reviews, surveys, analysis, and visualization components work together to provide a comprehensive feedback solution.

## Architecture

The showcase follows the hexagonal architecture pattern, with the core feedback system components at the center:

```
┌─────────────────────────────────────────────────────────────┐
│                    Feedback Showcase App                    │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Reviews   │  │   Survey    │  │ Feedback Components │  │
│  │             │  │             │  │                     │  │
│  │  - Models   │  │  - Models   │  │  - Core             │  │
│  │  - Entities │  │  - Questions│  │  - Analysis         │  │
│  │  - Metadata │  │  - Responses│  │  - Visualization    │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Key Features Demonstrated

### 1. Review System Integration
- Creation of product reviews with scientific ratings
- Flexible attribute system for additional metadata
- Demographic information attachment
- Review validation

### 2. Survey System Integration
- Multiple question types (StarRating, TextResponse, MultipleChoice)
- Survey response mapping to reviews
- Survey validation

### 3. Federation Concepts
- Federated review wrapper with metadata
- Consent rules for data sharing
- Federation group definitions

### 4. Error Handling
- Unified error types across packages
- Proper error propagation
- Error context preservation

### 5. Statistical Analysis
- Rating distribution calculations
- Correlation analysis
- Trend analysis

### 6. Data Visualization
- PNG heatmap generation
- SVG trend comparison charts
- HTML correlation matrix

## Data Flow

1. **Data Creation**: Sample reviews and surveys are created with realistic data
2. **Validation**: All data is validated using package-specific validation rules
3. **Analysis**: Statistical analysis is performed on the review data
4. **Visualization**: Charts and graphs are generated from the analyzed data
5. **Output**: Results are displayed in console and saved as files

## Technical Implementation

### Error Handling
The showcase demonstrates unified error handling using the `FeedbackError` type from `feedback_core`:

```rust
use feedback_core::FeedbackError;

fn demonstrate_error_unification() -> Result<(), FeedbackError> {
    // Errors from different packages are unified
    let analysis_error = calculate_correlation(&[(1.0, 2.0)])?;
    // ...
}
```

### Statistical Analysis
The showcase uses functions from `feedback_analysis`:

```rust
use feedback_analysis::{RatingDistribution, calculate_correlation, TrendResult};

let distribution = RatingDistribution::new("quality".to_string());
let correlation = calculate_correlation(&data)?;
let trend = TrendResult::new();
```

### Visualization
The showcase demonstrates all visualization components from `feedback_visualization`:

```rust
use feedback_visualization::{Heatmap, TrendComparison, CorrelationMatrix};

let heatmap = Heatmap::new("Rating Distribution".to_string());
let png_data = heatmap.render_png(&distribution)?;

let trend_comparison = TrendComparison::new("Trends".to_string());
let svg_data = trend_comparison.render_svg(&trends)?;

let correlation_matrix = CorrelationMatrix::new("Correlations".to_string());
let html_data = correlation_matrix.render_html(&correlations)?;
```

## Output Files

When run, the showcase generates:
- `rating_distribution.png` - A heatmap visualization
- `trend_comparison.svg` - A trend comparison chart
- `correlation_matrix.html` - A correlation matrix visualization
- Console output showing federated review data

## Usage

To run the showcase:

```bash
cd apps/feedback_showcase
cargo run
```

To run the example:

```bash
cd apps/feedback_showcase
cargo run --example basic_usage
```

## Dependencies

The showcase integrates with the following CPC shared packages:
- `feedback_core` - Core error types and shared functionality
- `feedback_analysis` - Statistical analysis functions
- `feedback_visualization` - Visualization components
- `reviews` - Review models and federation concepts
- `survey` - Survey models and integration

## Future Enhancements

Possible future enhancements for the showcase:
1. Interactive web interface using Yew
2. Real-time data streaming
3. Advanced machine learning analysis
4. Integration with the p2panda network
5. More sophisticated visualization options
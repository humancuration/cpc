# Feedback System Integration Guide

This document explains how the various feedback system components integrate together in the showcase application.

## Component Overview

### 1. Reviews Package (`reviews`)
The reviews package provides the core data models for product/service reviews:
- `Review<T: Entity>` - Generic review structure
- `Rating` - Scientific rating metrics (0.0-1.0 scale)
- `Attribute` - Flexible key-value pairs
- `Demographics` - Optional demographic information
- `FederatedReview<T>` - Wrapper for federated sharing

### 2. Survey Package (`survey`)
The survey package handles structured feedback collection:
- `Survey` - Survey definition with questions
- `Question` - Multiple question types
- `SurveyResponse` - User responses
- `Answer` - Answer types corresponding to questions

### 3. Feedback Core (`feedback_core`)
Provides shared functionality:
- `FeedbackError` - Unified error type
- Common traits and utilities

### 4. Feedback Analysis (`feedback_analysis`)
Statistical analysis components:
- `RatingDistribution` - Distribution calculations
- `calculate_correlation` - Correlation analysis
- `TrendResult` - Trend analysis results

### 5. Feedback Visualization (`feedback_visualization`)
Data visualization components:
- `Heatmap` - Rating distribution visualization
- `TrendComparison` - Trend comparison charts
- `CorrelationMatrix` - Correlation matrix visualization

## Integration Points

### Review-Survey Integration
Reviews can include optional survey responses:

```rust
let review = Review {
    // ... other fields
    survey_response: Some(survey_response),
    // ...
};
```

### Analysis Integration
Analysis functions work with review data:

```rust
let distribution = analytics_engine.rating_distribution(&reviews, "quality");
let correlation = calculate_correlation(&rating_pairs)?;
```

### Visualization Integration
Visualization components render analysis results:

```rust
let heatmap = Heatmap::new("Quality Ratings".to_string());
let png_data = heatmap.render_png(&distribution)?;
```

## Data Flow in Showcase

1. **Entity Creation**: A `Product` entity is created implementing the `Entity` trait
2. **Review Creation**: Multiple `Review<Product>` instances are created with ratings, attributes, and demographics
3. **Survey Creation**: A `Survey` with multiple question types is created
4. **Response Mapping**: `SurveyResponse` instances are mapped to corresponding reviews
5. **Federation Wrapping**: Reviews are wrapped in `FederatedReview` with metadata and consent rules
6. **Analysis**: Statistical analysis is performed on review data
7. **Visualization**: Charts and graphs are generated from analysis results
8. **Output**: Results are displayed in console and saved as files

## Error Handling Integration

All components use the unified `FeedbackError` type:

```rust
use feedback_core::FeedbackError;

// Analysis errors
let correlation = calculate_correlation(&data)
    .map_err(|e| FeedbackError::Analysis(e.to_string()))?;

// Visualization errors
let png_data = heatmap.render_png(&distribution)
    .map_err(|e| FeedbackError::Visualization(e.to_string()))?;

// Validation errors (from reviews package)
review.validate()
    .map_err(|e| FeedbackError::Validation(e.to_string()))?;
```

## Federation Integration

The showcase demonstrates federation concepts:

```rust
let federated_review = FederatedReview {
    local_review: review,
    shared_metadata: FederationMetadata {
        shared_at: Some(Utc::now()),
        source_node: Some("node-1.example.com".to_string()),
        version: 1,
    },
    consent_rules: vec![
        ConsentRule {
            data_category: "ratings".to_string(),
            shared_with: FederationGroup::Public,
        },
        // ...
    ],
};
```

## Best Practices Demonstrated

### 1. Type Safety
Using Rust's type system to ensure data integrity:

```rust
impl<T: Entity> Review<T> {
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Compile-time guarantees about data structure
    }
}
```

### 2. Error Handling
Proper error propagation with context:

```rust
fn perform_analysis(reviews: &[Review<Product>]) -> Result<AnalysisResult, FeedbackError> {
    // Errors are properly propagated with context
    let distribution = calculate_distribution(reviews)
        .map_err(|e| FeedbackError::Analysis(format!("Distribution failed: {}", e)))?;
    // ...
}
```

### 3. Separation of Concerns
Each package has a single responsibility:
- `reviews`: Review data models and validation
- `survey`: Survey data models and responses
- `feedback_analysis`: Statistical analysis
- `feedback_visualization`: Data visualization
- `feedback_core`: Shared utilities

### 4. Extensibility
The design allows for easy extension:

```rust
// Adding a new question type to surveys
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Question {
    // ... existing types
    CustomQuestion {
        custom_field: String,
    },
}
```

## Running the Integration

To see all components working together:

```bash
cd apps/feedback_showcase
cargo run
```

This will:
1. Create sample data
2. Perform validation
3. Run statistical analysis
4. Generate visualizations
5. Demonstrate federation concepts
6. Show unified error handling

The output includes:
- Console messages showing data flow
- `rating_distribution.png` - Heatmap visualization
- `trend_comparison.svg` - Trend chart
- `correlation_matrix.html` - Correlation matrix
# Feedback System Integration

## Overview

This document describes the integration of the survey and reviews packages into a unified feedback analysis system. The integration focuses on:

- Shared statistical analysis capabilities
- Reusable visualization components
- Unified error handling
- Federation support with consent-based sharing
- Template system integration

## Architecture

The integration introduces three new shared packages:

### feedback_core
Contains unified error types and core functionality for the feedback system.

### feedback_analysis
Provides common statistical analysis functions that can be used across both reviews and survey packages.

### feedback_visualization
Offers reusable visualization components for creating charts, graphs, and other visual representations of feedback data.

## Key Components

### Shared Analysis Module
Located in `shared_packages/feedback_analysis`, this module provides:
- Statistical functions (mean, median, correlation)
- Rating distribution calculations
- Time-series analysis

### Visualization Framework
Located in `shared_packages/feedback_visualization`, this module provides:
- Heatmap visualization for rating distributions
- Trend comparison charts
- Correlation matrix visualization

### Error Handling Unification
Located in `shared_packages/feedback_core`, this module provides:
- A unified `FeedbackError` enum
- Error mapping from existing error types

### Federation Support
Added to the reviews package:
- `FederatedReview` wrapper struct
- `FederationMetadata` for tracking sharing information
- `ConsentRule` for controlling data sharing

### Template Integration
Extended the survey package:
- Added `ReviewAttributeMapping` to `SurveyTemplate`
- Enables automatic population of review attributes from survey responses

## Migration Plan

1. Created new shared packages:
   - `feedback_analysis`
   - `feedback_visualization`
   - `feedback_core`

2. Refactored existing packages:
   - Moved common statistical functions to `feedback_analysis`
   - Updated visualization code to use shared components
   - Mapped existing errors to `FeedbackError`

3. Implemented federation support:
   - Added `FederatedReview` to reviews package
   - Added consent management structures

4. Extended survey templates:
   - Added attribute mapping configuration
   - Enabled auto-population of review attributes

## Usage Examples

### Using Shared Analysis Functions

```rust
use feedback_analysis::{calculate_correlation, RatingDistribution};

// Calculate correlation between two variables
let data = vec![(1.0, 2.0), (2.0, 4.0), (3.0, 6.0)];
let correlation = calculate_correlation(&data)?;

// Create and populate a rating distribution
let mut dist = RatingDistribution::new("quality".to_string());
dist.add_rating(0.8)?;
dist.add_rating(0.9)?;
```

### Using Visualization Components

```rust
use feedback_visualization::{Heatmap, TrendComparison, CorrelationMatrix};

// Create a heatmap visualization
let heatmap = Heatmap::new("Quality Ratings".to_string());
let png_data = heatmap.render_png(&distribution)?;

// Create a trend comparison
let comparison = TrendComparison::new("Rating Trends".to_string());
let svg_data = comparison.render_svg(&trends)?;

// Create a correlation matrix
let matrix = CorrelationMatrix::new("Metric Correlations".to_string());
let html_data = matrix.render_html(&correlations)?;
```

### Federation Support

```rust
use reviews::models::{FederatedReview, FederationMetadata, ConsentRule, FederationGroup};

// Create a federated review
let federated_review = FederatedReview {
    local_review: review,
    shared_metadata: FederationMetadata {
        shared_at: Some(Utc::now()),
        source_node: Some("node1".to_string()),
        version: 1,
    },
    consent_rules: vec![
        ConsentRule {
            data_category: "ratings".to_string(),
            shared_with: FederationGroup::Public,
        }
    ],
};
```

### Template Integration

```rust
use survey::models::{SurveyTemplate, ReviewAttributeMapping};

// Create a survey template with review attribute mappings
let template = SurveyTemplate {
    id: Uuid::new_v4(),
    name: "Product Feedback".to_string(),
    description: "Feedback survey for products".to_string(),
    questions: vec![/* questions */],
    version: 1,
    created_at: Utc::now(),
    updated_at: Utc::now(),
    review_attribute_mappings: vec![
        ReviewAttributeMapping {
            survey_question_id: Uuid::new_v4(),
            review_attribute_key: "quality".to_string(),
        }
    ],
};
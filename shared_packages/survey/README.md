# Survey Module

A reusable survey system for the CPC software ecosystem.

## Features

- Multiple question types (StarRating, TextResponse, MultipleChoice, LikertScale, Matrix)
- Survey response validation
- Statistical analysis helpers
- Data visualization capabilities
- Template system for reusable surveys
- Integration with the review system

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
survey = { path = "../shared_packages/survey" }
```

## Usage

### Creating a Survey

```rust
use survey::{Survey, Question};

let survey = Survey {
    id: Uuid::new_v4(),
    title: "Product Satisfaction".to_string(),
    description: "Tell us about your experience".to_string(),
    questions: vec![
        Question::StarRating {
            min: 0.0,
            max: 5.0,
            step: 0.5
        },
        Question::TextResponse {
            max_length: Some(500)
        }
    ],
    scoring_config: None
};
```

### Creating a Response

```rust
use survey::{SurveyResponse, Answer};
use chrono::Utc;

let response = SurveyResponse {
    survey_id: survey.id,
    answers: vec![
        Answer::StarRating(4.5),
        Answer::TextResponse("Great product!".to_string())
    ],
    created_at: Utc::now()
};
```

### Statistical Analysis

```rust
use survey::analysis;

// Calculate average star rating
let avg = analysis::calculate_average_star_rating(&responses, 0);

// Calculate correlation between two questions
let correlation = analysis::calculate_correlation(&responses, 0, 1);

// Analyze trends over time
let trends = analysis::analyze_trends(&responses, 0, analysis::TimePeriod::Monthly);
```

### Data Visualization

```rust
use survey::visualization;

// Create a histogram
let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let histogram = visualization::plot::histogram(&data, 5);

// Create a heatmap
let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
let heatmap = visualization::plot::heatmap(&matrix);
```

### Template System

```rust
use survey::template_service;

// Create a template from a survey
let template = template_service::create_template(survey);

// Apply a template to create a new survey
let new_survey = template_service::apply_template(template);
```

## Modules

- `models` - Core domain models for surveys, questions, responses, and answers
- `validation` - Validation logic for surveys and survey responses
- `analysis` - Statistical analysis helpers for survey data
- `visualization` - Data visualization capabilities
- `template_service` - Template system for reusable surveys
- `error` - Custom error types used throughout the survey system

## Error Handling

The survey module uses custom error types for different components:

- `ValidationError` - Errors related to survey validation
- `AnalysisError` - Errors related to statistical analysis
- `VisualizationError` - Errors related to data visualization
- `TemplateError` - Errors related to template operations

All error types implement the `std::error::Error` trait and provide descriptive error messages with error codes.

## Testing

Run tests with:

```bash
cargo test
```

## License

This module is licensed under the CPC license.
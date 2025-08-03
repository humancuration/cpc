# Survey Module Changes

## Overview

This document summarizes the changes made to enhance the survey module with new features including advanced statistical analysis, data visualization, template system, and improved error handling.

## New Features

### 1. Advanced Statistical Analysis

Added new functions to the `analysis` module:
- `calculate_correlation` - Calculate correlation between two numerical questions
- `analyze_trends` - Analyze trends over time periods
- `analyze_sentiment` - Analyze sentiment of text responses
- `compare_demographic_groups` - Compare demographic groups for a specific question
- `sampled_responses` - Get a random sample of responses
- `with_cache` - Check cache for computed results (placeholder implementation)

### 2. Data Visualization

Created a new `visualization` module with plotting functions:
- `histogram` - Create histograms from numerical data
- `heatmap` - Create heatmaps from matrix data
- `word_cloud` - Create word clouds from word-frequency pairs
- `trend_line` - Create trend lines from time-series data

### 3. Template System

Created a new `template_service` module:
- `SurveyTemplate` and `QuestionTemplate` models for reusable survey designs
- `create_template` - Convert a Survey to a SurveyTemplate
- `get_template` - Retrieve a template by ID (placeholder implementation)
- `version_template` - Create a new version of a template
- `apply_template` - Apply a template to create a new Survey

### 4. Improved Error Handling

Created a new `error` module with custom error types:
- `AnalysisError` - Errors related to statistical analysis
- `VisualizationError` - Errors related to data visualization
- `TemplateError` - Errors related to template operations
- `ValidationError` - Errors related to survey validation (enhanced with error codes)

All error types now include error codes for easier identification and handling.

### 5. Performance Optimizations

Added caching and sampling capabilities:
- `with_cache` function for caching computed results
- `sampled_responses` function for working with large datasets

## Files Modified

1. `shared_packages/survey/design.md` - Updated with new sections for template system, visualization, and advanced statistical analysis
2. `shared_packages/survey/adr.md` - Added new ADRs for visualization integration, template system, and advanced statistical methods
3. `shared_packages/survey/src/analysis.rs` - Enhanced with new statistical functions and error handling
4. `shared_packages/survey/src/validation.rs` - Enhanced with error codes
5. `shared_packages/survey/src/models.rs` - Added template structs
6. `shared_packages/survey/src/visualization.rs` - New file with plotting functions
7. `shared_packages/survey/src/template_service.rs` - New file with template management functions
8. `shared_packages/survey/src/error.rs` - New file with custom error types
9. `shared_packages/survey/src/lib.rs` - Updated to include new modules and exports
10. `shared_packages/survey/src/tests.rs` - Updated with tests for new functionality
11. `shared_packages/survey/Cargo.toml` - Added new dependencies (rand, plotters, serde_json)
12. `shared_packages/survey/README.md` - Updated with documentation for new features
13. `shared_packages/survey/examples/advanced_analysis.rs` - New example demonstrating advanced features
14. `Cargo.toml` - Updated with new workspace dependencies

## Dependencies Added

- `rand = "0.8"` - For random sampling in analysis
- `plotters = "0.3"` - For data visualization
- `serde_json = "1.0"` - For template configuration

## Usage Examples

See the new example file `shared_packages/survey/examples/advanced_analysis.rs` for a comprehensive demonstration of the new features.

## Testing

All new functionality is covered by tests in `shared_packages/survey/src/tests.rs`.

## Error Handling

All new functions return appropriate error types with descriptive error messages and error codes.
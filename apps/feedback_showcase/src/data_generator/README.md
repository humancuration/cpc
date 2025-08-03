# Data Generator Module

This module provides functionality to generate realistic sample data for the feedback showcase application.

## Overview

The data generator creates synthetic but realistic data for:

1. Product reviews with ratings, attributes, and demographic information
2. Survey responses mapped to reviews
3. Federated reviews with metadata and consent rules

## Modules

### config
Contains configuration structures for controlling the data generation process.

### generators
Contains the actual generation logic for different types of data:
- `products`: Product entity generation
- `reviews`: Review generation with ratings and attributes
- `surveys`: Survey response generation
- `federation`: Federated review generation with metadata

### utils
Utility functions for common operations and default configurations.

## Usage

```rust
use data_generator::{DataGeneratorConfig, generate_reviews, generate_survey_responses, generate_federated_reviews};
use data_generator::utils::create_default_config;

// Create configuration
let config = create_default_config();

// Generate data
let product = generate_product(&config.product_types[0]);
let mut reviews = generate_reviews(&config, product);
let survey = create_sample_survey();
let survey_responses = generate_survey_responses(&config, &survey, &mut reviews);
let federated_reviews = generate_federated_reviews(reviews);
```

## Performance

The generator uses parallel processing with Rayon for large datasets (>10,000 records) and streaming generation for very large datasets.

## Dependencies

- `rand`: Random number generation
- `fake`: Fake data generation
- `rayon`: Parallel processing
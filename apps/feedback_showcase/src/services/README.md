# Feedback Showcase Services

This directory contains service modules that bridge the UI components with the core data generation logic.

## Services

### GeneratorService
Provides an interface between the UI and the data generation functions:

- `generate_data(config: DataGeneratorConfig) -> Result<Vec<Review<Product>>, String>`
  Asynchronously generates feedback data based on the provided configuration.

- `GenerationMetrics`
  A structure that holds metrics about the generation process:
  - items_processed: Number of items generated
  - items_per_second: Generation rate
  - memory_usage: Memory usage in MB
  - progress: Progress percentage (0-100)

## Usage

```rust
use crate::services::generator_service::GeneratorService;
use crate::data_generator::config::DataGeneratorConfig;

let config = create_default_config();
let results = GeneratorService::generate_data(config).await;
```

## Architecture

The service follows the hexagonal architecture pattern, providing a clear separation between the UI layer and the core business logic. This allows for easier testing and maintenance.
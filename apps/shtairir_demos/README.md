# Shtairir Demo Applications

This directory contains real-world example applications that demonstrate how to compose Shtairir standard library blocks into more complex workflows. These examples serve as documentation and inspiration for community members.

## Overview

The Shtairir ecosystem provides a powerful visual programming language that allows users to compose reusable building blocks into complex workflows. These demos showcase practical applications of this approach.

## Available Examples

### 1. Real-time Data Processing Pipeline

A stream processing workflow that ingests sensor data, processes it, and generates analytics.

**Key Components:**
- `collection.map` to transform raw sensor readings
- `collection.filter` to remove outliers
- `math.mean` for statistical analysis
- `string.format` to create human-readable reports
- `collection.reduce` as a stateful-breaker for counting readings

**Run the demo:**
```bash
cargo run -p shtairir_demos_data_processing --bin data_processing_demo
```

### 2. User Profile Management Workflow

A workflow for managing user profile data with validation and transformation.

**Key Components:**
- `string.trim` and `string.format` for data normalization
- `collection.filter` for data validation
- `collection.map` for data transformation
- `string.concat` for creating derived fields

**Run the demo:**
```bash
cargo run -p shtairir_demos_user_profiles --bin user_profiles_demo
```

### 3. Machine Learning Feature Pipeline

A pipeline for preparing data for machine learning models.

**Key Components:**
- `math.vector_add` for feature engineering
- `math.mean` and `math.stddev` for normalization
- `collection.random_sample` for creating training/test splits
- `collection.stats_summary` for data quality checks

**Run the demo:**
```bash
cargo run -p shtairir_demos_ml_features --bin ml_features_demo
```

## Structure

Each example follows a consistent structure:

```
example_name/
├── Cargo.toml              # Package manifest
├── MODULE.toml             # Shtairir module definition
├── README.md               # Example documentation
├── build.rs                # Build script
├── blocks/                 # Custom block definitions
│   └── mock_data.toml      # Mock data generator block
├── graphs/                 # Graph definitions
│   └── workflow.toml       # Main workflow graph
├── src/                    # Rust implementation
│   ├── lib.rs              # Library entry point
│   ├── main.rs             # Demo binary
│   ├── workflow.rs         # Workflow implementation
│   └── utils.rs            # Utility functions
└── tests/                  # Integration tests
    └── workflow_test.rs    # Workflow tests
```

## Usage

To run all demos:

```bash
# Run data processing demo
cargo run -p shtairir_demos_data_processing

# Run user profiles demo
cargo run -p shtairir_demos_user_profiles

# Run ML features demo
cargo run -p shtairir_demos_ml_features
```

To run tests:

```bash
# Test data processing demo
cargo test -p shtairir_demos_data_processing

# Test user profiles demo
cargo test -p shtairir_demos_user_profiles

# Test ML features demo
cargo test -p shtairir_demos_ml_features
```

## Key Concepts Demonstrated

### Graph Composition
Each example shows how to compose standard library blocks into complex workflows using TOML graph specifications.

### Error Handling
Examples demonstrate proper error handling patterns for both deterministic and non-deterministic operations.

### Performance Metrics
All examples include performance metrics collection to measure execution time and throughput.

### Data Validation
Examples show various approaches to data validation and transformation.

### Mock Data Generation
Each example includes custom block implementations for generating realistic test data.

## Learning Resources

- [Shtairir Documentation](../../../docs/shtairir/)
- [Standard Library Blocks](../../../shared_packages/shtairir_blocks/)
- [Shtairir Registry](../../../shared_packages/shtairir_registry/)
- [Shtairir Core](../../../shared_packages/shtairir_core/)

## Contributing

These examples are meant to be living documentation. Feel free to:
- Add new examples
- Improve existing examples
- Fix bugs
- Add more comprehensive tests

See the main CPC repository for contribution guidelines.
# Using Shtairir Demo Applications

This guide explains how to use the Shtairir demo applications within the CPC software ecosystem.

## Prerequisites

Before running these examples, ensure you have:

1. Rust toolchain installed (latest stable version)
2. The CPC codebase cloned and built
3. All workspace dependencies resolved

## Running the Examples

### From the Root Directory

You can run any of the demos from the root of the CPC repository:

```bash
# Data Processing Pipeline
cargo run -p shtairir_demos_data_processing

# User Profile Management
cargo run -p shtairir_demos_user_profiles

# ML Feature Pipeline
cargo run -p shtairir_demos_ml_features
```

### From Individual Example Directories

You can also run each example from its own directory:

```bash
# Navigate to the data processing example
cd apps/shtairir_demos/data_processing
cargo run

# Navigate to the user profiles example
cd apps/shtairir_demos/user_profiles
cargo run

# Navigate to the ML features example
cd apps/shtairir_demos/ml_features
cargo run
```

## Understanding the Structure

Each example demonstrates different aspects of the Shtairir system:

### 1. Graph Composition
- TOML files define how blocks connect to form workflows
- Custom blocks extend the standard library functionality
- Functions are defined inline for complex operations

### 2. Runtime Execution
- The Shtairir runtime executes graphs using the registry
- Data flows through connected blocks
- Results are collected and processed

### 3. Integration Patterns
- Examples show how to integrate with external systems
- Error handling patterns are demonstrated
- Performance monitoring is included

## Customizing the Examples

### Modifying Input Parameters

Each example accepts parameters that can be modified:

```rust
// In data_processing/src/main.rs
let reading_count = 100i64; // Change this value

// In user_profiles/src/main.rs
let profile_count = 10i64; // Change this value

// In ml_features/src/main.rs
let sample_count = 1000i64; // Change these values
let feature_count = 10i64;
```

### Adding New Blocks

To add new blocks to an example:

1. Create a new TOML file in the `blocks/` directory
2. Add it to the `blocks` array in `MODULE.toml`
3. Implement the block logic in Rust if needed
4. Reference it in your graph definitions

### Creating New Graphs

To create a new graph:

1. Create a new TOML file in the `graphs/` directory
2. Add it to the `graphs` array in `MODULE.toml`
3. Define nodes, edges, and functions
4. Export the results you need

## Testing

Each example includes comprehensive tests:

```bash
# Run tests for data processing
cargo test -p shtairir_demos_data_processing

# Run tests for user profiles
cargo test -p shtairir_demos_user_profiles

# Run tests for ML features
cargo test -p shtairir_demos_ml_features

# Run all tests
cargo test -p shtairir_demos_data_processing -p shtairir_demos_user_profiles -p shtairir_demos_ml_features
```

## Extending the Examples

### Adding New Features

1. Identify which standard library blocks you need
2. Create new custom blocks if necessary
3. Design your graph composition
4. Implement Rust integration code
5. Add tests and documentation

### Performance Optimization

The examples include basic performance metrics. To extend this:

1. Add more detailed timing measurements
2. Implement memory usage tracking
3. Add throughput calculations
4. Create benchmark tests

## Integration with Other CPC Components

These examples can be integrated with other CPC applications:

### Data Flow
- Examples can consume data from other CPC services
- Results can be published to the event bus
- Database operations can be performed through the db_abstraction layer

### User Interface
- Graphs can be visualized using the shtairir_editor
- Results can be displayed in web or desktop UIs
- User input can drive graph execution

### Collaboration
- Graphs can be shared through the collaboration_engine
- Multiple users can work on the same workflow
- Changes can be synchronized in real-time

## Best Practices Demonstrated

### Error Handling
- Proper error propagation through the Shtairir runtime
- Graceful handling of invalid input data
- Clear error messages for debugging

### Resource Management
- Efficient memory usage patterns
- Proper cleanup of temporary data
- Connection pooling for external services

### Modularity
- Clear separation of concerns
- Reusable components
- Well-defined interfaces

## Troubleshooting

### Common Issues

1. **Module not found**: Ensure the registry is loading from the correct path
2. **Block not found**: Check that all required blocks are defined in MODULE.toml
3. **Graph execution errors**: Verify that all required inputs are provided

### Debugging Tips

1. Enable verbose logging with `RUST_LOG=debug`
2. Use the registry inspection methods to list available blocks
3. Check the integrity hashes in TOML files

## Contributing

These examples are meant to evolve with the community. Consider contributing:

1. New example applications
2. Additional standard library block usage
3. Performance improvements
4. Better documentation and comments
5. Additional test cases

See the main CPC repository CONTRIBUTING.md for detailed guidelines.
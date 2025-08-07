# Extending Shtairir Demo Applications

This guide explains how to extend the existing Shtairir demo applications or create new ones based on the patterns established in these examples.

## Creating New Examples

### 1. Project Structure

Start by copying one of the existing examples as a template:

```bash
# Copy the data processing example as a starting point
cp -r apps/shtairir_demos/data_processing apps/shtairir_demos/my_new_example
```

### 2. Update Metadata

Modify the following files with your new example's information:

- `MODULE.toml` - Update name, title, description
- `Cargo.toml` - Update package name and description
- `README.md` - Document your example's purpose and usage

### 3. Define Custom Blocks

Create new TOML files in the `blocks/` directory for any domain-specific functionality:

```toml
# blocks/my_custom_block.toml
id = "demos.shtairir.my_example/my_custom_block@0.1.0"
namespace = "demos.shtairir.my_example"
name = "my_custom_block"
version = "0.1.0"
title = "My Custom Block"
description = "Description of what this block does"

inputs = [
  { name = "input_data", ty = "list<object>" }
]

outputs = [
  { name = "processed_data", ty = "list<object>" }
]

[engine]
version_req = "^0.2"
```

### 4. Design Graph Workflows

Create TOML graph specifications in the `graphs/` directory:

```toml
# graphs/my_workflow.toml
schema_version = "0.2"
id = "graph:demos.shtairir.my_example/my_workflow@0.1.0"
name = "my_workflow"
# ... graph definition
```

## Extending Existing Examples

### Adding New Features

To add new features to existing examples:

1. **Identify Required Blocks**: Determine which standard library blocks you need
2. **Create Custom Blocks**: Implement any domain-specific functionality
3. **Modify Graphs**: Update TOML graph definitions to include new nodes
4. **Update Rust Code**: Extend the integration code with new functionality
5. **Add Tests**: Create tests for new features

### Example: Adding Database Integration

To add database integration to the user profiles example:

1. **Create a Database Block**:
   ```toml
   # blocks/save_to_db.toml
   id = "demos.shtairir.user_profiles/save_to_db@0.1.0"
   # ... block definition for database saving
   ```

2. **Update the Graph**:
   ```toml
   # graphs/user_profile_workflow.toml
   [[nodes]]
   id = "save_profiles"
   kind = "block"
   fq_block = "demos.shtairir.user_profiles/save_to_db"
   # ... node configuration
   ```

3. **Extend the Rust Implementation**:
   ```rust
   // src/database.rs
   pub fn save_profiles(profiles: Vec<UserProfile>) -> Result<()> {
       // Database saving logic
   }
   ```

## Best Practices

### Graph Design

1. **Modularity**: Break complex workflows into smaller, reusable graphs
2. **Clarity**: Use descriptive node and port names
3. **Error Handling**: Design paths for both success and failure cases
4. **Performance**: Consider data flow and batching opportunities

### Block Implementation

1. **Single Responsibility**: Each block should have one clear purpose
2. **Type Safety**: Use appropriate type signatures
3. **Documentation**: Include examples and clear descriptions
4. **Validation**: Check inputs and provide meaningful errors

### Performance Optimization

1. **Batch Processing**: Process collections rather than individual items when possible
2. **Memory Management**: Reuse data structures where appropriate
3. **Lazy Evaluation**: Implement streaming where beneficial
4. **Caching**: Cache expensive computations when appropriate

## Integration Patterns

### External Service Integration

To integrate with external services:

1. **Create Async Blocks**: Use effect blocks for I/O operations
2. **Handle Timeouts**: Implement proper timeout handling
3. **Retry Logic**: Add retry mechanisms for transient failures
4. **Connection Pooling**: Reuse connections when possible

Example block for HTTP requests:
```toml
# blocks/http_request.toml
purity = "effect"
effects = ["network"]
determinism = "NonDeterministic"

inputs = [
  { name = "url", ty = "string" },
  { name = "method", ty = "string" },
  { name = "body", ty = "string" }
]

outputs = [
  { name = "response", ty = "object" }
]
```

### Event-Driven Workflows

To create event-driven workflows:

1. **Trigger Blocks**: Create blocks that respond to events
2. **State Management**: Implement stateful blocks for complex workflows
3. **Fan-out Patterns**: Use map operations to process multiple events
4. **Aggregation**: Use reduce operations to combine results

## Testing Strategies

### Unit Testing

Test individual components in isolation:

```rust
#[test]
fn test_my_custom_function() {
    let input = create_test_data();
    let expected = create_expected_result();
    let actual = my_function(input);
    assert_eq!(expected, actual);
}
```

### Integration Testing

Test complete workflows:

```rust
#[tokio::test]
async fn test_my_workflow() -> Result<()> {
    let registry = Registry::load(&[".".into()])?;
    let result = execute_my_workflow(&registry).await?;
    assert!(result.is_successful());
    Ok(())
}
```

### Performance Testing

Measure execution characteristics:

```rust
#[test]
fn test_performance() {
    let start = Instant::now();
    process_large_dataset();
    let duration = start.elapsed();
    assert!(duration < Duration::from_secs(1));
}
```

## Documentation Guidelines

### Block Documentation

Each block should include:

1. **Clear Description**: What the block does
2. **Input/Output Types**: Precise type signatures
3. **Usage Examples**: Concrete examples
4. **Error Conditions**: When the block might fail
5. **Performance Characteristics**: Time/memory complexity

### Graph Documentation

Each graph should document:

1. **Purpose**: What problem it solves
2. **Workflow**: High-level data flow
3. **Key Decisions**: Important design choices
4. **Assumptions**: What the graph expects
5. **Limitations**: Known constraints

## Community Contributions

### Sharing Extensions

To share your extensions with the community:

1. **Follow Naming Conventions**: Use consistent module and block names
2. **Provide Examples**: Include usage examples
3. **Write Tests**: Ensure your code is well-tested
4. **Document Thoroughly**: Explain usage and limitations
5. **Submit Pull Requests**: Contribute back to the main repository

### Collaboration Patterns

When working with others:

1. **Version Compatibility**: Maintain backward compatibility
2. **Clear Interfaces**: Define stable block interfaces
3. **Comprehensive Testing**: Ensure changes don't break existing functionality
4. **Detailed Changelog**: Document all significant changes
5. **Code Reviews**: Have others review your contributions

## Advanced Topics

### Custom Block Types

Beyond the standard block categories:

1. **Stateful Blocks**: Maintain state between executions
2. **Streaming Blocks**: Process data incrementally
3. **Conditional Blocks**: Change behavior based on inputs
4. **Composite Blocks**: Combine multiple operations

### Graph Optimization

Techniques for improving graph performance:

1. **Node Fusion**: Combine compatible operations
2. **Parallel Execution**: Identify independent paths
3. **Memory Planning**: Optimize data movement
4. **Caching Strategies**: Reuse computed results

### Domain-Specific Extensions

Creating specialized libraries:

1. **Financial Blocks**: For financial calculations
2. **Scientific Blocks**: For research applications
3. **Web Blocks**: For web development workflows
4. **IoT Blocks**: For sensor data processing

## Conclusion

The Shtairir demo applications provide a solid foundation for creating powerful visual programming workflows. By following the patterns and best practices outlined in this guide, you can extend these examples to solve your own domain-specific problems.

Key takeaways:

1. **Start Simple**: Begin with basic compositions of standard blocks
2. **Iterate Quickly**: Add complexity gradually
3. **Test Thoroughly**: Ensure reliability of your workflows
4. **Document Clearly**: Help others understand and use your extensions
5. **Share Generously**: Contribute back to the community

The true power of Shtairir lies in its composability - simple blocks can be combined in endless ways to create sophisticated applications. These examples demonstrate just the beginning of what's possible.
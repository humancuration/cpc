# Social Graph Examples

This directory contains examples demonstrating various features of the social graph package.

## Examples

### 1. basic_usage.rs
Basic usage of the social graph package, showing how to create users, relationships, and activities.

### 2. full_example.rs
Complete example showing all components of the social graph package working together.

### 3. content_provider_example.rs
Demonstrates the ContentProvider system for universal feed with state migration, including:
- State preservation guarantees
- Error handling for registration/update failures
- Dependency validation during updates

### 4. state_migration_example.rs
Demonstrates state migration between content provider versions, including:
- Serialization/deserialization error handling
- Rollback mechanism during failed migration
- Concurrent access scenario during migration

### 5. dynamic_provider_example.rs
Demonstrates the dynamic content provider system, including:
- Dependency conflict scenarios (missing deps, version conflicts)
- Circular dependency detection
- Conflict resolution workflow

### 6. hot_swap_example.rs
Demonstrates hot-swapping of content providers with state migration, including:
- Seamless provider updates without downtime
- State preservation during version upgrades
- Backward compatibility verification

## Running Examples

To run any example, use the following command:

```bash
cargo run --example EXAMPLE_NAME
```

Replace `EXAMPLE_NAME` with the name of the example you want to run (e.g., `content_provider_example`).
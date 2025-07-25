# Protobuf Model Migration Plan

## Overview
This document outlines the migration steps to transition from manual model implementations to protobuf-driven code generation.

## Phase 1: Setup (1 Day)
1. Add protobuf definitions for core models
   - Create `.proto` files in `packages/cpc-protos/proto/core/`
   - Define all core models with validation annotations
2. Update build pipeline
   - Modify `packages/cpc-protos/build.rs` to include core models
   - Add validation attributes and JNI binding generation
3. Generate initial code
   - Run `cargo build` in `packages/cpc-protos` to generate Rust structs

## Phase 2: Core Integration (2 Days)
1. Refactor model imports
   - Replace manual structs with generated protobuf types
   - Update `packages/cpc-core/src/product/model.rs`:
```rust
// Replace existing structs with protobuf-generated ones
pub use cpc_protos::core::{Product, Money, WarehouseLocation};
```
2. Migrate business logic
   - Implement custom methods as extensions to generated structs
```rust
impl Product {
    pub fn calculate_total_value(&self) -> f64 {
        // Custom logic
    }
}
```
3. Update serialization
   - Replace manual conversions in `serialization.rs` with protobuf's built-in serialization

## Phase 3: Android Integration (1 Day)
1. Refactor JNI bridge
   - Replace manual JNI functions with generated bindings
   - Update `jni_bridge.rs` to use auto-generated serialization functions
2. Update Android app
   - Modify Kotlin code to match new JNI interface
   - Handle validation errors from Rust

## Phase 4: Validation & Testing (2 Days)
1. Add validation tests
```rust
#[test]
fn test_product_validation() {
    let mut product = Product::default();
    assert!(product.validate().is_err());
    
    product.id = "test".to_string();
    product.name = "Valid".to_string();
    assert!(product.validate().is_ok());
}
```
2. Test cross-platform compatibility
   - Verify data round-trip between Rust and Kotlin
   - Test edge cases (null handling, validation failures)

## Phase 5: Cleanup (1 Day)
1. Remove legacy code
   - Delete manual conversion code in `serialization.rs`
   - Remove old model definitions
2. Update documentation
   - Mark old model documentation as deprecated

## Rollback Plan
1. Revert to previous commit
2. Run full test suite
3. Verify Android app functionality

## Dependencies
- Update `cpc-protos` dependencies:
```toml
[build-dependencies]
tonic-build = "0.11"

[dependencies]
prost = "0.12"
validator = "0.16"
```

## Estimated Timeline
```mermaid
gantt
    title Migration Timeline
    dateFormat  YYYY-MM-DD
    section Setup
    Protobuf Definitions     :2025-07-25, 1d
    Build Pipeline Update    :2025-07-25, 1d
    section Core
    Model Integration        :2025-07-26, 2d
    section Android
    JNI Refactoring          :2025-07-28, 1d
    section Testing
    Validation & Testing     :2025-07-29, 2d
    section Cleanup
    Final Cleanup            :2025-07-31, 1d
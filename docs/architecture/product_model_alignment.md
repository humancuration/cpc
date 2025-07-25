# Product Model Alignment Strategy

## Overview
This document outlines our approach to aligning protobuf-generated structs with Rust models in the CPC ecosystem. The strategy ensures type consistency, validation parity, and business logic preservation across our codebase.

## Core Principles
1. **Single Source of Truth**: Protobuf definitions drive model generation
2. **Validation First**: Apply validation at generation time when possible
3. **Business Logic Separation**: Use extension traits for domain-specific logic
4. **Cross-Language Consistency**: Maintain type alignment across Rust, Kotlin, and Swift

## Key Alignment Areas

### Type Mapping
| Protobuf Type | Rust Type | Notes |
|---------------|-----------|-------|
| double | f64 | Used for financial/measurement fields |
| float | f32 | Deprecated in our models |
| uint32 | u32 | For non-negative integers |
| string | String | With length validations |
| Timestamp | chrono::NaiveDateTime | Via helper conversions |
| optional | Option | Handled natively by prost |

### Validation Strategy
Validation rules are applied at code generation time using prost/tonic-build attributes:

```rust
tonic_build::configure()
    .field_attribute("id", "#[validate(length(min = 1))]")
    .field_attribute("price", "#[validate(range(min = 0))]")
    .type_attribute("Product", "#[derive(Validate)]")
```

### Business Logic Preservation
We use extension traits to maintain business logic:

```rust
pub trait ProductExt {
    fn total_cost(&self) -> Option<f64>;
    fn validate_supply_chain(&self) -> Result<(), ModelError>;
}

impl ProductExt for Product {
    // Implementation of business logic
}
```

## Migration Process
1. **Analysis Phase**: Compare proto definitions with existing models
2. **Generation Updates**: Modify build scripts for validation attributes
3. **Extension Traits**: Port business logic from legacy models
4. **Refinement**: Update serialization/deserialization helpers
5. **Verification**: Test JNI boundary and API contracts

## Best Practices
1. Always use protobuf Timestamp for time fields
2. Prefer f64 for financial values to prevent rounding errors
3. Add #[non_exhaustive] to enums for forward compatibility
4. Validate all string fields with length constraints
5. Use option wrappers for nullable fields

## Reference Files
- Proto definitions: `packages/cpc-protos/proto/core/product.proto`
- Rust models: `packages/cpc-core/src/models.rs`
- Extension traits: `packages/cpc-core/src/product/extensions.rs`
- Serialization helpers: `packages/cpc-core/src/serialization.rs`
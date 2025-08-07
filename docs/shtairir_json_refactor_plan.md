# Shtairir JSON Refactor Architectural Plan

## Overview

This document outlines the architectural plan for refactoring Shtairir to eliminate JSON dependencies and create a more Rust-like type system. The goal is to replace generic JSON types with proper Rust ADT (Algebraic Data Type) representations that align with our existing type system.

## Current Issues

### 1. JSON in Type System
- **Location**: `shared_packages/shtairir_registry/src/types.rs` line 172
- **Issue**: `ScalarType::Json` is a generic catch-all type that doesn't fit with our Rust-like ADT system
- **Impact**: Weakens type safety and prevents proper compile-time validation

### 2. JSON Values in Execution
- **Locations**: 
  - `shared_packages/shtairir_execution/src/executor.rs`
  - `shared_packages/shtairir_execution/src/scheduler.rs`
- **Issue**: Using `serde_json::Value` for node outputs and context
- **Impact**: Runtime type checking, loss of compile-time guarantees

### 3. JSON in Model Definitions
- **Location**: `shared_packages/shtairir_registry/src/model.rs`
- **Issue**: Multiple uses of `serde_json::Value` for:
  - Default values in `PortSpec`
  - Parameter constraints in `ParamAllowed`
  - Test references in `TestRef`
  - Graph/node metadata
  - Edge policy parameters
- **Impact**: Runtime validation, unclear data structures

## Architectural Solution

### 1. Replace `ScalarType::Json` with Structured Types

Instead of a generic JSON type, we'll introduce specific structured types that can represent JSON-like data in a type-safe way:

```rust
// Replace ScalarType::Json with:
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScalarType {
    // Existing scalar types...
    I64,
    F64,
    Bool,
    String,
    Bytes,
    Decimal,
    DateTime,
    Duration,
    Uuid,
    // No more Json - replaced by structured types below
}

// New structured types for dynamic data
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StructuredType {
    // Heterogeneous collection - replacement for JSON objects
    Object(BTreeMap<String, Value>),
    
    // Heterogeneous list - replacement for JSON arrays
    Array(Vec<Value>),
    
    // Null value
    Null,
}

// Unified value type that can represent any Shtairir value
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // Scalar values
    I64(i64),
    F64(f64),
    Bool(bool),
    String(String),
    Bytes(Vec<u8>),
    Decimal(rust_decimal::Decimal),
    DateTime(chrono::DateTime<chrono::Utc>),
    Duration(chrono::Duration),
    Uuid(uuid::Uuid),
    
    // Structured values (replacing JSON)
    Object(BTreeMap<String, Value>),
    Array(Vec<Value>),
    Null,
    
    // Complex ADT types
    List(Vec<Value>),
    Map(BTreeMap<String, Value>),
    Option(Option<Box<Value>>),
    Tuple(Vec<Value>),
    Struct(StructValue),
    Enum(EnumValue),
    
    // Stream/Event types (with boxed values)
    Stream(Box<Value>),
    Event(Box<Value>),
}

// Struct instance values
#[derive(Debug, Clone, PartialEq)]
pub struct StructValue {
    pub type_name: String,
    pub fields: BTreeMap<String, Value>,
}

// Enum instance values
#[derive(Debug, Clone, PartialEq)]
pub struct EnumValue {
    pub type_name: String,
    pub variant_name: String,
    pub value: Option<Box<Value>>,
}
```

### 2. Update Type System Integration

The `Type` enum will be updated to work with the new `Value` type:

```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    // Scalar types
    Scalar(ScalarType),
    
    // Collection types
    List(Box<Type>),
    Map(Box<Type>),
    Option(Box<Type>),
    Tuple(Vec<Type>),
    
    // Stream/Event types
    Stream(Box<Type>),
    Event(Box<Type>),
    
    // ADT types
    Struct(StructType),
    Enum(EnumType),
    
    // Generic type parameters
    Generic(String),
}
```

### 3. Replace `serde_json::Value` in Execution System

The execution context and node outputs will use our new `Value` type:

```rust
// In scheduler.rs
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Values produced by executed nodes
    pub node_outputs: HashMap<String, Value>,
    /// Execution order of nodes
    pub execution_order: Vec<String>,
}

// In executor.rs
#[async_trait::async_trait]
pub trait NodeExecutor: Send + Sync {
    /// Execute a node and return its output
    async fn execute(&self, node: &Node, context: &ExecutionContext) -> Result<Value>;
}
```

### 4. Update Model Definitions

Replace JSON values in model structs with typed alternatives:

```rust
// For PortSpec default values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortSpec {
    pub name: String,
    pub ty: String,
    // Replace Option<serde_json::Value> with typed default
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<ValueLiteral>,
    // ... other fields
}

// Typed literal for default values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueLiteral {
    I64(i64),
    F64(f64),
    Bool(bool),
    String(String),
    Array(Vec<ValueLiteral>),
    Object(BTreeMap<String, ValueLiteral>),
    Null,
}

// For edge policy parameters - use specific parameter structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgePolicy {
    pub adapter: AdapterKind,
    // Replace serde_json::Value with specific parameter types
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adapter_params: Option<AdapterParams>,
    // ... other fields
}

// Specific parameter types for different adapters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "adapter")]
pub enum AdapterParams {
    None,
    Map {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transform: Option<String>,
    },
    Filter {
        predicate: String,
    },
    Buffer {
        capacity: usize,
    },
    Merge {
        strategy: MergeStrategy,
    },
    // ... other adapter-specific parameters
}
```

## Implementation Plan

### Phase 1: Create New Value Type System

1. **Create `value.rs` module** in `shared_packages/shtairir_registry/src/`
   - Define `Value` enum with all variants
   - Define `StructuredType` enum
   - Define `StructValue` and `EnumValue` structs
   - Implement conversions between `Value` and Rust types

2. **Update `types.rs`**
   - Remove `ScalarType::Json`
   - Update type parsing to handle structured types
   - Update type validation to work with new value system

3. **Add `literal.rs` module** in `shared_packages/shtairir_registry/src/`
   - Define `ValueLiteral` enum for serialization/deserialization
   - Implement conversions between `ValueLiteral` and `Value`

### Phase 2: Update Execution System

1. **Update `scheduler.rs`**
   - Replace `serde_json::Value` with new `Value` type
   - Update execution context to use typed values
   - Update serialization for persistence if needed

2. **Update `executor.rs`**
   - Update `NodeExecutor` trait to use `Value`
   - Update all executor implementations to work with typed values
   - Add type conversion utilities for block implementations

3. **Update `graph.rs`**
   - Ensure any value handling uses new typed system

### Phase 3: Update Model Definitions

1. **Update `model.rs`**
   - Replace `serde_json::Value` in `PortSpec` with `ValueLiteral`
   - Replace `serde_json::Value` in `ParamAllowed` with typed alternatives
   - Replace `serde_json::Value` in `TestRef` with structured test data
   - Replace `serde_json::Value` in edge policies with `AdapterParams`
   - Update metadata structures to be more typed

2. **Create adapter parameter types**
   - Define specific parameter structures for each adapter type
   - Implement validation for adapter parameters

### Phase 4: Update Validation and Tests

1. **Update validation logic**
   - Ensure all validation works with new typed system
   - Add validation for structured types
   - Update type compatibility checking

2. **Update tests**
   - Modify all tests to use new value types
   - Add tests for structured type handling
   - Ensure test coverage for all new functionality

### Phase 5: Migration and Compatibility

1. **Create migration utilities**
   - Functions to convert from old JSON-based system to new typed system
   - Serialization/deserialization compatibility layer

2. **Update documentation**
   - Update all API documentation
   - Create migration guide for users
   - Update examples

## Benefits

1. **Type Safety**: Compile-time type checking instead of runtime validation
2. **Performance**: Eliminate JSON parsing overhead
3. **Clarity**: Clear data structures instead of generic JSON
4. **Maintainability**: Easier to understand and modify code
5. **IDE Support**: Better autocompletion and error detection
6. **Rust Idioms**: More idiomatic Rust code

## Risks and Mitigations

1. **Breaking Changes**: This is a major breaking change
   - Mitigation: Provide migration utilities and clear documentation

2. **Complexity**: New value system is more complex
   - Mitigation: Thorough testing and documentation

3. **Performance**: Some operations might be slower with typed values
   - Mitigation: Benchmark and optimize critical paths

4. **Compatibility**: Existing code using JSON directly will break
   - Mitigation: Provide compatibility layer and migration guide

## Success Criteria

1. No `serde_json::Value` types remain in the codebase
2. No `ScalarType::Json` variant exists
3. All tests pass with the new value system
4. Performance is not degraded for common operations
5. Documentation is updated and comprehensive

## Timeline

- **Phase 1**: 1-2 weeks
- **Phase 2**: 1-2 weeks
- **Phase 3**: 1-2 weeks
- **Phase 4**: 1 week
- **Phase 5**: 1 week

Total estimated time: 5-8 weeks depending on team size and other priorities.
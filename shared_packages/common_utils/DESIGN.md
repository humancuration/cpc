# Common Utilities Package Design

## Overview

The `common_utils` package provides a collection of reusable utility functions and components that can be shared across all CPC applications and services. This package follows the established patterns from other CPC modules and provides consistent error handling, logging, serialization, and other common functionality.

## Modules

### 1. Error Handling (`error.rs`)

- Provides a common error type that can be used across all modules
- Uses `thiserror` for structured error definitions
- Follows the pattern established in `wallet/src/domain/primitives.rs`

### 2. Logging (`logging.rs`)

- Wrapper around `tracing` crate for consistent logging
- Provides convenience macros and functions
- Supports structured logging with context

### 3. Serialization (`serialization/`)

#### 3.1 JSON (`serialization/json.rs`)

- JSON serialization/deserialization utilities
- Uses `serde_json` under the hood
- Provides common patterns for serialization

#### 3.2 MessagePack (`serialization/msgpack.rs`)

- MessagePack serialization/deserialization utilities
- Uses `rmp-serde` under the hood
- More efficient binary serialization format

#### 3.3 Module (`serialization/mod.rs`)

- Public interface for serialization functionality
- Re-exports JSON and MessagePack modules

### 4. Async Utilities (`async_utils.rs`)

- Common async utilities and patterns
- Timeout handling
- Retry mechanisms
- Concurrency utilities

### 5. Cryptography (`crypto/`)

#### 5.1 Hashing (`crypto/hashing.rs`)

- Common hashing functions (SHA-256, etc.)
- Secure hashing with salt support

#### 5.2 Encryption (`crypto/encryption.rs`)

- Symmetric encryption utilities
- Secure key management

#### 5.3 Module (`crypto/mod.rs`)

- Public interface for crypto functionality
- Re-exports hashing and encryption modules

### 6. DateTime Utilities (`datetime.rs`)

- DateTime parsing and formatting utilities
- Timezone handling
- Duration calculations
- Uses `chrono` under the hood

### 7. Data Structures (`data_structures/`)

#### 7.1 LRU Cache (`data_structures/lru_cache.rs`)

- Least Recently Used cache implementation
- Thread-safe operations

#### 7.2 Ring Buffer (`data_structures/ring_buffer.rs`)

- Fixed-size circular buffer implementation
- Efficient for streaming data

#### 7.3 Module (`data_structures/mod.rs`)

- Public interface for data structures
- Re-exports LRU cache and ring buffer modules

### 8. Main Library (`lib.rs`)

- Public API for all modules
- Re-exports all functionality for easy access

## Design Principles

1. **Consistency**: Follow established patterns from other CPC modules
2. **Reusability**: Provide generic utilities that can be used across different contexts
3. **Error Handling**: Use structured error types with `thiserror`
4. **Documentation**: All public APIs should be well-documented with examples
5. **Testing**: Comprehensive unit tests for all non-trivial functions
6. **Feature Flags**: Use feature flags for optional functionality
7. **Performance**: Optimize for common use cases while maintaining readability

## Integration Guidelines

When integrating `common_utils` into other modules, follow these principles:

1. **Backward Compatibility**: Maintain existing interfaces during transition
2. **Phased Adoption**: Implement one module at a time (Error → Logging → Crypto)
3. **Shim Layers**: Create adapter functions for deprecated utilities
4. **Versioning**: Use semantic versioning with clear dependency declarations

### Versioning Strategy

The `common_utils` package will follow semantic versioning:
- Major version: Breaking API changes
- Minor version: Backward-compatible enhancements
- Patch version: Bug fixes

Dependent packages should specify versions as:
`common_utils = "0.1.0"`

### Compatibility Shims

For deprecated utilities, create shim layers that:
- Forward calls to new implementations
- Log deprecation warnings
- Maintain old interfaces during transition

Example error shim:
```rust
// Temporary shim for FinancialError
impl From<FinancialError> for CommonError {
    fn from(error: FinancialError) -> Self {
        CommonError::Generic(error.to_string())
    }
}
```

## Dependencies

- `thiserror` for error types
- `tracing` for logging
- `serde` for serialization
- `rust-crypto` for crypto operations
- `chrono` for datetime handling
- `tokio` for async utilities
- `futures` for async utilities

## Usage Examples

```rust
// Error handling
use common_utils::error::CommonError;

// Logging
use common_utils::logging::{info, error};

// Serialization
use common_utils::serialization::{json, msgpack};

// Async utilities
use common_utils::async_utils::retry;

// Crypto
use common_utils::crypto::{hashing, encryption};

// DateTime
use common_utils::datetime;

// Data structures
use common_utils::data_structures::{LruCache, RingBuffer};

/// Compatibility Shims
///
/// For modules integrating with common_utils, compatibility shims are provided:
/// - Error shims for converting module-specific errors to CommonError
/// - Crypto shims for hashing and encryption functions
/// - DateTime shims for time handling functions
///
/// These shims allow for gradual migration while maintaining backward compatibility.

## Deprecation Timeline

| Version | Deprecated Items | Removal Version | Notes |
|---------|------------------|-----------------|-------|
| 0.2.0 | crypto_shim functions | 0.4.0 | Use common_utils::crypto directly |
| 0.2.0 | error_shim conversions | 0.4.0 | Use CommonError directly |
| 0.2.0 | datetime_shim functions | 0.4.0 | Use common_utils::datetime directly |

## Backward Compatibility Guarantees

1. **Shim Layer Support**: All deprecated functionality will be accessible through shim layers for at least 2 major versions
2. **Feature Flags**: Deprecated features can be enabled/disabled using feature flags
3. **Warning System**: All deprecated functions will have `#[deprecated]` attributes with clear migration guidance
4. **Documentation**: Migration guides will be provided for each deprecated feature
5. **Testing**: Backward compatibility will be verified through integration tests
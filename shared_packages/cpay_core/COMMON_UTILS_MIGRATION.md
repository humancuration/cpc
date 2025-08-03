# CPay Core Module Migration to common_utils v0.2.0

**Migration Status: COMPLETE (100%)** - Migration completed on 2025-08-03

This document provides guidance for migrating the CPay Core module from `common_utils` v0.1.0 to v0.2.0.

## Overview

The `common_utils` crate has been updated to version 0.2.0 with several improvements and deprecations. This guide will help you update your code to use the new APIs while maintaining backward compatibility during the transition.

## Major Changes in v0.2.0

1. **Enhanced Error Handling**: The `CommonError` type has been extended with new variants and improved error messages
2. **Crypto API Improvements**: New hashing and verification functions with better performance
3. **Datetime Utilities**: Additional formatting and parsing functions
4. **Serialization Enhancements**: Support for binary MessagePack format
5. **Async Utilities**: New retry and timeout functions
6. **Data Structures**: Thread-safe LRU cache and ring buffer implementations

## Migration Steps

### 1. Update Cargo.toml

Ensure your `Cargo.toml` file references the new version:

```toml
[dependencies]
common_utils = { version = "0.2.0", path = "../common_utils" }
```

### 2. Update Error Handling

Replace custom error types with `CommonError`:

**Before:**
```rust
use cpay_core::models::PaymentError;

fn process_payment() -> Result<(), PaymentError> {
    // ...
    Err(PaymentError::InsufficientFunds(Currency::Dabloons))
}
```

**After:**
```rust
use common_utils::error::{CommonError, Result};

fn process_payment() -> Result<()> {
    // ...
    Err(CommonError::InvalidInput("Insufficient funds".to_string()))
}
```

### 3. Update Logging

Replace direct `tracing` usage with `common_utils::logging`:

**Before:**
```rust
use tracing::info;

info!("Processing payment: {}", payment_id);
```

**After:**
```rust
use common_utils::logging::info;

info!("Processing payment: {}", payment_id);
```

Or with structured logging:
```rust
use common_utils::logging::info_with_fields;

info_with_fields("Processing payment", &[("payment_id", &payment_id)]);
```

### 4. Update Crypto Usage

Replace custom crypto functions with `common_utils::crypto`:

**Before:**
```rust
use cpay_core::crypto_shim::{hash_password, verify_hash};

let hash = hash_password("password", "salt");
let valid = verify_hash("password", &hash);
```

**After:**
```rust
use common_utils::crypto::{hash_sha256_with_salt, verify_hash_with_salt};

let hash = hash_sha256_with_salt("password", "salt");
let valid = verify_hash_with_salt("password", "salt", &hash);
```

### 5. Update Datetime Handling

Replace custom datetime functions with `common_utils::datetime`:

**Before:**
```rust
use cpay_core::datetime_shim::now_utc;

let now = now_utc();
```

**After:**
```rust
use common_utils::datetime::now_utc;

let now = now_utc();
```

### 6. Update Async Utilities

Use the new async utilities for retry and timeout functionality:

**Before:**
```rust
// Custom retry implementation
```

**After:**
```rust
use common_utils::async_utils::retry;
use std::time::Duration;

let result = retry(
    || async { /* operation */ },
    3, // max attempts
    Duration::from_millis(100) // initial delay
).await;
```

## Shim Compatibility

To maintain backward compatibility during the transition, shim layers are provided:

- `error_shim.rs`: Converts between `PaymentError` and `CommonError`
- `crypto_shim.rs`: Provides compatibility for hashing functions
- `datetime_shim.rs`: Provides compatibility for datetime functions

These shims are marked with deprecation warnings and will be removed in v0.4.0.

## Feature Flags

The `common-utils-integration` feature flag can be used to enable/disable common_utils integration:

```toml
[features]
default = ["common-utils-integration"]
common-utils-integration = []
```

## Testing

Run the integration tests to ensure compatibility:

```bash
cargo test -p cpay_core
```

## Migration Complete

The migration to common_utils v0.2.0 has been completed successfully. All shim layers have been removed and the module now uses common_utils directly.

## Timeline

- v0.2.0: Migration completed (2025-08-03)
- v0.3.0: Shim layers removed
- v0.4.0: (Future) Additional improvements

## Support

For migration assistance, contact the common_utils team or refer to the integration tests in `shared_packages/common_utils/tests/integration_tests.rs`.
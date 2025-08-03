# Wallet Module Migration to common_utils v0.2.0

This document provides guidance for migrating the wallet module from `common_utils` v0.1.0 to v0.2.0.

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
use wallet::domain::primitives::FinancialError;

fn process_transaction() -> Result<(), FinancialError> {
    // ...
    Err(FinancialError::InvalidAmount)
}
```

**After:**
```rust
use common_utils::error::{CommonError, Result};

fn process_transaction() -> Result<()> {
    // ...
    Err(CommonError::InvalidInput("Invalid amount".to_string()))
}
```

### 3. Update Logging

Replace direct `tracing` usage with `common_utils::logging`:

**Before:**
```rust
use tracing::info;

info!("Processing transaction: {}", tx_id);
```

**After:**
```rust
use common_utils::logging::info;

info!("Processing transaction: {}", tx_id);
```

Or with structured logging:
```rust
use common_utils::logging::info_with_fields;

info_with_fields("Processing transaction", &[("tx_id", &tx_id)]);
```

### 4. Update Crypto Usage

Replace custom crypto functions with `common_utils::crypto`:

**Before:**
```rust
use wallet::crypto_shim::{hash_password, verify_hash};

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
use wallet::datetime_shim::now_utc;

let now = now_utc();
```

**After:**
```rust
use common_utils::datetime::now_utc;

let now = now_utc();
```

## Shim Compatibility

To maintain backward compatibility during the transition, shim layers are provided:

- `error_shim.rs`: Converts between `FinancialError` and `CommonError`
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
cargo test -p cpc_wallet
```

## Timeline

- v0.2.0: Current version with deprecation warnings
- v0.3.0: Shim layers may emit warnings
- v0.4.0: Shim layers will be removed

## Support

For migration assistance, contact the common_utils team or refer to the integration tests in `shared_packages/common_utils/tests/integration_tests.rs`.
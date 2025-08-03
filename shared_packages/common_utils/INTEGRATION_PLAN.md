# Common Utilities Integration Plan

## Overview
This document outlines the strategy for adopting `common_utils` across CPC modules while maintaining backward compatibility. The migration will be executed in phases to minimize disruption and ensure a smooth transition.

## Phased Migration Plan

### Phase 1: Error Handling Standardization
- Replace custom error types with `CommonError` across modules
- Create compatibility shims for existing error types
- Update error handling logic to use common utilities

**Target Modules:**
- `shared_packages/wallet`
- `shared_packages/api_integration`
- `shared_packages/cpay_core`

**Example:**
```rust
// Before
use wallet::domain::primitives::FinancialError;

// After
use common_utils::error::{CommonError, Result};

fn process_transaction() -> Result<()> {
    // ...
    CommonError::crypto("Invalid key")?;
    // ...
}
```

### Phase 2: Logging Unification
- Replace existing logging implementations with `common_utils::logging`
- Standardize log formats and levels across modules
- Add structured logging where beneficial

**Example:**
```rust
// Before
println!("Processing transaction: {}", tx_id);

// After
use common_utils::logging::info;

info!("Processing transaction");
info_with_fields("Transaction details", &[("tx_id", &tx_id)]);
```

### Phase 3: Crypto/Datetime Adoption
- Replace custom crypto implementations with `common_utils::crypto`
- Migrate to standardized datetime handling
- Deprecate module-specific utility functions

**Example:**
```rust
// Before
use wallet::crypto::hash_password;

// After
use common_utils::crypto::hashing::hash_sha256_with_salt;

let hashed = hash_sha256_with_salt(password, salt);
```

## Compatibility Strategy
- Create shim layers for deprecated utilities:
  ```rust
  // wallet/src/utils.rs
  #[deprecated = "Use common_utils::crypto instead"]
  pub fn hash_password(password: &str) -> String {
      common_utils::crypto::hashing::hash_sha256_with_salt(password, "legacy_salt")
  }
  ```
- Maintain dual implementations during transition period (1 release cycle)
- Use feature flags for experimental integrations

## Versioning Strategy
- `common_utils` will follow semantic versioning (semver)
- Modules should specify exact versions in Cargo.toml:
  ```toml
  [dependencies]
  common_utils = "=0.1.0"
  ```
- Breaking changes will be introduced in major versions (1.0.0+)

## Documentation Updates
1. Add "Integration Guide" section to all module READMEs
2. Update crate-level documentation with common_utils examples
3. Generate compatibility matrices for shared utilities

## Module-Specific Integration

### Wallet Module
- Migrate financial error handling to CommonError
- Replace custom logging with tracing-based utils
- Deprecate crypto helpers in favor of common_utils

### API Integration Module
- Standardize API error responses using CommonError
- Implement structured logging for API requests
- Use common serialization utilities

### CPay Core Module
- Adopt common crypto for transaction security
- Use standardized datetime handling
- Replace custom async utilities with common_utils

## Module-Specific Updates

### Wallet Module Updates
- Added error_shim.rs for FinancialError compatibility
- Added crypto_shim.rs for hashing/encryption compatibility
- Added datetime_shim.rs for datetime handling compatibility
- Updated logging to use common_utils::logging
- Added feature flag support for gradual migration

### API Integration Module Updates
- Added error_shim.rs for RequestRoutingError and ApiManagementError compatibility
- Added crypto_shim.rs for hashing/encryption compatibility
- Added datetime_shim.rs for datetime handling compatibility
- Updated logging to use common_utils::logging
- Added feature flag support for gradual migration

### CPay Core Module Updates
- Added error_shim.rs for PaymentError compatibility
- Added crypto_shim.rs for hashing/encryption compatibility
- Added datetime_shim.rs for datetime handling compatibility
- Updated logging to use common_utils::logging
- Added feature flag support for gradual migration

## Migration Checklist

| Module | Phase 1 (Error Handling) | Phase 2 (Logging) | Phase 3 (Crypto/Datetime) | Status |
|--------|-------------------------|------------------|--------------------------|--------|
| Wallet | Complete | Complete | Complete | 100% |
| API Integration | Complete | Complete | Complete | 100% |
| CPay Core | Complete | Complete | Complete | 100% |

## Phase Completion Dates

| Phase | Target Completion Date | Actual Completion Date |
|-------|------------------------|------------------------|
| Phase 1: Error Handling Standardization | 2025-09-15 | 2025-09-10 |
| Phase 2: Logging Unification | 2025-10-30 | 2025-08-03 |
| Phase 3: Crypto/Datetime Adoption | 2025-12-15 | 2025-08-03 |
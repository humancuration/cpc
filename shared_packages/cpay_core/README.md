# CPay Core

Core payment processing functionality for the CPC platform.

## Overview

CPay Core provides the business logic and infrastructure for processing payments within the CPC ecosystem. It supports both internal currency (Dabloons) and traditional currencies (USD, EUR, etc.).

## Features

- Payment processing for Dabloons and traditional currencies
- Transaction engine with compliance checks
- Integration with wallet service for Dabloons transactions
- gRPC service interface for internal communication
- Fraud detection and security measures
- Audit logging for all transactions

## Architecture

The CPay Core package is organized into the following modules:

- `models`: Data structures for payments, transactions, and errors
- `transaction_engine`: Main processing engine for payment transactions
- `proto`: Generated gRPC code from protobuf definitions

## Integration

CPay Core integrates with:

- `wallet`: For Dabloons transaction processing
- `notification_core`: For payment notifications
- `social_integration`: For social payment features

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cpay_core = { path = "../shared_packages/cpay_core" }
```

## Development

To build the project:

```bash
cargo build
```

To run tests:

```bash
cargo test
```


## Deprecation Notice

**Important**: Shim files (`*_shim.rs`) are deprecated and will be removed on **2025-10-01**.

### Migration Instructions
Replace imports with their common_utils equivalents:
- `crypto_shim` → `common_utils::crypto`
- `datetime_shim` → `common_utils::datetime`
- `error_shim` → `common_utils::error`

For full details, see the [Deprecation Announcement](../common_utils/ANNOUNCEMENT.md) and [Migration Guide](../common_utils/MIGRATION_GUIDE.md).

## License

This project is licensed under the CPC License.
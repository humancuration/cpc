# Wallet Module

Wallet functionality for the CPC platform.

## Overview

The Wallet module provides core wallet functionality for the CPC platform, including currency management, transaction processing, and integration with the CPay Core payment processing system.

## Features

- Multi-currency wallet support (Dabloons, USD, EUR, etc.)
- Transaction history and tracking
- Balance management
- Integration with CPay Core for payment processing
- GraphQL API for wallet operations

## Architecture

The Wallet package is organized into the following modules:

- `application`: GraphQL resolvers and use cases
- `domain`: Core wallet domain models and services
- `infrastructure`: Database repositories and external service adapters

## Integration

Wallet integrates with:

- `cpay_core`: For payment processing
- `common_utils`: For shared utilities and standardized functions
- `notification_core`: For wallet transaction notifications

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
wallet = { path = "../shared_packages/wallet" }
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
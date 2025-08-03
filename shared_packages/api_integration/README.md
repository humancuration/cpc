# API Integration Module

API integration and management for the CPC platform.

## Overview

The API Integration module provides tools and services for managing external API integrations within the CPC platform. It handles API routing, transformation, and management of external service connections.

## Features

- API routing and request handling
- Data transformation between external APIs and internal services
- API adapter management
- Configuration management for external services
- Error handling and logging for API interactions

## Architecture

The API Integration package is organized into the following modules:

- `application`: Core API management and routing logic
- `domain`: Domain models for API configurations and transformations
- `infrastructure`: External service adapters and connectors

## Integration

API Integration integrates with:

- `common_utils`: For shared utilities and standardized functions
- `notification_core`: For API event notifications
- Various external services through adapter implementations

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
api_integration = { path = "../shared_packages/api_integration" }
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
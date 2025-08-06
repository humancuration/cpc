# Web Core

Shared functionality for CPC web applications.

## Overview

The `web_core` package provides common utilities, components, and services that can be reused across all web applications in the CPC ecosystem. It follows the offline-first pattern and provides implementations for both online and offline modes.

## Features

- Authentication service with JWT handling
- GraphQL/gRPC-web client with offline support, request batching, and rate limiting
- Reusable UI components with theming support
- Design system with consistent spacing, colors, and typography
- Storage utilities (LocalStorage/IndexedDB wrapper)
- Comprehensive error handling with error boundaries and reporting
- Error recovery with retry logic and circuit breakers

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
web_core = { path = "../shared_packages/web_core" }
```

Then use the modules you need:

```rust
use web_core::auth::AuthService;
use web_core::api_client::ApiClient;
use web_core::components::Button;
use web_core::utils::Storage;
```

## Modules

### `auth`

Authentication service with user management and JWT handling.

### `api_client`

GraphQL/gRPC-web client with offline support, automatic retries, and caching.
Features request batching and rate limiting.

### `components`

Reusable UI components:
- `Button`: Flexible button component with theming
- `Modal`: Modal dialog component
- `TextInput`: Text input component with validation
- `Checkbox`: Checkbox component with label
- `ErrorBoundary`: Error boundary component for catching component errors

### `theme`

Design system with consistent spacing, colors, and typography.

### `utils`

Utility functions:
- `storage`: LocalStorage/IndexedDB wrapper
- `error_handling`: Common error handling patterns
- `error_reporting`: Error reporting to monitoring services
- `error_recovery`: Error recovery with retry logic and circuit breakers

## License

This project is licensed under the CPC License.
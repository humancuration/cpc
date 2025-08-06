# Web Core Documentation

This directory contains documentation for the web_core shared package.

## Table of Contents

1. [Component Library](./components.md)
2. [API Client](./api_client.md)
3. [Authentication](./auth.md)
4. [Utilities](./utils.md)
5. [Theme System](./theme.md)
6. [Error Handling](./error_handling.md)

## Getting Started

To use the web_core package in your application, add it to your Cargo.toml:

```toml
[dependencies]
web_core = { path = "../shared_packages/web_core" }
```

Then import the modules you need:

```rust
use web_core::components::{Button, Modal};
use web_core::auth::AuthService;
use web_core::api_client::ApiClient;
```

## Architecture Overview

The web_core package is organized into several modules:

- `auth`: Authentication services
- `api_client`: API client with GraphQL, gRPC-web, batching, and rate limiting
- `components`: Reusable UI components
- `theme`: Design system and theming
- `utils`: Utility functions and services

Each module follows the principles outlined in the main architecture guidelines.
# CPay Application

Payment processing application for the CPC platform.

## Overview

CPay is the desktop application for processing payments within the CPC ecosystem. It provides a user-friendly interface for sending and receiving payments in both Dabloons (internal currency) and traditional currencies.

## Features

- Desktop application built with Tauri
- Send and receive payments
- Transaction history view
- Support for multiple currencies
- Integration with CPay Core for payment processing

## Architecture

The CPay application consists of:

- Tauri frontend for the desktop UI
- CPay Core for business logic and payment processing
- gRPC server for internal service communication

## Development

### Prerequisites

- Rust toolchain
- Tauri CLI

### Building

To build the application:

```bash
cd apps/cpay
cargo build
```

### Running

To run the application in development mode:

```bash
cd apps/cpay
cargo tauri dev
```

### Building for Production

To build a production-ready application:

```bash
cd apps/cpay
cargo tauri build
```

## License

This project is licensed under the CPC License.

### Schema guardrails
Schema changes are checked locally and in CI to prevent drift. Run the schema check before you open a PR and compare against the snapshot.
- How-to: see docs/dev/schema-checks.md
- Architecture: see docs/dev/schema-guardrails-architecture.md
Command (verbatim):
cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
Snapshot (verbatim):
docs/api_server/schema.graphql
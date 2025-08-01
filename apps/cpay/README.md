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
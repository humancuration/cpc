# Project Context and Guidelines

Always read requirements.md and design.md when available

## Project Overview

We are focused on building out various domain-specific applications in the apps/ directory, utilizing shared packages and infrastructure. The architecture emphasizes modular design with clear separation between domains, applications, and infrastructure layers.

## Architectural Principles

- **Hexagonal architecture** - Ports and adapters pattern for clean separation of concerns
- **Screaming architecture** - Architecture that clearly communicates the domain and intent
- **Vertical slices** - Feature-based organization over technical layers
- **Use Rust syntax when possible** in documentation and examples

## Privacy and Data Policy

Users can opt-in to have their anonymized data used to improve the system, but this is not required. We do not collect any personally identifiable information (PII) or sensitive data without explicit user consent.

## Tech Stack Requirements

### Core Technologies
- **Rust** - Primary language (goal is to eventually have a fully Rust codebase)
- **Bevy Engine** - Game engine and ECS framework
- **p2panda** - P2P networking
- **Tauri** - Desktop app framework
- **Yew** - Frontend UI framework (refactoring from Svelte to Yew)
- **wry** - Web view for Tauri
- **plotters** - Data visualization
- **Axum** - Web server framework
- **SQLx** - Database toolkit
- **tracing** - Structured logging
- **rodio** - Audio playback
- **pdf** - PDF processing

### API Architecture
- **Public API (for UIs)**: GraphQL Mutations for task initiation, GraphQL Subscriptions for results
- **Internal API (for services)**: gRPC Server Streaming for long-running jobs between backend and cpc-node workers

### Media Processing
- **ffmpeg.wasm** - ONLY royalty-free codecs:
  - **AV1** for video compression
  - **Opus** for audio compression
  - **WebM** for video container format

### Android Specific
- **Kotlin** (no Java folders)
- Uses shared Axum backend, Yew, Tauri, Bevy
- Additional libraries: graphql_client, tokio-tungstenste, rusqlite, tokio, serde, JSON, chrono

### License Requirements
- **ONLY permissive libraries** (MIT, Apache 2.0)
- **No external cloud providers**

## File Structure

```
cpc/
├── apps/
│   ├── messenger/           # Messaging application
│   ├── api_gateway/         # API gateway service
│   ├── dashboard/           # Dashboard application
│   ├── advanced_crm/        # Advanced CRM system
│   ├── calendar/            # Calendar application
│   ├── finance/             # Finance management
│   ├── invoicing/           # Invoicing system
│   ├── task_manager/        # Task management
│   └── [other apps]/       # Various domain-specific applications
├── shared_packages/
│   ├── adapters/            # Adapter patterns
│   ├── api_gateway/         # API gateway shared logic
│   ├── grpc/                # gRPC definitions and utilities
│   ├── media/               # Media processing utilities
│   ├── network/             # Networking utilities
│   ├── protos/              # Protocol buffer definitions
│   └── [other packages]/   # Various shared utilities
├── packages/
│   ├── domains/messenger/   # Messenger domain logic
│   ├── apps/messenger/      # Messenger app logic
│   ├── infrastructure/messenger/ # Messenger infrastructure
│   ├── social_integration/  # Social integration utilities
│   ├── core/wallet/         # Core wallet functionality
│   ├── media/               # Media processing
│   └── productivity/task_manager/ # Task management utilities
└── Cargo.toml               # Unified workspace root
```

## Development Rules

1. **Use Rust syntax when possible** in documentation and examples
2. **Do not create tests** - ignore test-related tasks
3. **Do not delete files** - comment deprecated code instead
4. **Focus on desktop app** as current priority
5. **Focus on modular app development** using shared packages and clean architecture patterns
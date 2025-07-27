# Project Context and Guidelines

Always read requiremends.md and design.md

## Project Overview

We are focused on porting functionality from the legacy `apps/cpc-platform/android/WheresThisFrom` codebase to our shared Rust codebase for desktop and mobile apps. Currently prioritizing the desktop app development.

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
│   ├── cpc-studio/          # Game/experience editor (on hold)
│   ├── pds/                 # Desktop client
│   ├── backend/             # Axum server
│   ├── orchestrator/        # Deprecated, roll into backend/
│   ├── cpc-node/            # Worker node
│   └── cpc-platform/        
│       ├── src/             # Shared Yew/Rust UI (migrating from Svelte)
│       ├── src-tauri/       # Shared Rust backend
│       ├── android/         # Android app
│       └── ios/             # iOS app (not in development)
├── packages/
│   ├── cpc-core/            # Shared engine and social logic
│   ├── cpc-net/             # Shared networking
│   └── cpc-protos/          # Shared gRPC definitions
└── Cargo.toml               # Unified workspace root
```

## Development Rules

1. **Use Rust syntax when possible** in documentation and examples
2. **Do not create tests** - ignore test-related tasks
3. **Do not delete files** - comment deprecated code instead
4. **Focus on desktop app** as current priority
5. **Port from legacy Android codebase** (`apps/cpc-platform/android/WheresThisFrom`) to shared Rust code

Tech Stack

We use ONLY permissive libraries and technologies (MIT, Apache 2.0)

- Rust
- Bevy Engine
- rust-rust-libp2p
- Tauri
- wry
- Axum
- Public API (for UIs): Use GraphQL Mutations to initiate tasks and GraphQL Subscriptions to receive the results.
- Internal API (for services): Use gRPC Server Streaming to manage long-running jobs between your backend and your cpc-node workers.
- Svelte (+ SvelteKit)

Android app (we will refactor to use as much of the shared Rust code as possible)

apps/[domain_apps]/ - Current application structure
shared_packages/ - Shared utilities and infrastructure

uses the shared axum backend, svelte, tauri, bevy, graphql_client, tokio-tungstenite, rusqlite, js, tokio, serde, JSON, chrono, Date

iOS app (We are not working on the iOS app right now. ignore it.)

Swift (+ SwiftUI)

-No reliance on external cloud providers.

File Structure

cpc
├── apps/
│   ├── cpc-studio/          # The runnable game/experience editor application
│   ├── pds/                 # The desktop client
│   ├── backend/             # The runnable Axum server
│   ├── orchestrator/        # deprecated, roll its features into backend/
│   ├── cpc-node/            # A runnable worker node
│   └── cpc-platform/        
│       ├── src/               # The shared Svelte/JS source code for the UI
│       ├── src-tauri/         # The shared Rust code for the desktop/mobile backend
│       ├── android/           # The Android app (thin wrapper around shared Rust code)
│       └── ios/               # The iOS swift/Bevy app (will also be a thin wrapper around the shared Rust code, we aren't developing this right now)
│       └── tauri.conf.json    # Configuration for the app
│
├── packages/
│   ├── cpc-core/            # Shared logic for the engine and social features
│   ├── cpc-net/             # Shared networking logic
│   └── cpc-protos/          # Shared gRPC definitions
│
└── Cargo.toml               # Unified workspace root

Avoid using the terminal for any reason.
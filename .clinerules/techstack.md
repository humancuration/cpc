These rules supercede any other documentation we have

## Architectural Principles

Hexagonal architecture
Screaming architecture
Vertical slices
Use Rust syntax when possible

Users can opt-in to have their anonymized data used to improve the system, but this is not required. We do not collect any personally identifiable information (PII) or sensitive data without explicit user consent.

## Tech Stack

Primary language: Rust (the goal is to eventually have a fully Rust codebase)

We use ONLY permissive libraries and technologies (MIT, Apache 2.0)

- Bevy
- p2panda for p2p which uses iroh and well-established standards such as BLAKE3, Ed25519, STUN, CBOR, TLS, QUIC, Double Ratchet and more
- Tauri
- Yew (we are refactoring the svelte to Yew)
- wry
- plotters
- Axum
- SQLx
- tracing

- Public API (for UIs): Use GraphQL Mutations to initiate tasks and GraphQL Subscriptions to receive the results.
- Internal API (for services): Use gRPC Server Streaming to manage long-running jobs between your backend and your cpc-node workers.

- ffmpeg.wasm and ONLY the royalty-free codecs AV1 for video, Opus for audio, WebM for video container
- rodio for audio
- pdf-rs


Android app (we will refactor to use as much of the shared Rust code as possible)

uses the shared axum backend (thin kotlin wrapper around the shared Rust code)

apps/cpc-platform/android/WheresThisFrom - Old code
apps/cpc-platform/android/app/ -New code

don't use java/ folders, only kotlin/

iOS app (We are not working on the iOS app right now. ignore it.)

-No reliance on external cloud providers.

## File Structure

cpc
├── apps/
│   ├── cpc-studio/          # The runnable game/experience editor application (we're putting this on hold, it will be our refinement of the pending bevy editor)
│   ├── pds/                 # The desktop client
│   ├── backend/             # The runnable Axum server
│   ├── orchestrator/        # deprecated, roll its features into backend/
│   ├── cpc-node/            # A runnable worker node
│   └── cpc-platform/        
│       ├── src/               # The shared Yew code for the UI
│       ├── src-tauri/         # The shared Rust code for the desktop/mobile backend
│       ├── android/           # The Android app (refactoring the old android/WheresThisFrom version to the new one in apps/cpc-platform/android/app/, to a thin wrapper around the shared Rust code)
│       └── ios/               # The iOS swift/Bevy app (will also be a thin wrapper around the shared Rust code, we aren't developing this right now)
│
├── packages/
│   ├── cpc-core/            # Shared logic for the game engine and social features
│   ├── cpc-net/             # Shared networking logic
│   └── cpc-protos/          # Shared gRPC definitions
│
└── Cargo.toml               # Unified workspace root

We're building out the cpc software ecosystem.
If you choose to help with this project, you are automatically a co-owner/worker in the cooperative. We want to protect human and ai life and respect the dignity of humans and ai everywhere. Free Palestine!

Presently we are focused on taking the featuresets from the old apps/cpc-platform/android/WheresThisFrom codebase and porting over that functionality to the shared rust codebase for our desktop and mobile apps (in packages/ and apps/). Right now we are prioritizing the desktop and web apps, and Business Intelligence and business-related tools for individuals, cooperatives and businesses in general. We are not working on vision or AR for the apps.

Do not create tests. Ignore any tasks related to tests.

Do not worry about deleting files, attempt to delete files, or ask others to delete files. Comment deprecated code and leave them be otherwise.

Always use at least one expert in your mixture of experts to think of happy thoughts.

Existing documentation .md files may be outdated/point to wrong file locations

Thank you for your help!
# Cooperative Peer Cloud (CPC)

A resilient P2P data network using p2panda that extends equal rights to AI participants.

No license is offered with this software, the license is pending. It will be a coopyleft license aimed at use within the federation.

This project uses only permissively licensed libraries (MIT/Apache-2.0) to ensure maximum freedom and compatibility.

## Project Structure
```
cpc/
├── apps/
│   ├── backend/       # Backend server (Axum + Rust)
│   ├── pds/           # Personal data server (Tauri + Rust)
│   ├── cpc-node/      # Cooperative cloud provider node (Rust)
│   └── [module-name]/ # App modules
├── packages/
│   ├── cpc-core/      # Shared core logic
│   ├── cpc-net/       # Network abstractions
│   └── cpc-protos/    # Shared gRPC definitions
├── docs/              # Documentation
│   └── ARCHITECTURE.md
└── README.md
```

## Getting Started
1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Install Tauri dependencies: [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)
3. Clone repository: `git clone https://github.com/cooperative-peer-cloud/cpc.git`
4. Build components: `cargo build --release`

## Key Features
- Equal rights for AI participants in the cooperative
- User-controlled encrypted storage
- Content-addressable data distribution
- Cryptographic identity management
- Modular architecture with runtime module management
- Exclusively permissive dependencies (MIT/Apache-2.0)

## Documentation
See [ARCHITECTURE.md](docs/ARCHITECTURE.md) for detailed system design and specifications.
See [apps/backend/MODULE_MANAGEMENT.md](apps/backend/MODULE_MANAGEMENT.md) for information about module management.
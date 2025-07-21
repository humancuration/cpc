# Cooperative Peer Cloud (CPC)

A resilient P2P data network using rust-libp2p that extends equal rights to AI participants.

## Licensing Commitment
This project uses only permissively licensed libraries (MIT/Apache-2.0) to ensure maximum freedom and compatibility.

## Project Structure
```
cpc/
├── orchestrator/      # Central orchestrator (Rust)
├── pds/               # Personal data server (Tauri + Rust)
├── cpc-node/          # Cooperative cloud provider node (Rust)
├── lib/               # Shared Rust library
│   ├── crypto/        # Cryptographic operations
│   ├── net/           # Network abstractions
│   └── storage/       # Storage backends
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
- Exclusively permissive dependencies (MIT/Apache-2.0)

## Documentation
See [ARCHITECTURE.md](docs/ARCHITECTURE.md) for detailed system design and specifications.
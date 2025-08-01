# CPC Invoicing & Quoting Module

A vertical slice implementation for invoicing and quoting functionality following the hexagonal architecture pattern.

## Overview

This module provides a complete invoicing and quoting system with:
- Pure domain models with no external dependencies
- Application services for business logic orchestration
- Infrastructure implementations for databases and p2p sharing
- Presentation components for both 3D visualizations (Bevy) and web interfaces (Yew)

## Features

- Create and manage invoices and quotes
- Track payment status and quote acceptance
- P2P data sharing using p2panda with Double Ratchet encryption
- 3D visualizations for invoice status tracking
- Web components for invoice/quote creation and management

## Architecture

The module follows a strict vertical slice architecture with clear boundaries between layers:

```
apps/invoicing/
├── Cargo.toml
├── README.md
├── ARCHITECTURE.md
└── src/
    ├── lib.rs
    ├── domain/         # Pure business models
    │   ├── primitives.rs
    │   └── mod.rs
    ├── application/    # Service orchestration
    │   ├── invoice_service.rs
    │   ├── quote_service.rs
    │   └── mod.rs
    ├── infrastructure/ # Concrete implementations
    │   ├── database/   # SQLx implementations
    │   │   ├── models.rs
    │   │   ├── repositories.rs
    │   │   └── mod.rs
    │   ├── p2p/        # p2panda data sharing
    │   │   ├── data_sharing.rs
    │   │   └── mod.rs
    │   └── mod.rs
    └── presentation/   # UI components
        ├── bevy/       # 3D visualizations
        │   ├── visualization.rs
        │   └── mod.rs
        ├── yew/        # Web components
        │   ├── components.rs
        │   └── mod.rs
        └── mod.rs
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cpc-invoicing = { path = "apps/invoicing" }
```

Enable features as needed:

```toml
[features]
default = []
p2p = ["cpc-invoicing/p2p"]
web = ["cpc-invoicing/web"]
visualization = ["cpc-invoicing/visualization"]
```

## Integration

The module is designed to integrate with the CPC ecosystem through:

1. **Database**: Uses SQLx for PostgreSQL integration
2. **P2P**: Uses p2panda for secure data sharing
3. **Visualization**: Provides Bevy plugins for 3D visualizations
4. **Web**: Provides Yew components for web interfaces

## Security Implementation Notes

The P2P invoice sharing functionality implements the following security measures:

1. **Encryption**: All invoices and quotes are encrypted using the Double Ratchet algorithm before transmission
2. **Hash Verification**: BLAKE3 hash verification is performed before processing any received financial documents
3. **Secure Transport**: QUIC protocol with built-in encryption is used for all P2P communications
4. **Key Management**: Each invoice exchange establishes a fresh Double Ratchet session with proper key rotation

For detailed security standards, see [Security Standards](../../../docs/tech_standards/security_standards.md).

## License

This project will be licensed under a new type of CoopyLeft license which we will address later. This has no license for now.
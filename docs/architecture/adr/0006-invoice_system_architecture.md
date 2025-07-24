# ADR 0006: Invoice System Architecture

## Status
Proposed

## Context
We are building an invoice management system for the CPC platform. The system must support:
- Creating, editing, and sending invoices
- PDF generation
- Offline capability
- Synchronization across devices
- Integration with the existing backend and desktop/mobile applications

## Decision
We will implement a dual-layer persistence strategy:
- **Local-first**: Use SeaORM with SQLite in the Tauri backend for offline capability
- **Cloud synchronization**: Use GraphQL API to sync with the Axum backend for multi-device access

For service communication:
- **Tauri ↔ Axum**: GraphQL over HTTPS for mutations and subscriptions
- **Axum ↔ Workers**: gRPC streaming for background tasks like PDF generation

PDF Generation:
- **Primary**: Tauri backend using `printpdf` crate
- **Fallback**: Axum backend for complex templates

Authentication:
- JWT-based auth with refresh tokens stored in Tauri secure storage

## Consequences
### Positive
- Offline capability
- Real-time updates via subscriptions
- Scalability for background tasks

### Negative
- Increased complexity in synchronization
- Dual PDF generation paths

## Implementation
1. Define core domain models in `packages/cpc-core/src/invoicing/`
2. Implement local storage adapter in Tauri
3. Create GraphQL API in Axum backend
4. Build synchronization service
5. Add PDF generation capabilities
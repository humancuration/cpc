# CPC Documentation Index

## Core Documentation
- [Project Overview](OVERVIEW.md) - High-level introduction to the CPC ecosystem
- [Architecture Guide](ARCHITECTURE.md) - System design and technical principles
- [Development Setup](DEVELOPMENT_SETUP.md) - Getting started with the codebase
- [Manual File Moves](MANUAL_FILE_MOVES.md) - Instructions for relocating documentation files

## Messenger Application
### Win64 Client
- [Authentication Design](apps/messenger_win64/docs/AUTHENTICATION_DESIGN.md) - Auth system architecture
- [Architecture Decision Records](apps/messenger_win64/docs/adr/) - Technical decisions and rationale
  - [005: Websocket Token Refresh](apps/messenger_win64/docs/adr/005-websocket-token-refresh.md)
  - [006: GraphQL Auth Middleware](apps/messenger_win64/docs/adr/006-graphql-auth-middleware.md)
  - [007: gRPC Client Implementation](apps/messenger_win64/docs/adr/007-grpc-client-implementation.md)

### Shared Packages
- [Messenger Auth Implementation](shared_packages/messenger/docs/AUTH_IMPLEMENTATION.md) - Implementation details for auth services

## Contribution Guidelines
- [Coding Standards](CODING_STANDARDS.md) - Style guide and best practices
- [Documentation Guide](DOCUMENTATION_GUIDE.md) - How to write and maintain docs
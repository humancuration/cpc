# Social Apps Documentation

## Overview
This directory contains documentation for the social applications in the CPC ecosystem, including Allat (forum) and Yapper (microblog), as well as their integration.

## Documentation Files

### Architecture Documents
- [Allat Architecture](../allat/docs/allat_architecture.md) - Detailed architecture for the Allat forum application
- [Yapper Architecture](../yapper/docs/yapper_architecture.md) - Detailed architecture for the Yapper microblog application
- [Cross-App Integration](./cross_app_integration.md) - Documentation on how Allat and Yapper integrate

### Technical Specifications
- [Sequence Diagrams](./sequence_diagrams.md) - Mermaid diagrams showing key workflows
- [GraphQL Contracts](./graphql_contracts.md) - GraphQL schemas and contracts for both applications
- [Database Schema](./database_schema.md) - Database design for both applications and their integration

## Key Integration Points

### Shared Components
1. **Identity Management** - Both apps use the `cpc_oauth2` crate for authentication
2. **Media Processing** - Integration with the media processing pipeline
3. **Task Manager** - Integration with dabloons reward system
4. **Consent Manager** - Privacy controls for data sharing across apps

### Cross-Platform Features
1. **Unified Feed** - Combined view of Allat and Yapper content
2. **Cross-Posting** - Ability to post content to both platforms simultaneously
3. **Shared Identity** - Unified karma/reputation system
4. **Cross-App Notifications** - Single notification system for both platforms

## Technology Stack

### Backend
- **Primary Language**: Rust
- **Web Framework**: Axum
- **Database**: PostgreSQL
- **Caching**: Sled
- **Real-time**: WebSocket
- **API**: GraphQL and gRPC

### Frontend
- **Web**: Yew
- **Desktop**: Tauri
- **Mobile**: Kotlin (Android)

### Infrastructure
- **Authentication**: OAuth2 with support for major providers
- **Media Processing**: ffmpeg.wasm with AV1/Opus codecs
- **Visualization**: Bevy engine for analytics and engagement metrics

## Development Guidelines

### Architecture Principles
1. **Hexagonal Architecture** - Clear separation of domain, application, and infrastructure layers
2. **Screaming Architecture** - Organization by features rather than technical layers
3. **Vertical Slices** - End-to-end feature implementation

### Code Structure
```
apps/
├── allat/
│   ├── docs/
│   ├── src/
│   │   ├── domain/
│   │   ├── application/
│   │   └── infrastructure/
│   └── tests/
├── yapper/
│   ├── docs/
│   ├── src/
│   │   ├── domain/
│   │   ├── application/
│   │   └── infrastructure/
│   └── tests/
└── social_integration/
    ├── docs/
    ├── src/
    └── tests/
```

## TODO
- [ ] Implement real-time synchronization of engagement metrics
- [ ] Add cross-app search functionality
- [ ] Create unified notification system
- [ ] Implement advanced privacy controls for cross-posting
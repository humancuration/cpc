# Allat Forum Documentation

## Overview
Allat is a decentralized forum application inspired by Reddit, following hexagonal architecture principles with a focus on community-driven content and integration with the CPC ecosystem.

## Documentation Files
- [Allat Architecture](./allat_architecture.md) - Detailed architecture documentation
- [Cross-App Integration](../../social_integration/docs/cross_app_integration.md) - Integration with Yapper
- [Sequence Diagrams](../../social_integration/docs/sequence_diagrams.md) - Workflow diagrams
- [GraphQL Contracts](../../social_integration/docs/graphql_contracts.md) - API contracts
- [Database Schema](../../social_integration/docs/database_schema.md) - Database design

## Key Features
- Community/subreddit management with customizable rules
- Threaded discussions with nesting (up to 10 levels)
- Voting system (upvote/downvote) with karma tracking
- Moderation tools: post removal, user bans, community settings
- Rich text and media support (images, videos, links) in posts
- Search functionality within communities and across platform
- Integration with dabloons system for rewarding content creators

## Technology Stack
- **Language**: Rust
- **Web Framework**: Axum
- **Database**: PostgreSQL with Redis caching
- **API**: GraphQL
- **Real-time**: WebSocket
- **Frontend**: Yew (web) and Tauri (desktop)

## Integration Points
- **Identity**: Uses `cpc_oauth2` for authentication
- **Media**: Integrates with media processing pipeline
- **Task Manager**: Tracks rewards via dabloons system
- **Yapper**: Cross-posting and unified feed
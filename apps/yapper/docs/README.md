# Yapper Microblog Documentation

## Overview
Yapper is a Twitter-style microblogging platform with character-limited posts, real-time feeds, and social engagement features. It follows screaming architecture principles organized by features.

## Documentation Files
- [Yapper Architecture](./yapper_architecture.md) - Detailed architecture documentation
- [Cross-App Integration](../../social_integration/docs/cross_app_integration.md) - Integration with Allat
- [Sequence Diagrams](../../social_integration/docs/sequence_diagrams.md) - Workflow diagrams
- [GraphQL Contracts](../../social_integration/docs/graphql_contracts.md) - API contracts
- [Database Schema](../../social_integration/docs/database_schema.md) - Database design

## Key Features
- Character-limited posts (280 characters)
- Real-time feeds with algorithmic and chronological sorting options
- Hashtag support for topic discovery
- Engagement metrics (likes, shares, views) per post
- Media attachments (images, short videos) in posts
- Follow users and see their posts in your feed
- Direct messaging capability (via Messenger integration)

## Technology Stack
- **Language**: Rust
- **Web Framework**: Axum
- **Database**: PostgreSQL with Sled caching
- **API**: GraphQL
- **Real-time**: WebSocket
- **Visualization**: Bevy engine for analytics
- **Frontend**: Yew (web) and Tauri (desktop)

## Integration Points
- **Identity**: Uses `cpc_oauth2` for authentication
- **Messenger**: Direct messaging capability
- **Media**: Integrates with media processing pipeline
- **Task Manager**: Integration with dabloons system for tipping and rewards
- **Allat**: Cross-posting and unified feed
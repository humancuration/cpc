# Website Builder Module

This module provides functionality for creating both full websites and link-in-bio sites within the CPC platform. It follows a hexagonal architecture pattern with clear separation of concerns.

## Features

- Create and manage full websites with multiple pages
- Create and manage link-in-bio sites with customizable links
- Template system for both site types
- Analytics tracking for site visits and link clicks
- P2P storage integration for published sites
- Mobile-first responsive design

## Architecture

The module follows a hexagonal architecture with the following layers:

### Domain Layer
- Core business models and value objects
- Custom error types

### Application Layer
- Site service for managing sites
- Template service for managing templates
- Analytics service for tracking usage

### Infrastructure Layer
- Database repository implementation
- P2P storage integration
- Media processing utilities

### Web Layer
- GraphQL API implementation
- REST API routes
- Module wiring for integration with the main backend

## Database Schema

The module uses the following tables:

- `sites` - Main site entities
- `pages` - Pages for full websites
- `link_items` - Links for link-in-bio sites
- `templates` - Site templates
- `site_analytics` - Analytics data

## GraphQL API

The module exposes the following GraphQL operations:

### Queries
- `site(id: ID!)` - Get site details
- `sitesByOwner(ownerId: ID!)` - List all sites for a cooperative member
- `templates` - List available templates
- `siteAnalytics(siteId: ID!, period: AnalyticsPeriod)` - Get analytics

### Mutations
- `createSite(input: CreateSiteInput!)` - Create a new site
- `updateSiteSettings(input: UpdateSiteSettingsInput!)` - Update site settings
- `updateSiteContent(input: UpdateSiteContentInput!)` - Update site content
- `publishSite(siteId: ID!)` - Publish a site
- `trackLinkClick(linkId: ID!)` - Track a link click

### Subscriptions
- `sitePublished(siteId: ID!)` - Site publishing updates
- `linkClicked(siteId: ID!)` - Link click events

## Integration

The module integrates with:

- `cpc-core` - For cooperative member models and authentication
- `cpc-net` - For p2panda integration
- `cpc-protos` - For gRPC definitions

## Dependencies

All dependencies use permissive licenses (MIT, Apache 2.0):

- serde, serde_json - Serialization
- uuid - UUID generation
- chrono - Date/time handling
- thiserror - Error handling
- sqlx - Database access
- tokio - Async runtime
- async-trait - Async traits
- async-graphql - GraphQL implementation
- axum - Web framework
- tracing - Logging
- ffmpeg-wasm - Media processing
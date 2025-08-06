# Website Builder Module

This is the backend component of the Website Builder. This module provides functionality for creating both full websites and link-in-bio sites within the CPC platform. It follows a hexagonal architecture pattern with clear separation of concerns.

The frontend UI components are implemented within this module using Yew for web and Tauri for desktop, following vertical slice architecture principles.

## Features

- Create and manage full websites with multiple pages
- Create and manage link-in-bio sites with customizable links
- Create and manage fundraising campaign sites with integration to the cooperative fundraising service
- Template system for all site types
- Analytics tracking for site visits and link clicks
- P2P storage integration for published sites
- Mobile-first responsive design

## Architecture

The module follows a vertical slice architecture with both backend and frontend components:

### Backend Layers

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
- gRPC client for fundraising service integration

### Frontend Layer
- UI components implemented in src/frontend directory
- Yew for web-based UI
- Tauri for desktop application packaging
- Bevy for visual editor components

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

The `sites` table has been extended with additional columns for fundraising campaign data.

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
- `createFundraisingCampaign(input: CreateSiteInput!)` - Create a fundraising campaign site

### Subscriptions
- `sitePublished(siteId: ID!)` - Site publishing updates
- `linkClicked(siteId: ID!)` - Link click events


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
- tonic - gRPC client implementation
- prost - Protocol buffer serialization
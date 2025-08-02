# Website Builder Module Architecture

This document explains the architecture of the website builder module and how its components interact.

## Overview

The website builder module follows a hexagonal architecture (also known as ports and adapters) with a clear separation of concerns between different layers:

1. **Domain Layer** - Core business logic and entities
2. **Application Layer** - Use cases and business logic orchestration
3. **Infrastructure Layer** - Technical implementations (database, external services)
4. **Web Layer** - Adapters for web interfaces (GraphQL, REST)

## Domain Layer

The domain layer contains the core business entities and value objects:

- `Site` - The main entity representing either a full website, link-in-bio site, or fundraising campaign site
- `SiteType` - An enum that distinguishes between full websites, link-in-bio sites, and fundraising campaign sites
- `Template` - Site templates that define the structure and appearance
- `AnalyticsReport` - Data structure for analytics information
- Value objects like `ColorHex`, `ValidUrl`, and `TemplateId` for type-safe domain values
- Custom error types in `WebsiteBuilderError`

This layer has no dependencies on external frameworks or libraries, making it easy to test and maintain.

## Application Layer

The application layer contains the services that implement the business logic:

- `SiteService` - Manages the lifecycle of sites (creation, updating, publishing)
- `TemplateService` - Manages templates (creation, application to sites)
- `AnalyticsService` - Handles analytics tracking and reporting

These services depend on abstractions (traits) defined in the domain layer, not on concrete implementations.

## Infrastructure Layer

The infrastructure layer contains the technical implementations:

- `SiteRepository` - Database implementation for storing and retrieving sites
- `P2pandaClient` - Client for interacting with the p2panda network
- `MediaProcessor` - Handles media processing tasks
- `FundraisingClient` - gRPC client for interacting with the cooperative fundraising service

These components implement the interfaces (traits) defined in the domain layer.

## Web Layer

The web layer contains adapters for web interfaces:

- `routes.rs` - REST API routes using Axum
- `graphql.rs` - GraphQL schema and resolvers using async-graphql
- `module.rs` - Module initialization and wiring
- `types.rs` - GraphQL input/output types

This layer depends on the application layer and adapts the domain concepts for web consumption.

## Data Flow

1. A request comes in through the web layer (GraphQL or REST)
2. The web layer translates the request into domain concepts and calls the application service
3. The application service implements the business logic, calling infrastructure components as needed
4. The infrastructure components interact with external systems (database, p2p network, etc.)
5. The result flows back through the layers to the web layer, which formats the response

## Dependency Direction

The dependency direction follows the Dependency Inversion Principle:

```
Web Layer -> Application Layer -> Domain Layer <- Infrastructure Layer
```

The domain layer is at the center and has no dependencies. The application layer depends on the domain layer's abstractions. The infrastructure layer implements those abstractions. The web layer depends on the application layer.

This architecture allows for:
- Easy testing of the domain logic
- Swapping out infrastructure implementations
- Independent development of different layers
- Clear separation of concerns
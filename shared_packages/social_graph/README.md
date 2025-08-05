# Social Graph Package

This package provides functionality for managing social relationships and interactions within the CPC ecosystem.

## Features

- User relationship management (friends, followers, etc.)
- Universal activity feed generation with multiple content types
- Extensible ContentProvider system for integrating new content sources
- Social graph querying
- Consent integration with consent_manager crate
- GraphQL API for social interactions
- Multiple repository implementations (in-memory, PostgreSQL)
- Comprehensive testing and benchmarking
- State migration for hot-swappable content providers
- Robust error handling during provider registration and updates
- Advanced dependency management with conflict resolution
- Guaranteed state preservation through rollback mechanisms

## Architecture

The package follows hexagonal architecture principles with clear separation of concerns:

- `domain`: Core business logic and models
- `application`: Service layer combining domain and infrastructure
- `infrastructure`: External service integrations and repository implementations
- `presentation`: GraphQL API layer

## Key Components

- **User**: Represents a user in the social graph
- **Relationship**: Represents connections between users
- **Activity**: Represents user activities and interactions
- **ContentProvider**: Trait for integrating new content sources into the universal feed
- **RelationshipRepository**: Trait for relationship storage implementations
- **ConsentService**: Trait for consent management
- **SocialService**: Application service combining repositories and consent management
- **GraphQL Schema**: Provides queries for social interactions
- **DependencyResolver**: Handles dependency validation and version checking for content providers
- **StateMigrator**: Manages state transfer during provider updates

## GraphQL API

The package provides a complete GraphQL API with the following queries:

- `getFriends(user_id)`: Get friends of a user
- `getActivityFeed(userId, after, limit, filters)`: Get universal activity feed for a user with filtering and pagination
- `getRecommendations(user_id)`: Get recommended users

See [GraphQL API Documentation](docs/graphql_api.md) for detailed schema information.

## Repository Implementations

### In-Memory Repository

For testing and development:
```rust
use social_graph::InMemoryRelationshipRepository;

let repository = InMemoryRelationshipRepository::new();
```

### PostgreSQL Repository

For production use:
```rust
use social_graph::PostgresRelationshipRepository;
use sqlx::PgPool;

let pool = PgPool::connect("postgresql://user:password@localhost/database").await?;
let repository = PostgresRelationshipRepository::new(pool);
```

## Consent Integration

The social_graph package integrates with the consent system to ensure that all social interactions respect user consent preferences. The ConsentService provides a clean interface for checking consent levels for social interactions.

```rust
use social_graph::{ConsentService, ConsentServiceImpl};

let consent_service = ConsentServiceImpl::new(repository);
let has_consent = consent_service.can_view_content(viewer_id, content_owner_id, visibility).await?;
```

## Usage Example

```rust
use social_graph::{
    User, Relationship, RelationshipType,
    InMemoryRelationshipRepository, RelationshipRepository,
    ConsentServiceImpl, SocialService
};
use std::sync::Arc;

// Create users
let user1 = User::new("alice".to_string(), "Alice Smith".to_string(), "alice@example.com".to_string());
let user2 = User::new("bob".to_string(), "Bob Johnson".to_string(), "bob@example.com".to_string());

// Create repository
let repository = Arc::new(InMemoryRelationshipRepository::new());

// Create consent service
let consent_service = Arc::new(ConsentServiceImpl::new(repository.clone()));

// Create social service
let social_service = SocialService::new(repository, consent_service, content_providers);

// Create friendship
let friendship = social_service.create_friendship(user1.id, user2.id).await?;
```

## Documentation

- [Usage Guide](docs/usage_guide.md): Comprehensive guide to using the package
- [ContentProvider Guide](docs/content_provider_guide.md): Guide to implementing and using ContentProviders
- [GraphQL API](docs/graphql_api.md): Detailed GraphQL schema documentation
- [Migration Scripts](migrations/): Database migration scripts
- [State Migration Guide](docs/state_migration_guide.md): Guide to state migration for content providers

## Examples

See the `examples/` directory for complete usage examples:

- `basic_usage.rs`: Basic package usage
- `full_example.rs`: Complete example showing all components
- `content_provider_example.rs`: Example demonstrating the ContentProvider system
- `state_migration_example.rs`: Example demonstrating state migration with error handling
- `dynamic_provider_example.rs`: Example demonstrating dynamic provider registration with dependency resolution
- `hot_swap_example.rs`: Example demonstrating hot-swapping of providers with state migration

To run an example:
```bash
cargo run --example basic_usage
```

## Testing

The package includes comprehensive tests:

```bash
# Run unit tests
cargo test

# Run benchmarks
cargo bench
```

## License

This package is part of the CPC software ecosystem and is licensed under the CPC license.
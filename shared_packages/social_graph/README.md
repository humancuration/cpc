# Social Graph Package

This package provides functionality for managing social relationships and interactions within the CPC ecosystem.

## Features

- User relationship management (friends, followers, etc.)
- Activity feed generation
- Social graph querying
- Consent integration with consent_manager crate
- GraphQL API for social interactions
- Multiple repository implementations (in-memory, PostgreSQL)
- Comprehensive testing and benchmarking

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
- **RelationshipRepository**: Trait for relationship storage implementations
- **ConsentAdapter**: Integrates with consent_manager crate for consent management
- **SocialService**: Application service combining repositories and consent management
- **GraphQL Schema**: Provides queries for social interactions

## GraphQL API

The package provides a complete GraphQL API with the following queries:

- `getFriends(user_id)`: Get friends of a user
- `getActivityFeed(user_id)`: Get activity feed for a user
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

The social_graph package integrates with the consent_manager crate to ensure that all social interactions respect user consent preferences. The ConsentAdapter provides a clean interface for checking and updating consent levels for social interactions.

```rust
use social_graph::ConsentAdapter;

let consent_adapter = ConsentAdapter::new(consent_service);
let has_consent = consent_adapter.check_consent(user_id, target_user_id).await?;
```

## Usage Example

```rust
use social_graph::{
    User, Relationship, RelationshipType,
    InMemoryRelationshipRepository, RelationshipRepository,
    ConsentAdapter, SocialService
};
use std::sync::Arc;

// Create users
let user1 = User::new("alice".to_string(), "Alice Smith".to_string(), "alice@example.com".to_string());
let user2 = User::new("bob".to_string(), "Bob Johnson".to_string(), "bob@example.com".to_string());

// Create repository
let repository = Arc::new(InMemoryRelationshipRepository::new());

// Create consent adapter
let consent_adapter = Arc::new(ConsentAdapter::new(consent_service));

// Create social service
let social_service = SocialService::new(repository, consent_adapter);

// Create friendship
let friendship = social_service.create_friendship(user1.id, user2.id).await?;
```

## Documentation

- [Usage Guide](docs/usage_guide.md): Comprehensive guide to using the package
- [GraphQL API](docs/graphql_api.md): Detailed GraphQL schema documentation
- [Migration Scripts](migrations/): Database migration scripts

## Examples

See the `examples/` directory for complete usage examples:

- `basic_usage.rs`: Basic package usage
- `full_example.rs`: Complete example showing all components

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
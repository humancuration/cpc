# Social Graph Usage Guide

This guide explains how to use the social_graph package in your CPC applications.

## Overview

The social_graph package provides functionality for managing social relationships and interactions within the CPC ecosystem. It includes:

- User relationship management (friends, followers, etc.)
- Activity feed generation
- Social graph querying
- Consent integration with the consent_manager crate
- GraphQL API for social interactions

## Installation

Add the following to your Cargo.toml:

```toml
[dependencies]
social_graph = { path = "../shared_packages/social_graph" }
```

## Basic Usage

### Creating Users

```rust
use social_graph::User;

let user = User::new(
    "alice".to_string(),
    "Alice Smith".to_string(),
    "alice@example.com".to_string()
);
```

### Creating Relationships

```rust
use social_graph::{Relationship, RelationshipType};
use uuid::Uuid;

let user1_id = Uuid::new_v4();
let user2_id = Uuid::new_v4();

let relationship = Relationship::new(
    user1_id,
    user2_id,
    RelationshipType::Friend
);
```

### Using Repositories

The package provides both in-memory and PostgreSQL repository implementations:

```rust
use social_graph::{InMemoryRelationshipRepository, RelationshipRepository};
use std::sync::Arc;

let repository = Arc::new(InMemoryRelationshipRepository::new());
let relationship = repository.create_relationship(relationship).await?;
```

### Using the Social Service

The SocialService combines repositories and consent management:

```rust
use social_graph::{SocialService, InMemoryRelationshipRepository, ConsentAdapter};
use std::sync::Arc;

let repository = Arc::new(InMemoryRelationshipRepository::new());
let consent_adapter = Arc::new(ConsentAdapter::new(/* consent_service */));
let social_service = SocialService::new(repository, consent_adapter);

// Create a friendship
let friendship = social_service.create_friendship(user1_id, user2_id).await?;
```

## Consent Integration

The social_graph package integrates with the consent_manager crate to ensure all social interactions respect user consent preferences:

```rust
use social_graph::ConsentAdapter;

let consent_adapter = ConsentAdapter::new(consent_service);
let has_consent = consent_adapter.check_consent(user_id, target_user_id).await?;
```

## GraphQL API

The package provides a GraphQL schema for social interactions:

```rust
use social_graph::{create_schema, SocialGraphSchema};

let schema = create_schema();
```

Available queries:
- `getFriends(user_id)`: Get friends of a user
- `getActivityFeed(userId, after, limit, filters)`: Get universal activity feed for a user with filtering and pagination
- `getRecommendations(user_id)`: Get recommended users

### Universal Feed

The universal feed aggregates content from across the CPC ecosystem. It supports:

- Multiple content types (SocialPost, Video, JobPosting, etc.)
- Content visibility levels (Public, FriendsOnly, etc.)
- Pagination with cursor-based navigation
- Flexible filtering by content type, package, and visibility

Example GraphQL query:

```graphql
query {
  getActivityFeed(
    userId: "123e4567-e89b-12d3-a456-426614174000"
    limit: 10
    filters: [
      { contentType: SOCIAL_POST }
      { visibility: PUBLIC }
    ]
  ) {
    id
    contentType
    package
    content
    timestamp
    visibility
  }
}
```

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

## Testing

The package includes comprehensive tests:

```bash
# Run unit tests
cargo test

# Run benchmarks
cargo bench
```

## Examples

See the `examples/` directory for complete usage examples:

- `basic_usage.rs`: Basic package usage
- `full_example.rs`: Complete example showing all components

To run an example:

```bash
cargo run --example basic_usage
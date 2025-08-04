# Social Graph Package - File Summary

This document provides a summary of all files in the social_graph package.

## Package Structure

```
social_graph/
├── Cargo.toml
├── README.md
├── SUMMARY.md
├── docs/
│   ├── usage_guide.md
│   └── graphql_api.md
├── migrations/
│   └── 001_create_social_tables.sql
├── examples/
│   ├── basic_usage.rs
│   └── full_example.rs
├── benches/
│   └── repository_benchmark.rs
├── tests/
│   ├── integration_test.rs
│   └── repository_integration_test.rs
└── src/
    ├── lib.rs
    ├── domain/
    │   ├── mod.rs
    │   └── model/
    │       ├── mod.rs
    │       ├── user.rs
    │       ├── relationship.rs
    │       └── activity.rs
    ├── application/
    │   ├── mod.rs
    │   └── social_service.rs
    ├── infrastructure/
    │   ├── mod.rs
    │   ├── consent_adapter/
    │   │   └── mod.rs
    │   ├── in_memory_repository.rs
    │   └── postgres_repository.rs
    ├── domain/
    │   ├── mod.rs
    │   ├── model/
    │   │   ├── mod.rs
    │   │   ├── user.rs
    │   │   ├── relationship.rs
    │   │   └── activity.rs
    │   └── repository/
    │       ├── mod.rs
    │       └── relationship_repo.rs
    ├── infrastructure/
    │   ├── mod.rs
    │   ├── consent_adapter/
    │   │   └── mod.rs
    │   ├── in_memory_repository.rs
    │   └── postgres_repository.rs
    ├── application/
    │   ├── mod.rs
    │   └── social_service.rs
    └── presentation/
        ├── mod.rs
        └── graphql/
            ├── mod.rs
            └── schema.rs
```

## Core Files

### Package Configuration
- `Cargo.toml`: Package dependencies and metadata
- `README.md`: Package overview and usage instructions
- `SUMMARY.md`: This file

### Documentation
- `docs/usage_guide.md`: Comprehensive usage guide
- `docs/graphql_api.md`: GraphQL API documentation
- `migrations/001_create_social_tables.sql`: Database migration script

### Examples
- `examples/basic_usage.rs`: Basic package usage example
- `examples/full_example.rs`: Complete example showing all components

### Benchmarks
- `benches/repository_benchmark.rs`: Performance benchmarks for repositories

### Tests
- `tests/integration_test.rs`: Basic integration tests
- `tests/repository_integration_test.rs`: Repository integration tests

## Source Code

### Library Root
- `src/lib.rs`: Main library file with module declarations and exports

### Domain Layer
The domain layer contains the core business logic and models.

- `src/domain/mod.rs`: Domain module declaration
- `src/domain/model/mod.rs`: Model module declaration
- `src/domain/model/user.rs`: User entity and related logic
- `src/domain/model/relationship.rs`: Relationship entity and related logic
- `src/domain/model/activity.rs`: Activity entity and related logic
- `src/domain/repository/mod.rs`: Repository module declaration
- `src/domain/repository/relationship_repo.rs`: Relationship repository trait

### Application Layer
The application layer contains service implementations that coordinate between domain and infrastructure.

- `src/application/mod.rs`: Application module declaration
- `src/application/social_service.rs`: Social service implementation

### Infrastructure Layer
The infrastructure layer contains implementations of external interfaces and repository implementations.

- `src/infrastructure/mod.rs`: Infrastructure module declaration
- `src/infrastructure/consent_adapter/mod.rs`: Consent adapter implementation
- `src/infrastructure/in_memory_repository.rs`: In-memory repository implementation
- `src/infrastructure/postgres_repository.rs`: PostgreSQL repository implementation

### Presentation Layer
The presentation layer contains the GraphQL API implementation.

- `src/presentation/mod.rs`: Presentation module declaration
- `src/presentation/graphql/mod.rs`: GraphQL module declaration
- `src/presentation/graphql/schema.rs`: GraphQL schema implementation

## Key Features Implemented

1. **Domain Models**:
   - User entity with creation and management
   - Relationship entity with different types (Friend, Follower, Blocked, Pending)
   - Activity entity with various activity types
   - Proper validation and business logic

2. **Repository Pattern**:
   - RelationshipRepository trait defining the interface
   - InMemoryRelationshipRepository for testing
   - PostgresRelationshipRepository for production

3. **Consent Integration**:
   - ConsentAdapter integrating with consent_manager crate
   - Consent checking and management

4. **Application Services**:
   - SocialService coordinating between repositories and consent management
   - Friendship creation with consent validation
   - Friend retrieval with consent checking

5. **GraphQL API**:
   - Complete GraphQL schema with queries
   - Type definitions for all domain entities
   - Proper mapping between domain and GraphQL types

6. **Testing**:
   - Unit tests for all domain models
   - Integration tests for repositories
   - Performance benchmarks

7. **Documentation**:
   - Comprehensive README
   - Usage guide
   - GraphQL API documentation
   - Database migration scripts

## Dependencies

The package depends on several key crates:
- `async-graphql`: GraphQL implementation
- `async-trait`: Async trait support
- `serde`: Serialization framework
- `uuid`: UUID generation and handling
- `chrono`: Date and time handling
- `tokio`: Async runtime
- `sqlx`: Database access
- `consent_manager`: Consent management integration

## Architecture

The package follows hexagonal architecture principles with clear separation of concerns:
- Domain layer contains pure business logic
- Application layer coordinates use cases
- Infrastructure layer handles external integrations
- Presentation layer exposes the API
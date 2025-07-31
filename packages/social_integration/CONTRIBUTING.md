# Contributing to Social Integration

Thank you for your interest in contributing to the CPC Social Integration project! This document provides guidelines and information to help you contribute effectively.

## Getting Started

1. Fork the repository
2. Create a new branch for your feature or bug fix
3. Make your changes
4. Write tests for your changes
5. Ensure all tests pass
6. Submit a pull request

## Testing Guidelines

### Database Integration Tests

We use `sqlx::test` for database integration tests. These tests automatically set up a temporary PostgreSQL database for testing.

#### Running Tests

```bash
cd packages/social_integration
cargo test
```

#### Writing Database Tests

1. Use the `#[sqlx::test]` attribute
2. Include the `pool: PgPool` parameter in your test function
3. Create a repository instance using the pool
4. Test both success and error cases
5. Clean up any test data if necessary

Example:
```rust
#[sqlx::test]
async fn test_save_and_find_by_id(pool: PgPool) -> sqlx::Result<()> {
    let repo = PostgresUnifiedPostRepository::new(pool);
    let author_id = Uuid::new_v4();
    let post = create_test_post(AppSource::Allat, author_id);
    let post_id = post.id;
    
    // Save the post
    repo.save(&post).await.unwrap();
    
    // Find the post by ID
    let found = repo.find_by_id(post_id).await.unwrap();
    assert!(found.is_some());
    
    Ok(())
}
```

### Unit Tests

For pure logic tests that don't require a database, write standard unit tests using Rust's built-in testing framework.

## Code Style Conventions

### Rust Formatting

We use `rustfmt` for code formatting. Please ensure your code is properly formatted before submitting a pull request:

```bash
cargo fmt
```

### Naming Conventions

1. Use `snake_case` for variables and functions
2. Use `PascalCase` for types and traits
3. Use `SCREAMING_SNAKE_CASE` for constants
4. Use descriptive names that clearly indicate purpose

### Error Handling

1. Use `Result<T, Box<dyn std::error::Error + Send + Sync>>` for repository methods
2. Propagate errors using the `?` operator
3. Provide meaningful error messages
4. Handle all possible error cases

### Documentation

1. Document all public APIs with rustdoc comments
2. Include examples for complex functions
3. Keep documentation up to date with code changes
4. Use clear, concise language

## Pull Request Process

### Before Submitting

1. Ensure your code follows the style guidelines
2. Run all tests and ensure they pass
3. Update documentation if necessary
4. Write a clear, descriptive commit message

### Pull Request Review Process

1. All pull requests must be reviewed by at least one maintainer
2. Automated tests must pass before merging
3. Address all review comments before merging
4. Squash commits when appropriate

### What to Include in Your Pull Request

1. A clear description of the changes
2. Motivation for the changes
3. Any breaking changes
4. Related issues or pull requests
5. Test results

## Database Setup for Development

### Prerequisites

1. PostgreSQL 12 or higher
2. Rust toolchain
3. SQLx CLI (optional, but recommended)

### Setting Up the Database

1. Create a development database:
   ```sql
   CREATE DATABASE cpc_social_dev;
   ```

2. Set the DATABASE_URL environment variable:
   ```bash
   export DATABASE_URL=postgresql://username:password@localhost/cpc_social_dev
   ```

3. Run the migrations:
   ```bash
   cd packages/social_integration
   sqlx migrate run
   ```

### Running Migrations

To add a new migration:

1. Create a new migration file:
   ```bash
   sqlx migrate add migration_name
   ```

2. Write your migration SQL in the generated file
3. Run the migration:
   ```bash
   sqlx migrate run
   ```

## Architecture Principles

### Hexagonal Architecture

We follow hexagonal architecture principles:

1. Domain layer contains business logic
2. Application layer contains use cases
3. Infrastructure layer contains implementation details
4. Dependencies point inward (domain doesn't depend on infrastructure)

### Repository Pattern

All data access should go through repository traits:

1. Define repository traits in the application layer
2. Implement repositories in the infrastructure layer
3. Use dependency injection to provide repository implementations

### Dependency Injection

Services should receive their dependencies through constructors:

```rust
impl SocialIntegrationService {
    pub fn new(
        post_repository: Box<dyn UnifiedPostRepository + Send + Sync>,
    ) -> Self {
        Self {
            post_repository,
            app_clients: HashMap::new(),
        }
    }
}
```

## Questions or Need Help?

If you have questions or need help with your contribution:

1. Check existing issues and pull requests
2. Open a new issue if needed
3. Join our developer community chat
4. Contact the maintainers directly

Thank you for contributing to CPC Social Integration!
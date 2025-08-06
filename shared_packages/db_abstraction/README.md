# Database Abstraction Package (`db_abstraction`)

This package provides a higher-level database abstraction using Diesel ORM. This package aims to provide type-safe, efficient, and consistent database interactions across all CPC applications.

## Features

- Database schema definitions using Diesel
- CRUD operations with type safety
- Query building utilities
- Migration support
- Support for both PostgreSQL and SQLite

## Usage

```rust
// Initialize database manager
let db_manager = DbManager::new(pool);

// Use repository
let user_repo = UserRepository::new(db_manager);
let user = user_repo.find_by_id(user_id).await?;

// Transaction management
let tx = db_manager.begin_transaction().await?;
// Perform multiple operations
tx.commit().await?;
```
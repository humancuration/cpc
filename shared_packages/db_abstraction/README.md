# Database Abstraction Package (`db_abstraction`)

This package provides a higher-level database abstraction using Diesel ORM. This package aims to provide type-safe, efficient, and consistent database interactions across all CPC applications.

## Features

- Database schema definitions using Diesel
- CRUD operations with type safety
- Query building utilities
- Migration support
- Support for both PostgreSQL and SQLite

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
db_abstraction = { path = "../shared_packages/db_abstraction" }
```

## Usage

### Initialize Database Manager

```rust
use db_abstraction::DbManager;
use db_pool::{PoolFactory, DatabaseConfig, PoolConfig};

// Create database configuration
let db_config = DatabaseConfig::from_env()?;
let pool_config = PoolConfig::default();

// Create connection pool
let pool = PoolFactory::create_postgres_pool(&db_config, &pool_config).await?;

// Initialize database manager
let db_manager = DbManager::new_postgres(pool);
```

### Define Models

```rust
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

// Example user model
#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// New user model for insert operations
#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
}
```

### Use Repositories

```rust
use db_abstraction::{DbManager, UserRepository};

// Create repository
let user_repo = UserRepository::new(db_manager);

// Create a new user
let new_user = NewUser {
    username: "john_doe".to_string(),
    email: "john@example.com".to_string(),
};

let user = user_repo.create(new_user).await?;

// Find user by ID
let user = user_repo.find_by_id(user_id).await?;

// Update user
let update_user = UpdateUser {
    username: Some("jane_doe".to_string()),
    email: None,
};

let updated_user = user_repo.update(user_id, update_user).await?;
```

### Transaction Management

```rust
use db_abstraction::DbManager;

// Begin transaction
let mut conn = db_manager.get_connection().await?;
let tx = Transaction::new(&mut conn)?;

// Perform multiple operations
// ... do work ...

// Commit transaction
tx.commit()?;
```

## Integration Examples

### Finance App Integration

```rust
// Define finance-specific models
#[derive(Queryable, Insertable)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    // fields
}

// Create repository
pub struct TransactionRepository {
    db_manager: DbManager,
}

impl TransactionRepository {
    pub async fn create_transaction(&self, transaction: NewTransaction) -> Result<Transaction, DbError> {
        // Implementation
    }
    
    pub async fn get_transaction_history(&self, user_id: Uuid) -> Result<Vec<Transaction>, DbError> {
        // Implementation
    }
}
```

### CRM App Integration

```rust
// Define CRM-specific models
#[derive(Queryable, Insertable)]
#[diesel(table_name = customers)]
pub struct Customer {
    // fields
}

// Create repository
pub struct CustomerRepository {
    db_manager: DbManager,
}

impl CustomerRepository {
    pub async fn create_customer(&self, customer: NewCustomer) -> Result<Customer, DbError> {
        // Implementation
    }
    
    pub async fn find_customer_by_email(&self, email: &str) -> Result<Option<Customer>, DbError> {
        // Implementation
    }
}
```

## Schema Migrations

The package includes migration support using Diesel migrations:

```rust
use db_abstraction::migrations::{run_migrations, MIGRATIONS};
use diesel::prelude::*;

// Run pending migrations
let mut conn = establish_connection();
run_migrations(&mut conn)?;
```

## Testing

The package includes both unit tests and integration tests. To run the tests:

```bash
cargo test
```

For integration tests with actual database instances, you'll need to have PostgreSQL and SQLite available.

## Performance Considerations

1. **Query Optimization**: Diesel's query builder generates efficient SQL
2. **Connection Pooling**: Integration with `db_pool` for efficient connection reuse
3. **Batch Operations**: Support for batch insertions and updates
4. **Indexing**: Proper database indexing for performance
5. **Caching**: Integration with caching layers where appropriate

## Security Considerations

1. **SQL Injection Prevention**: Diesel's query builder prevents SQL injection
2. **Data Validation**: Model-level validation before database operations
3. **Access Control**: Repository-level access control
4. **Audit Trails**: Optional audit trail functionality
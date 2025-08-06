# Database Abstraction Package (`db_abstraction`) Architecture

## Overview

The `db_abstraction` package provides a higher-level database abstraction using Diesel ORM. This package aims to provide type-safe, efficient, and consistent database interactions across all CPC applications.

## Architecture

### Core Components

1. **Schema Definitions**
   - Database schema using Diesel DSL
   - Table definitions with proper constraints

2. **Model Layer**
   - Domain models mapped to database tables
   - Serialization/deserialization utilities

3. **Repository Layer**
   - CRUD operations with type safety
   - Query building utilities

4. **Migration System**
   - Database schema versioning
   - Automated migration application

5. **Connection Management**
   - Integration with `db_pool` package
   - Transaction management

### Data Flow

```
App Domain Models
       ↓
Repository Layer
       ↓
Diesel ORM
       ↓
Database Connection (from db_pool)
       ↓
Database
```

## Implementation Details

### Schema Definitions

Schema definitions use Diesel's DSL to define tables with proper constraints:

```rust
// Example schema definition
diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
```

### Model Layer

Domain models are mapped to database tables with proper serialization:

```rust
// Example domain model
#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
```

### Repository Layer

Repositories provide type-safe CRUD operations:

```rust
pub struct UserRepository {
    pool: bb8::Pool<PostgresConnectionManager>,
}

impl UserRepository {
    pub async fn create(&self, user: NewUser) -> Result<User, DbError>;
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DbError>;
    pub async fn find_all(&self) -> Result<Vec<User>, DbError>;
    pub async fn update(&self, id: Uuid, user: UpdateUser) -> Result<Option<User>, DbError>;
    pub async fn delete(&self, id: Uuid) -> Result<bool, DbError>;
}
```

### Migration System

The migration system uses Diesel's migration framework:

```rust
// Example migration
pub fn up() -> Result<(), Box<dyn std::error::Error>> {
    use diesel::prelude::*;
    use diesel_migrations::embed_migrations;
    
    embed_migrations!();
    let connection = establish_connection();
    embedded_migrations::run(&connection)?;
    Ok(())
}
```

### Connection Management

Integration with the `db_pool` package for connection management:

```rust
pub struct DbManager {
    pool: bb8::Pool<PostgresConnectionManager>,
}

impl DbManager {
    pub async fn get_connection(&self) -> Result<PooledConnection<PostgresConnectionManager>, DbError>;
    pub async fn begin_transaction(&self) -> Result<Transaction<PostgresConnectionManager>, DbError>;
}
```

## API Design

### Main Interface

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

## Error Handling

### DbError
A unified error type for database operations:

```rust
#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),
    
    #[error("Connection error: {0}")]
    Connection(#[from] bb8::RunError<tokio_postgres::Error>),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Not found: {0}")]
    NotFound(String),
}
```

## Integration with Apps

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
    pub async fn create_transaction(&self, transaction: NewTransaction) -> Result<Transaction, DbError>;
    pub async fn get_transaction_history(&self, user_id: Uuid) -> Result<Vec<Transaction>, DbError>;
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
    pub async fn create_customer(&self, customer: NewCustomer) -> Result<Customer, DbError>;
    pub async fn find_customer_by_email(&self, email: &str) -> Result<Option<Customer>, DbError>;
}
```

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

## Testing Strategy

1. **Unit Tests**: Test individual models and repositories
2. **Integration Tests**: Test with actual database instances
3. **Migration Tests**: Test schema migrations
4. **Performance Tests**: Benchmark database operations
5. **Security Tests**: Test for SQL injection and other vulnerabilities

## Migration Strategy

1. **Versioning**: Semantic versioning for schema changes
2. **Backward Compatibility**: Maintain backward compatibility when possible
3. **Rollback Support**: Support for rolling back migrations
4. **Automated Deployment**: Automated migration application during deployment

## Deployment Considerations

1. **Configuration**: Externalize database configuration
2. **Monitoring**: Export database metrics for monitoring systems
3. **Logging**: Comprehensive logging for debugging
4. **Backup Strategy**: Integration with backup systems
5. **Disaster Recovery**: Recovery procedures for database failures
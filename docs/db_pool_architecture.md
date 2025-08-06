# Database Connection Pool Package (`db_pool`) Architecture

## Overview

The `db_pool` package provides a shared database connection pool using bb8 with support for both PostgreSQL and SQLite. This package aims to provide efficient, reliable, and consistent database connection management across all CPC applications.

## Architecture

### Core Components

1. **Connection Managers**
   - PostgreSQL Connection Manager
   - SQLite Connection Manager

2. **Configuration**
   - Database configuration struct
   - Connection pool configuration

3. **Pool Factory**
   - Factory for creating connection pools
   - Health check utilities

4. **Metrics**
   - Connection pool metrics collection
   - Performance monitoring

### Data Flow

```
App Configuration
       ↓
Connection Configuration
       ↓
Pool Factory
       ↓
Connection Pool (bb8)
       ↓
Database Connections
```

## Implementation Details

### Connection Managers

#### PostgreSQL Connection Manager
- Uses `tokio-postgres` for PostgreSQL connections
- Implements `bb8::ManageConnection` trait
- Handles connection lifecycle (creation, testing, recycling)

#### SQLite Connection Manager
- Uses `rusqlite` for SQLite connections
- Implements `bb8::ManageConnection` trait
- Handles connection lifecycle (creation, testing, recycling)

### Configuration

#### DatabaseConfig
```rust
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub ssl_mode: Option<String>,
}
```

#### PoolConfig
```rust
pub struct PoolConfig {
    pub min_connections: u32,
    pub max_connections: u32,
    pub connection_timeout: Duration,
    pub idle_timeout: Option<Duration>,
    pub max_lifetime: Option<Duration>,
}
```

### Pool Factory

The pool factory is responsible for creating connection pools based on the provided configuration.

```rust
pub struct PoolFactory;

impl PoolFactory {
    pub async fn create_postgres_pool(
        db_config: &DatabaseConfig,
        pool_config: &PoolConfig,
    ) -> Result<bb8::Pool<PostgresConnectionManager>, DatabaseError>;

    pub async fn create_sqlite_pool(
        path: &str,
        pool_config: &PoolConfig,
    ) -> Result<bb8::Pool<SqliteConnectionManager>, DatabaseError>;
}
```

### Health Checks

Health checks are implemented to monitor the status of database connections.

```rust
pub struct HealthChecker;

impl HealthChecker {
    pub async fn check_postgres_health(
        pool: &bb8::Pool<PostgresConnectionManager>,
    ) -> Result<HealthStatus, DatabaseError>;

    pub async fn check_sqlite_health(
        pool: &bb8::Pool<SqliteConnectionManager>,
    ) -> Result<HealthStatus, DatabaseError>;
}
```

### Metrics

Metrics collection provides insights into connection pool performance.

```rust
pub struct MetricsCollector;

impl MetricsCollector {
    pub fn record_connection_acquired(duration: Duration);
    pub fn record_connection_returned(duration: Duration);
    pub fn record_connection_created(duration: Duration);
    pub fn record_connection_dropped();
}
```

## API Design

### Main Interface

```rust
// Create a PostgreSQL connection pool
let pool = PoolFactory::create_postgres_pool(&db_config, &pool_config).await?;

// Create a SQLite connection pool
let pool = PoolFactory::create_sqlite_pool("database.db", &pool_config).await?;

// Get a connection from the pool
let conn = pool.get().await?;

// Check pool health
let health = HealthChecker::check_postgres_health(&pool).await?;
```

## Error Handling

### DatabaseError
A unified error type for database operations:

```rust
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Connection error: {0}")]
    Connection(#[from] bb8::RunError<tokio_postgres::Error>),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Health check failed: {0}")]
    HealthCheck(String),
}
```

## Integration with Apps

### Finance App Integration
```rust
// In finance app initialization
let db_config = DatabaseConfig::from_env()?;
let pool_config = PoolConfig::default();
let pool = PoolFactory::create_postgres_pool(&db_config, &pool_config).await?;

// Use pool in repositories
let transaction_repo = TransactionRepository::new(pool);
```

### CRM App Integration
```rust
// In CRM app initialization
let db_config = DatabaseConfig::from_env()?;
let pool_config = PoolConfig::default();
let pool = PoolFactory::create_postgres_pool(&db_config, &pool_config).await?;

// Use pool in repositories
let customer_repo = CustomerRepository::new(pool);
```

## Performance Considerations

1. **Connection Reuse**: bb8 efficiently manages connection reuse to minimize connection overhead
2. **Pool Sizing**: Configurable pool sizes allow optimization for different workloads
3. **Timeouts**: Configurable timeouts prevent resource exhaustion
4. **Health Checks**: Regular health checks ensure connection validity
5. **Metrics**: Performance metrics help identify bottlenecks

## Security Considerations

1. **Connection Encryption**: SSL/TLS support for PostgreSQL connections
2. **Credential Management**: Secure handling of database credentials
3. **Access Control**: Applications only get connections from the pool, not direct database access

## Testing Strategy

1. **Unit Tests**: Test individual components (connection managers, health checks)
2. **Integration Tests**: Test with actual database instances
3. **Performance Tests**: Benchmark connection pool performance
4. **Failure Tests**: Test error handling and recovery

## Deployment Considerations

1. **Configuration**: Externalize configuration for different environments
2. **Monitoring**: Export metrics for monitoring systems
3. **Logging**: Comprehensive logging for debugging
4. **Resource Limits**: Set appropriate resource limits to prevent exhaustion
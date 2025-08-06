# Database Connection Pool Package (`db_pool`)

This package provides a shared database connection pool using bb8 with support for both PostgreSQL and SQLite. This package aims to provide efficient, reliable, and consistent database connection management across all CPC applications.

## Features

- Connection pooling for PostgreSQL using `bb8` and `tokio-postgres`
- Connection pooling for SQLite using `bb8` and `rusqlite`
- Configuration management for connection parameters
- Health checks for database connections
- Metrics collection for connection pool usage

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
db_pool = { path = "../shared_packages/db_pool" }
```

## Usage

### Creating a PostgreSQL Connection Pool

```rust
use db_pool::{DatabaseConfig, PoolConfig, PoolFactory};

// Create database configuration
let db_config = DatabaseConfig::new(
    "localhost".to_string(),
    5432,
    "mydb".to_string(),
    "username".to_string(),
    "password".to_string(),
    Some("prefer".to_string()),
);

// Create pool configuration
let pool_config = PoolConfig::default();

// Create the connection pool
let pool = PoolFactory::create_postgres_pool(&db_config, &pool_config).await?;

// Get a connection from the pool
let conn = pool.get().await?;
```

### Creating a SQLite Connection Pool

```rust
use db_pool::{PoolConfig, PoolFactory};

// Create pool configuration
let pool_config = PoolConfig::default();

// Create the connection pool
let pool = PoolFactory::create_sqlite_pool("database.db", &pool_config).await?;

// Get a connection from the pool
let conn = pool.get().await?;
```

### Health Checks

```rust
use db_pool::HealthChecker;

// Check PostgreSQL pool health
let health = HealthChecker::check_postgres_health(&pool).await?;
match health {
    HealthStatus::Healthy => println!("Database is healthy"),
    HealthStatus::Degraded(msg) => println!("Database is degraded: {}", msg),
    HealthStatus::Unhealthy(msg) => println!("Database is unhealthy: {}", msg),
}
```

### Configuration

The package provides configuration structures for both database and pool settings:

```rust
use db_pool::{DatabaseConfig, PoolConfig};
use std::time::Duration;

// Database configuration from environment variables
let db_config = DatabaseConfig::from_env()?;

// Custom pool configuration
let pool_config = PoolConfig::new(
    5,   // min_connections
    20,  // max_connections
    Duration::from_secs(30),  // connection_timeout
    Some(Duration::from_secs(600)),  // idle_timeout
    Some(Duration::from_secs(1800)), // max_lifetime
);
```

## Integration Examples

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

## Testing

The package includes both unit tests and integration tests. To run the tests:

```bash
cargo test
```

For integration tests with actual database instances, you'll need to have PostgreSQL and SQLite available.

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
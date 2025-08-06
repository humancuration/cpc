# Database Connection Pool Package (`db_pool`)

This package provides a shared database connection pool using bb8 with support for both PostgreSQL and SQLite. This package aims to provide efficient, reliable, and consistent database connection management across all CPC applications.

## Features

- Connection pooling for PostgreSQL using `bb8` and `tokio-postgres`
- Connection pooling for SQLite using `bb8` and `rusqlite`
- Configuration management for connection parameters
- Health checks for database connections
- Metrics collection for connection pool usage

## Usage

```rust
// Create a PostgreSQL connection pool
let db_config = DatabaseConfig::from_env()?;
let pool_config = PoolConfig::default();
let pool = PoolFactory::create_postgres_pool(&db_config, &pool_config).await?;

// Create a SQLite connection pool
let pool = PoolFactory::create_sqlite_pool("database.db", &pool_config).await?;

// Get a connection from the pool
let conn = pool.get().await?;

// Check pool health
let health = HealthChecker::check_postgres_health(&pool).await?;
```
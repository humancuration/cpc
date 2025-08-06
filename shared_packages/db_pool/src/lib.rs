//! Database connection pooling utilities using bb8
//!
//! This module provides connection pooling for both PostgreSQL and SQLite databases
//! using the bb8 connection pool library.

pub mod config;
pub mod postgres;
pub mod sqlite;
pub mod health;
pub mod metrics;

pub use config::{DatabaseConfig, PoolConfig};
pub use postgres::PostgresConnectionManager;
pub use sqlite::SqliteConnectionManager;
pub use health::{HealthChecker, HealthStatus};
pub use metrics::MetricsCollector;

use bb8::Pool;
use thiserror::Error;
use std::time::Duration;

/// Main factory for creating database connection pools
pub struct PoolFactory;

impl PoolFactory {
    /// Create a PostgreSQL connection pool
    pub async fn create_postgres_pool(
        db_config: &DatabaseConfig,
        pool_config: &PoolConfig,
    ) -> Result<Pool<PostgresConnectionManager>, DatabaseError> {
        let manager = PostgresConnectionManager::new(db_config);
        let pool = Pool::builder()
            .max_size(pool_config.max_connections)
            .min_idle(Some(pool_config.min_connections))
            .connection_timeout(pool_config.connection_timeout)
            .idle_timeout(pool_config.idle_timeout)
            .max_lifetime(pool_config.max_lifetime)
            .build(manager)
            .await
            .map_err(DatabaseError::Connection)?;
        
        Ok(pool)
    }

    /// Create a SQLite connection pool
    pub async fn create_sqlite_pool(
        path: &str,
        pool_config: &PoolConfig,
    ) -> Result<Pool<SqliteConnectionManager>, DatabaseError> {
        let manager = SqliteConnectionManager::new(path);
        let pool = Pool::builder()
            .max_size(pool_config.max_connections)
            .min_idle(Some(pool_config.min_connections))
            .connection_timeout(pool_config.connection_timeout)
            .idle_timeout(pool_config.idle_timeout)
            .max_lifetime(pool_config.max_lifetime)
            .build(manager)
            .await
            .map_err(DatabaseError::Connection)?;
        
        Ok(pool)
    }
}

/// Unified error type for database operations
#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Connection error: {0}")]
    Connection(#[from] bb8::RunError<tokio_postgres::Error>),
    
    #[error("SQLite connection error: {0}")]
    SqliteConnection(#[from] bb8::RunError<rusqlite::Error>),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Health check failed: {0}")]
    HealthCheck(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_database_config_from_env() {
        // This test would require setting environment variables
        // In a real test, we would set the env vars and test the function
        let config = DatabaseConfig::new(
            "localhost".to_string(),
            5432,
            "testdb".to_string(),
            "testuser".to_string(),
            "testpass".to_string(),
            Some("prefer".to_string()),
        );
        
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 5432);
        assert_eq!(config.database, "testdb");
        assert_eq!(config.username, "testuser");
        assert_eq!(config.password, "testpass");
        assert_eq!(config.ssl_mode, Some("prefer".to_string()));
    }

    #[test]
    fn test_pool_config_default() {
        let config = PoolConfig::default();
        assert_eq!(config.min_connections, 5);
        assert_eq!(config.max_connections, 20);
        assert_eq!(config.connection_timeout, Duration::from_secs(30));
    }
}
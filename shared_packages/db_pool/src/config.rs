//! Database configuration structures

use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::env;

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub ssl_mode: Option<String>,
}

impl DatabaseConfig {
    /// Create a new database configuration
    pub fn new(
        host: String,
        port: u16,
        database: String,
        username: String,
        password: String,
        ssl_mode: Option<String>,
    ) -> Self {
        Self {
            host,
            port,
            database,
            username,
            password,
            ssl_mode,
        }
    }

    /// Create a database configuration from environment variables
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            host: env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string()).parse()?,
            database: env::var("DB_NAME").unwrap_or_else(|_| "cpc".to_string()),
            username: env::var("DB_USER").unwrap_or_else(|_| "cpc".to_string()),
            password: env::var("DB_PASSWORD").unwrap_or_else(|_| "cpc".to_string()),
            ssl_mode: env::var("DB_SSL_MODE").ok(),
        })
    }

    /// Get the PostgreSQL connection string
    pub fn postgres_url(&self) -> String {
        let ssl_mode = self.ssl_mode.as_deref().unwrap_or("prefer");
        format!(
            "postgresql://{}:{}@{}:{}/{}?sslmode={}",
            self.username, self.password, self.host, self.port, self.database, ssl_mode
        )
    }
}

/// Pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub min_connections: u32,
    pub max_connections: u32,
    pub connection_timeout: Duration,
    pub idle_timeout: Option<Duration>,
    pub max_lifetime: Option<Duration>,
}

impl PoolConfig {
    /// Create a new pool configuration
    pub fn new(
        min_connections: u32,
        max_connections: u32,
        connection_timeout: Duration,
        idle_timeout: Option<Duration>,
        max_lifetime: Option<Duration>,
    ) -> Self {
        Self {
            min_connections,
            max_connections,
            connection_timeout,
            idle_timeout,
            max_lifetime,
        }
    }
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            min_connections: 5,
            max_connections: 20,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Some(Duration::from_secs(600)),
            max_lifetime: Some(Duration::from_secs(1800)),
        }
    }
}
//! Health checking utilities for database connections

use crate::{PostgresConnectionManager, SqliteConnectionManager, DatabaseError};
use bb8::Pool;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Health status enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded(String),
    Unhealthy(String),
}

/// Health checker for database pools
pub struct HealthChecker;

impl HealthChecker {
    /// Check the health of a PostgreSQL connection pool
    pub async fn check_postgres_health(
        pool: &Pool<PostgresConnectionManager>,
    ) -> Result<HealthStatus, DatabaseError> {
        let start = Instant::now();
        
        match pool.get().await {
            Ok(conn) => {
                // Execute a simple query to test the connection
                if let Err(e) = conn.simple_query("SELECT 1").await {
                    return Ok(HealthStatus::Unhealthy(format!("Query failed: {}", e)));
                }
                
                let duration = start.elapsed();
                if duration.as_millis() > 1000 {
                    Ok(HealthStatus::Degraded(format!("Slow response: {}ms", duration.as_millis())))
                } else {
                    Ok(HealthStatus::Healthy)
                }
            }
            Err(e) => Ok(HealthStatus::Unhealthy(format!("Connection failed: {}", e))),
        }
    }

    /// Check the health of a SQLite connection pool
    pub async fn check_sqlite_health(
        pool: &Pool<SqliteConnectionManager>,
    ) -> Result<HealthStatus, DatabaseError> {
        let start = Instant::now();
        
        match pool.get().await {
            Ok(conn) => {
                // Execute a simple query to test the connection
                if let Err(e) = conn.execute_batch("SELECT 1") {
                    return Ok(HealthStatus::Unhealthy(format!("Query failed: {}", e)));
                }
                
                let duration = start.elapsed();
                if duration.as_millis() > 1000 {
                    Ok(HealthStatus::Degraded(format!("Slow response: {}ms", duration.as_millis())))
                } else {
                    Ok(HealthStatus::Healthy)
                }
            }
            Err(e) => Ok(HealthStatus::Unhealthy(format!("Connection failed: {}", e))),
        }
    }
}
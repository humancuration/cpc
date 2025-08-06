//! Example of using the db_pool package with PostgreSQL

use db_pool::{DatabaseConfig, PoolConfig, PoolFactory, HealthChecker, HealthStatus};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create database configuration
    let db_config = DatabaseConfig::new(
        env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
        env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string()).parse()?,
        env::var("DB_NAME").unwrap_or_else(|_| "testdb".to_string()),
        env::var("DB_USER").unwrap_or_else(|_| "testuser".to_string()),
        env::var("DB_PASSWORD").unwrap_or_else(|_| "testpass".to_string()),
        Some("prefer".to_string()),
    );

    // Create pool configuration
    let pool_config = PoolConfig::default();

    // Create the connection pool
    println!("Creating PostgreSQL connection pool...");
    let pool = PoolFactory::create_postgres_pool(&db_config, &pool_config).await?;

    // Check pool health
    println!("Checking pool health...");
    let health = HealthChecker::check_postgres_health(&pool).await?;
    match health {
        HealthStatus::Healthy => println!("Database is healthy"),
        HealthStatus::Degraded(msg) => println!("Database is degraded: {}", msg),
        HealthStatus::Unhealthy(msg) => println!("Database is unhealthy: {}", msg),
    }

    // Get a connection from the pool
    println!("Getting connection from pool...");
    let conn = pool.get().await?;
    println!("Successfully got connection from pool");

    // Execute a simple query
    println!("Executing simple query...");
    let result = conn.simple_query("SELECT version()").await?;
    if let Some(row) = result.get(0) {
        if let Some(version) = row.get("version") {
            println!("PostgreSQL version: {}", version);
        }
    }

    println!("Example completed successfully!");
    Ok(())
}
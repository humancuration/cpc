use skill_development::GrpcServer;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Get database URL from environment
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://skill_dev_user:secure_password@localhost/skill_dev_db".to_string());

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Get server address from environment or use default
    let address = env::var("SERVER_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:50051".to_string());

    // Create and start gRPC server
    let server = GrpcServer::new(pool, address);
    
    println!("Starting Skill Development gRPC server on {}", server.address);
    server.start().await?;

    Ok(())
}
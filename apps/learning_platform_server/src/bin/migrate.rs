use sqlx::PgPool;
use learning_platform_server::config::AppConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = AppConfig::from_env()?;
    
    // Create database connection pool
    println!("Connecting to database: {}", config.database_url);
    let pool = PgPool::connect(&config.database_url).await?;
    
    // Run migrations
    println!("Running database migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    println!("Migrations completed successfully!");
    
    Ok(())
}
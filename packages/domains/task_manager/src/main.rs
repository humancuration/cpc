use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing::{info, Level};

use task_manager::initialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // Load configuration
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("SERVER_PORT must be a valid port number");

    // Create database pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // Initialize services (using mock implementations for now)
    let notification_service = Arc::new(task_manager::infrastructure::notification::MockNotificationService);
    let p2p_sync_service = Arc::new(task_manager::infrastructure::p2p_sync::MockP2pSyncService);

    // Initialize task manager module
    let task_manager = initialize(pool, notification_service, p2p_sync_service)
        .await
        .expect("Failed to initialize task manager");

    info!("Starting server at {}:{}", server_host, server_port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .configure(|cfg| {
                task_manager::configure_actix_routes(cfg, task_manager.service.clone());
            })
    })
    .bind((server_host.as_str(), server_port))?
    .run()
    .await
}
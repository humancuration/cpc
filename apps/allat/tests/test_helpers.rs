use testcontainers::{clients, images::postgres::Postgres};
use sqlx::PgPool;
use std::sync::Once;

static INIT: Once = Once::new();

pub async fn setup_test_db() -> PgPool {
    INIT.call_once(|| {
        // Initialize tracing or other setup if needed
    });

    let docker = clients::Cli::default();
    let postgres = docker.run(Postgres::default());
    let port = postgres.get_host_port_ipv4(5432);
    
    let database_url = format!("postgres://postgres:postgres@localhost:{}/postgres", port);
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");
    
    // Run migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");
        
    pool
}
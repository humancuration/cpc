use tonic::transport::Server;
use std::net::SocketAddr;
use tracing_subscriber::fmt::format::FmtSpan;
use sqlx::postgres::PgPoolOptions;
use valkey::ValkeyPool;
use opensearch::OpenSearch;
use crate::{node_registry::NodeRegistryService, identity::IdentityService, discovery::DiscoveryService, metrics::MetricsService, middleware::metrics::MetricsMiddleware, secret_manager::{SecretManager, SecretStorage}};
use crate::cpc_orchestrator::node_orchestration_server::NodeOrchestrationServer;
use crate::cpc_orchestrator::identity_service_server::IdentityServiceServer;
use crate::cpc_orchestrator::discovery_service_server::DiscoveryServiceServer;

mod node_registry;
mod identity;
mod discovery;
mod metrics;
mod secret_manager;
mod cpc_orchestrator {
    tonic::include_proto!("cpc.orchestrator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // Create database connection pool
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@localhost:5432/cpc".to_string());
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Create Valkey connection pool
    let valkey_url = std::env::var("VALKEY_URL")
        .unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let valkey_pool = ValkeyPool::new(valkey_url, 5)?;

    // Create OpenSearch client
    let search_client = OpenSearch::new(
        opensearch::http::transport::SingleNodeConnectionPool::new(
            std::env::var("OPENSEARCH_URL")
                .unwrap_or_else(|_| "http://localhost:9200".to_string())
                .parse()?
        )
    );

    // Initialize services
    let node_registry = NodeRegistryService::new(db_pool.clone());
    // Create secret manager
    let secret_manager = Arc::new(SecretManager::new(
        if std::env::var("PRODUCTION").is_ok() {
            SecretStorage::Valkey(valkey_pool.clone())
        } else {
            SecretStorage::LocalEnv
        }
    ).expect("Failed to create secret manager"));

    let identity = IdentityService::new(
        db_pool.clone(),
        secret_manager.clone()
    );
    let discovery = DiscoveryService::new(db_pool.clone(), search_client.clone());
    let metrics = MetricsService::new();

    // Start metrics server
    metrics.start_metrics_server("0.0.0.0:9090".parse()?);

    // Create gRPC server
    let addr: SocketAddr = "[::]:50051".parse()?;
    tracing::info!("gRPC server listening on {}", addr);

    // Wrap node registry with metrics middleware
    let wrapped_registry = MetricsMiddleware::new(node_registry, Arc::new(metrics.clone()));
    
    // Create secret service
    let secret_service = SecretServiceImpl::new(secret_manager.clone());

    Server::builder()
        .add_service(NodeOrchestrationServer::new(wrapped_registry))
        .add_service(IdentityServiceServer::new(identity))
        .add_service(DiscoveryServiceServer::new(discovery))
        .add_service(SecretServiceServer::new(secret_service))
        .serve(addr)
        .await?;

    Ok(())
}
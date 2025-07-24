//! Cooperative Node for Cooperative Peer Cloud - Job Processing Worker

use cpc_lib::{storage::LruStorage, net::NetworkBuilder};
use cpc_protos::job_service::job_service_client::JobServiceClient;
use std::error::Error;
use tokio::sync::mpsc;
use cpc_core::business::financial_forecasting::{FinancialForecast, ForecastParameters};
use uuid::Uuid;
use sqlx::postgres::PgPoolOptions;
use cpc_protos::job_service::{JobSubscriptionRequest, JobType};

mod job_worker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Cooperative Node starting...");

    // Initialize storage backend
    let mut storage = LruStorage::new(1024 * 1024 * 1024 * 10); // 10GB limit

    // Join P2P network
    let mut network = NetworkBuilder::new()
        .with_tcp()
        .with_quic()
        .enable_kademlia()
        .enable_bitswap()
        .build();

    // Get node ID
    let node_id = network.local_peer_id().to_string();
    println!("Starting job processing service for node: {}", node_id);

    // Initialize database connection pool
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/cpc".to_string());
    
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Connected to database");

    // Create gRPC client to connect to backend
    let backend_url = std::env::var("BACKEND_URL")
        .unwrap_or_else(|_| "http://localhost:50051".to_string());
    
    let mut client = JobServiceClient::connect(backend_url).await?;
    println!("Connected to backend at {}", backend_url);

    // Create job worker with database pool
    let job_worker = job_worker::JobWorker::new(
        node_id.clone(),
        db_pool.clone()
    );

    // Subscribe to jobs from backend
    let request = JobSubscriptionRequest {
        worker_id: node_id.clone(),
        capabilities: vec![
            "financial_forecasting".to_string(),
            "compute".to_string()
        ],
        available_resources: None,
    };

    println!("Subscribing to jobs from backend...");
    let mut job_stream = client
        .subscribe_to_jobs(request)
        .await?
        .into_inner();

    // Process jobs as they arrive
    while let Some(job) = job_stream.message().await? {
        println!("Received job: {} of type: {:?}", job.job_id, job.job_type());
        
        // Clone client for async task
        let mut client_clone = client.clone();
        
        // Process job in separate task
        let job_worker_clone = job_worker.clone();
        tokio::spawn(async move {
            if let Err(e) = job_worker_clone.process_job(job, &mut client_clone).await {
                eprintln!("Error processing job: {}", e);
            }
        });
    }

    println!("Job stream ended");
    Ok(())
}
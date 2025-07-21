//! Central Orchestrator for Cooperative Peer Cloud

use cpc_lib::{net::NetworkBuilder, storage::StorageMetrics};
use warp::Filter;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    println!("CPC Orchestrator starting...");

    // Initialize network components
    let mut network = NetworkBuilder::new()
        .with_tcp()
        .with_quic()
        .build();

    // Start metrics server (OpenMetrics format) - TODO: Implement actual metrics server
    println!("Metrics server would start at 0.0.0.0:8080");

    // Start REST API server using warp
    let health_route = warp::path!("health").map(|| "OK");
    let api_addr: SocketAddr = "127.0.0.1:3030".parse().unwrap();
    
    println!("Starting REST API server at {}", api_addr);
    warp::serve(health_route)
        .run(api_addr)
        .await;

    // TODO: Start gRPC server for internal communication
    // Placeholder for future implementation
}
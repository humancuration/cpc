//! Central Orchestrator for Cooperative Peer Cloud

use cpc_lib::{net::NetworkBuilder, storage::StorageMetrics, grpc::server};
use warp::Filter;
use std::net::SocketAddr;
use tonic::transport::Server;

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
    
    // Start gRPC server for internal communication
    let grpc_addr = "0.0.0.0:50051".parse().unwrap();
    println!("Starting gRPC server at {}", grpc_addr);
    let node_orchestration = server::server();
    
    let grpc_server = Server::builder()
        .add_service(node_orchestration)
        .serve(grpc_addr);
    
    // Run both servers concurrently
    tokio::select! {
        _ = warp::serve(health_route).run(api_addr) => {},
        _ = grpc_server => {}
    }
}
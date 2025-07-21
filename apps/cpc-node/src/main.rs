//! Cooperative Node for Cooperative Peer Cloud

use cpc_lib::{storage::LruStorage, net::NetworkBuilder, grpc::client::OrchestratorClient, grpc::internal::{NodeRegistrationRequest, Resources}};
use std::error::Error;
use std::time::Duration;

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

    // Create gRPC client and register with orchestrator
    println!("Connecting to orchestrator...");
    let mut client = OrchestratorClient::connect("http://orchestrator:50051".to_string())
        .await
        .expect("Failed to connect to orchestrator");

    println!("Registering node with orchestrator...");
    let response = client.register_node(NodeRegistrationRequest {
        node_id: network.local_peer_id().to_string(),
        resources: Some(Resources {
            memory: 8192, // 8GB
            storage: 100, // 100GB
            cores: 4,
            bandwidth: 100, // 100 Mbps
        }),
        location: "us-west".to_string(),
        capabilities: vec!["storage".to_string(), "compute".to_string()],
    }).await?;

    println!("Registration response: {}", response.message);

    // Keep the node running
    loop {
        // Handle network events
        // Placeholder for actual event handling
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
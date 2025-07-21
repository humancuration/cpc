//! Personal Data Server for Cooperative Peer Cloud

use cpc_lib::{storage::LruStorage, net::NetworkBuilder, crypto::KeyPair};
use tauri::Builder;

fn main() {
    println!("Personal Data Server starting...");

    // Initialize local storage
    let mut storage = LruStorage::new(1024 * 1024 * 1024); // 1GB limit

    // Generate keys for encryption
    let keys = KeyPair::generate_x25519();

    // Connect to P2P network
    let mut network = NetworkBuilder::new()
        .with_websocket()
        .build();

    // Initialize Tauri application with state
    Builder::default()
        .manage(storage)
        .manage(network)
        .run(tauri::generate_context!())
        .expect("error running tauri application");
}
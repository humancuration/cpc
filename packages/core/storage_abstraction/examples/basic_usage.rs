//! Basic usage example for the storage abstraction layer

use storage_abstraction::{
    DataStore,
    StorageManager,
    StorageConfig,
    SledStore,
    InMemoryStore,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create storage backends
    let edge_store = Arc::new(SledStore::new_default()?);
    let cloud_store = Arc::new(InMemoryStore::new());
    
    // Create storage manager
    let config = StorageConfig {
        router: storage_abstraction::domain::routing::RoutingConfig::default(),
    };
    
    let manager = StorageManager::new(edge_store, cloud_store, config);
    
    // Store some data
    let key = "user:123:profile";
    let value = serde_json::to_vec(&serde_json::json!({
        "name": "John Doe",
        "email": "john@example.com",
        "preferences": {
            "theme": "dark",
            "notifications": true
        }
    }))?;
    
    manager.set(key, value).await?;
    
    // Retrieve the data
    let retrieved = manager.get(key).await?;
    if let Some(data) = retrieved {
        let profile: serde_json::Value = serde_json::from_slice(&data)?;
        println!("Retrieved profile: {:?}", profile);
    }
    
    // Delete the data
    manager.delete(key).await?;
    
    println!("Storage abstraction example completed successfully!");
    Ok(())
}
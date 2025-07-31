//! Integration tests for the storage abstraction layer

use storage_abstraction::{
    DataStore,
    StorageManager,
    StorageConfig,
    SledStore,
    InMemoryStore,
};
use std::sync::Arc;

#[tokio::test]
async fn test_storage_manager_with_in_memory_stores() {
    // Create in-memory stores for testing
    let edge_store = Arc::new(InMemoryStore::new());
    let cloud_store = Arc::new(InMemoryStore::new());
    
    // Create storage manager
    let config = StorageConfig {
        router: storage_abstraction::domain::routing::RoutingConfig::default(),
    };
    
    let manager = StorageManager::new(edge_store, cloud_store, config);
    
    // Test set and get
    let key = "test_key";
    let value = b"test_value".to_vec();
    
    manager.set(key, value.clone()).await.unwrap();
    
    let retrieved = manager.get(key).await.unwrap();
    assert_eq!(retrieved, Some(value));
    
    // Test delete
    manager.delete(key).await.unwrap();
    
    let retrieved = manager.get(key).await.unwrap();
    assert_eq!(retrieved, None);
}

#[tokio::test]
async fn test_storage_manager_with_sled_store() {
    // Create Sled store for edge storage
    let edge_store = Arc::new(SledStore::new_default().unwrap());
    let cloud_store = Arc::new(InMemoryStore::new());
    
    // Create storage manager
    let config = StorageConfig {
        router: storage_abstraction::domain::routing::RoutingConfig::default(),
    };
    
    let manager = StorageManager::new(edge_store, cloud_store, config);
    
    // Test set and get
    let key = "sled_test_key";
    let value = b"sled_test_value".to_vec();
    
    manager.set(key, value.clone()).await.unwrap();
    
    let retrieved = manager.get(key).await.unwrap();
    assert_eq!(retrieved, Some(value));
}
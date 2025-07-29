//! Health module integration example for the storage abstraction layer

use storage_abstraction::{
    DataStore,
    StorageManager,
    StorageConfig,
    SledStore,
    InMemoryStore,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Health data record
#[derive(Debug, Clone, Serialize, Deserialize)]
struct HealthRecord {
    id: String,
    user_id: String,
    timestamp: DateTime<Utc>,
    data_type: String,
    value: serde_json::Value,
    tags: Vec<String>,
}

/// Health data service that uses the storage abstraction
struct HealthDataService {
    storage: StorageManager,
}

impl HealthDataService {
    /// Create a new health data service
    pub fn new(storage: StorageManager) -> Self {
        Self { storage }
    }
    
    /// Store wearable data with appropriate routing
    pub async fn store_wearable_data(&self, record: HealthRecord) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("wearable:{}:{}", record.user_id, record.timestamp.timestamp());
        let value = serde_json::to_vec(&record)?;
        
        self.storage.set(&key, value).await?;
        Ok(())
    }
    
    /// Retrieve historical data for a user
    pub async fn get_history(
        &self,
        user_id: &str,
        data_type: &str,
        limit: usize,
    ) -> Result<Vec<HealthRecord>, Box<dyn std::error::Error>> {
        // In a real implementation, we would use a more sophisticated query mechanism
        // For this example, we'll just return some dummy data
        
        let mut records = Vec::new();
        
        for i in 0..limit {
            let key = format!("wearable:{}:{}", user_id, chrono::Utc::now().timestamp() - i as i64);
            
            if let Some(data) = self.storage.get(&key).await? {
                if let Ok(record) = serde_json::from_slice::<HealthRecord>(&data) {
                    if record.data_type == data_type {
                        records.push(record);
                    }
                }
            }
        }
        
        Ok(records)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create storage backends
    let edge_store = Arc::new(SledStore::new_default()?);
    let cloud_store = Arc::new(InMemoryStore::new());
    
    // Create storage manager with health-specific routing
    let mut router_config = storage_abstraction::domain::routing::RoutingConfig::default();
    router_config.edge_patterns.push("wearable:*".to_string());
    router_config.cloud_patterns.push("health:*".to_string());
    
    let config = StorageConfig {
        router: router_config,
    };
    
    let manager = StorageManager::new(edge_store, cloud_store, config);
    
    // Create health data service
    let health_service = HealthDataService::new(manager);
    
    // Store some wearable data
    let record = HealthRecord {
        id: "123".to_string(),
        user_id: "user_123".to_string(),
        timestamp: chrono::Utc::now(),
        data_type: "heart_rate".to_string(),
        value: serde_json::json!({
            "bpm": 72,
            "confidence": 0.95
        }),
        tags: vec!["fitness".to_string(), "cardio".to_string()],
    };
    
    health_service.store_wearable_data(record).await?;
    
    // Retrieve historical data
    let history = health_service.get_history("user_123", "heart_rate", 10).await?;
    println!("Retrieved {} health records", history.len());
    
    println!("Health integration example completed successfully!");
    Ok(())
}
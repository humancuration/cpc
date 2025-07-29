//! Finance module integration example for the storage abstraction layer

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
use rust_decimal::Decimal;
use uuid::Uuid;

/// Financial transaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transaction {
    id: Uuid,
    user_id: String,
    timestamp: DateTime<Utc>,
    amount: Decimal,
    currency: String,
    description: String,
    category: String,
    tags: Vec<String>,
}

/// Finance data service that uses the storage abstraction
struct FinanceDataService {
    storage: StorageManager,
}

impl FinanceDataService {
    /// Create a new finance data service
    pub fn new(storage: StorageManager) -> Self {
        Self { storage }
    }
    
    /// Store transaction data with appropriate routing
    pub async fn store_transaction(&self, transaction: Transaction) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("finance:transaction:{}:{}", transaction.user_id, transaction.id);
        let value = serde_json::to_vec(&transaction)?;
        
        self.storage.set(&key, value).await?;
        Ok(())
    }
    
    /// Retrieve transactions for a user
    pub async fn get_transactions(
        &self,
        user_id: &str,
        limit: usize,
    ) -> Result<Vec<Transaction>, Box<dyn std::error::Error>> {
        // In a real implementation, we would use a more sophisticated query mechanism
        // For this example, we'll just return some dummy data
        
        let mut transactions = Vec::new();
        
        for i in 0..limit {
            let key = format!("finance:transaction:{}:{}", user_id, Uuid::new_v4());
            
            if let Some(data) = self.storage.get(&key).await? {
                if let Ok(transaction) = serde_json::from_slice::<Transaction>(&data) {
                    transactions.push(transaction);
                }
            }
        }
        
        Ok(transactions)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create storage backends
    let edge_store = Arc::new(SledStore::new_default()?);
    let cloud_store = Arc::new(InMemoryStore::new());
    
    // Create storage manager with finance-specific routing
    let mut router_config = storage_abstraction::domain::routing::RoutingConfig::default();
    router_config.cloud_patterns.push("finance:*".to_string());
    router_config.cloud_patterns.push("transaction:*".to_string());
    
    let config = StorageConfig {
        router: router_config,
    };
    
    let manager = StorageManager::new(edge_store, cloud_store, config);
    
    // Create finance data service
    let finance_service = FinanceDataService::new(manager);
    
    // Store some transaction data
    let transaction = Transaction {
        id: Uuid::new_v4(),
        user_id: "user_123".to_string(),
        timestamp: chrono::Utc::now(),
        amount: Decimal::new(12345, 2), // $123.45
        currency: "USD".to_string(),
        description: "Grocery shopping".to_string(),
        category: "Food".to_string(),
        tags: vec!["essential".to_string(), "weekly".to_string()],
    };
    
    finance_service.store_transaction(transaction).await?;
    
    // Retrieve transactions
    let transactions = finance_service.get_transactions("user_123", 10).await?;
    println!("Retrieved {} transactions", transactions.len());
    
    println!("Finance integration example completed successfully!");
    Ok(())
}
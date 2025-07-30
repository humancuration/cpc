//! Example showing how to use the sync infrastructure
//!
//! This example demonstrates the complete flow of using the sync infrastructure
//! with network monitoring, composite adapters, and the sync queue.

use packages::infra::core::network::monitor::NetworkStatusMonitor;
use packages::infra::core::adapters::composite::CompositeUserPreferences;
use packages::infra::core::factories::user_preferences_factory::UserPreferencesFactory;
use packages::infra::sync::queue::SyncQueue;
use packages::infra::sync::storage::{SledQueueStorage, SyncOperation, OperationPriority};
use packages::infra::sync::backoff::ExponentialBackoff;
use packages::infra::sync::conflict::TimestampConflictResolver;
use packages::infra::grpc::clients::user_preferences::{UserPreferencesClient, GrpcUserPreferences};
use packages::infra::sled::adapters::user_preferences::SledUserPreferences;
use packages::domains::finance::domain::primitives::Currency;
use std::sync::Arc;
use uuid::Uuid;
use tempfile::TempDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup
    let temp_dir = TempDir::new()?;
    let db = sled::open(temp_dir.path())?;
    
    // Create network monitor
    let network_monitor = Arc::new(NetworkStatusMonitor::new());
    
    // Create gRPC client (mocked for this example)
    let grpc_client = UserPreferencesClient::new();
    
    // Create factory and composite adapter
    let preferences = UserPreferencesFactory::create(&db, network_monitor.clone(), grpc_client);
    
    // Create sync queue with storage
    let storage = SledQueueStorage::new(&db)?;
    let queue = SyncQueue::new(
        Box::new(storage),
        Arc::new(TimestampConflictResolver::new()),
        Box::new(ExponentialBackoff::default())
    );
    
    // Create a user ID
    let user_id = Uuid::new_v4();
    
    // Use the preferences service (will automatically switch based on network status)
    match preferences.set_preferred_currency(user_id, Currency::USD).await {
        Ok(_) => println!("Successfully set currency"),
        Err(e) => {
            println!("Failed to set currency: {}, queuing for sync", e);
            
            // Queue the operation for later sync
            let operation = SyncOperation::SetCurrency {
                user_id,
                currency: Currency::USD,
                priority: OperationPriority::High,
                attempts: 0,
                scheduled_at: std::time::SystemTime::now(),
            };
            
            queue.enqueue(operation)?;
        }
    }
    
    // Simulate network status change
    network_monitor.set_connected(false);
    println!("Network disconnected");
    
    // Try another operation (will go directly to offline storage)
    match preferences.set_preferred_currency(user_id, Currency::EUR).await {
        Ok(_) => println!("Successfully set currency while offline"),
        Err(e) => println!("Failed to set currency while offline: {}", e),
    }
    
    // Simulate network restoration
    network_monitor.set_connected(true);
    println!("Network restored");
    
    // Process the sync queue
    let summary = queue.process(&*preferences).await?;
    println!("Processed {} operations successfully", summary.successful.len());
    println!("{} operations scheduled for retry", summary.retried.len());
    println!("{} operations failed permanently", summary.failed.len());
    
    Ok(())
}
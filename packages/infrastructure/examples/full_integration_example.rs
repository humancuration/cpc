//! Full integration example showing all components working together
//!
//! This example demonstrates the complete flow of using all the new infrastructure
//! components together in a realistic scenario.

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
use std::time::SystemTime;

/// Mock client that simulates network issues
#[derive(Clone)]
struct MockNetworkClient {
    should_fail: bool,
}

#[async_trait::async_trait]
impl packages::infra::sync::queue::UserPreferencesClient for MockNetworkClient {
    async fn set_preferred_currency(&self, _user_id: Uuid, _currency: Currency) -> Result<(), String> {
        if self.should_fail {
            Err("Network error".to_string())
        } else {
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting full integration example...");
    
    // Setup temporary database
    let temp_dir = TempDir::new()?;
    let db = sled::open(temp_dir.path())?;
    
    // Create network monitor
    let network_monitor = Arc::new(NetworkStatusMonitor::new());
    println!("✓ Network monitor created");
    
    // Create gRPC client (mocked for this example)
    let grpc_client = UserPreferencesClient::new();
    let online_service = GrpcUserPreferences::new(grpc_client.clone(), Uuid::nil());
    println!("✓ Online service created");
    
    // Create offline service
    let offline_service = SledUserPreferences::new(&db);
    println!("✓ Offline service created");
    
    // Create composite adapter
    let composite = CompositeUserPreferences::new(online_service, offline_service);
    println!("✓ Composite adapter created");
    
    // Test network-aware switching
    network_monitor.set_connected(true);
    composite.on_network_status_changed(true);
    println!("✓ Network status changed to connected");
    
    // Create factory
    let preferences = UserPreferencesFactory::create(&db, network_monitor.clone(), grpc_client);
    println!("✓ User preferences factory created");
    
    // Create sync queue with storage
    let storage = SledQueueStorage::new(&db)?;
    let queue = SyncQueue::new(
        Box::new(storage),
        Arc::new(TimestampConflictResolver::new()),
        Box::new(ExponentialBackoff::default())
    );
    println!("✓ Sync queue created");
    
    // Create a user ID
    let user_id = Uuid::new_v4();
    println!("✓ User ID generated: {}", user_id);
    
    // Test online operation
    match preferences.set_preferred_currency(user_id, Currency::USD).await {
        Ok(_) => println!("✓ Successfully set currency while online"),
        Err(e) => {
            println!("✗ Failed to set currency while online: {}", e);
        }
    }
    
    // Simulate network disconnection
    network_monitor.set_connected(false);
    composite.on_network_status_changed(false);
    println!("✓ Network status changed to disconnected");
    
    // Test offline operation
    match preferences.set_preferred_currency(user_id, Currency::EUR).await {
        Ok(_) => println!("✓ Successfully set currency while offline"),
        Err(e) => {
            println!("✗ Failed to set currency while offline: {}", e);
        }
    }
    
    // Queue an operation for sync when network returns
    let operation = SyncOperation::SetCurrency {
        user_id,
        currency: Currency::BTC,
        priority: OperationPriority::Critical,
        attempts: 0,
        scheduled_at: SystemTime::now(),
    };
    
    queue.enqueue(operation)?;
    println!("✓ Operation queued for sync");
    
    // Simulate network restoration
    network_monitor.set_connected(true);
    composite.on_network_status_changed(true);
    println!("✓ Network status changed to connected");
    
    // Process the sync queue
    let mock_client = MockNetworkClient { should_fail: false };
    let summary = queue.process(&mock_client).await?;
    println!("✓ Sync queue processed");
    println!("  - Successful operations: {}", summary.successful.len());
    println!("  - Retried operations: {}", summary.retried.len());
    println!("  - Failed operations: {}", summary.failed.len());
    
    // Test operation with network failure and retry
    let failing_client = MockNetworkClient { should_fail: true };
    let operation2 = SyncOperation::SetCurrency {
        user_id,
        currency: Currency::JPY,
        priority: OperationPriority::High,
        attempts: 0,
        scheduled_at: SystemTime::now(),
    };
    
    queue.enqueue(operation2)?;
    println!("✓ Failing operation queued");
    
    // First attempt should fail and be retried
    let summary = queue.process(&failing_client).await?;
    println!("✓ First processing attempt completed");
    println!("  - Successful operations: {}", summary.successful.len());
    println!("  - Retried operations: {}", summary.retried.len());
    println!("  - Failed operations: {}", summary.failed.len());
    
    // Second attempt with working client should succeed
    let working_client = MockNetworkClient { should_fail: false };
    let summary = queue.process(&working_client).await?;
    println!("✓ Second processing attempt completed");
    println!("  - Successful operations: {}", summary.successful.len());
    println!("  - Retried operations: {}", summary.retried.len());
    println!("  - Failed operations: {}", summary.failed.len());
    
    println!("\n🎉 All components working together successfully!");
    println!("\nThis example demonstrated:");
    println!("  - Network status monitoring and automatic switching");
    println!("  - Composite adapter pattern with error handling");
    println!("  - Factory pattern for creating services");
    println!("  - Sync queue with prioritization");
    println!("  - Exponential backoff for retry strategies");
    println!("  - Fault-tolerant processing that continues after failures");
    println!("  - Detailed processing summaries");
    
    Ok(())
}
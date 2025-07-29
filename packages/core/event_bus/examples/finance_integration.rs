//! Finance module integration example for the event bus system

use event_bus::{
    EventBus,
    DomainEvent,
    EventSource,
    EventFilter,
    Subscription,
};
use storage_abstraction::InMemoryStore;
use std::sync::Arc;
use serde_json::json;

/// Finance service that uses the event bus
struct FinanceService {
    event_bus: EventBus,
}

impl FinanceService {
    /// Create a new finance service
    pub fn new(event_bus: EventBus) -> Self {
        Self { event_bus }
    }
    
    /// Create a new transaction
    pub async fn create_transaction(
        &self,
        user_id: &str,
        amount: f64,
        merchant: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Business logic would go here
        println!("Creating transaction for user {}: ${} at {}", user_id, amount, merchant);
        
        // Publish event to all subscribers
        let event = DomainEvent::new_local(
            "finance".to_string(),
            "transaction_created".to_string(),
            json!({ 
                "user_id": user_id, 
                "amount": amount,
                "merchant": merchant,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }),
        );
        
        self.event_bus.publish(event).await?;
        
        Ok(())
    }
}

/// Transaction alert system
struct TransactionAlerts {
    user_id: String,
}

impl TransactionAlerts {
    /// Create a new transaction alert system
    pub fn new(user_id: String) -> Self {
        Self { user_id }
    }
    
    /// Handle transaction alerts
    pub async fn handle_transaction_alerts(&self, event_bus: &EventBus) -> Result<(), Box<dyn std::error::Error>> {
        // Subscribe to transaction events for this user
        let filter = EventFilter {
            domain: Some("finance".to_string()),
            event_types: vec!["transaction_created".to_string()],
            user_id: Some(self.user_id.clone()),
        };
        
        let mut subscription = event_bus.subscribe(filter).await;
        
        // Process incoming events
        while let Some(event) = subscription.recv().await {
            if let Some(payload) = event.payload.as_object() {
                let amount = payload.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let merchant = payload.get("merchant").and_then(|v| v.as_str()).unwrap_or("Unknown");
                
                // Send push notification for high-value transactions
                if amount > 100.0 {
                    self.send_push_notification(
                        "High-Value Transaction",
                        &format!("${:.2} at {}", amount, merchant)
                    ).await;
                }
            }
        }
        
        Ok(())
    }
    
    /// Send a push notification
    async fn send_push_notification(&self, title: &str, body: &str) {
        println!("PUSH NOTIFICATION to {}: {} - {}", self.user_id, title, body);
        // In a real implementation, this would send an actual push notification
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create storage backend
    let storage = Arc::new(InMemoryStore::new());
    
    // Create event bus
    let event_bus = EventBus::new(storage);
    
    // Create finance service
    let finance_service = FinanceService::new(event_bus.clone());
    
    // Create transaction alerts
    let transaction_alerts = TransactionAlerts::new("user_123".to_string());
    
    // Start listening for transaction alerts
    let event_bus_clone = event_bus.clone();
    tokio::spawn(async move {
        if let Err(e) = transaction_alerts.handle_transaction_alerts(&event_bus_clone).await {
            eprintln!("Error handling transaction alerts: {}", e);
        }
    });
    
    // Create some transactions
    finance_service.create_transaction("user_123", 25.50, "Coffee Shop").await?;
    finance_service.create_transaction("user_123", 150.00, "Electronics Store").await?; // This should trigger an alert
    finance_service.create_transaction("user_456", 75.00, "Restaurant").await?; // This won't trigger our alerts
    
    // Give some time for events to be processed
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    println!("Finance integration example completed successfully!");
    Ok(())
}
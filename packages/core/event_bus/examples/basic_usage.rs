//! Basic usage example for the event bus system

use event_bus::{
    EventBus,
    DomainEvent,
    EventSource,
    EventFilter,
    EventBusPlugin,
};
use storage_abstraction::InMemoryStore;
use std::sync::Arc;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create storage backend
    let storage = Arc::new(InMemoryStore::new());
    
    // Create event bus
    let event_bus = EventBus::new(storage);
    
    // Create subscription for health events
    let filter = EventFilter {
        domain: Some("health".to_string()),
        event_types: vec!["mood_updated".to_string()],
        user_id: None,
    };
    
    let mut subscription = event_bus.subscribe(filter).await;
    
    // Spawn a task to handle events
    tokio::spawn(async move {
        while let Some(event) = subscription.recv().await {
            println!("Received event: {} - {}", event.event_type, event.event_id);
            
            // Process the event payload
            if let Some(mood) = event.payload.get("mood") {
                println!("User mood updated to: {:?}", mood);
            }
        }
    });
    
    // Publish some events
    let event1 = DomainEvent::new_local(
        "health".to_string(),
        "mood_updated".to_string(),
        json!({ "user_id": "user_123", "mood": "happy", "timestamp": "2023-01-01T12:00:00Z" }),
    );
    
    let event2 = DomainEvent::new_local(
        "finance".to_string(),
        "transaction_created".to_string(),
        json!({ "user_id": "user_123", "amount": 50.0, "currency": "USD", "description": "Groceries" }),
    );
    
    event_bus.publish(event1).await?;
    event_bus.publish(event2).await?;
    
    // Give some time for events to be processed
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    println!("Event bus example completed successfully!");
    Ok(())
}
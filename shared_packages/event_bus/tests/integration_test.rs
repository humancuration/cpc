//! Integration tests for the event bus system

use event_bus::{
    EventBus,
    DomainEvent,
    EventSource,
    EventFilter,
    EventStorage,
};
use storage_abstraction::InMemoryStore;
use std::sync::Arc;
use serde_json::json;

struct TestEventStorage {
    inner: InMemoryStore,
}

impl TestEventStorage {
    fn new() -> Self {
        Self {
            inner: InMemoryStore::new(),
        }
    }
}

#[async_trait::async_trait]
impl EventStorage for TestEventStorage {
    async fn store_event(&self, event: &DomainEvent) -> Result<(), event_bus::EventError> {
        self.inner.store_event(event).await
    }
    
    async fn get_events(&self, query: event_bus::application::service::EventQuery) -> Result<Vec<DomainEvent>, event_bus::EventError> {
        self.inner.get_events(query).await
    }
}

#[tokio::test]
async fn test_event_publish_and_subscribe() {
    // Create event bus
    let storage = Arc::new(TestEventStorage::new());
    let event_bus = EventBus::new(storage);
    
    // Create subscription
    let filter = EventFilter {
        domain: Some("health".to_string()),
        event_types: vec!["mood_updated".to_string()],
        user_id: None,
    };
    
    let mut subscription = event_bus.subscribe(filter).await;
    
    // Publish event
    let event = DomainEvent::new_local(
        "health".to_string(),
        "mood_updated".to_string(),
        json!({ "user_id": "user_123", "mood": "happy" }),
    );
    
    event_bus.publish(event.clone()).await.unwrap();
    
    // Receive event
    let received = tokio::time::timeout(std::time::Duration::from_millis(100), subscription.recv()).await;
    assert!(received.is_ok());
    
    let received_event = received.unwrap();
    assert_eq!(received_event.event_id, event.event_id);
    assert_eq!(received_event.domain, "health");
    assert_eq!(received_event.event_type, "mood_updated");
}

#[tokio::test]
async fn test_event_filtering() {
    // Create event bus
    let storage = Arc::new(TestEventStorage::new());
    let event_bus = EventBus::new(storage);
    
    // Create subscription for specific event type
    let filter = EventFilter {
        domain: Some("finance".to_string()),
        event_types: vec!["transaction_created".to_string()],
        user_id: None,
    };
    
    let mut subscription = event_bus.subscribe(filter).await;
    
    // Publish matching event
    let matching_event = DomainEvent::new_local(
        "finance".to_string(),
        "transaction_created".to_string(),
        json!({ "amount": 100.0 }),
    );
    
    event_bus.publish(matching_event).await.unwrap();
    
    // Publish non-matching event
    let non_matching_event = DomainEvent::new_local(
        "finance".to_string(),
        "budget_updated".to_string(),
        json!({ "category": "food", "amount": 500.0 }),
    );
    
    event_bus.publish(non_matching_event).await.unwrap();
    
    // Should only receive the matching event
    let received = tokio::time::timeout(std::time::Duration::from_millis(100), subscription.recv()).await;
    assert!(received.is_ok());
    
    // Should not receive another event (non-matching should be filtered out)
    let not_received = tokio::time::timeout(std::time::Duration::from_millis(100), subscription.recv()).await;
    assert!(not_received.is_err());
}
# Migration Guide for Event Bus System

This document provides guidance on migrating existing modules to use the new Event Bus System.

## Overview

The Event Bus System generalizes the Bevy ECS integration pattern from `consent_manager` into a standardized event distribution system for real-time updates across applications.

## Migration Steps

### 1. Update Cargo.toml

Add the event bus dependency to your module's Cargo.toml:

```toml
[dependencies]
event_bus = { path = "../event_bus" }
```

### 2. Replace Direct Bevy Integration

#### Before (Consent Manager Example)
```rust
// Direct Bevy integration
use bevy_ecs::prelude::*;
use bevy_app::prelude::*;

#[derive(Event, Clone, Debug)]
pub struct ConsentChangeEvent {
    pub user_id: String,
    pub domain: Domain,
    pub new_level: DataSharingLevel,
    pub timestamp: DateTime<Utc>,
}

static GLOBAL_CONSENT_EVENT_CHANNEL: OnceLock<ConsentEventChannel> = OnceLock::new();

#[derive(Resource, Clone)]
pub struct ConsentEventChannel {
    sender: Arc<broadcast::Sender<ConsentChangeEvent>>,
}

impl ConsentEventChannel {
    pub fn publish(&self, event: ConsentChangeEvent) {
        let _ = self.sender.send(event);
    }
}
```

#### After (Using Event Bus)
```rust
// Using Event Bus
use event_bus::{EventBus, DomainEvent};

struct ConsentService {
    event_bus: EventBus,
}

impl ConsentService {
    async fn update_consent_level(&self, user_id: &str, domain: Domain, level: DataSharingLevel) -> Result<(), ConsentError> {
        // Business logic...
        
        // Publish event using Event Bus
        let event = DomainEvent::new_local(
            "consent".to_string(),
            "consent_updated".to_string(),
            serde_json::json!({
                "user_id": user_id,
                "domain": domain.as_str(),
                "level": level.as_str(),
            }),
        );
        
        self.event_bus.publish(event).await
            .map_err(|e| ConsentError::EventError(e.to_string()))?;
        
        Ok(())
    }
}
```

### 3. Update Service Initialization

#### Before (Consent Manager Example)
```rust
// Direct Bevy initialization
use bevy_app::prelude::*;

let mut app = App::new();
app.add_plugins(ConsentEventPlugin);
```

#### After (Using Event Bus)
```rust
// Using Event Bus
use event_bus::EventBus;
use storage_abstraction::{InMemoryStore};
use std::sync::Arc;

// Create storage backend
let storage = Arc::new(InMemoryStore::new());

// Create event bus
let event_bus = EventBus::new(storage);

// Use in service
let service = ConsentService::new(event_bus);
```

## Event Handling Migration

### Before (Direct Event Handling)
```rust
// Direct event handling
pub fn handle_consent_updates(
    mut consent_events: EventReader<ConsentChangeEvent>,
) {
    for event in consent_events.read() {
        // Handle the consent change event
        tracing::debug!(
            "Consent changed for user {} in domain {:?} to level {:?}",
            event.user_id,
            event.domain,
            event.new_level
        );
    }
}
```

### After (Using Event Bus Subscription)
```rust
// Using Event Bus subscription
use event_bus::{EventBus, EventFilter};

struct ConsentEventHandler {
    event_bus: EventBus,
}

impl ConsentEventHandler {
    async fn start_handling(&self) {
        // Subscribe to consent events
        let filter = EventFilter {
            domain: Some("consent".to_string()),
            event_types: vec!["consent_updated".to_string()],
            user_id: None,
        };
        
        let mut subscription = self.event_bus.subscribe(filter).await;
        
        // Process incoming events
        while let Some(event) = subscription.recv().await {
            // Handle the consent change event
            tracing::debug!(
                "Consent event: domain={}, type={}, id={}",
                event.domain,
                event.event_type,
                event.event_id
            );
            
            // Process event payload
            if let Some(user_id) = event.payload.get("user_id").and_then(|v| v.as_str()) {
                tracing::debug!("Consent changed for user: {}", user_id);
            }
        }
    }
}
```

## Integration with Bevy Applications

For applications that still use Bevy ECS, you can integrate the Event Bus with Bevy:

```rust
use bevy_app::prelude::*;
use event_bus::{EventBusPlugin, EventBus};

fn main() {
    let mut app = App::new();
    
    // Add the EventBusPlugin
    app.add_plugins(EventBusPlugin);
    
    // You can still access the global event channel if needed
    if let Some(channel) = event_bus::infrastructure::bevy::EventChannel::get_global() {
        // Use the channel directly
    }
    
    app.run();
}
```

## Health Module Integration Example

```rust
// Health service using Event Bus
use event_bus::{EventBus, DomainEvent};

struct HealthService {
    event_bus: EventBus,
}

impl HealthService {
    async fn record_mood(&self, user_id: &str, mood: &str) -> Result<(), HealthError> {
        // Business logic...
        
        // Publish mood update event
        let event = DomainEvent::new_local(
            "health".to_string(),
            "mood_updated".to_string(),
            serde_json::json!({
                "user_id": user_id,
                "mood": mood,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            }),
        );
        
        self.event_bus.publish(event).await?;
        
        Ok(())
    }
}

// Mood tracker component
struct MoodTracker {
    user_id: String,
}

impl MoodTracker {
    async fn handle_mood_updates(&self, event_bus: &EventBus) -> Result<(), Box<dyn std::error::Error>> {
        // Subscribe to mood updates for this user
        let filter = event_bus::domain::subscription::EventFilter {
            domain: Some("health".to_string()),
            event_types: vec!["mood_updated".to_string()],
            user_id: Some(self.user_id.clone()),
        };
        
        let mut subscription = event_bus.subscribe(filter).await;
        
        // Process incoming events
        while let Some(event) = subscription.recv().await {
            if let Some(mood) = event.payload.get("mood").and_then(|v| v.as_str()) {
                println!("UI Update: User {} mood is now {}", self.user_id, mood);
                // Trigger UI update here
            }
        }
        
        Ok(())
    }
}
```

## Finance Module Integration Example

```rust
// Finance service using Event Bus
use event_bus::{EventBus, DomainEvent};

struct FinanceService {
    event_bus: EventBus,
}

impl FinanceService {
    async fn create_transaction(&self, user_id: &str, amount: f64, merchant: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Business logic...
        
        // Publish transaction event
        let event = DomainEvent::new_local(
            "finance".to_string(),
            "transaction_created".to_string(),
            serde_json::json!({
                "user_id": user_id,
                "amount": amount,
                "merchant": merchant,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            }),
        );
        
        self.event_bus.publish(event).await?;
        
        Ok(())
    }
}

// Transaction alert system
struct TransactionAlerts {
    user_id: String,
}

impl TransactionAlerts {
    async fn handle_transaction_alerts(&self, event_bus: &EventBus) -> Result<(), Box<dyn std::error::Error>> {
        // Subscribe to transaction events for this user
        let filter = event_bus::domain::subscription::EventFilter {
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
    
    async fn send_push_notification(&self, title: &str, body: &str) {
        println!("PUSH NOTIFICATION to {}: {} - {}", self.user_id, title, body);
        // In a real implementation, this would send an actual push notification
    }
}
```

## Testing During Migration

Use the in-memory storage implementation for testing:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use storage_abstraction::InMemoryStore;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_with_in_memory_storage() {
        let storage = Arc::new(InMemoryStore::new());
        let event_bus = EventBus::new(storage);
        let service = MyService::new(event_bus);
        
        // Test your service logic
        // ...
    }
}
```

## Performance Considerations

1. The Event Bus System uses efficient broadcast channels for event distribution.

2. Event filtering is done at the subscription level to minimize unnecessary event processing.

3. Events are stored using the Storage Abstraction Layer for persistence and history.

## Troubleshooting

### Common Issues

1. **Serialization Errors**: Ensure all event payloads are properly serializable to JSON.

2. **Subscription Issues**: Verify that event filters match the events being published.

3. **Storage Errors**: Check that the storage backend is properly configured and accessible.

### Logging and Monitoring

The Event Bus System uses tracing for logging. Enable tracing in your application to monitor event operations:

```rust
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // ... rest of your application
}
```

This will provide detailed logs of event operations, including publishing, subscription, and any errors that occur.
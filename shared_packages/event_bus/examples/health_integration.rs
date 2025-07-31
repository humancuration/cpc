//! Health module integration example for the event bus system

use event_bus::{
    EventBus,
    DomainEvent,
    EventSource,
    EventFilter,
};
use storage_abstraction::InMemoryStore;
use std::sync::Arc;
use serde_json::json;

/// Health service that uses the event bus
struct HealthService {
    event_bus: EventBus,
}

impl HealthService {
    /// Create a new health service
    pub fn new(event_bus: EventBus) -> Self {
        Self { event_bus }
    }
    
    /// Record a user's mood
    pub async fn record_mood(
        &self,
        user_id: &str,
        mood: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Business logic would go here
        println!("Recording mood for user {}: {}", user_id, mood);
        
        // Publish event to all subscribers
        let event = DomainEvent::new_local(
            "health".to_string(),
            "mood_updated".to_string(),
            json!({ 
                "user_id": user_id, 
                "mood": mood,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }),
        );
        
        self.event_bus.publish(event).await?;
        
        Ok(())
    }
}

/// Mood tracking component for UI
struct MoodTracker {
    user_id: String,
}

impl MoodTracker {
    /// Create a new mood tracker
    pub fn new(user_id: String) -> Self {
        Self { user_id }
    }
    
    /// Handle mood updates
    pub async fn handle_mood_updates(&self, event_bus: &EventBus) -> Result<(), Box<dyn std::error::Error>> {
        // Subscribe to mood updates for this user
        let filter = EventFilter {
            domain: Some("health".to_string()),
            event_types: vec!["mood_updated".to_string()],
            user_id: Some(self.user_id.clone()),
        };
        
        let mut subscription = event_bus.subscribe(filter).await;
        
        // Process incoming events
        while let Some(event) = subscription.recv().await {
            if let Some(mood) = event.payload.get("mood") {
                if let Some(mood_str) = mood.as_str() {
                    println!("UI Update: User {} mood is now {}", self.user_id, mood_str);
                    // Trigger UI update here
                }
            }
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create storage backend
    let storage = Arc::new(InMemoryStore::new());
    
    // Create event bus
    let event_bus = EventBus::new(storage);
    
    // Create health service
    let health_service = HealthService::new(event_bus.clone());
    
    // Create mood tracker
    let mood_tracker = MoodTracker::new("user_123".to_string());
    
    // Start listening for mood updates
    let event_bus_clone = event_bus.clone();
    tokio::spawn(async move {
        if let Err(e) = mood_tracker.handle_mood_updates(&event_bus_clone).await {
            eprintln!("Error handling mood updates: {}", e);
        }
    });
    
    // Record some moods
    health_service.record_mood("user_123", "happy").await?;
    health_service.record_mood("user_123", "excited").await?;
    health_service.record_mood("user_456", "sad").await?; // This won't trigger our tracker
    
    // Give some time for events to be processed
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    println!("Health integration example completed successfully!");
    Ok(())
}
//! Bevy ECS integration for real-time event updates
//! 
//! This module provides integration with Bevy ECS for distributing
//! event changes to all interested applications in real-time.

use bevy_ecs::prelude::*;
use bevy_app::prelude::*;
use crate::domain::event::DomainEvent;
use std::sync::{Arc, OnceLock};
use tokio::sync::broadcast;

/// Global instance of the event channel
static GLOBAL_EVENT_CHANNEL: OnceLock<EventChannel> = OnceLock::new();

/// Resource that holds the broadcast channel for events
#[derive(Resource, Clone)]
pub struct EventChannel {
    sender: Arc<broadcast::Sender<DomainEvent>>,
}

impl EventChannel {
    /// Create a new event channel
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(1000);
        Self {
            sender: Arc::new(sender),
        }
    }
    
    /// Get the global event channel
    pub fn get_global() -> Option<&'static Self> {
        GLOBAL_EVENT_CHANNEL.get()
    }
    
    /// Set the global event channel
    pub fn set_global(channel: Self) -> Result<(), Self> {
        GLOBAL_EVENT_CHANNEL.set(channel)
    }
    
    /// Get a receiver for the events
    pub fn subscribe(&self) -> broadcast::Receiver<DomainEvent> {
        self.sender.subscribe()
    }
    
    /// Publish an event
    pub fn publish(&self, event: DomainEvent) {
        let _ = self.sender.send(event);
    }
}

impl Default for EventChannel {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin for integrating events with Bevy
pub struct EventBusPlugin;

impl Plugin for EventBusPlugin {
    fn build(&self, app: &mut App) {
        let channel = EventChannel::new();
        // Try to set the global channel, ignore if it's already set
        let _ = EventChannel::set_global(channel.clone());
        
        app
            .insert_resource(channel)
            .add_event::<DomainEvent>();
    }
}

/// System for handling event updates in Bevy ECS
/// 
/// This system should be added to applications that need to react
/// to events in real-time.
pub fn handle_events(
    mut event_events: EventReader<DomainEvent>,
    // Add query parameters for components that need to be updated
    // For example:
    // mut query: Query<&mut SomeComponent>,
) {
    for event in event_events.read() {
        // Handle the event
        // This is where applications would update their UI components
        // or take other actions based on the event
        
        // Example of what an application might do:
        // for mut component in &mut query {
        //     if component.user_id == event_user_id && component.domain == event.domain {
        //         component.update_from_event(event);
        //     }
        // }
        
        // Log the event for debugging
        tracing::debug!(
            "Event processed: domain={}, type={}, id={}",
            event.domain,
            event.event_type,
            event.event_id
        );
    }
}
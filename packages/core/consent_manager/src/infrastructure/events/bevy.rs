//! Bevy ECS integration for real-time consent updates
//!
//! This module provides integration with Bevy ECS for distributing
//! consent change events to all interested applications in real-time.

use bevy_ecs::prelude::*;
use bevy_app::prelude::*;
use crate::domain::consent::{DataSharingLevel, Domain};
use std::sync::{Arc, OnceLock};
use tokio::sync::broadcast;

/// Global instance of the consent event channel
static GLOBAL_CONSENT_EVENT_CHANNEL: OnceLock<ConsentEventChannel> = OnceLock::new();

/// Event that is sent when consent changes
#[derive(Event, Clone, Debug)]
pub struct ConsentChangeEvent {
    /// The user ID whose consent changed
    pub user_id: String,
    /// The domain that was affected
    pub domain: Domain,
    /// The new consent level
    pub new_level: DataSharingLevel,
    /// The timestamp of the change
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Resource that holds the broadcast channel for consent events
#[derive(Resource, Clone)]
pub struct ConsentEventChannel {
    sender: Arc<broadcast::Sender<ConsentChangeEvent>>,
}

impl ConsentEventChannel {
    /// Create a new consent event channel
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self {
            sender: Arc::new(sender),
        }
    }
    
    /// Get the global consent event channel
    pub fn get_global() -> Option<&'static Self> {
        GLOBAL_CONSENT_EVENT_CHANNEL.get()
    }
    
    /// Set the global consent event channel
    pub fn set_global(channel: Self) -> Result<(), Self> {
        GLOBAL_CONSENT_EVENT_CHANNEL.set(channel)
    }
    
    /// Get a receiver for the consent events
    pub fn subscribe(&self) -> broadcast::Receiver<ConsentChangeEvent> {
        self.sender.subscribe()
    }
    
    /// Publish a consent change event
    pub fn publish(&self, event: ConsentChangeEvent) {
        let _ = self.sender.send(event);
    }
}

impl Default for ConsentEventChannel {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin for integrating consent events with Bevy
pub struct ConsentEventPlugin;

impl Plugin for ConsentEventPlugin {
    fn build(&self, app: &mut App) {
        let channel = ConsentEventChannel::new();
        // Try to set the global channel, ignore if it's already set
        let _ = ConsentEventChannel::set_global(channel.clone());
        
        app
            .insert_resource(channel)
            .add_event::<ConsentChangeEvent>();
    }
}

/// System for handling consent updates in Bevy ECS
///
/// This system should be added to applications that need to react
/// to consent changes in real-time.
pub fn handle_consent_updates(
    mut consent_events: EventReader<ConsentChangeEvent>,
    // Add query parameters for components that need to be updated
    // For example:
    // mut query: Query<&mut ConsentIndicator>,
) {
    for event in consent_events.read() {
        // Handle the consent change event
        // This is where applications would update their UI components
        // or take other actions based on the consent change
        
        // Example of what an application might do:
        // for mut indicator in &mut query {
        //     if indicator.user_id == event.user_id && indicator.domain == event.domain {
        //         indicator.update_level(event.new_level.clone());
        //     }
        // }
        
        // Log the event for debugging
        tracing::debug!(
            "Consent changed for user {} in domain {:?} to level {:?}",
            event.user_id,
            event.domain,
            event.new_level
        );
    }
}
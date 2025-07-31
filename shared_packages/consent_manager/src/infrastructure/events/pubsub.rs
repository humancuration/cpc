//! Publish-subscribe system for consent events.

use tokio::sync::broadcast;
use crate::domain::{
    consent::{Domain, DataSharingLevel},
    audit::Actor,
};

/// Consent event types
#[derive(Debug, Clone)]
pub enum ConsentEvent {
    /// Consent level was updated
    LevelUpdated {
        user_id: String,
        domain: Domain,
        old_level: DataSharingLevel,
        new_level: DataSharingLevel,
        actor: Actor,
    },
    /// Consent was revoked
    Revoked {
        user_id: String,
        domain: Domain,
        actor: Actor,
    },
}

/// Event publisher for consent changes
pub struct EventPublisher {
    sender: broadcast::Sender<ConsentEvent>,
}

impl EventPublisher {
    /// Create a new event publisher
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { sender }
    }

    /// Publish a consent event
    pub fn publish(&self, event: ConsentEvent) {
        // Try to send the event, but don't panic if there are no receivers
        let _ = self.sender.send(event);
    }

    /// Subscribe to consent events
    pub fn subscribe(&self) -> broadcast::Receiver<ConsentEvent> {
        self.sender.subscribe()
    }
}

impl Default for EventPublisher {
    fn default() -> Self {
        Self::new()
    }
}
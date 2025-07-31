//! Event listeners for consent events.

use tokio::sync::broadcast::Receiver;
use tracing::info;
use crate::infrastructure::events::pubsub::ConsentEvent;

/// Event listener for consent events
pub struct EventListener {
    receiver: Receiver<ConsentEvent>,
}

impl EventListener {
    /// Create a new event listener
    pub fn new(receiver: Receiver<ConsentEvent>) -> Self {
        Self { receiver }
    }

    /// Start listening for events
    pub async fn listen(mut self) {
        while let Ok(event) = self.receiver.recv().await {
            self.handle_event(event).await;
        }
    }

    /// Handle a consent event
    async fn handle_event(&self, event: ConsentEvent) {
        match event {
            ConsentEvent::LevelUpdated {
                user_id,
                domain,
                old_level,
                new_level,
                actor,
            } => {
                info!(
                    "Consent level updated for user {} in domain {:?}: {:?} -> {:?} by {:?}",
                    user_id, domain, old_level, new_level, actor
                );
                // In a real implementation, this might trigger notifications,
                // update caches, or synchronize with other systems
            }
            ConsentEvent::Revoked {
                user_id,
                domain,
                actor,
            } => {
                info!(
                    "Consent revoked for user {} in domain {:?} by {:?}",
                    user_id, domain, actor
                );
                // In a real implementation, this might trigger data deletion,
                // update caches, or synchronize with other systems
            }
        }
    }
}
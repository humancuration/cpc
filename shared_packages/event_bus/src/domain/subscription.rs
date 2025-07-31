//! Subscription management
//! 
//! This module handles event subscriptions and filtering.

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use crate::domain::event::DomainEvent;

/// Event filter for subscriptions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventFilter {
    /// Filter by domain (None means all domains)
    pub domain: Option<String>,
    
    /// Filter by event types (empty means all event types)
    pub event_types: Vec<String>,
    
    /// Filter by user ID (None means all users)
    pub user_id: Option<String>,
}

/// Event subscription
pub struct Subscription {
    /// Channel receiver for events
    pub receiver: mpsc::UnboundedReceiver<DomainEvent>,
    
    /// Filter used for this subscription
    pub filter: EventFilter,
}

impl Subscription {
    /// Create a new subscription
    pub fn new(receiver: mpsc::UnboundedReceiver<DomainEvent>, filter: EventFilter) -> Self {
        Self { receiver, filter }
    }
    
    /// Receive the next event
    pub async fn recv(&mut self) -> Option<DomainEvent> {
        self.receiver.recv().await
    }
    
    /// Check if an event matches this subscription's filter
    pub fn matches(&self, event: &DomainEvent) -> bool {
        // Check domain filter
        if let Some(ref domain) = self.filter.domain {
            if &event.domain != domain {
                return false;
            }
        }
        
        // Check event type filter
        if !self.filter.event_types.is_empty() {
            if !self.filter.event_types.contains(&event.event_type) {
                return false;
            }
        }
        
        // Check user ID filter (extract from payload if needed)
        if let Some(ref user_id) = self.filter.user_id {
            if let Some(event_user_id) = event.payload.get("user_id") {
                if let Some(event_user_id_str) = event_user_id.as_str() {
                    if event_user_id_str != user_id {
                        return false;
                    }
                }
            }
        }
        
        true
    }
}
//! Shared cursor presence tracking for collaborative editing

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::core::Position;
use event_bus::{EventBus, DomainEvent};
use serde_json::json;

/// Represents a user's presence in a collaborative session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPresence {
    pub user_id: Uuid,
    pub cursor: Option<Position>,
    pub selection: Option<(Position, Position)>,
    pub last_activity: DateTime<Utc>,
    pub is_typing: bool,
    pub qos_tier: u8, // Quality of Service tier (0=critical, 1=medium, 2=low)
}

/// Manages presence information for a collaborative document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceManager {
    pub document_id: Uuid,
    pub users: HashMap<Uuid, UserPresence>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip)]
    pub event_bus: Option<EventBus>,
}

impl PresenceManager {
    /// Create a new presence manager for a document
    pub fn new(document_id: Uuid) -> Self {
        Self {
            document_id,
            users: HashMap::new(),
            updated_at: Utc::now(),
            event_bus: None,
        }
    }
    
    /// Set the event bus for this presence manager
    pub fn set_event_bus(&mut self, event_bus: EventBus) {
        self.event_bus = Some(event_bus);
    }
/// Update a user's presence information
pub fn update_presence(
    &mut self,
    user_id: Uuid,
    cursor: Option<Position>,
    selection: Option<(Position, Position)>,
    is_typing: bool,
    qos_tier: u8,
) -> Result<(), crate::core::CollaborationError> {
    let now = Utc::now();
    let presence = UserPresence {
        user_id,
        cursor,
        selection,
        last_activity: now,
        is_typing,
        qos_tier,
    };

    self.users.insert(user_id, presence.clone());
    self.cleanup_inactive_presences();
    self.updated_at = Utc::now();
        
        // Publish event if event bus is available
        if let Some(ref event_bus) = self.event_bus {
            let event = DomainEvent::new_local(
                "collaboration".to_string(),
                "PresenceUpdated".to_string(),
                json!({
                    "document_id": self.document_id,
                    "user_id": presence.user_id,
                    "cursor": presence.cursor,
                    "selection": presence.selection,
                    "is_typing": presence.is_typing,
                    "qos_tier": presence.qos_tier,
                }),
            );
            
            event_bus.publish(event).map_err(|_| crate::core::CollaborationError::EventPublishError)?;
        }
        
        Ok(())
    }

    /// Remove a user's presence
    pub fn remove_presence(&mut self, user_id: Uuid) -> Result<(), crate::core::CollaborationError> {
        self.users.remove(&user_id);
        self.updated_at = Utc::now();
        
        // Publish event if event bus is available
        if let Some(ref event_bus) = self.event_bus {
            let event = DomainEvent::new_local(
                "collaboration".to_string(),
                "UserLeftDocument".to_string(),
                json!({
                    "document_id": self.document_id,
                    "user_id": user_id,
                }),
            );
            
            event_bus.publish(event).map_err(|_| crate::core::CollaborationError::EventPublishError)?;
        }
        
        Ok(())
    }

    /// Get all active presences
    pub fn get_presences(&self) -> Vec<UserPresence> {
        self.users.values().cloned().collect()
    }

    /// Get a specific user's presence
    pub fn get_user_presence(&self, user_id: Uuid) -> Option<UserPresence> {
        self.users.get(&user_id).cloned()
    }

    /// Clean up inactive presences (older than 30 seconds)
    fn cleanup_inactive_presences(&mut self) {
        let now = Utc::now();
        let threshold = now - chrono::Duration::seconds(30);
        
        self.users.retain(|_, presence| presence.last_activity > threshold);
    }

    /// Check if a user is currently active
    pub fn is_user_active(&self, user_id: Uuid) -> bool {
        if let Some(presence) = self.users.get(&user_id) {
            let now = Utc::now();
            let threshold = now - chrono::Duration::seconds(30);
            presence.last_activity > threshold
        } else {
            false
        }
    }

    /// Get the count of active users
    pub fn active_user_count(&self) -> usize {
        let now = Utc::now();
        let threshold = now - chrono::Duration::seconds(30);
        
        self.users
            .values()
            .filter(|presence| presence.last_activity > threshold)
            .count()
    }
    
    /// Cleanup inactive users based on a custom threshold
    pub fn cleanup_inactive(&mut self, threshold: std::time::Duration) {
        let now = Utc::now();
        self.users.retain(|_, presence| {
            let elapsed = now - presence.last_activity;
            elapsed <= chrono::Duration::from_std(threshold).unwrap()
        });
    }
}

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// P2P event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum P2PEvent {
    MediaShared {
        content_id: String,
        content_hash: String,
        peer_id: String,
        timestamp: DateTime<Utc>,
    },
    MediaRequested {
        content_id: String,
        requesting_peer: String,
        timestamp: DateTime<Utc>,
    },
    MediaTransferred {
        content_id: String,
        from_peer: String,
        to_peer: String,
        timestamp: DateTime<Utc>,
    },
    PeerConnected {
        peer_id: String,
        timestamp: DateTime<Utc>,
    },
    PeerDisconnected {
        peer_id: String,
        timestamp: DateTime<Utc>,
    },
}

/// Event system for P2P operations
pub struct EventSystem {
    events: Arc<Mutex<VecDeque<P2PEvent>>>,
    max_events: usize,
}

impl EventSystem {
    pub fn new(max_events: usize) -> Self {
        Self {
            events: Arc::new(Mutex::new(VecDeque::new())),
            max_events,
        }
    }

    pub fn emit_event(&self, event: P2PEvent) {
        let mut events = self.events.lock().unwrap();
        
        // Add new event
        events.push_back(event.clone());
        
        // Remove old events if we exceed the limit
        while events.len() > self.max_events {
            events.pop_front();
        }
        
        // Log the event
        match &event {
            P2PEvent::MediaShared { content_id, peer_id, .. } => {
                log::info!("Media shared: {} by peer {}", content_id, peer_id);
            }
            P2PEvent::MediaRequested { content_id, requesting_peer, .. } => {
                log::info!("Media requested: {} by peer {}", content_id, requesting_peer);
            }
            P2PEvent::MediaTransferred { content_id, from_peer, to_peer, .. } => {
                log::info!("Media transferred: {} from {} to {}", content_id, from_peer, to_peer);
            }
            P2PEvent::PeerConnected { peer_id, .. } => {
                log::info!("Peer connected: {}", peer_id);
            }
            P2PEvent::PeerDisconnected { peer_id, .. } => {
                log::info!("Peer disconnected: {}", peer_id);
            }
        }
    }

    pub fn get_recent_events(&self, count: usize) -> Vec<P2PEvent> {
        let events = self.events.lock().unwrap();
        events.iter()
            .rev()
            .take(count)
            .cloned()
            .collect()
    }

    pub fn get_events_by_type(&self, event_type: &str) -> Vec<P2PEvent> {
        let events = self.events.lock().unwrap();
        events.iter()
            .filter(|event| {
                match (event_type, event) {
                    ("media_shared", P2PEvent::MediaShared { .. }) => true,
                    ("media_requested", P2PEvent::MediaRequested { .. }) => true,
                    ("media_transferred", P2PEvent::MediaTransferred { .. }) => true,
                    ("peer_connected", P2PEvent::PeerConnected { .. }) => true,
                    ("peer_disconnected", P2PEvent::PeerDisconnected { .. }) => true,
                    _ => false,
                }
            })
            .cloned()
            .collect()
    }

    pub fn clear_events(&self) {
        let mut events = self.events.lock().unwrap();
        events.clear();
    }
}

impl Default for EventSystem {
    fn default() -> Self {
        Self::new(1000) // Default to storing 1000 events
    }
}
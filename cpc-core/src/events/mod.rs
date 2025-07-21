use std::collections::{BTreeMap, BinaryHeap};
use std::sync::{Arc, Mutex};
use once_cell::sync::OnceCell;
use prost::Message;
use uuid::Uuid;
use crate::p2p::NetworkHandler;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum EventType {
    UIInteraction,
    GameStateUpdate,
    NetworkCommand,
}

use crate::p2p::reconciliation::HybridTimestamp;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct P2PEvent {
    pub event_id: String,
    pub timestamp: HybridTimestamp,
    pub event_type: EventType,
    pub source_device: String,
    pub payload: Vec<u8>,
    pub vector_clock: BTreeMap<String, u64>,
    pub conflict_flag: bool,
}

impl P2PEvent {
    pub fn from_ui_event(event: UIEvent) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
            
        P2PEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp: (now, 0), // (wall time, logical time)
            event_type: EventType::UIInteraction,
            source_device: "android".to_string(),
            payload: serde_json::to_vec(&event).unwrap(),
            vector_clock: BTreeMap::new(),
            conflict_flag: false,
        }
    }
    
    pub fn mark_conflict(&mut self) {
        self.conflict_flag = true;
    }
}

impl Ord for P2PEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Prioritize UI interactions highest
        match (&self.event_type, &other.event_type) {
            (EventType::UIInteraction, _) => std::cmp::Ordering::Greater,
            (_, EventType::UIInteraction) => std::cmp::Ordering::Less,
            _ => self.timestamp.cmp(&other.timestamp),
        }
    }
}

impl PartialOrd for P2PEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct EventSystem {
    vector_clock: BTreeMap<String, u64>,
    event_queue: BinaryHeap<P2PEvent>,
    network_handler: Arc<NetworkHandler>,
}

// Global singleton instance
static EVENT_SYSTEM_INSTANCE: OnceCell<Arc<Mutex<EventSystem>>> = OnceCell::new();

impl EventSystem {
    pub fn get_instance(network_handler: Arc<NetworkHandler>) -> Arc<Mutex<Self>> {
        EVENT_SYSTEM_INSTANCE.get_or_init(|| {
            Arc::new(Mutex::new(EventSystem::new(network_handler)))
        }).clone()
    }

    fn new(network_handler: Arc<NetworkHandler>) -> Self {
        EventSystem {
            vector_clock: BTreeMap::new(),
            event_queue: BinaryHeap::new(),
            network_handler,
        }
    }

    pub fn handle_incoming_event(&mut self, event: P2PEvent) {
        // Apply vector clock logic
        self.vector_clock.entry(event.source_device.clone())
            .and_modify(|e| *e = (*e).max(event.vector_clock[&event.source_device]))
            .or_insert(event.vector_clock[&event.source_device]);
        
        // Add to prioritized queue
        self.event_queue.push(event);
    }

    pub fn broadcast_event(&self, event: P2PEvent) {
        // Prioritize based on event type
        let priority = match event.event_type {
            EventType::UIInteraction => 0,
            EventType::GameStateUpdate => 1,
            EventType::NetworkCommand => 2,
        };
        
        // Serialize with protobuf
        let mut buf = Vec::new();
        event.encode(&mut buf).unwrap();
        
        // Broadcast to peers using the unified NetworkHandler
        self.network_handler.broadcast_event(&buf, priority);
    }

    pub fn resolve_conflicts(&mut self) {
        // Conflict resolution logic would go here
        // For now, we'll just process events in order
        while let Some(event) = self.event_queue.pop() {
            // Process event
        }
    }
}

pub fn compress_event(event: &P2PEvent) -> Vec<u8> {
    // Simple compression - use in production would use a real compression library
    let mut compressed = Vec::new();
    compressed.extend_from_slice(&event.event_id.as_bytes());
    compressed.extend_from_slice(&event.timestamp.to_be_bytes());
    compressed
}

// UIEvent struct needed for Android bridge
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UIEvent {
    pub component: String,
    pub action: String,
    pub data: serde_json::Value,
}
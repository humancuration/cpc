use bevy::prelude::*;
use cpc_core::events::{P2PEvent, EventType};
use cpc_core::p2p::NetworkHandler;
use std::sync::{Arc, Mutex};
use std::collections::BTreeMap;
use uuid::Uuid;
use crate::editor_core::events::CursorMovedEvent;

#[derive(Event)]
pub struct EditorEvent {
    pub event_type: String,
    pub data: Vec<u8>,
}

#[derive(Event)]
pub struct EditorCommand {
    pub command_type: String,
    pub data: Vec<u8>,
}

pub struct NetworkEventBridge {
    pub network_handler: Arc<NetworkHandler>,
    pub vector_clock: BTreeMap<String, u64>,
    pub device_id: String,
}

impl NetworkEventBridge {
    pub fn new(network_handler: Arc<NetworkHandler>, device_id: &str) -> Self {
        let mut vector_clock = BTreeMap::new();
        vector_clock.insert(device_id.to_string(), 0);
        
        NetworkEventBridge {
            network_handler,
            vector_clock,
            device_id: device_id.to_string(),
        }
    }

    pub fn increment_vector_clock(&mut self) {
        let counter = self.vector_clock.entry(self.device_id.clone())
            .or_insert(0);
        *counter += 1;
    }
}

pub fn editor_event_to_p2p(
    mut editor_events: EventReader<EditorEvent>,
    mut bridge: ResMut<NetworkEventBridge>,
) {
    for event in editor_events.iter() {
        bridge.increment_vector_clock();
        
        let p2p_event = P2PEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            event_type: EventType::GameStateUpdate,
            source_device: bridge.device_id.clone(),
            payload: event.data.clone(),
            vector_clock: bridge.vector_clock.clone(),
            conflict_flag: false,
        };
        
        let serialized = serialize_event(&p2p_event);
        bridge.network_handler.broadcast_event(&serialized, 0);
    }
}

pub fn cursor_event_to_p2p(
    mut cursor_events: EventReader<CursorMovedEvent>,
    bridge: ResMut<NetworkEventBridge>,
) {
    for event in cursor_events.iter() {
        let payload = bincode::serialize(&event.position).unwrap();
        
        let p2p_event = P2PEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            event_type: EventType::CursorPosition,
            source_device: bridge.device_id.clone(),
            payload,
            vector_clock: BTreeMap::new(), // Cursor events don't affect state
            conflict_flag: false,
        };
        
        let serialized = serialize_event(&p2p_event);
        bridge.network_handler.broadcast_event(&serialized, 1); // High priority
    }
}

pub fn p2p_event_to_editor(
    mut bridge: ResMut<NetworkEventBridge>,
    mut editor_commands: EventWriter<EditorCommand>,
) {
    let mut network_handler = bridge.network_handler.swarm.lock().unwrap();
    
    while let Some(event) = network_handler.next_event() {
        let p2p_event = deserialize_event(&event);
        
        // Handle conflict resolution using vector clocks
        let mut conflict = false;
        for (device, counter) in &p2p_event.vector_clock {
            let local_counter = bridge.vector_clock.get(device).unwrap_or(&0);
            if counter > local_counter {
                conflict = true;
                break;
            }
        }
        
        if !conflict {
            // Update vector clock
            for (device, counter) in p2p_event.vector_clock {
                bridge.vector_clock.entry(device)
                    .and_modify(|e| *e = (*e).max(counter))
                    .or_insert(counter);
            }
            
            // Convert to editor command
            editor_commands.send(EditorCommand {
                command_type: "APPLY_STATE".to_string(),
                data: p2p_event.payload,
            });
        } else {
            // Handle conflict - in production would use reconciliation protocol
            warn!("Conflict detected for event {}", p2p_event.event_id);
        }
    }
}

fn serialize_event(event: &P2PEvent) -> Vec<u8> {
    let mut buf = Vec::new();
    // Simple serialization - would use protobuf in production
    buf.extend_from_slice(event.event_id.as_bytes());
    buf.extend_from_slice(&event.timestamp.to_be_bytes());
    buf.extend_from_slice(&(event.payload.len() as u32).to_be_bytes());
    buf.extend_from_slice(&event.payload);
    buf
}

fn deserialize_event(data: &[u8]) -> P2PEvent {
    // Simple deserialization - would use protobuf in production
    let event_id = String::from_utf8(data[0..36].to_vec()).unwrap();
    let timestamp = u64::from_be_bytes(data[36..44].try_into().unwrap());
    let payload_len = u32::from_be_bytes(data[44..48].try_into().unwrap()) as usize;
    let payload = data[48..48+payload_len].to_vec();
    
    P2PEvent {
        event_id,
        timestamp,
        event_type: EventType::GameStateUpdate,
        source_device: "remote".to_string(),
        payload,
        vector_clock: BTreeMap::new(),
        conflict_flag: false,
    }
}
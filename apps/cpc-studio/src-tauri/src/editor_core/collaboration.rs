use bevy::prelude::*;
use cpc_core::p2p::NetworkHandler;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Component)]
pub struct Collaborator {
    pub peer_id: String,
    pub color: Color,
    pub cursor_position: Vec2,
    pub selection_range: Option<(Vec2, Vec2)>,
    pub has_conflict: bool,
}

#[derive(Resource)]
pub struct Collaborators {
    pub local_peer_id: String,
    pub collaborators: HashMap<String, Entity>,
    pub color_map: HashMap<String, Color>,
}

impl Collaborators {
    pub fn new(local_peer_id: &str) -> Self {
        Collaborators {
            local_peer_id: local_peer_id.to_string(),
            collaborators: HashMap::new(),
            color_map: HashMap::new(),
        }
    }

    pub fn get_or_create_color(&mut self, peer_id: &str) -> Color {
        if let Some(color) = self.color_map.get(peer_id) {
            return *color;
        }

        // Generate distinct color based on peer ID hash
        let hash = peer_id.chars().fold(0, |acc, c| acc * 31 + c as u32);
        let r = ((hash >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hash >> 8) & 0xFF) as f32 / 255.0;
        let b = (hash & 0xFF) as f32 / 255.0;
        
        let color = Color::rgb(r, g, b);
        self.color_map.insert(peer_id.to_string(), color);
        color
    }
}

pub fn add_collaborator(
    mut commands: Commands,
    mut collaborators: ResMut<Collaborators>,
    peer_id: String,
) {
    if peer_id == collaborators.local_peer_id {
        return; // Skip local peer
    }

    let color = collaborators.get_or_create_color(&peer_id);
    let entity = commands.spawn((
        Collaborator {
            peer_id: peer_id.clone(),
            color,
            cursor_position: Vec2::ZERO,
            selection_range: None,
            has_conflict: false,
        },
        Name::new(format!("Collaborator: {}", peer_id)),
    )).id();

    collaborators.collaborators.insert(peer_id, entity);
}

pub fn remove_collaborator(
    mut commands: Commands,
    mut collaborators: ResMut<Collaborators>,
    peer_id: String,
) {
    if let Some(entity) = collaborators.collaborators.remove(&peer_id) {
        commands.entity(entity).despawn();
    }
}

pub fn update_collaborator_position(
    mut collaborators: ResMut<Collaborators>,
    mut cursor_events: EventReader<CursorMovedEvent>,
    network_handler: Res<Arc<NetworkHandler>>,
) {
    for event in cursor_events.iter() {
        // Update local collaborator position
        if let Some(entity) = collaborators.collaborators.get(&collaborators.local_peer_id) {
            if let Some(mut collaborator) = collaborators.get_mut(*entity) {
                collaborator.cursor_position = event.position;
            }
        }

        // Broadcast cursor position to peers
        let payload = bincode::serialize(&event.position).unwrap();
        network_handler.broadcast_event(
            &P2PEvent::new(
                Uuid::new_v4().to_string(),
                EventType::CursorPosition,
                collaborators.local_peer_id.clone(),
                payload,
            ),
            1, // High priority for cursor events
        );
    }
}

pub fn handle_remote_cursor_position(
    mut collaborators: ResMut<Collaborators>,
    mut network_events: EventReader<P2PEvent>,
) {
    for event in network_events.iter() {
        if event.event_type != EventType::CursorPosition {
            continue;
        }

        if let Ok(position) = bincode::deserialize::<Vec2>(&event.payload) {
            if let Some(entity) = collaborators.collaborators.get(&event.source_device) {
                if let Some(mut collaborator) = collaborators.get_mut(*entity) {
                    collaborator.cursor_position = position;
                }
            }
        }
    }
}
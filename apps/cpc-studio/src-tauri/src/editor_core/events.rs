use bevy::prelude::*;
use super::assets::{AssetMetadata, AssetType};
use crate::cpc_core::VectorClock;

#[derive(Event, Debug, Serialize, Deserialize)]
pub enum AssetEvent {
    Created {
        metadata: AssetMetadata,
        clock: VectorClock,
    },
    MetadataUpdated {
        asset_id: uuid::Uuid,
        delta: serde_json::Value,
        clock: VectorClock,
    },
    ContentUpdated {
        asset_id: uuid::Uuid,
        content_hash: String,
        clock: VectorClock,
    },
    Deleted {
        asset_id: uuid::Uuid,
        clock: VectorClock,
    },
}

#[derive(Event)]
pub struct EditorCommand {
    pub command_type: String,
    pub data: Vec<u8>,
}

#[derive(Event)]
pub struct CursorMovedEvent {
    pub position: Vec2,
}

// Preserve existing EditorEvent for non-asset related events
#[derive(Event)]
pub struct EditorEvent {
    pub event_type: String,
    pub data: Vec<u8>,
}
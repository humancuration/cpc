/*!
# Asset Browser Module

Defines core data structures and types for the Asset Browser feature.
*/

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Unique identifier for an asset
pub type AssetId = uuid::Uuid;

/// Represents metadata for a single asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMetadata {
    pub id: AssetId,
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub asset_type: AssetType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub thumbnail_path: Option<PathBuf>,
}

/// Types of assets supported by the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    Image,
    Video,
    Audio,
    Document,
    Model3D,
    Other,
}

/// Represents a folder node in the asset browser tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderNode {
    pub path: PathBuf,
    pub name: String,
    pub children: Vec<FolderNode>,
}

/// Preview data for assets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PreviewData {
    Image { path: PathBuf, width: u32, height: u32 },
    Video { thumbnail_path: PathBuf, duration: f32 },
    Audio { waveform_path: PathBuf },
    Document { page_count: u32 },
    Unavailable,
}

/// View modes for the asset browser UI
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ViewMode {
    Grid,
    List,
    Gallery,
}

/// Event payload for asset creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetCreatedEvent {
    pub asset_id: AssetId,
    pub metadata: AssetMetadata,
}

/// Event payload for asset update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetUpdatedEvent {
    pub asset_id: AssetId,
    pub metadata: AssetMetadata,
}

/// Event payload for asset deletion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetDeletedEvent {
    pub asset_id: AssetId,
}

/// Event payload for asset conflicts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetConflictEvent {
    pub asset_id: AssetId,
    pub reason: String,
}
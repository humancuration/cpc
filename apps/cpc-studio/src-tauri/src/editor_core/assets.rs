use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use crate::cpc_core::{PeerId, VectorClock};

/// Core asset metadata structure with CRDT capabilities
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetMetadata {
    pub asset_id: Uuid,
    pub name: String,
    pub path: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_path: Option<PathBuf>,
    pub asset_type: AssetType,
    pub version: u64,
    pub vector_clock: BTreeMap<PeerId, u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_info: Option<LockInfo>,
    #[serde(flatten)]
    pub type_specific: TypeSpecificMetadata,
}

/// Categorization of asset types with type-specific properties
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AssetType {
    Texture,
    Model,
    Audio,
    Script,
    Prefab,
}

impl AssetType {
    /// Returns true if this asset type supports thumbnail generation
    pub fn supports_thumbnail(&self) -> bool {
        match self {
            AssetType::Texture => true,
            AssetType::Model => true,
            _ => false,
        }
    }
}

/// Type-specific metadata properties using flattened enum
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum TypeSpecificMetadata {
    Texture {
        format: String,
        srgb: bool,
        generate_mips: bool,
        compression: TextureCompression,
    },
    Model {
        import_materials: bool,
        lod_levels: u8,
        collision_type: CollisionType,
    },
    Audio {
        streaming: bool,
        quality: AudioQuality,
    },
    Script {
        entry_point: String,
    },
    Prefab {
        component_count: usize,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TextureCompression {
    None,
    Bc7,
    Etc2,
    Astc,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CollisionType {
    None,
    ConvexHull,
    Mesh,
    Primitive,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AudioQuality {
    Low,
    Medium,
    High,
}

/// Information about an asset lock
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LockInfo {
    pub user_id: Uuid,
    pub user_name: String,
}

/// Error type for asset operations
#[derive(Debug, thiserror::Error)]
pub enum AssetError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("UUID parse error: {0}")]
    Uuid(#[from] uuid::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Asset not found: {0}")]
    NotFound(Uuid),
    #[error("Network event failed: {0}")]
    NetworkEventFailed(String),
    #[error("Asset is locked by {0}")]
    Locked(String),
}
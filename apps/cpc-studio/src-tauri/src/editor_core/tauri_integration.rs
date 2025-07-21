use crate::editor_core::{
    assets::{AssetError, AssetMetadata, AssetType, TypeSpecificMetadata, TextureCompression},
    state::{EditorState, SharedEditorState},
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;
use std::path::PathBuf;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct EditorCore {
    pub bevy_app: App,
}

impl EditorCore {
    pub fn new() -> Self {
        let mut bevy_app = App::new();
        bevy_app.add_plugins(MinimalPlugins);
        EditorCore { bevy_app }
    }
}

/// Unified error handling for Tauri commands
#[derive(Debug, thiserror::Error, Serialize)]
pub enum CommandError {
    #[error("Asset error: {0}")]
    Asset(#[from] AssetError),
    #[error("State lock error")]
    StateLock,
    #[error("Serialization error: {0}")]
    Serialization(String),
}

impl From<serde_json::Error> for CommandError {
    fn from(e: serde_json::Error) -> Self {
        CommandError::Serialization(e.to_string())
    }
}

#[tauri::command]
#[tauri::command]
pub async fn import_asset(
    state: State<'_, SharedEditorState>,
    path: String,
) -> Result<Uuid, CommandError> {
    let path_buf = PathBuf::from(&path);
    let asset_type = detect_asset_type(&path);
    let mut metadata = AssetMetadata {
        asset_id: Uuid::new_v4(),
        name: path_buf.file_name().unwrap().to_string_lossy().to_string(),
        path: path_buf.clone(),
        thumbnail_path: None,
        asset_type,
        version: 1,
        vector_clock: BTreeMap::new(), // TODO: Initialize with local peer
        type_specific: TypeSpecificMetadata::default_for_type(asset_type),
    };

    // Generate thumbnail for supported asset types
    if asset_type.supports_thumbnail() {
        match asset_processor::generate_thumbnail(&metadata).await {
            Ok(thumb_path) => {
                println!("Thumbnail generated at: {:?}", thumb_path);
                metadata.thumbnail_path = Some(thumb_path);
            }
            Err(e) => {
                eprintln!("Failed to generate thumbnail for asset {}: {}", metadata.asset_id, e);
                // Fallback to default icon
                metadata.thumbnail_path = Some(PathBuf::from("assets/default_icon.png"));
            }
        }
    }

    // Lock state only when needed
    let mut state = state.lock().map_err(|_| CommandError::StateLock)?;
    
    // TODO: Send to reconciliation engine
    // state.reconciliation_engine.handle_event(...);
    
    // TEMPORARY: Print metadata for verification
    println!("Imported asset metadata: {:?}", metadata);
    
    Ok(metadata.asset_id)
}

#[tauri::command]
pub fn get_asset_metadata(
    state: State<'_, SharedEditorState>,
    asset_id: Uuid,
) -> Result<AssetMetadata, CommandError> {
    let state = state.lock().map_err(|_| CommandError::StateLock)?;
    
    // TODO: Query reconciliation engine
    Err(AssetError::NotFound(asset_id).into())
}

// ===== Scene Hierarchy Commands =====

use crate::editor_core::scene::{Entity, SceneHierarchy};
use crate::editor_core::scene::entity_sync::EntitySync;

#[derive(Serialize)]
pub struct SceneHierarchyDTO {
    entities: HashMap<Uuid, EntityDTO>,
    parent_child_pairs: Vec<(Uuid, Uuid)>,
}

#[derive(Serialize)]
pub struct EntityDTO {
    id: Uuid,
    name: String,
    transform: TransformDTO,
}

#[derive(Serialize)]
pub struct TransformDTO {
    translation: [f32; 3],
    rotation: [f32; 4],
    scale: [f32; 3],
}

#[tauri::command]
pub fn get_scene_hierarchy(
    state: State<'_, SharedEditorState>,
) -> Result<SceneHierarchyDTO, CommandError> {
    let state = state.lock().map_err(|_| CommandError::StateLock)?;
    let hierarchy = state.bevy_app.world.get_resource::<SceneHierarchy>()
        .ok_or_else(|| CommandError::Serialization("SceneHierarchy resource not found".to_string()))?;

    let mut entities = HashMap::new();
    let mut parent_child_pairs = Vec::new();

    // Convert entities to DTOs
    for (id, entity) in &hierarchy.entities {
        entities.insert(*id, EntityDTO {
            id: *id,
            name: entity.name.clone(),
            transform: TransformDTO {
                translation: entity.transform.translation.into(),
                rotation: entity.transform.rotation.into(),
                scale: entity.transform.scale.into(),
            },
        });
    }

    // Collect parent-child relationships
    for (parent_id, children) in &hierarchy.children {
        for child_id in children {
            parent_child_pairs.push((*parent_id, *child_id));
        }
    }

    Ok(SceneHierarchyDTO { entities, parent_child_pairs })
}

#[tauri::command]
pub fn create_entity(
    state: State<'_, SharedEditorState>,
    parent: Option<Uuid>,
) -> Result<Uuid, CommandError> {
    let mut state = state.lock().map_err(|_| CommandError::StateLock)?;
    let hierarchy = state.bevy_app.world.get_resource_mut::<SceneHierarchy>()
        .ok_or_else(|| CommandError::Serialization("SceneHierarchy resource not found".to_string()))?;
    
    let entity_sync = state.bevy_app.world.get_resource_mut::<EntitySync>()
        .ok_or_else(|| CommandError::Serialization("EntitySync resource not found".to_string()))?;

    let entity_id = Uuid::new_v4();
    let entity = Entity {
        id: entity_id,
        name: format!("Entity {}", entity_id),
        parent,
        transform: Default::default(),
    };

    // Add entity to hierarchy
    hierarchy.add_entity(entity);

    // Automatically lock the new entity for the current user
    // TODO: Get current user ID and name from session
    let user_id = Uuid::new_v4();
    let user_name = "Current User".to_string();
    entity_sync.acquire_lock(entity_id, user_id, user_name)?;

    Ok(entity_id)
}

#[tauri::command]
pub fn delete_entities(
    state: State<'_, SharedEditorState>,
    ids: Vec<Uuid>,
) -> Result<(), CommandError> {
    let mut state = state.lock().map_err(|_| CommandError::StateLock)?;
    let hierarchy = state.bevy_app.world.get_resource_mut::<SceneHierarchy>()
        .ok_or_else(|| CommandError::Serialization("SceneHierarchy resource not found".to_string()))?;
    
    let entity_sync = state.bevy_app.world.get_resource_mut::<EntitySync>()
        .ok_or_else(|| CommandError::Serialization("EntitySync resource not found".to_string()))?;

    for id in ids {
        // Check lock before deletion
        if let Some(lock_info) = entity_sync.get_lock_info(id) {
            return Err(CommandError::Asset(AssetError::Locked(lock_info.user_name.clone())));
        }
        
        hierarchy.remove_entity(id);
    }

    Ok(())
}

#[tauri::command]
pub fn reparent_entities(
    state: State<'_, SharedEditorState>,
    entity_map: HashMap<Uuid, Option<Uuid>>,
) -> Result<(), CommandError> {
    let mut state = state.lock().map_err(|_| CommandError::StateLock)?;
    let hierarchy = state.bevy_app.world.get_resource_mut::<SceneHierarchy>()
        .ok_or_else(|| CommandError::Serialization("SceneHierarchy resource not found".to_string()))?;
    
    let entity_sync = state.bevy_app.world.get_resource_mut::<EntitySync>()
        .ok_or_else(|| CommandError::Serialization("EntitySync resource not found".to_string()))?;

    for (entity_id, new_parent) in entity_map {
        // Check lock before reparenting
        if let Some(lock_info) = entity_sync.get_lock_info(entity_id) {
            return Err(CommandError::Asset(AssetError::Locked(lock_info.user_name.clone())));
        }

        if let Some(parent_id) = new_parent {
            hierarchy.add_child(parent_id, entity_id);
        } else {
            // If new_parent is None, remove from current parent
            if let Some(entity) = hierarchy.get_entity_mut(&entity_id) {
                entity.parent = None;
            }
        }
    }

    Ok(())
}

// Simple asset type detection based on file extension
fn detect_asset_type(path: &str) -> AssetType {
    if path.ends_with(".png") || path.ends_with(".jpg") || path.ends_with(".jpeg") {
        AssetType::Texture
    } else if path.ends_with(".glb") || path.ends_with(".gltf") {
        AssetType::Model
    } else if path.ends_with(".wav") || path.ends_with(".mp3") {
        AssetType::Audio
    } else if path.ends_with(".rs") || path.ends_with(".js") {
        AssetType::Script
    } else if path.ends_with(".prefab") {
        AssetType::Prefab
    } else {
        AssetType::Texture // Default to texture
    }
}

impl TypeSpecificMetadata {
    pub fn default_for_type(asset_type: AssetType) -> Self {
        match asset_type {
            AssetType::Texture => TypeSpecificMetadata::Texture {
                format: "png".to_string(),
                srgb: true,
                generate_mips: true,
                compression: TextureCompression::None,
            },
            AssetType::Model => TypeSpecificMetadata::Model {
                import_materials: true,
                lod_levels: 1,
                collision_type: CollisionType::None,
            },
            AssetType::Audio => TypeSpecificMetadata::Audio {
                streaming: false,
                quality: AudioQuality::Medium,
            },
            AssetType::Script => TypeSpecificMetadata::Script {
                entry_point: "main".to_string(),
            },
            AssetType::Prefab => TypeSpecificMetadata::Prefab {
                component_count: 0,
            },
        }
    }
}

#[tauri::command]
pub fn update_asset_metadata(
    state: State<'_, SharedEditorState>,
    asset_id: Uuid,
    delta: serde_json::Value,
) -> Result<(), CommandError> {
    let mut state = state.lock().map_err(|_| CommandError::StateLock)?;
    
    // TODO: Implement metadata patching and vector clock update
    // let mut metadata = state.get_asset(asset_id)?;
    // apply_patch(&mut metadata, delta);
    // metadata.vector_clock.increment(local_peer_id);
    // send_to_reconciliation_engine(...);
    
    Ok(())
}

#[tauri::command]
pub fn get_assets_in_path(
    state: State<'_, SharedEditorState>,
    path: String,
) -> Result<Vec<AssetMetadata>, CommandError> {
    let state = state.lock().map_err(|_| CommandError::StateLock)?;
    
    // TODO: Implement proper path-based query from asset storage
    // For now, return all assets with matching path prefix
    let mut assets = Vec::new();
    // Placeholder - in real implementation this would query the asset store
    assets.push(AssetMetadata {
        asset_id: Uuid::new_v4(),
        name: "example".to_string(),
        path: PathBuf::from(&path),
        thumbnail_path: Some(PathBuf::from("assets/thumbnails/example.png")),
        asset_type: AssetType::Texture,
        version: 1,
        vector_clock: BTreeMap::new(),
        type_specific: TypeSpecificMetadata::Texture {
            format: "png".to_string(),
            srgb: true,
            generate_mips: true,
            compression: TextureCompression::None,
        },
    });
    
    Ok(assets)
}

// Existing project load/save commands remain unchanged
// ...

/// Updated asset folder structure with UUID-based identifiers
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetFolder {
    pub uuid: Uuid,
    pub name: String,
    pub path: PathBuf,
    pub children: Vec<AssetFolder>,
}
use tauri::State;
use std::sync::{Arc, Mutex};
use crate::editor_core::scene::manager::SceneManager;
use crate::editor_core::scene::command::{UpdateComponentCommand, AddComponentCommand, RemoveComponentCommand};
use uuid::Uuid;
use serde_json::Value;

#[tauri::command]
pub fn update_component(
    entity_id: Uuid,
    component_type_name: String,
    new_value: Value,
    scene_manager_state: State<Arc<Mutex<SceneManager>>>
) -> Result<(), String> {
    let mut scene_manager = scene_manager_state.lock().unwrap();
    
    // Get the old value for undo operation
    let old_value = scene_manager.get_entity(entity_id)
        .and_then(|e| e.components.get(&component_type_name).cloned())
        .unwrap_or(Value::Null);

    let command = Box::new(UpdateComponentCommand {
        entity_id,
        component_type_name,
        old_value,
        new_value,
    });

    scene_manager.execute_command(command);
    Ok(())
}

#[tauri::command]
pub fn add_component(
    entity_id: Uuid,
    component_type_name: String,
    component_data: Value,
    scene_manager_state: State<Arc<Mutex<SceneManager>>>
) -> Result<(), String> {
    let mut scene_manager = scene_manager_state.lock().unwrap();
    
    let command = Box::new(AddComponentCommand {
        entity_id,
        component_type_name,
        component_data,
    });

    scene_manager.execute_command(command);
    Ok(())
}

#[tauri::command]
pub fn remove_component(
    entity_id: Uuid,
    component_type_name: String,
    scene_manager_state: State<Arc<Mutex<SceneManager>>>
) -> Result<(), String> {
    let mut scene_manager = scene_manager_state.lock().unwrap();
    
    // Get current value for undo operation
    let current_value = scene_manager.get_entity(entity_id)
        .and_then(|e| e.components.get(&component_type_name).cloned())
        .unwrap_or(Value::Null);

    let command = Box::new(RemoveComponentCommand {
        entity_id,
        component_type_name,
        removed_component_data: current_value,
    });

    scene_manager.execute_command(command);
    Ok(())
}

#[tauri::command]
pub fn undo(scene_manager_state: State<Arc<Mutex<SceneManager>>>) -> Result<(), String> {
    scene_manager_state.lock().unwrap().undo();
    Ok(())
}

#[tauri::command]
pub fn redo(scene_manager_state: State<Arc<Mutex<SceneManager>>>) -> Result<(), String> {
    scene_manager_state.lock().unwrap().redo();
    Ok(())
}

#[tauri::command]
pub fn create_entity(
    parent_id: Option<Uuid>,
    scene_manager_state: State<Arc<Mutex<SceneManager>>>
) -> Result<Uuid, String> {
    let mut scene_manager = scene_manager_state.lock().unwrap();
    let entity_id = Uuid::new_v7();
    
    let command = Box::new(CreateEntityCommand {
        entity_id,
        parent_id,
        initial_components: ComponentStorage::new(),
    });
    
    scene_manager.execute_command(command);
    Ok(entity_id)
}

#[tauri::command]
pub fn delete_entities(
    entity_ids: Vec<Uuid>,
    scene_manager_state: State<Arc<Mutex<SceneManager>>>
) -> Result<(), String> {
    let mut scene_manager = scene_manager_state.lock().unwrap();
    
    // Capture current state for undo
    let deleted_entities: Vec<_> = entity_ids.iter()
        .filter_map(|id| scene_manager.hierarchy.entities.get(id).cloned())
        .collect();
        
    let parent_child_pairs: Vec<_> = scene_manager.hierarchy.parent_child_pairs
        .iter()
        .filter(|(p, c)| entity_ids.contains(p) || entity_ids.contains(c))
        .cloned()
        .collect();

    let command = Box::new(DeleteEntitiesCommand {
        entities: entity_ids.clone(),
        deleted_entities,
        parent_child_pairs,
    });
    
    scene_manager.execute_command(command);
    Ok(())
}

#[tauri::command]
pub fn reparent_entities(
    entity_ids: Vec<Uuid>,
    new_parent_id: Option<Uuid>,
    scene_manager_state: State<Arc<Mutex<SceneManager>>>
) -> Result<(), String> {
    let mut scene_manager = scene_manager_state.lock().unwrap();
    
    // Capture current parent states for undo
    let old_parents: Vec<_> = entity_ids.iter()
        .map(|id| scene_manager.hierarchy.entities.get(id).and_then(|e| e.parent))
        .collect();

    let command = Box::new(ReparentEntitiesCommand {
        entities: entity_ids,
        new_parent_id,
        old_parents,
    });
    
    scene_manager.execute_command(command);
    Ok(())
}
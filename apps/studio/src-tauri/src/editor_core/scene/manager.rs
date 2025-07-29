use std::collections::{HashMap, VecDeque};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::editor_core::scene::entity::Entity;
use crate::editor_core::scene::command::Command;
use crate::editor_core::scene::hierarchy::HierarchyGraph;
use serde::Serialize;
use tauri::AppHandle;

#[derive(Serialize)]
pub struct CommandEventPayload {
    pub command_type: String,
    pub entity_id: Option<Uuid>,
    pub component_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_data: Option<serde_json::Value>,
    pub parent_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    pub version_vector: HashMap<Uuid, u64>,
}

pub struct SceneManager {
    pub current_user: Uuid,
    pub session_id: Uuid,
    pub version_vector: HashMap<Uuid, u64>,
    pub entities: HashMap<Uuid, Entity>,
    pub hierarchy: HierarchyGraph,
    undo_stack: VecDeque<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
    #[allow(dead_code)]
    app_handle: Option<AppHandle>, // For event emission
}

impl SceneManager {
    pub fn new(current_user: Uuid, session_id: Uuid) -> Self {
        SceneManager {
            entities: HashMap::new(),
            hierarchy: HierarchyGraph::new(),
            undo_stack: VecDeque::new(),
            redo_stack: Vec::new(),
            app_handle: None,
            current_user,
            session_id,
            version_vector: HashMap::new(),
        }
    }

    pub fn set_app_handle(&mut self, app_handle: AppHandle) {
        self.app_handle = Some(app_handle);
    }

    pub fn increment_version(&mut self, entity_id: Uuid) {
        let version = self.version_vector.entry(entity_id).or_insert(0);
        *version += 1;
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id, entity);
    }

    pub fn get_entity(&self, id: Uuid) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_entity_mut(&mut self, id: Uuid) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    pub fn remove_entity(&mut self, id: Uuid) -> Option<Entity> {
        self.entities.remove(&id)
    }

    pub fn execute_command(&mut self, command: Box<dyn Command>) -> Result<(), String> {
        command.execute(self)?;
        
        // Emit detailed command event if app handle is available
        let result = command.execute(self);
        
        if let Some(app_handle) = &self.app_handle {
            let payload = CommandEventPayload {
                command_type: command.command_type().to_string(),
                entity_id: command.entity_id(),
                component_type: command.component_type().map(|c| c.to_string()),
                component_data: command.serialized_component_data(),
                parent_id: command.parent_id(),
                timestamp: Utc::now(),
                user_id: self.current_user,
                session_id: self.session_id,
                success: result.is_ok(),
                error_message: result.as_ref().err().map(|e| e.to_string()),
                version_vector: self.version_vector.clone(),
            };
            app_handle.emit_all("command-executed", &payload)
                .map_err(|e| format!("Failed to emit event: {}", e))?;
        }
        
        result?; // Propagate error if command failed
        self.undo_stack.push_back(command);
        self.redo_stack.clear();
        Ok(())
    }

    pub fn undo(&mut self) {
        if let Some(command) = self.undo_stack.pop_back() {
            command.undo(self);
            self.redo_stack.push_back(command);
        }
    }

    pub fn redo(&mut self) {
        if let Some(command) = self.redo_stack.pop_back() {
            command.execute(self);
            self.undo_stack.push_back(command);
        }
    }
}
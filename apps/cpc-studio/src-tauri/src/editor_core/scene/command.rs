use crate::editor_core::scene::manager::SceneManager;
use serde::{Serialize, Deserialize};
use std::any::Any;
use uuid::Uuid;
use serde_json::Value;

use serde_json::Value;
use uuid::Uuid;

pub trait Command: Send + Sync {
    fn execute(&self, scene: &mut SceneManager);
    fn undo(&self, scene: &mut SceneManager);
    fn as_any(&self) -> &dyn Any;
    
    // New methods for command metadata
    fn command_type(&self) -> &str;
    fn entity_id(&self) -> Option<Uuid>;
    fn component_type(&self) -> Option<&str>;
    fn parent_id(&self) -> Option<Uuid>;
    fn serialized_component_data(&self) -> Option<Value>;
}

#[derive(Serialize, Deserialize)]
pub struct UpdateComponentCommand {
    entity_id: Uuid,
    component_type_name: String,
    old_value: Value,
    new_value: Value,
}

impl Command for UpdateComponentCommand {
    fn execute(&self, scene: &mut SceneManager) {
        if let Some(entity) = scene.get_entity_mut(self.entity_id) {
            entity.components.insert(
                self.component_type_name.clone(),
                self.new_value.clone(),
            );
        }
    }

    fn undo(&self, scene: &mut SceneManager) {
        if let Some(entity) = scene.get_entity_mut(self.entity_id) {
            entity.components.insert(
                self.component_type_name.clone(),
                self.old_value.clone(),
            );
        }
    }

    fn as_any(&self) -> &dyn Any { self }
    
    fn command_type(&self) -> &str { "UpdateComponent" }
    
    fn entity_id(&self) -> Option<Uuid> { Some(self.entity_id) }
    
    fn component_type(&self) -> Option<&str> { Some(&self.component_type_name) }
    
    fn parent_id(&self) -> Option<Uuid> { None }
    
    fn serialized_component_data(&self) -> Option<Value> { Some(self.new_value.clone()) }
}

#[derive(Serialize, Deserialize)]
pub struct AddComponentCommand {
    entity_id: Uuid,
    component_type_name: String,
    component_data: Value,
}

impl Command for AddComponentCommand {
    fn execute(&self, scene: &mut SceneManager) {
        if let Some(entity) = scene.get_entity_mut(self.entity_id) {
            entity.components.insert(
                self.component_type_name.clone(),
                self.component_data.clone(),
            );
        }
    }

    fn undo(&self, scene: &mut SceneManager) {
        if let Some(entity) = scene.get_entity_mut(self.entity_id) {
            entity.components.remove(&self.component_type_name);
        }
    }

    fn as_any(&self) -> &dyn Any { self }
    
    fn command_type(&self) -> &str { "AddComponent" }
    
    fn entity_id(&self) -> Option<Uuid> { Some(self.entity_id) }
    
    fn component_type(&self) -> Option<&str> { Some(&self.component_type_name) }
    
    fn parent_id(&self) -> Option<Uuid> { None }
    
    fn serialized_component_data(&self) -> Option<Value> { Some(self.component_data.clone()) }
}

#[derive(Serialize, Deserialize)]
pub struct RemoveComponentCommand {
    entity_id: Uuid,
    component_type_name: String,
    removed_component_data: Value,
}

impl Command for RemoveComponentCommand {
    fn execute(&self, scene: &mut SceneManager) {
        if let Some(entity) = scene.get_entity_mut(self.entity_id) {
            entity.components.remove(&self.component_type_name);
        }
    }

    fn undo(&self, scene: &mut SceneManager) {
        if let Some(entity) = scene.get_entity_mut(self.entity_id) {
            entity.components.insert(
                self.component_type_name.clone(),
                self.removed_component_data.clone(),
            );
        }
    }

    fn as_any(&self) -> &dyn Any { self }
    
    fn command_type(&self) -> &str { "RemoveComponent" }
    
    fn entity_id(&self) -> Option<Uuid> { Some(self.entity_id) }
    
    fn component_type(&self) -> Option<&str> { Some(&self.component_type_name) }
    
    fn parent_id(&self) -> Option<Uuid> { None }
    
    fn serialized_component_data(&self) -> Option<Value> { Some(self.removed_component_data.clone()) }
}

#[derive(Serialize, Deserialize)]
pub struct CreateEntityCommand {
    entity_id: Uuid,
    parent_id: Option<Uuid>,
    initial_components: ComponentStorage,
}

impl Command for CreateEntityCommand {
    pub fn execute(&self, scene: &mut SceneManager) {
        let entity = Entity {
            id: self.entity_id,
            transform: Transform::default(),
            components: self.initial_components.clone(),
        };
        
        // Add to entity storage
        scene.entities.insert(self.entity_id, entity);
        
        // Add to hierarchy
        scene.hierarchy.add_entity(self.entity_id, self.parent_id);
    }

    fn undo(&self, scene: &mut SceneManager) {
        // Remove from hierarchy first
        scene.hierarchy.remove_entity(self.entity_id);
        
        // Then remove from entity storage
        scene.entities.remove(&self.entity_id);
    }

    fn as_any(&self) -> &dyn Any { self }
    
    fn command_type(&self) -> &str { "CreateEntity" }
    
    fn entity_id(&self) -> Option<Uuid> { Some(self.entity_id) }
    
    fn component_type(&self) -> Option<&str> { None }
    
    fn parent_id(&self) -> Option<Uuid> { self.parent_id }
    
    fn serialized_component_data(&self) -> Option<Value> { None }
}

#[derive(Serialize, Deserialize)]
pub struct DeleteEntitiesCommand {
    entities: Vec<Uuid>,
    deleted_entities: Vec<Entity>,
    parent_child_pairs: Vec<(Uuid, Uuid)>,
}

impl Command for DeleteEntitiesCommand {
    fn execute(&self, scene: &mut SceneManager) {
        for entity_id in &self.entities {
            scene.hierarchy.entities.remove(entity_id);
        }
        
        // Remove all parent-child relationships involving the deleted entities
        scene.hierarchy.parent_child_pairs.retain(|(parent, child)| {
            !self.entities.contains(parent) && !self.entities.contains(child)
        });
    }

    fn undo(&self, scene: &mut SceneManager) {
        for entity in &self.deleted_entities {
            scene.hierarchy.entities.insert(entity.id, entity.clone());
        }
        
        for (parent, child) in &self.parent_child_pairs {
            scene.hierarchy.parent_child_pairs.push((*parent, *child));
        }
    }

    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Serialize, Deserialize)]
pub struct ReparentEntitiesCommand {
    entities: Vec<Uuid>,
    new_parent_id: Option<Uuid>,
    old_parents: Vec<Option<Uuid>>,
}

impl Command for ReparentEntitiesCommand {
    fn execute(&self, scene: &mut SceneManager) {
        for (i, entity_id) in self.entities.iter().enumerate() {
            if let Some(entity) = scene.hierarchy.entities.get_mut(entity_id) {
                // Remove from old parent
                if let Some(old_parent_id) = entity.parent {
                    if let Some(old_parent) = scene.hierarchy.entities.get_mut(&old_parent_id) {
                        old_parent.children.retain(|id| id != entity_id);
                    }
                }
                
                // Add to new parent
                entity.parent = self.new_parent_id;
                
                if let Some(new_parent_id) = self.new_parent_id {
                    if let Some(new_parent) = scene.hierarchy.entities.get_mut(&new_parent_id) {
                        new_parent.children.push(*entity_id);
                    }
                }
            }
        }
    }

    fn undo(&self, scene: &mut SceneManager) {
        for (i, entity_id) in self.entities.iter().enumerate() {
            if let Some(entity) = scene.hierarchy.entities.get_mut(entity_id) {
                // Remove from new parent
                if let Some(new_parent_id) = self.new_parent_id {
                    if let Some(new_parent) = scene.hierarchy.entities.get_mut(&new_parent_id) {
                        new_parent.children.retain(|id| id != entity_id);
                    }
                }
                
                // Add back to old parent
                entity.parent = self.old_parents[i];
                
                if let Some(old_parent_id) = self.old_parents[i] {
                    if let Some(old_parent) = scene.hierarchy.entities.get_mut(&old_parent_id) {
                        old_parent.children.push(*entity_id);
                    }
                }
            }
        }
    }

    fn as_any(&self) -> &dyn Any { self }
}
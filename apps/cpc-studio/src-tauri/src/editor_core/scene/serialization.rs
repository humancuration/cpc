use bevy::prelude::*;
use rmp_serde::{Serializer, Deserializer};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::{Entity, ComponentStorage, SceneHierarchy};
use crate::editor_core::scene::component::SerializableComponent;
use cpc_core::scene::{SceneData, EntityData, ComponentData};
use cpc_core::error::PublishError;
use tracing;

impl SceneHierarchy {
    /// Serialize the scene hierarchy to MessagePack bytes
    #[tracing::instrument(skip(self))]
    pub fn serialize(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        let scene_data = SceneData {
            entities: self.entities.values()
                .map(|entity| EntityData {
                    id: entity.id,
                    transform: entity.transform.compute_matrix().to_cols_array(),
                    components: entity.components.serialize_components(),
                    parent: entity.parent,
                })
                .collect(),
        };
        
        tracing::debug!("Serializing scene with {} entities", scene_data.entities.len());
        
        let mut buf = Vec::new();
        scene_data.serialize(&mut Serializer::new(&mut buf))?;
        Ok(buf)
    }

    /// Deserialize MessagePack bytes into a scene hierarchy
    #[tracing::instrument]
    pub fn deserialize(data: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
        let mut de = Deserializer::new(data);
        let scene_data: SceneData = Deserialize::deserialize(&mut de)?;
        
        tracing::debug!("Deserializing scene with {} entities", scene_data.entities.len());
        
        let mut hierarchy = SceneHierarchy::default();
        for entity_data in scene_data.entities {
            let mut entity = Entity::with_id(entity_data.id);
            entity.transform = Transform::from_matrix(Mat4::from_cols_array(&entity_data.transform));
            entity.parent = entity_data.parent;
            
            for comp_data in entity_data.components {
                if let Some(component) = ComponentStorage::deserialize_component(&comp_data.type_name, &comp_data.data) {
                    entity.components.insert_dyn(component);
                }
            }
            
            hierarchy.add_entity(entity);
        }
        
        Ok(hierarchy)
    }
}

impl ComponentStorage {
    /// Serialize all components into a vector of ComponentData
    #[tracing::instrument(skip(self))]
    pub fn serialize_components(&self) -> Vec<ComponentData> {
        // In a real implementation, we would iterate through all components
        // For now, we'll leave this as a placeholder
        // Actual implementation requires a component registry
        vec![]
    }

    /// Deserialize a component from its type name and data
    #[tracing::instrument]
    pub fn deserialize_component(type_name: &str, data: &[u8]) -> Option<Box<dyn SerializableComponent>> {
        // In a real implementation, we would match on type_name
        // and deserialize the appropriate component type
        // For now, return None
        None
    }
    
    /// Insert a dynamic serializable component
    pub fn insert_dyn(&mut self, component: Box<dyn SerializableComponent>) {
        let type_id = component.type_id();
        self.components.insert(type_id, component);
    }
}
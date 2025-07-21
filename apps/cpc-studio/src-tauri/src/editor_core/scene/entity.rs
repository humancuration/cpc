use bevy::prelude::*;
use uuid::Uuid;
use std::collections::HashMap;
use crate::editor_core::scene::component::ComponentStorage;

/// Represents an entity in the scene graph
#[derive(Debug, Clone)]
pub struct Entity {
    /// Unique identifier (UUIDv7)
    pub id: Uuid,
    /// Local transform relative to parent
    pub transform: Transform,
    /// Map of components attached to this entity
    pub components: ComponentStorage,
    /// Optional parent entity ID
    pub parent: Option<Uuid>,
    /// List of child entity IDs
    pub children: Vec<Uuid>,
}

impl Entity {
    /// Create a new entity with default transform and no components
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v7(),
            transform: Transform::default(),
            components: ComponentStorage::new(),
            parent: None,
            children: Vec::new(),
        }
    }

    /// Create a new entity with a specific ID (used for deserialization)
    pub fn with_id(id: Uuid) -> Self {
        Self {
            id,
            transform: Transform::default(),
            components: ComponentStorage::new(),
            parent: None,
            children: Vec::new(),
        }
    }

    /// Add a component to the entity
    pub fn add_component<T: Component + 'static>(&mut self, component: T) {
        self.components.insert(component);
    }

    /// Get a reference to a component
    pub fn get_component<T: Component + 'static>(&self) -> Option<&T> {
        self.components.get::<T>()
    }

    /// Get a mutable reference to a component
    pub fn get_component_mut<T: Component + 'static>(&mut self) -> Option<&mut T> {
        self.components.get_mut::<T>()
    }

    /// Remove a component from the entity
    pub fn remove_component<T: Component + 'static>(&mut self) -> Option<T> {
        self.components.remove::<T>()
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self::new()
    }
}

use serde::{Serialize, Deserialize};

/// Represents the full scene hierarchy
#[derive(Debug, Serialize, Deserialize)]
pub struct SceneHierarchy {
    /// Map of all entities by ID
    pub entities: HashMap<Uuid, Entity>,
    /// List of parent-child relationships
    pub parent_child_pairs: Vec<(Uuid, Uuid)>,
}
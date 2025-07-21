use bevy::prelude::*;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Storage for components attached to an entity
#[derive(Debug, Default, Clone)]
pub struct ComponentStorage {
    components: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ComponentStorage {
    /// Create a new empty ComponentStorage
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    /// Add a component to the storage
    pub fn insert<T: Component + 'static>(&mut self, component: T) {
        let type_id = TypeId::of::<T>();
        self.components.insert(type_id, Box::new(component));
    }

    /// Get a reference to a component
    pub fn get<T: Component + 'static>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.components.get(&type_id)
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    /// Get a mutable reference to a component
    pub fn get_mut<T: Component + 'static>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.components.get_mut(&type_id)
            .and_then(|boxed| boxed.downcast_mut::<T>())
    }

    /// Remove a component from the storage
    pub fn remove<T: Component + 'static>(&mut self) -> Option<T> {
        let type_id = TypeId::of::<T>();
        self.components.remove(&type_id)
            .and_then(|boxed| boxed.downcast::<T>().ok())
            .map(|boxed| *boxed)
    }

    /// Check if the storage contains a specific component type
    pub fn contains<T: Component + 'static>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        self.components.contains_key(&type_id)
    }
}

/// Trait for serializable components
pub trait SerializableComponent: Component + Serialize + DeserializeOwned + 'static {}
impl<T: Component + Serialize + DeserializeOwned + 'static> SerializableComponent for T {}
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Represents a serializable scene
#[derive(Serialize, Deserialize)]
pub struct SceneData {
    pub entities: Vec<EntityData>,
}

/// Serializable representation of an entity
#[derive(Serialize, Deserialize)]
pub struct EntityData {
    pub id: Uuid,
    pub transform: [f32; 16], // Mat4 representation
    pub components: Vec<ComponentData>,
    pub parent: Option<Uuid>,
}

/// Serializable representation of a component
#[derive(Serialize, Deserialize)]
pub struct ComponentData {
    pub type_name: String,
    pub data: Vec<u8>, // Serialized component data
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PortType {
    Any,
    Number,
    String,
    Boolean,
    Object,
    Array,
    Custom(String),
}

impl PortType {
    pub fn are_compatible(from: &PortType, to: &PortType) -> bool {
        match (from, to) {
            (PortType::Any, _) | (_, PortType::Any) => true,
            (PortType::Number, PortType::Number) => true,
            (PortType::String, PortType::String) => true,
            (PortType::Boolean, PortType::Boolean) => true,
            (PortType::Object, PortType::Object) => true,
            (PortType::Array, PortType::Array) => true,
            (PortType::Custom(from_type), PortType::Custom(to_type)) => from_type == to_type,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PortDirection {
    Input,
    Output,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Port {
    pub id: String,
    pub name: String,
    pub port_type: PortType,
    pub direction: PortDirection,
}

impl Port {
    pub fn default_input() -> Self {
        Self {
            id: "default_input".to_string(),
            name: "input".to_string(),
            port_type: PortType::Any,
            direction: PortDirection::Input,
        }
    }
    
    pub fn default_output() -> Self {
        Self {
            id: "default_output".to_string(),
            name: "output".to_string(),
            port_type: PortType::Any,
            direction: PortDirection::Output,
        }
    }
}
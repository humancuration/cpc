//! Port specifications with type checking for Shtairir
//! 
//! This module defines the input and output port specifications used by blocks,
//! including type checking and validation capabilities.

use shtairir_registry::types::Type;
use shtairir_registry::value::Value;
use std::collections::HashMap;

/// Unique identifier for a port
pub type PortId = String;

/// Input port for a node
#[derive(Debug, Clone)]
pub struct InputPort {
    /// Unique identifier for the port
    pub id: PortId,
    
    /// Port name
    pub name: String,
    
    /// Port type
    pub ty: Type,
    
    /// Port kind (value, stream, event)
    pub kind: PortKind,
    
    /// Default value
    pub default: Option<Value>,
    
    /// Whether the port is required
    pub required: bool,
    
    /// Port description
    pub description: Option<String>,
    
    /// Validation rules
    pub validation: Vec<ValidationRule>,
}

impl InputPort {
    /// Create a new input port
    pub fn new(id: PortId, name: String, ty: Type, kind: PortKind) -> Self {
        Self {
            id,
            name,
            ty,
            kind,
            default: None,
            required: true,
            description: None,
            validation: vec![],
        }
    }
    
    /// Set default value
    pub fn with_default(mut self, default: Value) -> Self {
        self.default = Some(default);
        self
    }
    
    /// Set required flag
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
    
    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    
    /// Add validation rule
    pub fn with_validation(mut self, rule: ValidationRule) -> Self {
        self.validation.push(rule);
        self
    }
    
    /// Validate a value against this port's type and validation rules
    pub fn validate_value(&self, value: &Value) -> Result<(), ValidationError> {
        // TODO: Implement type checking against the registry type system
        // For now, we'll just check the validation rules
        
        for rule in &self.validation {
            if !rule.validate(value) {
                return Err(ValidationError::new(format!(
                    "Value failed validation rule: {}",
                    rule.description.as_deref().unwrap_or("unnamed rule")
                )));
            }
        }
        
        Ok(())
    }
}

/// Output port for a node
#[derive(Debug, Clone)]
pub struct OutputPort {
    /// Unique identifier for the port
    pub id: PortId,
    
    /// Port name
    pub name: String,
    
    /// Port type
    pub ty: Type,
    
    /// Port kind (value, stream, event)
    pub kind: PortKind,
    
    /// Port description
    pub description: Option<String>,
}

impl OutputPort {
    /// Create a new output port
    pub fn new(id: PortId, name: String, ty: Type, kind: PortKind) -> Self {
        Self {
            id,
            name,
            ty,
            kind,
            description: None,
        }
    }
    
    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}

/// Port kind
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortKind {
    /// Value port (synchronous data)
    Value,
    /// Stream port (asynchronous data stream)
    Stream,
    /// Event port (event notifications)
    Event,
    /// Composite port (structured data)
    Composite,
}

/// Validation rule for port values
#[derive(Debug, Clone)]
pub struct ValidationRule {
    /// Rule description
    pub description: Option<String>,
    
    /// Validation function
    pub validator: Box<dyn Fn(&Value) -> bool + Send + Sync>,
}

impl ValidationRule {
    /// Create a new validation rule
    pub fn new<F>(validator: F) -> Self
    where
        F: Fn(&Value) -> bool + Send + Sync + 'static,
    {
        Self {
            description: None,
            validator: Box::new(validator),
        }
    }
    
    /// Create a new validation rule with description
    pub fn with_description<F>(validator: F, description: String) -> Self
    where
        F: Fn(&Value) -> bool + Send + Sync + 'static,
    {
        Self {
            description: Some(description),
            validator: Box::new(validator),
        }
    }
    
    /// Validate a value
    pub fn validate(&self, value: &Value) -> bool {
        (self.validator)(value)
    }
}

/// Validation error
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Error message
    pub message: String,
    
    /// Error details
    pub details: Option<Value>,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(message: String) -> Self {
        Self {
            message,
            details: None,
        }
    }
    
    /// Create a new validation error with details
    pub fn with_details(mut self, details: Value) -> Self {
        self.details = Some(details);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shtairir_registry::types::ScalarType;
    
    #[test]
    fn test_input_port_creation() {
        let port = InputPort::new(
            "input1".to_string(),
            "first_input".to_string(),
            Type::Scalar(ScalarType::I64),
            PortKind::Value,
        )
        .with_default(Value::i64(0))
        .with_required(false)
        .with_description("First input parameter".to_string());
        
        assert_eq!(port.id, "input1");
        assert_eq!(port.name, "first_input");
        assert_eq!(port.ty, Type::Scalar(ScalarType::I64));
        assert_eq!(port.kind, PortKind::Value);
        assert_eq!(port.default, Some(Value::i64(0)));
        assert_eq!(port.required, false);
        assert_eq!(port.description, Some("First input parameter".to_string()));
    }
    
    #[test]
    fn test_output_port_creation() {
        let port = OutputPort::new(
            "output1".to_string(),
            "result".to_string(),
            Type::Scalar(ScalarType::String),
            PortKind::Value,
        )
        .with_description("Result output".to_string());
        
        assert_eq!(port.id, "output1");
        assert_eq!(port.name, "result");
        assert_eq!(port.ty, Type::Scalar(ScalarType::String));
        assert_eq!(port.kind, PortKind::Value);
        assert_eq!(port.description, Some("Result output".to_string()));
    }
    
    #[test]
    fn test_validation_rule() {
        let rule = ValidationRule::with_description(
            |value| match value {
                Value::I64(n) => *n > 0,
                _ => false,
            },
            "Positive integer required".to_string(),
        );
        
        assert!(rule.validate(&Value::i64(5)));
        assert!(!rule.validate(&Value::i64(-5)));
        assert!(!rule.validate(&Value::string("not a number")));
    }
    
    #[test]
    fn test_input_port_validation() {
        let port = InputPort::new(
            "input1".to_string(),
            "positive_number".to_string(),
            Type::Scalar(ScalarType::I64),
            PortKind::Value,
        )
        .with_validation(ValidationRule::with_description(
            |value| match value {
                Value::I64(n) => *n > 0,
                _ => false,
            },
            "Must be a positive integer".to_string(),
        ));
        
        assert!(port.validate_value(&Value::i64(5)).is_ok());
        assert!(port.validate_value(&Value::i64(-5)).is_err());
    }
}
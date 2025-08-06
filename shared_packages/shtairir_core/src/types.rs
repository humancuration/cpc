//! Type mapping system for Shtairir Core

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Represents a Shtairir value with enhanced type information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShtairirValue {
    /// Numeric value
    Number(f64),
    /// String value
    String(String),
    /// Boolean value
    Boolean(bool),
    /// Identifier/reference
    Identifier(String),
    /// Object/dictionary
    Object(HashMap<String, ShtairirValue>),
    /// Array/list
    Array(Vec<ShtairirValue>),
    /// Binary data
    Binary(Vec<u8>),
    /// Date/time value
    DateTime(DateTime<Utc>),
    /// Unique identifier
    Uuid(Uuid),
    /// Null value
    Null,
}

/// Type information for Shtairir values
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShtairirType {
    /// Number type (f64)
    Number,
    /// String type
    String,
    /// Boolean type
    Boolean,
    /// Identifier type
    Identifier,
    /// Object type with schema
    Object(Option<String>), // Optional schema name
    /// Array type with element type
    Array(Box<ShtairirType>),
    /// Binary data type
    Binary,
    /// Date/time type
    DateTime,
    /// UUID type
    Uuid,
    /// Null type
    Null,
    /// Any type (dynamic)
    Any,
}

/// Schema definition for complex types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    /// Schema name
    pub name: String,
    /// Schema version
    pub version: String,
    /// Field definitions
    pub fields: HashMap<String, FieldDefinition>,
    /// Optional parent schema
    pub inherits: Option<String>,
}

/// Field definition within a schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: ShtairirType,
    /// Whether the field is required
    pub required: bool,
    /// Default value (if any)
    pub default_value: Option<ShtairirValue>,
    /// Field description
    pub description: Option<String>,
    /// Validation rules
    pub validation: Option<Vec<ValidationRule>>,
}

/// Validation rule for field values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRule {
    /// Minimum value for numbers
    MinNumber(f64),
    /// Maximum value for numbers
    MaxNumber(f64),
    /// Minimum string length
    MinLength(usize),
    /// Maximum string length
    MaxLength(usize),
    /// Regex pattern for strings
    Pattern(String),
    /// Enum of allowed values
    Enum(Vec<ShtairirValue>),
    /// Custom validation function name
    Custom(String),
}

/// Type conversion between Shtairir and Rust types
pub trait ShtairirTypeConvert<T> {
    /// Convert from ShtairirValue to T
    fn from_shtairir(value: &ShtairirValue) -> ShtairirResult<T>;
    
    /// Convert from T to ShtairirValue
    fn to_shtairir(value: T) -> ShtairirValue;
}

/// Type registry for managing schemas and type definitions
#[derive(Debug, Default)]
pub struct TypeRegistry {
    /// Registered schemas
    schemas: HashMap<String, Schema>,
    /// Type aliases
    aliases: HashMap<String, ShtairirType>,
}

impl TypeRegistry {
    /// Create a new type registry
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Register a schema
    pub fn register_schema(&mut self, schema: Schema) -> ShtairirResult<()> {
        self.schemas.insert(schema.name.clone(), schema);
        Ok(())
    }
    
    /// Get a schema by name
    pub fn get_schema(&self, name: &str) -> Option<&Schema> {
        self.schemas.get(name)
    }
    
    /// Register a type alias
    pub fn register_alias(&mut self, name: String, type_def: ShtairirType) -> ShtairirResult<()> {
        self.aliases.insert(name, type_def);
        Ok(())
    }
    
    /// Get a type by name or alias
    pub fn get_type(&self, name: &str) -> Option<&ShtairirType> {
        self.aliases.get(name)
    }
    
    /// Validate a value against a schema
    pub fn validate_against_schema(&self, value: &ShtairirValue, schema_name: &str) -> ShtairirResult<()> {
        let schema = self.schemas.get(schema_name)
            .ok_or_else(|| ShtairirError::Type(format!("Schema not found: {}", schema_name)))?;
        
        match value {
            ShtairirValue::Object(obj) => {
                for (field_name, field_def) in &schema.fields {
                    if field_def.required && !obj.contains_key(field_name) {
                        return Err(ShtairirError::Validation(
                            format!("Required field missing: {}", field_name)
                        ));
                    }
                    
                    if let Some(field_value) = obj.get(field_name) {
                        self.validate_field(field_value, field_def)?;
                    }
                }
                Ok(())
            }
            _ => Err(ShtairirError::Type(
                format!("Expected object for schema {}, got {:?}", schema_name, value)
            ))
        }
    }
    
    /// Validate a field value against its definition
    fn validate_field(&self, value: &ShtairirValue, field_def: &FieldDefinition) -> ShtairirResult<()> {
        // Type checking
        if !self.types_compatible(value, &field_def.field_type) {
            return Err(ShtairirError::Validation(
                format!("Field {} type mismatch: expected {:?}, got {:?}", 
                       field_def.name, field_def.field_type, value)
            ));
        }
        
        // Validation rules
        if let Some(rules) = &field_def.validation {
            for rule in rules {
                self.apply_validation_rule(value, rule)?;
            }
        }
        
        Ok(())
    }
    
    /// Check if a value is compatible with a type
    fn types_compatible(&self, value: &ShtairirValue, type_def: &ShtairirType) -> bool {
        match (value, type_def) {
            (ShtairirValue::Number(_), ShtairirType::Number) => true,
            (ShtairirValue::String(_), ShtairirType::String) => true,
            (ShtairirValue::Boolean(_), ShtairirType::Boolean) => true,
            (ShtairirValue::Identifier(_), ShtairirType::Identifier) => true,
            (ShtairirValue::Object(_), ShtairirType::Object(_)) => true,
            (ShtairirValue::Array(_), ShtairirType::Array(_)) => true,
            (ShtairirValue::Binary(_), ShtairirType::Binary) => true,
            (ShtairirValue::DateTime(_), ShtairirType::DateTime) => true,
            (ShtairirValue::Uuid(_), ShtairirType::Uuid) => true,
            (ShtairirValue::Null, ShtairirType::Null) => true,
            (_, ShtairirType::Any) => true,
            _ => false,
        }
    }
    
    /// Apply a validation rule to a value
    fn apply_validation_rule(&self, value: &ShtairirValue, rule: &ValidationRule) -> ShtairirResult<()> {
        match (value, rule) {
            (ShtairirValue::Number(n), ValidationRule::MinNumber(min)) => {
                if n < min {
                    return Err(ShtairirError::Validation(
                        format!("Value {} is less than minimum {}", n, min)
                    ));
                }
            }
            (ShtairirValue::Number(n), ValidationRule::MaxNumber(max)) => {
                if n > max {
                    return Err(ShtairirError::Validation(
                        format!("Value {} is greater than maximum {}", n, max)
                    ));
                }
            }
            (ShtairirValue::String(s), ValidationRule::MinLength(min)) => {
                if s.len() < *min {
                    return Err(ShtairirError::Validation(
                        format!("String length {} is less than minimum {}", s.len(), min)
                    ));
                }
            }
            (ShtairirValue::String(s), ValidationRule::MaxLength(max)) => {
                if s.len() > *max {
                    return Err(ShtairirError::Validation(
                        format!("String length {} is greater than maximum {}", s.len(), max)
                    ));
                }
            }
            (ShtairirValue::String(s), ValidationRule::Pattern(pattern)) => {
                use regex::Regex;
                let regex = Regex::new(pattern)
                    .map_err(|e| ShtairirError::Validation(format!("Invalid regex pattern: {}", e)))?;
                if !regex.is_match(s) {
                    return Err(ShtairirError::Validation(
                        format!("String '{}' does not match pattern '{}'", s, pattern)
                    ));
                }
            }
            (_, ValidationRule::Enum(values)) => {
                if !values.contains(value) {
                    return Err(ShtairirError::Validation(
                        format!("Value {:?} is not in allowed enum", value)
                    ));
                }
            }
            (_, ValidationRule::Custom(_)) => {
                // Custom validation would be implemented with registered functions
                // For now, we'll allow it
            }
            _ => {
                return Err(ShtairirError::Validation(
                    format!("Validation rule {:?} cannot be applied to value {:?}", rule, value)
                ));
            }
        }
        Ok(())
    }
}

// Implement common type conversions
impl ShtairirTypeConvert<String> for String {
    fn from_shtairir(value: &ShtairirValue) -> ShtairirResult<String> {
        match value {
            ShtairirValue::String(s) => Ok(s.clone()),
            _ => Err(ShtairirError::Type("Expected string value".to_string())),
        }
    }
    
    fn to_shtairir(value: String) -> ShtairirValue {
        ShtairirValue::String(value)
    }
}

impl ShtairirTypeConvert<f64> for f64 {
    fn from_shtairir(value: &ShtairirValue) -> ShtairirResult<f64> {
        match value {
            ShtairirValue::Number(n) => Ok(*n),
            _ => Err(ShtairirError::Type("Expected number value".to_string())),
        }
    }
    
    fn to_shtairir(value: f64) -> ShtairirValue {
        ShtairirValue::Number(value)
    }
}

impl ShtairirTypeConvert<bool> for bool {
    fn from_shtairir(value: &ShtairirValue) -> ShtairirResult<bool> {
        match value {
            ShtairirValue::Boolean(b) => Ok(*b),
            _ => Err(ShtairirError::Type("Expected boolean value".to_string())),
        }
    }
    
    fn to_shtairir(value: bool) -> ShtairirValue {
        ShtairirValue::Boolean(value)
    }
}

impl ShtairirTypeConvert<Vec<ShtairirValue>> for Vec<ShtairirValue> {
    fn from_shtairir(value: &ShtairirValue) -> ShtairirResult<Vec<ShtairirValue>> {
        match value {
            ShtairirValue::Array(arr) => Ok(arr.clone()),
            _ => Err(ShtairirError::Type("Expected array value".to_string())),
        }
    }
    
    fn to_shtairir(value: Vec<ShtairirValue>) -> ShtairirValue {
        ShtairirValue::Array(value)
    }
}

impl ShtairirTypeConvert<HashMap<String, ShtairirValue>> for HashMap<String, ShtairirValue> {
    fn from_shtairir(value: &ShtairirValue) -> ShtairirResult<HashMap<String, ShtairirValue>> {
        match value {
            ShtairirValue::Object(obj) => Ok(obj.clone()),
            _ => Err(ShtairirError::Type("Expected object value".to_string())),
        }
    }
    
    fn to_shtairir(value: HashMap<String, ShtairirValue>) -> ShtairirValue {
        ShtairirValue::Object(value)
    }
}

impl ShtairirTypeConvert<DateTime<Utc>> for DateTime<Utc> {
    fn from_shtairir(value: &ShtairirValue) -> ShtairirResult<DateTime<Utc>> {
        match value {
            ShtairirValue::DateTime(dt) => Ok(*dt),
            ShtairirValue::String(s) => {
                DateTime::parse_from_rfc3339(s)
                    .map(|dt| dt.with_timezone(&Utc))
                    .map_err(|_| ShtairirError::Type("Invalid datetime string format".to_string()))
            }
            _ => Err(ShtairirError::Type("Expected datetime value".to_string())),
        }
    }
    
    fn to_shtairir(value: DateTime<Utc>) -> ShtairirValue {
        ShtairirValue::DateTime(value)
    }
}

impl ShtairirTypeConvert<Uuid> for Uuid {
    fn from_shtairir(value: &ShtairirValue) -> ShtairirResult<Uuid> {
        match value {
            ShtairirValue::Uuid(uuid) => Ok(*uuid),
            ShtairirValue::String(s) => {
                Uuid::parse_str(s)
                    .map_err(|_| ShtairirError::Type("Invalid UUID string format".to_string()))
            }
            _ => Err(ShtairirError::Type("Expected UUID value".to_string())),
        }
    }
    
    fn to_shtairir(value: Uuid) -> ShtairirValue {
        ShtairirValue::Uuid(value)
    }
}
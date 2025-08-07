//! Value types for Shtairir registry
//!
//! This module defines the Value enum, which represents values that can be
//! used in Shtairir manifests and during execution.

use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

/// Represents a value in Shtairir
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    /// 64-bit signed integer
    I64(i64),
    /// 64-bit floating point number
    F64(f64),
    /// Boolean value
    Bool(bool),
    /// String value
    String(String),
    /// List of values
    List(Vec<Value>),
    /// Object with string keys and values
    Object(BTreeMap<String, Value>),
    /// Null value
    Null,
}

impl Value {
    /// Create a new string value
    pub fn string(s: impl Into<String>) -> Self {
        Value::String(s.into())
    }
    
    /// Create a new integer value
    pub fn i64(n: i64) -> Self {
        Value::I64(n)
    }
    
    /// Create a new float value
    pub fn f64(n: f64) -> Self {
        Value::F64(n)
    }
    
    /// Create a new boolean value
    pub fn bool(b: bool) -> Self {
        Value::Bool(b)
    }
    
    /// Create a new list value
    pub fn list(values: Vec<Value>) -> Self {
        Value::List(values)
    }
    
    /// Create a new object value
    pub fn object(map: BTreeMap<String, Value>) -> Self {
        Value::Object(map)
    }
    
    /// Create a new null value
    pub fn null() -> Self {
        Value::Null
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    
    #[test]
    fn test_value_creation() {
        let str_val = Value::string("hello");
        assert_eq!(str_val, Value::String("hello".to_string()));
        
        let int_val = Value::i64(42);
        assert_eq!(int_val, Value::I64(42));
        
        let bool_val = Value::bool(true);
        assert_eq!(bool_val, Value::Bool(true));
        
        let null_val = Value::null();
        assert_eq!(null_val, Value::Null);
    }
    
    #[test]
    fn test_value_object() {
        let mut map = BTreeMap::new();
        map.insert("key1".to_string(), Value::string("value1"));
        map.insert("key2".to_string(), Value::i64(42));
        
        let obj = Value::object(map.clone());
        assert_eq!(obj, Value::Object(map));
    }
    
    #[test]
    fn test_value_list() {
        let values = vec![Value::i64(1), Value::i64(2), Value::i64(3)];
        let list = Value::list(values.clone());
        assert_eq!(list, Value::List(values));
    }
}
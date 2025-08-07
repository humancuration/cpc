//! Literal values for Shtairir registry
//! 
//! This module defines the ValueLiteral enum, which represents literal values
//! that can appear in Shtairir manifests and module definitions.

use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

/// Represents a literal value in Shtairir manifests
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValueLiteral {
    /// 64-bit signed integer
    I64(i64),
    /// 64-bit floating point number
    F64(f64),
    /// Boolean value
    Bool(bool),
    /// String value
    String(String),
    /// List of values
    List(Vec<ValueLiteral>),
    /// Object with string keys and values
    Object(BTreeMap<String, ValueLiteral>),
    /// Null value
    Null,
}

impl ValueLiteral {
    /// Create a new string value
    pub fn string(s: impl Into<String>) -> Self {
        ValueLiteral::String(s.into())
    }
    
    /// Create a new integer value
    pub fn i64(n: i64) -> Self {
        ValueLiteral::I64(n)
    }
    
    /// Create a new float value
    pub fn f64(n: f64) -> Self {
        ValueLiteral::F64(n)
    }
    
    /// Create a new boolean value
    pub fn bool(b: bool) -> Self {
        ValueLiteral::Bool(b)
    }
    
    /// Create a new list value
    pub fn list(values: Vec<ValueLiteral>) -> Self {
        ValueLiteral::List(values)
    }
    
    /// Create a new object value
    pub fn object(map: BTreeMap<String, ValueLiteral>) -> Self {
        ValueLiteral::Object(map)
    }
    
    /// Create a new null value
    pub fn null() -> Self {
        ValueLiteral::Null
}
    
    /// Create a new option with Some value
    pub fn option_some(value: ValueLiteral) -> Self {
        ValueLiteral::List(vec![value])
    }
    
    /// Create a new option with None value
    pub fn option_none() -> Self {
        ValueLiteral::List(vec![])
    }
    }
    
    /// Create a new option with Some value
    pub fn option_some(value: ValueLiteral) -> Self {
        ValueLiteral::List(vec![value])
    }
    
    /// Create a new option with None value
    pub fn option_none() -> Self {
        ValueLiteral::List(vec![])
    }
}

impl Default for ValueLiteral {
    fn default() -> Self {
        ValueLiteral::Null
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    
    #[test]
    fn test_value_literal_creation() {
        let str_val = ValueLiteral::string("hello");
        assert_eq!(str_val, ValueLiteral::String("hello".to_string()));
        
        let int_val = ValueLiteral::i64(42);
        assert_eq!(int_val, ValueLiteral::I64(42));
        
        let bool_val = ValueLiteral::bool(true);
        assert_eq!(bool_val, ValueLiteral::Bool(true));
        
        let null_val = ValueLiteral::null();
        assert_eq!(null_val, ValueLiteral::Null);
    }
    
    #[test]
    fn test_value_literal_object() {
        let mut map = BTreeMap::new();
        map.insert("key1".to_string(), ValueLiteral::string("value1"));
        map.insert("key2".to_string(), ValueLiteral::i64(42));
        
        let obj = ValueLiteral::object(map.clone());
        assert_eq!(obj, ValueLiteral::Object(map));
    }
    
    #[test]
    fn test_value_literal_list() {
        let values = vec![ValueLiteral::i64(1), ValueLiteral::i64(2), ValueLiteral::i64(3)];
        let list = ValueLiteral::list(values.clone());
        assert_eq!(list, ValueLiteral::List(values));
    }
}
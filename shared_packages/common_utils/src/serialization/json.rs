//! JSON serialization utilities
//!
//! This module provides convenient functions for serializing and deserializing
//! data to and from JSON format.

use serde::{Serialize, de::DeserializeOwned};
use crate::error::{CommonError, Result};

/// Serialize a value to a JSON string
pub fn to_json<T: Serialize>(value: &T) -> Result<String> {
    serde_json::to_string(value).map_err(CommonError::from)
}

/// Serialize a value to a JSON byte vector
pub fn to_json_vec<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    serde_json::to_vec(value).map_err(CommonError::from)
}

/// Deserialize a JSON string to a value
pub fn from_json<T: DeserializeOwned>(json: &str) -> Result<T> {
    serde_json::from_str(json).map_err(CommonError::from)
}

/// Deserialize a JSON byte slice to a value
pub fn from_json_slice<T: DeserializeOwned>(slice: &[u8]) -> Result<T> {
    serde_json::from_slice(slice).map_err(CommonError::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestStruct {
        name: String,
        age: u32,
    }

    #[test]
    fn test_to_json() {
        let test_struct = TestStruct {
            name: "Alice".to_string(),
            age: 30,
        };
        
        let json = to_json(&test_struct).unwrap();
        assert!(json.contains("\"name\":\"Alice\""));
        assert!(json.contains("\"age\":30"));
    }
    
    #[test]
    fn test_to_json_vec() {
        let test_struct = TestStruct {
            name: "Bob".to_string(),
            age: 25,
        };
        
        let json_vec = to_json_vec(&test_struct).unwrap();
        let json_str = String::from_utf8(json_vec).unwrap();
        assert!(json_str.contains("\"name\":\"Bob\""));
        assert!(json_str.contains("\"age\":25"));
    }
    
    #[test]
    fn test_from_json() {
        let json = r#"{"name":"Charlie","age":35}"#;
        let test_struct: TestStruct = from_json(json).unwrap();
        
        assert_eq!(test_struct.name, "Charlie");
        assert_eq!(test_struct.age, 35);
    }
    
    #[test]
    fn test_from_json_slice() {
        let json = r#"{"name":"David","age":40}"#.as_bytes();
        let test_struct: TestStruct = from_json_slice(json).unwrap();
        
        assert_eq!(test_struct.name, "David");
        assert_eq!(test_struct.age, 40);
    }
    
    #[test]
    fn test_invalid_json() {
        let invalid_json = r#"{"name":"Eve","age":}"#;
        let result: Result<TestStruct> = from_json(invalid_json);
        assert!(result.is_err());
    }
}
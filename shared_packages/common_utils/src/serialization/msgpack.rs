//! MessagePack serialization utilities
//!
//! This module provides convenient functions for serializing and deserializing
//! data to and from MessagePack format, which is more compact than JSON.

use serde::{Serialize, de::DeserializeOwned};
use crate::error::{CommonError, Result};

/// Serialize a value to MessagePack byte vector
pub fn to_msgpack<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    rmp_serde::to_vec(value).map_err(CommonError::from)
}

/// Serialize a value to MessagePack byte vector with named fields
pub fn to_msgpack_vec<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    rmp_serde::to_vec_named(value).map_err(CommonError::from)
}

/// Deserialize MessagePack byte slice to a value
pub fn from_msgpack<T: DeserializeOwned>(slice: &[u8]) -> Result<T> {
    rmp_serde::from_slice(slice).map_err(CommonError::from)
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
    fn test_to_msgpack() {
        let test_struct = TestStruct {
            name: "Alice".to_string(),
            age: 30,
        };
        
        let msgpack = to_msgpack(&test_struct).unwrap();
        assert!(!msgpack.is_empty());
        
        // Verify we can deserialize it back
        let deserialized: TestStruct = from_msgpack(&msgpack).unwrap();
        assert_eq!(deserialized, test_struct);
    }
    
    #[test]
    fn test_to_msgpack_vec() {
        let test_struct = TestStruct {
            name: "Bob".to_string(),
            age: 25,
        };
        
        let msgpack = to_msgpack_vec(&test_struct).unwrap();
        assert!(!msgpack.is_empty());
        
        // Verify we can deserialize it back
        let deserialized: TestStruct = from_msgpack(&msgpack).unwrap();
        assert_eq!(deserialized, test_struct);
    }
    
    #[test]
    fn test_from_msgpack() {
        // Create a MessagePack representation manually
        let msgpack_data = vec![130, 164, 110, 97, 109, 101, 165, 67, 104, 97, 114, 108, 105, 101, 163, 97, 103, 101, 35];
        // This represents {"name": "Charlie", "age": 35}
        
        let test_struct: TestStruct = from_msgpack(&msgpack_data).unwrap();
        assert_eq!(test_struct.name, "Charlie");
        assert_eq!(test_struct.age, 35);
    }
    
    #[test]
    fn test_invalid_msgpack() {
        let invalid_data = vec![0xc1]; // Invalid MessagePack data
        let result: Result<TestStruct> = from_msgpack(&invalid_data);
        assert!(result.is_err());
    }
}
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use crate::literal::ValueLiteral;

/// Canonicalizes a ValueLiteral for consistent hashing
pub fn canonicalize_value(value: &ValueLiteral) -> Result<String, anyhow::Error> {
    // Convert to a canonical form with sorted keys and normalized whitespace
    let canonical = match value {
        ValueLiteral::Object(map) => {
            let mut sorted_map = BTreeMap::new();
            for (k, v) in map.iter() {
                sorted_map.insert(k.clone(), canonicalize_value(v)?);
            }
            let mut result = String::from("{");
            let mut first = true;
            for (k, v) in sorted_map.iter() {
                if !first {
                    result.push(',');
                }
                result.push_str(&format!("\"{}\":{}", k, v));
                first = false;
            }
            result.push('}');
            result
        }
        ValueLiteral::List(arr) => {
            let mut result = String::from("[");
            for (i, v) in arr.iter().enumerate() {
                if i > 0 {
                    result.push(',');
                }
                result.push_str(&canonicalize_value(v)?);
            }
            result.push(']');
            result
        }
        ValueLiteral::String(s) => format!("\"{}\"", s),
        ValueLiteral::I64(n) => n.to_string(),
        ValueLiteral::F64(n) => n.to_string(),
        ValueLiteral::Bool(b) => b.to_string(),
        ValueLiteral::Null => "null".to_string(),
    };
    Ok(canonical)
}

/// Computes SHA256 hash of a canonicalized ValueLiteral
pub fn hash_content(value: &ValueLiteral) -> Result<String, anyhow::Error> {
    let canonical = canonicalize_value(value)?;
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    let result = hasher.finalize();
    Ok(format!("sha256:{}", hex::encode(result)))
}

/// Computes SHA256 hash of a TOML string
pub fn hash_toml_content(toml_str: &str) -> Result<String, anyhow::Error> {
    // For now, we'll just hash the TOML string directly
    // In a more sophisticated implementation, we might parse and canonicalize
    let mut hasher = Sha256::new();
    hasher.update(toml_str.as_bytes());
    let result = hasher.finalize();
    Ok(format!("sha256:{}", hex::encode(result)))
}

/// Verifies that a value matches the expected hash
pub fn verify_integrity(value: &ValueLiteral, expected_hash: &str) -> Result<bool, anyhow::Error> {
    let computed_hash = hash_content(value)?;
    Ok(computed_hash == expected_hash)
}

/// Verifies that a TOML string matches the expected hash
pub fn verify_toml_integrity(toml_str: &str, expected_hash: &str) -> Result<bool, anyhow::Error> {
    let computed_hash = hash_toml_content(toml_str)?;
    Ok(computed_hash == expected_hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_canonicalize_value_object() {
        let mut inner_map = BTreeMap::new();
        inner_map.insert("y".to_string(), ValueLiteral::Bool(true));
        inner_map.insert("x".to_string(), ValueLiteral::Bool(false));
        
        let mut value_map = BTreeMap::new();
        value_map.insert("b".to_string(), ValueLiteral::I64(2));
        value_map.insert("a".to_string(), ValueLiteral::I64(1));
        value_map.insert("c".to_string(), ValueLiteral::Object(inner_map));
        
        let value = ValueLiteral::Object(value_map);
        
        let canonical = canonicalize_value(&value).unwrap();
        // Keys should be sorted
        assert!(canonical.starts_with(r#"{"a":1,"b":2,"c":{"x":false,"y":true}}"#));
    }

    #[test]
    fn test_canonicalize_value_list() {
        let value = ValueLiteral::List(vec![
            ValueLiteral::I64(3),
            ValueLiteral::I64(1),
            ValueLiteral::I64(2)
        ]);
        let canonical = canonicalize_value(&value).unwrap();
        assert_eq!(canonical, "[1,2,3]");
    }

    #[test]
    fn test_hash_content() {
        let mut value_map = BTreeMap::new();
        value_map.insert("a".to_string(), ValueLiteral::I64(1));
        value_map.insert("b".to_string(), ValueLiteral::I64(2));
        let value = ValueLiteral::Object(value_map);
        
        let hash = hash_content(&value).unwrap();
        assert!(hash.starts_with("sha256:"));
        assert_eq!(hash.len(), 71); // "sha256:" + 64 hex chars
    }

    #[test]
    fn test_verify_integrity() {
        let mut value_map = BTreeMap::new();
        value_map.insert("test".to_string(), ValueLiteral::String("value".to_string()));
        let value = ValueLiteral::Object(value_map);
        
        let hash = hash_content(&value).unwrap();
        assert!(verify_integrity(&value, &hash).unwrap());
        
        // Different value should fail
        let mut different_map = BTreeMap::new();
        different_map.insert("test".to_string(), ValueLiteral::String("different".to_string()));
        let different_value = ValueLiteral::Object(different_map);
        
        assert!(!verify_integrity(&different_value, &hash).unwrap());
    }

    #[test]
    fn test_hash_toml_content() {
        let toml_str = r#"
        name = "test"
        version = "1.0.0"
        inputs = [{ name = "a", ty = "i64" }]
        "#;
        
        let hash = hash_toml_content(toml_str).unwrap();
        assert!(hash.starts_with("sha256:"));
        assert_eq!(hash.len(), 71);
    }
}
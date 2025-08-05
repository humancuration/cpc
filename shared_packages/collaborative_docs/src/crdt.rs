//! CRDT implementation for collaborative document editing using Automerge

use automerge::{AutoCommit, Automerge, Change, Cursor, ObjType, Prop, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

/// Error types for CRDT operations
#[derive(Error, Debug)]
pub enum CrdtError {
    #[error("Automerge error: {0}")]
    AutomergeError(String),
    #[error("Document not found: {0}")]
    DocumentNotFound(Uuid),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("Conflict resolution failed: {0}")]
    ResolutionFailed(String),
}

/// CRDT document wrapper
pub struct CrdtDocument {
    doc: Automerge,
    actor_id: String,
}

impl CrdtDocument {
    /// Create a new CRDT document
    pub fn new(actor_id: String) -> Self {
        let mut doc = Automerge::new();
        doc.set_actor(actor_id.clone().into());
        Self { doc, actor_id }
    }

    /// Load a CRDT document from existing data
    pub fn load(data: &[u8], actor_id: String) -> Result<Self, CrdtError> {
        let doc = Automerge::load(data)
            .map_err(|e| CrdtError::AutomergeError(e.to_string()))?;
        Ok(Self { doc, actor_id })
    }

    /// Save the document to bytes
    pub fn save(&self) -> Result<Vec<u8>, CrdtError> {
        Ok(self.doc.save())
    }

    /// Apply changes from another document
    pub fn apply_changes(&mut self, changes: &[Change]) -> Result<(), CrdtError> {
        self.doc
            .apply_changes(changes.to_vec())
            .map_err(|e| CrdtError::AutomergeError(e.to_string()))
    }

    /// Get changes since a heads
    pub fn get_changes(&self, heads: &[automerge::ChangeHash]) -> Vec<Change> {
        self.doc.get_changes(heads)
    }

    /// Get current heads
    pub fn get_heads(&self) -> Vec<automerge::ChangeHash> {
        self.doc.get_heads()
    }

    /// Insert text at a position
    pub fn insert_text(
        &mut self,
        obj_id: &automerge::ObjId,
        index: usize,
        text: &str,
    ) -> Result<(), CrdtError> {
        let mut tx = self.doc.transaction();
        tx.splice_text(obj_id, index, 0, text)
            .map_err(|e| CrdtError::AutomergeError(e.to_string()))?;
        tx.commit();
        Ok(())
    }

    /// Delete text at a position
    pub fn delete_text(
        &mut self,
        obj_id: &automerge::ObjId,
        index: usize,
        length: usize,
    ) -> Result<(), CrdtError> {
        let mut tx = self.doc.transaction();
        tx.splice_text(obj_id, index, length as isize, "")
            .map_err(|e| CrdtError::AutomergeError(e.to_string()))?;
        tx.commit();
        Ok(())
    }

    /// Put a key-value pair in a map
    pub fn put(
        &mut self,
        obj_id: &automerge::ObjId,
        key: &str,
        value: serde_json::Value,
    ) -> Result<(), CrdtError> {
        let mut tx = self.doc.transaction();
        put_value(&mut tx, obj_id, key, value)
            .map_err(|e| CrdtError::AutomergeError(e.to_string()))?;
        tx.commit();
        Ok(())
    }

    /// Get a value from a map
    pub fn get(
        &self,
        obj_id: &automerge::ObjId,
        key: &str,
    ) -> Result<Option<serde_json::Value>, CrdtError> {
        let value = self.doc.get(obj_id, key);
        match value {
            Ok(Some((value, _))) => {
                let json_value = value_to_json(value);
                Ok(Some(json_value))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(CrdtError::AutomergeError(e.to_string())),
        }
    }

    /// Get the document as JSON
    pub fn to_json(&self) -> Result<serde_json::Value, CrdtError> {
        let root = self.doc.get_object_root();
        obj_to_json(&self.doc, &root)
    }
}

/// Helper function to convert serde_json::Value to Automerge value
fn put_value(
    tx: &mut automerge::transaction::Transactable<&mut AutoCommit>,
    obj_id: &automerge::ObjId,
    key: &str,
    value: serde_json::Value,
) -> Result<(), automerge::AutomergeError> {
    match value {
        serde_json::Value::String(s) => {
            tx.put(obj_id, key, s)?;
        }
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                tx.put(obj_id, key, i)?;
            } else if let Some(u) = n.as_u64() {
                tx.put(obj_id, key, u)?;
            } else if let Some(f) = n.as_f64() {
                tx.put(obj_id, key, f)?;
            }
        }
        serde_json::Value::Bool(b) => {
            tx.put(obj_id, key, b)?;
        }
        serde_json::Value::Array(arr) => {
            let list_id = tx.put_object(obj_id, key, ObjType::List)?;
            for (i, item) in arr.iter().enumerate() {
                put_value_in_list(tx, &list_id, i, item.clone())?;
            }
        }
        serde_json::Value::Object(obj) => {
            let map_id = tx.put_object(obj_id, key, ObjType::Map)?;
            for (k, v) in obj {
                put_value(tx, &map_id, &k, v)?;
            }
        }
        serde_json::Value::Null => {
            tx.put(obj_id, key, ())?;
        }
    }
    Ok(())
}

/// Helper function to put a value in a list
fn put_value_in_list(
    tx: &mut automerge::transaction::Transactable<&mut AutoCommit>,
    list_id: &automerge::ObjId,
    index: usize,
    value: serde_json::Value,
) -> Result<(), automerge::AutomergeError> {
    match value {
        serde_json::Value::String(s) => {
            tx.insert(list_id, index, s)?;
        }
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                tx.insert(list_id, index, i)?;
            } else if let Some(u) = n.as_u64() {
                tx.insert(list_id, index, u)?;
            } else if let Some(f) = n.as_f64() {
                tx.insert(list_id, index, f)?;
            }
        }
        serde_json::Value::Bool(b) => {
            tx.insert(list_id, index, b)?;
        }
        serde_json::Value::Array(arr) => {
            let nested_list_id = tx.insert_object(list_id, index, ObjType::List)?;
            for (i, item) in arr.iter().enumerate() {
                put_value_in_list(tx, &nested_list_id, i, item.clone())?;
            }
        }
        serde_json::Value::Object(obj) => {
            let map_id = tx.insert_object(list_id, index, ObjType::Map)?;
            for (k, v) in obj {
                put_value(tx, &map_id, &k, v)?;
            }
        }
        serde_json::Value::Null => {
            tx.insert(list_id, index, ())?;
        }
    }
    Ok(())
}

/// Helper function to convert Automerge value to serde_json::Value
fn value_to_json(value: Value<'_>) -> serde_json::Value {
    match value {
        Value::Scalar(scalar) => match scalar.as_ref() {
            automerge::ScalarValue::Str(s) => serde_json::Value::String(s.to_string()),
            automerge::ScalarValue::Int(i) => serde_json::Value::Number((*i).into()),
            automerge::ScalarValue::Uint(u) => {
                serde_json::Value::Number(serde_json::Number::from(*u))
            }
            automerge::ScalarValue::F64(f) => {
                serde_json::Number::from_f64(*f).map_or(serde_json::Value::Null, serde_json::Value::Number)
            }
            automerge::ScalarValue::Counter(c) => serde_json::Value::Number((*c).into()),
            automerge::ScalarValue::Timestamp(t) => serde_json::Value::Number((*t).into()),
            automerge::ScalarValue::Boolean(b) => serde_json::Value::Bool(*b),
            automerge::ScalarValue::Null => serde_json::Value::Null,
            _ => serde_json::Value::String(format!("{:?}", scalar)),
        },
        _ => serde_json::Value::String(format!("{:?}", value)),
    }
}

/// Helper function to convert Automerge object to JSON
fn obj_to_json(doc: &Automerge, obj_id: &automerge::ObjId) -> Result<serde_json::Value, CrdtError> {
    let obj_type = doc.object_type(obj_id);
    match obj_type {
        Ok(ObjType::Map) => {
            let mut map = serde_json::Map::new();
            for key in doc.keys(obj_id) {
                if let Ok(Some((value, _))) = doc.get(obj_id, &key) {
                    let json_value = value_to_json(value);
                    map.insert(key, json_value);
                }
            }
            Ok(serde_json::Value::Object(map))
        }
        Ok(ObjType::List) => {
            let mut arr = Vec::new();
            for i in 0..doc.length(obj_id) {
                if let Ok(Some((value, _))) = doc.get(obj_id, i) {
                    let json_value = value_to_json(value);
                    arr.push(json_value);
                }
            }
            Ok(serde_json::Value::Array(arr))
        }
        _ => Err(CrdtError::InvalidOperation("Unsupported object type".to_string())),
    }
}

/// Conflict resolver for merging documents
pub struct ConflictResolver;

impl ConflictResolver {
    /// Merge two documents, resolving conflicts
    pub fn merge_documents(
        doc1: &mut CrdtDocument,
        doc2: &CrdtDocument,
    ) -> Result<(), CrdtError> {
        let heads1 = doc1.get_heads();
        let changes = doc2.get_changes(&heads1);
        doc1.apply_changes(&changes)?;
        Ok(())
    }

    /// Resolve conflicts between multiple documents
    pub fn resolve_conflicts(documents: &mut [CrdtDocument]) -> Result<(), CrdtError> {
        if documents.is_empty() {
            return Ok(());
        }

        // Use the first document as the base
        let base_heads = documents[0].get_heads();
        
        // Apply changes from all other documents
        for doc in documents.iter().skip(1) {
            let changes = doc.get_changes(&base_heads);
            documents[0].apply_changes(&changes)?;
        }

        // Update all other documents with the merged result
        let merged_changes = documents[0].get_changes(&base_heads);
        for doc in documents.iter_mut().skip(1) {
            doc.apply_changes(&merged_changes)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crdt_document_creation() {
        let doc = CrdtDocument::new("actor1".to_string());
        assert!(doc.get_heads().is_empty());
    }

    #[test]
    fn test_put_and_get() {
        let mut doc = CrdtDocument::new("actor1".to_string());
        let root = doc.doc.get_object_root();
        
        doc.put(&root, "key1", serde_json::Value::String("value1".to_string())).unwrap();
        
        let value = doc.get(&root, "key1").unwrap();
        assert_eq!(value, Some(serde_json::Value::String("value1".to_string())));
    }

    #[test]
    fn test_merge_documents() {
        let mut doc1 = CrdtDocument::new("actor1".to_string());
        let doc2 = CrdtDocument::new("actor2".to_string());
        let root1 = doc1.doc.get_object_root();
        let root2 = doc2.doc.get_object_root();
        
        doc1.put(&root1, "key1", serde_json::Value::String("value1".to_string())).unwrap();
        doc2.put(&root2, "key2", serde_json::Value::String("value2".to_string())).unwrap();
        
        ConflictResolver::merge_documents(&mut doc1, &doc2).unwrap();
        
        let value1 = doc1.get(&root1, "key1").unwrap();
        let value2 = doc1.get(&root1, "key2").unwrap();
        assert_eq!(value1, Some(serde_json::Value::String("value1".to_string())));
        assert_eq!(value2, Some(serde_json::Value::String("value2".to_string())));
    }
}
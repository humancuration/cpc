#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use serde_json::json;
    
    #[test]
    fn test_crdt_document_creation() {
        let node_id = Uuid::new_v4();
        let document = CRDTDocument::new(node_id);
        
        assert_eq!(document.node_id, node_id);
        assert_eq!(document.logical_clock, 0);
        assert_eq!(document.operation_counter, 0);
    }
    
    #[test]
    fn test_crdt_id_generation() {
        let node_id = Uuid::new_v4();
        let mut document = CRDTDocument::new(node_id);
        
        let id1 = document.generate_id();
        assert_eq!(id1.node_id, node_id);
        assert_eq!(id1.counter, 1);
        assert_eq!(id1.timestamp, 1);
        
        let id2 = document.generate_id();
        assert_eq!(id2.node_id, node_id);
        assert_eq!(id2.counter, 2);
        assert_eq!(id2.timestamp, 2);
    }
    
    #[test]
    fn test_insert_operation() {
        let node_id = Uuid::new_v4();
        let mut document = CRDTDocument::new(node_id);
        
        let id = document.generate_id();
        let value = json!({"text": "Hello, world!"});
        let operation = DocumentOperation::Insert {
            position: 0,
            value: value.clone(),
            id: id.clone(),
            parent_id: None,
        };
        
        assert!(document.apply_operation(&operation, node_id));
        
        let elements = document.get_elements();
        assert_eq!(elements.len(), 1);
        assert!(elements.contains_key(&id));
        
        let element_state = elements.get(&id).unwrap();
        assert_eq!(element_state.value, value);
        assert_eq!(element_state.deleted, false);
    }
    
    #[test]
    fn test_delete_operation() {
        let node_id = Uuid::new_v4();
        let mut document = CRDTDocument::new(node_id);
        
        // First insert an element
        let id = document.generate_id();
        let value = json!({"text": "Hello, world!"});
        let insert_operation = DocumentOperation::Insert {
            position: 0,
            value: value.clone(),
            id: id.clone(),
            parent_id: None,
        };
        
        assert!(document.apply_operation(&insert_operation, node_id));
        
        // Then delete it
        let delete_operation = DocumentOperation::Delete {
            id: id.clone(),
            timestamp: 2,
        };
        
        assert!(document.apply_operation(&delete_operation, node_id));
        
        let elements = document.get_elements();
        assert_eq!(elements.len(), 1);
        assert!(elements.contains_key(&id));
        
        let element_state = elements.get(&id).unwrap();
        assert_eq!(element_state.deleted, true);
    }
    
    #[test]
    fn test_update_operation() {
        let node_id = Uuid::new_v4();
        let mut document = CRDTDocument::new(node_id);
        
        // First insert an element
        let id = document.generate_id();
        let value = json!({"text": "Hello, world!"});
        let insert_operation = DocumentOperation::Insert {
            position: 0,
            value: value.clone(),
            id: id.clone(),
            parent_id: None,
        };
        
        assert!(document.apply_operation(&insert_operation, node_id));
        
        // Then update it
        let updated_value = json!({"text": "Hello, CRDT!"});
        let update_operation = DocumentOperation::Update {
            id: id.clone(),
            value: updated_value.clone(),
            timestamp: 2,
        };
        
        assert!(document.apply_operation(&update_operation, node_id));
        
        let elements = document.get_elements();
        assert_eq!(elements.len(), 1);
        assert!(elements.contains_key(&id));
        
        let element_state = elements.get(&id).unwrap();
        assert_eq!(element_state.value, updated_value);
    }
    
    #[test]
    fn test_version_vector() {
        let node_id1 = Uuid::new_v4();
        let node_id2 = Uuid::new_v4();
        let mut document = CRDTDocument::new(node_id1);
        
        // Apply an operation from node1
        let id1 = document.generate_id();
        let operation1 = DocumentOperation::Insert {
            position: 0,
            value: json!({"text": "Hello"}),
            id: id1,
            parent_id: None,
        };
        
        assert!(document.apply_operation(&operation1, node_id1));
        
        // Apply an operation from node2
        let id2 = CRDTId::new(node_id2, 1, 1);
        let operation2 = DocumentOperation::Insert {
            position: 1,
            value: json!({"text": "World"}),
            id: id2,
            parent_id: None,
        };
        
        assert!(document.apply_operation(&operation2, node_id2));
        
        let version_vector = document.get_version_vector();
        assert_eq!(version_vector.len(), 2);
        assert_eq!(version_vector.get(&node_id1), Some(&1));
        assert_eq!(version_vector.get(&node_id2), Some(&1));
    }
    
    #[test]
    fn test_version_vector_comparison() {
        let node_id1 = Uuid::new_v4();
        let node_id2 = Uuid::new_v4();
        let node_id3 = Uuid::new_v4();
        let mut document = CRDTDocument::new(node_id1);
        
        // Apply operations to create a version vector
        let id1 = document.generate_id();
        let operation1 = DocumentOperation::Insert {
            position: 0,
            value: json!({"text": "Hello"}),
            id: id1,
            parent_id: None,
        };
        assert!(document.apply_operation(&operation1, node_id1));
        
        let id2 = CRDTId::new(node_id2, 1, 1);
        let operation2 = DocumentOperation::Insert {
            position: 1,
            value: json!({"text": "World"}),
            id: id2,
            parent_id: None,
        };
        assert!(document.apply_operation(&operation2, node_id2));
        
        // Create another version vector for comparison
        let mut other_version_vector = HashMap::new();
        other_version_vector.insert(node_id1, 1); // Same as local
        other_version_vector.insert(node_id2, 2); // Ahead of local
        other_version_vector.insert(node_id3, 1); // Not in local
        
        // Compare version vectors
        let comparison = document.compare_version_vectors(&other_version_vector);
        
        match comparison {
            VersionVectorComparison::Concurrent { local_ahead, remote_ahead, concurrent } => {
                assert_eq!(local_ahead.len(), 0);
                assert_eq!(remote_ahead.len(), 2);
                assert_eq!(remote_ahead, vec![node_id2, node_id3]);
                assert_eq!(concurrent.len(), 1);
                assert_eq!(concurrent, vec![node_id1]);
            }
            _ => panic!("Expected Concurrent comparison result"),
        }
    }
}
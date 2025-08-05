//! CRDT functionality tests for the collaborative documentation system

use collaborative_docs::{
    crdt::{CrdtDocument, ConflictResolver, CrdtError},
};
use serde_json::json;
use uuid::Uuid;

#[test]
fn test_crdt_document_creation() {
    let actor_id = "actor1".to_string();
    let doc = CrdtDocument::new(actor_id.clone());
    
    // Verify the actor ID is set correctly
    // Note: We can't directly access the actor ID, but we can verify the document was created
    assert!(doc.get_heads().is_empty());
}

#[test]
fn test_crdt_put_and_get() -> Result<(), CrdtError> {
    let mut doc = CrdtDocument::new("actor1".to_string());
    let root = doc.doc.get_object_root();
    
    // Put a value
    doc.put(&root, "key1", json!("value1"))?;
    
    // Get the value back
    let value = doc.get(&root, "key1")?;
    assert_eq!(value, Some(json!("value1")));
    
    // Put a complex object
    let complex_value = json!({
        "name": "John Doe",
        "age": 30,
        "active": true,
        "scores": [10, 20, 30]
    });
    
    doc.put(&root, "user", complex_value.clone())?;
    
    // Get the complex object back
    let retrieved = doc.get(&root, "user")?;
    assert_eq!(retrieved, Some(complex_value));
    
    Ok(())
}

#[test]
fn test_crdt_merge_documents() -> Result<(), CrdtError> {
    // Create two documents
    let mut doc1 = CrdtDocument::new("actor1".to_string());
    let doc2 = CrdtDocument::new("actor2".to_string());
    let root1 = doc1.doc.get_object_root();
    let root2 = doc2.doc.get_object_root();
    
    // Make changes to both documents
    doc1.put(&root1, "key1", json!("value1"))?;
    doc2.put(&root2, "key2", json!("value2"))?;
    
    // Get the initial heads of doc1
    let heads1 = doc1.get_heads();
    
    // Get changes from doc2 that are not in doc1
    let changes = doc2.get_changes(&heads1);
    
    // Apply changes from doc2 to doc1
    doc1.apply_changes(&changes)?;
    
    // Verify both values are now in doc1
    let value1 = doc1.get(&root1, "key1")?;
    let value2 = doc1.get(&root1, "key2")?;
    
    assert_eq!(value1, Some(json!("value1")));
    assert_eq!(value2, Some(json!("value2")));
    
    Ok(())
}

#[test]
fn test_conflict_resolver() -> Result<(), CrdtError> {
    // Create multiple documents
    let mut doc1 = CrdtDocument::new("actor1".to_string());
    let mut doc2 = CrdtDocument::new("actor2".to_string());
    let mut doc3 = CrdtDocument::new("actor3".to_string());
    
    let root1 = doc1.doc.get_object_root();
    let root2 = doc2.doc.get_object_root();
    let root3 = doc3.doc.get_object_root();
    
    // Make initial change to doc1
    doc1.put(&root1, "shared_key", json!("initial_value"))?;
    
    // Get heads before making concurrent changes
    let heads = doc1.get_heads();
    
    // Make concurrent changes to doc2 and doc3
    doc2.put(&root2, "shared_key", json!("value_from_actor2"))?;
    doc3.put(&root3, "shared_key", json!("value_from_actor3"))?;
    
    // Apply changes from doc2 to doc1
    let changes2 = doc2.get_changes(&heads);
    doc1.apply_changes(&changes2)?;
    
    // Apply changes from doc3 to doc1
    let changes3 = doc3.get_changes(&heads);
    doc1.apply_changes(&changes3)?;
    
    // The final value should be deterministically resolved
    // In Automerge, the actor with the "highest" actor ID wins in case of conflict
    let final_value = doc1.get(&root1, "shared_key")?;
    
    // Since "actor3" > "actor2" lexicographically, actor3's value should win
    assert_eq!(final_value, Some(json!("value_from_actor3")));
    
    Ok(())
}

#[test]
fn test_crdt_serialization() -> Result<(), CrdtError> {
    // Create a document with some data
    let mut doc = CrdtDocument::new("actor1".to_string());
    let root = doc.doc.get_object_root();
    
    doc.put(&root, "name", json!("Test Document"))?;
    doc.put(&root, "content", json!("This is test content"))?;
    
    // Serialize the document
    let data = doc.save()?;
    
    // Create a new document from the serialized data
    let mut loaded_doc = CrdtDocument::load(&data, "actor2".to_string())?;
    
    // Verify the data was loaded correctly
    let name = loaded_doc.get(&loaded_doc.doc.get_object_root(), "name")?;
    let content = loaded_doc.get(&loaded_doc.doc.get_object_root(), "content")?;
    
    assert_eq!(name, Some(json!("Test Document")));
    assert_eq!(content, Some(json!("This is test content")));
    
    // Make a change to the loaded document
    loaded_doc.put(&loaded_doc.doc.get_object_root(), "new_field", json!("new_value"))?;
    
    // Verify the change was applied
    let new_field = loaded_doc.get(&loaded_doc.doc.get_object_root(), "new_field")?;
    assert_eq!(new_field, Some(json!("new_value")));
    
    Ok(())
}

#[test]
fn test_crdt_list_operations() -> Result<(), CrdtError> {
    let mut doc = CrdtDocument::new("actor1".to_string());
    let root = doc.doc.get_object_root();
    
    // Create a list
    let list_id = {
        let mut tx = doc.doc.transaction();
        let id = tx.put_object(&root, "items", automerge::ObjType::List)?;
        tx.commit();
        id
    };
    
    // Add items to the list
    {
        let mut tx = doc.doc.transaction();
        tx.insert(&list_id, 0, "item1")?;
        tx.insert(&list_id, 1, "item2")?;
        tx.insert(&list_id, 2, "item3")?;
        tx.commit();
    }
    
    // Verify list contents
    // Note: For simplicity, we're not implementing list retrieval in our wrapper
    // In a real implementation, we would have methods to work with lists
    
    assert_eq!(doc.doc.length(&list_id), 3);
    
    Ok(())
}
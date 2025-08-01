//! Unit tests for operation conversion logic

use cpc_document_editor::crdt::operations::{DocumentOperation, FormatType, FormattingStyle};
use cpc_document_editor::crdt::id::CRDTId;
use cpc_document_editor::collaboration::conversion;
use collaboration_engine::core::{Operation, Position};
use uuid::Uuid;
use chrono::Utc;

#[test]
fn test_document_operation_to_engine_operation_conversion() {
    let user_id = Uuid::new_v4();
    
    // Test Insert conversion
    let doc_op = DocumentOperation::Insert {
        position: 1005,
        value: serde_json::Value::String("Hello".to_string()),
        id: CRDTId::new(user_id, 1234567890),
        parent_id: None,
    };
    let engine_op: Operation = doc_op.into();
    match engine_op {
        Operation::Insert { position, text, .. } => {
            assert_eq!(position.line, 1);
            assert_eq!(position.column, 5);
            assert_eq!(text, "Hello");
        },
        _ => panic!("Expected Insert operation"),
    }
    
    // Test Delete conversion
    let doc_op = DocumentOperation::Delete {
        id: CRDTId::new(user_id, 1234567890),
        timestamp: 1234567890,
    };
    let engine_op: Operation = doc_op.into();
    match engine_op {
        Operation::Delete { start, end, .. } => {
            assert_eq!(start.line, 0);
            assert_eq!(start.column, 0);
            assert_eq!(end.line, 0);
            assert_eq!(end.column, 1);
        },
        _ => panic!("Expected Delete operation"),
    }
    
    // Test Update conversion
    let doc_op = DocumentOperation::Update {
        id: CRDTId::new(user_id, 1234567890),
        value: serde_json::Value::String("Updated".to_string()),
        timestamp: 1234567890,
    };
    let engine_op: Operation = doc_op.into();
    match engine_op {
        Operation::Replace { start, end, text, .. } => {
            assert_eq!(start.line, 0);
            assert_eq!(start.column, 0);
            assert_eq!(end.line, 0);
            assert_eq!(end.column, 1);
            assert_eq!(text, "Updated");
        },
        _ => panic!("Expected Replace operation"),
    }
    
    // Test Formatting conversion
    let doc_op = DocumentOperation::Formatting {
        id: CRDTId::new(user_id, 1234567890),
        style: FormattingStyle::Bold,
        timestamp: 1234567890,
    };
    let engine_op: Operation = doc_op.into();
    match engine_op {
        Operation::Insert { text, .. } => {
            assert_eq!(text, "");
        },
        _ => panic!("Expected Insert operation"),
    }
    
    // Test Format conversion
    let doc_op = DocumentOperation::Format {
        range: (Position { line: 0, column: 0 }, Position { line: 0, column: 5 }),
        format: FormatType::Bold,
    };
    let engine_op: Operation = doc_op.into();
    match engine_op {
        Operation::Insert { text, .. } => {
            assert!(text.contains("Bold"));
        },
        _ => panic!("Expected Insert operation"),
    }
    
    // Test InsertImage conversion
    let image_id = Uuid::new_v4();
    let doc_op = DocumentOperation::InsertImage {
        position: Position { line: 2, column: 10 },
        image_id,
        caption: "Test image".to_string(),
    };
    let engine_op: Operation = doc_op.into();
    match engine_op {
        Operation::Insert { position, text, .. } => {
            assert_eq!(position.line, 2);
            assert_eq!(position.column, 10);
            assert!(text.starts_with("[IMAGE:"));
            assert!(text.contains(&image_id.to_string()));
            assert!(text.contains("Test image"));
        },
        _ => panic!("Expected Insert operation"),
    }
}

#[test]
fn test_engine_operation_to_document_operation_conversion() {
    let user_id = Uuid::new_v4();
    
    // Test Insert conversion
    let engine_op = Operation::Insert {
        position: Position { line: 1, column: 5 },
        text: "Hello".to_string(),
        user_id,
        timestamp: Utc::now(),
    };
    let doc_op: Result<DocumentOperation, _> = engine_op.try_into();
    assert!(doc_op.is_ok());
    match doc_op.unwrap() {
        DocumentOperation::Insert { position, value, .. } => {
            assert_eq!(position, 1005);
            assert_eq!(value, serde_json::Value::String("Hello".to_string()));
        },
        _ => panic!("Expected Insert operation"),
    }
    
    // Test Delete conversion
    let engine_op = Operation::Delete {
        start: Position { line: 0, column: 0 },
        end: Position { line: 0, column: 5 },
        user_id,
        timestamp: Utc::now(),
    };
    let doc_op: Result<DocumentOperation, _> = engine_op.try_into();
    assert!(doc_op.is_ok());
    match doc_op.unwrap() {
        DocumentOperation::Delete { .. } => {
            // Success
        },
        _ => panic!("Expected Delete operation"),
    }
    
    // Test Replace conversion
    let engine_op = Operation::Replace {
        start: Position { line: 0, column: 0 },
        end: Position { line: 0, column: 5 },
        text: "Updated".to_string(),
        user_id,
        timestamp: Utc::now(),
    };
    let doc_op: Result<DocumentOperation, _> = engine_op.try_into();
    assert!(doc_op.is_ok());
    match doc_op.unwrap() {
        DocumentOperation::Update { value, .. } => {
            assert_eq!(value, serde_json::Value::String("Updated".to_string()));
        },
        _ => panic!("Expected Update operation"),
    }
}

#[test]
fn test_image_placeholder_conversion() {
    let user_id = Uuid::new_v4();
    let image_id = Uuid::new_v4();
    
    // Test conversion of image placeholder back to InsertImage
    let engine_op = Operation::Insert {
        position: Position { line: 2, column: 10 },
        text: format!("[IMAGE:{}:Test caption]", image_id),
        user_id,
        timestamp: Utc::now(),
    };
    let doc_op: Result<DocumentOperation, _> = engine_op.try_into();
    assert!(doc_op.is_ok());
    match doc_op.unwrap() {
        DocumentOperation::InsertImage { position, image_id: id, caption } => {
            assert_eq!(position.line, 2);
            assert_eq!(position.column, 10);
            assert_eq!(id, image_id);
            assert_eq!(caption, "Test caption");
        },
        _ => panic!("Expected InsertImage operation"),
    }
}

#[test]
fn test_invalid_image_placeholder_conversion() {
    let user_id = Uuid::new_v4();
    
    // Test conversion of invalid image placeholder (treated as regular text)
    let engine_op = Operation::Insert {
        position: Position { line: 2, column: 10 },
        text: "[IMAGE:invalid-id:Test caption]".to_string(), // Invalid UUID
        user_id,
        timestamp: Utc::now(),
    };
    let doc_op: Result<DocumentOperation, _> = engine_op.try_into();
    assert!(doc_op.is_ok());
    match doc_op.unwrap() {
        DocumentOperation::Insert { position, value, .. } => {
            assert_eq!(position, 2010); // 2 * 1000 + 10
            assert_eq!(value, serde_json::Value::String("[IMAGE:invalid-id:Test caption]".to_string()));
        },
        _ => panic!("Expected Insert operation"),
    }
}
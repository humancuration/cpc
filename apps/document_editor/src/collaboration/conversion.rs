//! Conversion utilities between document editor operations and collaboration engine operations

use crate::crdt::operations::{DocumentOperation, FormatType};
use collaboration_engine::core::{Operation, Position};
use uuid::Uuid;
use chrono::Utc;
use crate::domain::errors::DocumentError;

/// Convert a DocumentOperation to a collaboration engine Operation
impl From<DocumentOperation> for Operation {
    fn from(doc_op: DocumentOperation) -> Self {
        match doc_op {
            DocumentOperation::Insert { position, value, .. } => {
                // Convert position from usize to Position struct
                let pos = Position {
                    line: position / 1000, // Simple conversion - in a real implementation, this would be more sophisticated
                    column: position % 1000,
                };
                
                // Extract text from value
                let text = if let Ok(text) = serde_json::from_value::<String>(value.clone()) {
                    text
                } else if let Ok(num) = serde_json::from_value::<i64>(value) {
                    num.to_string()
                } else {
                    // Default to empty string if we can't convert
                    String::new()
                };
                
                Operation::Insert {
                    position: pos,
                    text,
                    user_id: Uuid::nil(), // This should be set properly in a real implementation
                    timestamp: Utc::now(),
                }
            },
            DocumentOperation::Delete { .. } => {
                // For now, we'll create a simple delete operation
                // In a real implementation, we would need to determine the proper range
                Operation::Delete {
                    start: Position { line: 0, column: 0 },
                    end: Position { line: 0, column: 1 },
                    user_id: Uuid::nil(),
                    timestamp: Utc::now(),
                }
            },
            DocumentOperation::Update { .. } => {
                // Convert to a replace operation
                Operation::Replace {
                    start: Position { line: 0, column: 0 },
                    end: Position { line: 0, column: 1 },
                    text: String::new(),
                    user_id: Uuid::nil(),
                    timestamp: Utc::now(),
                }
            },
            DocumentOperation::Formatting { .. } => {
                // Formatting operations don't directly map to collaboration engine operations
                // We'll create a simple insert operation as a placeholder
                Operation::Insert {
                    position: Position { line: 0, column: 0 },
                    text: String::new(),
                    user_id: Uuid::nil(),
                    timestamp: Utc::now(),
                }
            },
            DocumentOperation::Format { range, format } => {
                // Format operations don't directly map to collaboration engine operations
                // We'll create a simple insert operation as a placeholder
                Operation::Insert {
                    position: Position { line: 0, column: 0 },
                    text: format!("{:?}", format), // Include format info in text for debugging
                    user_id: Uuid::nil(),
                    timestamp: Utc::now(),
                }
            },
            DocumentOperation::InsertImage { position, image_id, caption } => {
                // Image operations don't directly map to collaboration engine operations
                // We'll create an insert operation with metadata about the image
                Operation::Insert {
                    position: Position {
                        line: position.line,
                        column: position.column,
                    },
                    text: format!("[IMAGE:{}:{}]", image_id, caption),
                    user_id: Uuid::nil(),
                    timestamp: Utc::now(),
                }
            },
        }
    }
}

/// Convert a collaboration engine Operation to a DocumentOperation
impl TryFrom<Operation> for DocumentOperation {
    type Error = DocumentError;
    
    fn try_from(op: Operation) -> Result<Self, Self::Error> {
        match op {
            Operation::Insert { position, text, user_id, .. } => {
                // Check if this is an image placeholder
                if text.starts_with("[IMAGE:") && text.ends_with("]") {
                    // Parse image placeholder
                    let content = &text[7..text.len()-1]; // Remove [IMAGE: and ]
                    let parts: Vec<&str> = content.split(':').collect();
                    if parts.len() >= 2 {
                        let image_id = parts[0].parse::<Uuid>().map_err(|_| DocumentError::ConversionError("Invalid image ID".to_string()))?;
                        let caption = parts[1..].join(":"); // Join remaining parts as caption
                        
                        Ok(DocumentOperation::InsertImage {
                            position: Position {
                                line: position.line,
                                column: position.column,
                            },
                            image_id,
                            caption,
                        })
                    } else {
                        // Not a valid image placeholder, treat as regular insert
                        Ok(DocumentOperation::Insert {
                            position: position.line * 1000 + position.column, // Simple conversion back
                            value: serde_json::Value::String(text),
                            id: crate::crdt::id::CRDTId::new(user_id, 0), // This should be set properly in a real implementation
                            parent_id: None,
                        })
                    }
                } else {
                    // Regular text insert
                    Ok(DocumentOperation::Insert {
                        position: position.line * 1000 + position.column, // Simple conversion back
                        value: serde_json::Value::String(text),
                        id: crate::crdt::id::CRDTId::new(user_id, 0), // This should be set properly in a real implementation
                        parent_id: None,
                    })
                }
            },
            Operation::Delete { start, end, .. } => {
                // Convert to a delete operation with a placeholder ID
                Ok(DocumentOperation::Delete {
                    id: crate::crdt::id::CRDTId::new(Uuid::nil(), 0), // This should be set properly in a real implementation
                    timestamp: 0, // This should be set properly in a real implementation
                })
            },
            Operation::Replace { start, end, text, .. } => {
                // Convert to an update operation with a placeholder ID
                Ok(DocumentOperation::Update {
                    id: crate::crdt::id::CRDTId::new(Uuid::nil(), 0), // This should be set properly in a real implementation
                    value: serde_json::Value::String(text),
                    timestamp: 0, // This should be set properly in a real implementation
                })
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crdt::id::CRDTId;
    use chrono::Utc;
    
    #[test]
    fn test_document_operation_to_operation_conversion() {
        // Test Insert conversion
        let doc_op = DocumentOperation::Insert {
            position: 1005,
            value: serde_json::Value::String("Hello".to_string()),
            id: CRDTId::new(Uuid::new_v4(), 1234567890),
            parent_id: None,
        };
        let op: Operation = doc_op.into();
        match op {
            Operation::Insert { position, text, .. } => {
                assert_eq!(position.line, 1);
                assert_eq!(position.column, 5);
                assert_eq!(text, "Hello");
            },
            _ => panic!("Expected Insert operation"),
        }
        
        // Test InsertImage conversion
        let doc_op = DocumentOperation::InsertImage {
            position: Position { line: 2, column: 10 },
            image_id: Uuid::new_v4(),
            caption: "Test image".to_string(),
        };
        let op: Operation = doc_op.into();
        match op {
            Operation::Insert { position, text, .. } => {
                assert_eq!(position.line, 2);
                assert_eq!(position.column, 10);
                assert!(text.starts_with("[IMAGE:"));
            },
            _ => panic!("Expected Insert operation"),
        }
    }
    
    #[test]
    fn test_operation_to_document_operation_conversion() {
        // Test Insert conversion
        let op = Operation::Insert {
            position: Position { line: 1, column: 5 },
            text: "Hello".to_string(),
            user_id: Uuid::new_v4(),
            timestamp: Utc::now(),
        };
        let doc_op: Result<DocumentOperation, DocumentError> = op.try_into();
        assert!(doc_op.is_ok());
        match doc_op.unwrap() {
            DocumentOperation::Insert { position, value, .. } => {
                assert_eq!(position, 1005);
                assert_eq!(value, serde_json::Value::String("Hello".to_string()));
            },
            _ => panic!("Expected Insert operation"),
        }
    }
    
    #[test]
    fn test_format_operation_conversion() {
        // Test Format conversion
        let doc_op = DocumentOperation::Format {
            range: (Position { line: 0, column: 0 }, Position { line: 0, column: 5 }),
            format: FormatType::Bold,
        };
        let op: Operation = doc_op.into();
        match op {
            Operation::Insert { text, .. } => {
                assert!(text.contains("Bold"));
            },
            _ => panic!("Expected Insert operation"),
        }
    }
}
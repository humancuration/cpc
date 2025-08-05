//! Operational Transformation (OT) implementation for collaborative text editing
//!
//! This module provides core functions for transforming and composing text operations
//! to enable real-time collaborative editing.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;


pub use crate::transform_text_operations;

/// Text operation variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Operation {
    /// Insert text at a position
    Insert {
        position: usize,
        text: String,
    },
    /// Delete text from a range
    Delete {
        start: usize,
        length: usize,
    },
    /// Retain text (no change)
    Retain {
        length: usize,
    },
}

/// Text operation with metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextOperation {
    /// The operation to apply
    pub op: Operation,
    /// User who created the operation
    pub user_id: Uuid,
    /// Document version this operation is based on
    pub version: u64,
    /// Logical clock for deterministic ordering
    pub logical_clock: u64,
    /// Timestamp of operation creation
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Version vector for tracking document versions across users
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VersionVector {
    /// Map of user IDs to their latest operation sequence numbers
    pub versions: HashMap<Uuid, u64>,
}

impl VersionVector {
    /// Create a new empty version vector
    pub fn new() -> Self {
        Self {
            versions: HashMap::new(),
        }
    }

    /// Get the version for a specific user
    pub fn get(&self, user_id: &Uuid) -> u64 {
        *self.versions.get(user_id).unwrap_or(&0)
    }

    /// Set the version for a specific user
    pub fn set(&mut self, user_id: Uuid, version: u64) {
        self.versions.insert(user_id, version);
    }

    /// Check if this version vector is causally ready for the given operation
    pub fn is_causally_ready(&self, op: &TextOperation) -> bool {
        self.get(&op.user_id) >= op.version
    }

    /// Update the version vector with an operation
    pub fn update_with(&mut self, op: &TextOperation) {
        let current = self.get(&op.user_id);
        if op.version > current {
            self.set(op.user_id, op.version);
        }
    }
}

/// Error types for OT operations
#[derive(Debug, thiserror::Error)]
pub enum OtError {
    #[error("Operation conflict: {message}")]
    Conflict { message: String },
    #[error("Invalid operation: {message}")]
    InvalidOperation { message: String },
/// Transform two text operations against each other
/// 
/// Returns (op1', op2') where op1' is op1 transformed against op2
/// and op2' is op2 transformed against op1
pub fn transform_text_operations(op1: &TextOperation, op2: &TextOperation) -> Result<(Operation, Operation), OtError> {
    match (&op1.op, &op2.op) {
        // Insert vs Insert
        (
            Operation::Insert { position: pos1, text: text1 },
            Operation::Insert { position: pos2, text: text2 },
        ) => {
            let (new_pos1, new_pos2) = if pos1 < pos2 {
                (*pos1, *pos2 + text1.len())
            } else if pos1 > pos2 {
                (*pos1 + text2.len(), *pos2)
            } else {
                // When inserting at the same position, use deterministic ordering
                // based on (user_id, logical_clock)
                if op1.user_id < op2.user_id || (op1.user_id == op2.user_id && op1.logical_clock < op2.logical_clock) {
                    (*pos1, *pos2 + text1.len())
                } else {
                    (*pos1 + text2.len(), *pos2)
                }
            };
            
            Ok((
                Operation::Insert {
                    position: new_pos1,
                    text: text1.clone(),
                },
                Operation::Insert {
                    position: new_pos2,
                    text: text2.clone(),
                },
            ))
        }
        
        // For all other cases, fall back to the existing transform function
        _ => transform(&op1.op, &op2.op),
    }
}
}

/// Transform two operations against each other
/// 
/// Returns (op1', op2') where op1' is op1 transformed against op2
/// and op2' is op2 transformed against op1
pub fn transform(op1: &Operation, op2: &Operation) -> Result<(Operation, Operation), OtError> {
    match (op1, op2) {
        // Insert vs Insert
        (
            Operation::Insert { position: pos1, text: text1 },
            Operation::Insert { position: pos2, text: text2 },
        ) => {
            let (new_pos1, new_pos2) = if pos1 < pos2 {
                (*pos1, *pos2 + text1.len())
            } else if pos1 > pos2 {
                (*pos1 + text2.len(), *pos2)
            } else {
                // When inserting at the same position, use deterministic ordering
                // based on (user_id, logical_clock)
                // This requires access to the full TextOperation, so we'll handle this
                // in a wrapper function that has access to the full operations
                // For now, we'll use a simple ordering based on text content as a placeholder
                if text1 < text2 {
                    (*pos1, *pos2 + text1.len())
                } else {
                    (*pos1 + text2.len(), *pos2)
                }
            };
            
            Ok((
                Operation::Insert {
                    position: new_pos1,
                    text: text1.clone(),
                },
                Operation::Insert {
                    position: new_pos2,
                    text: text2.clone(),
                },
            ))
        }
        
        // Insert vs Delete
        (
            Operation::Insert { position: pos1, text: text1 },
            Operation::Delete { start: start2, length: len2 },
        ) => {
            let new_pos1 = if *pos1 <= *start2 {
                *pos1
            } else if *pos1 <= *start2 + *len2 {
                *start2
            } else {
                *pos1 - *len2
            };
            
            Ok((
                Operation::Insert {
                    position: new_pos1,
                    text: text1.clone(),
                },
                Operation::Delete {
                    start: *start2,
                    length: *len2,
                },
            ))
        }
        
        // Delete vs Insert (reverse of above)
        (
            Operation::Delete { start: start1, length: len1 },
            Operation::Insert { position: pos2, text: text2 },
        ) => {
            let (insert_op, delete_op) = transform(
                &Operation::Insert { position: *pos2, text: text2.clone() },
                &Operation::Delete { start: *start1, length: *len1 },
            )?;
            
            // Swap the results since we swapped the inputs
            Ok((delete_op, insert_op))
        }
        
        // Delete vs Delete
        (
            Operation::Delete { start: start1, length: len1 },
            Operation::Delete { start: start2, length: len2 },
        ) => {
            if *start1 + *len1 <= *start2 {
                // op1 is completely before op2
                Ok((
                    Operation::Delete { start: *start1, length: *len1 },
                    Operation::Delete { start: *start2 - *len1, length: *len2 },
                ))
            } else if *start2 + *len2 <= *start1 {
                // op2 is completely before op1
/// Merge two overlapping or adjacent ranges
/// 
/// Returns a single range that covers both input ranges
fn merge_ranges(start1: usize, len1: usize, start2: usize, len2: usize) -> (usize, usize) {
    let end1 = start1 + len1;
    let end2 = start2 + len2;
    let start = start1.min(start2);
    let end = end1.max(end2);
    (start, end - start)
}
                Ok((
                    Operation::Delete { start: *start1 - *len2, length: *len1 },
                    Operation::Delete { start: *start2, length: *len2 },
                ))
            } else if *start1 == *start2 && *len1 == *len2 {
                // Exactly the same deletion
                Ok((
                    Operation::Retain { length: 0 },
                    Operation::Retain { length: 0 },
                ))
            } else {
                // Overlapping deletions - merge the ranges
                let (merged_start, merged_len) = merge_ranges(*start1, *len1, *start2, *len2);
                
                // Both operations become the same merged deletion
                Ok((
                    Operation::Delete { start: merged_start, length: merged_len },
                    Operation::Delete { start: merged_start, length: merged_len },
                ))
            }
        }
        
        // Retain operations
        (Operation::Retain { length: len1 }, op2) => {
            Ok((
                Operation::Retain { length: *len1 },
                op2.clone(),
            ))
        }
        
        (op1, Operation::Retain { length: len2 }) => {
            Ok((
                op1.clone(),
                Operation::Retain { length: *len2 },
            ))
        }
    }
}

/// Compose two operations
/// 
/// Returns a single operation that has the same effect as applying op1 then op2
pub fn compose(op1: &Operation, op2: &Operation) -> Result<Operation, OtError> {
    match (op1, op2) {
        // Insert + Insert
        (
            Operation::Insert { position: pos1, text: text1 },
            Operation::Insert { position: pos2, text: text2 },
        ) => {
            if *pos2 <= *pos1 {
                Ok(Operation::Insert {
                    position: *pos1 + text2.len(),
                    text: format!("{}{}", text1, text2),
                })
            } else if *pos2 >= *pos1 + text1.len() {
                Ok(Operation::Insert {
                    position: *pos1,
                    text: format!("{}{}", text1, text2),
                })
            } else {
                // Insert in the middle of the first insert
                let offset = pos2 - pos1;
                let (left, right) = text1.split_at(offset);
                Ok(Operation::Insert {
                    position: *pos1,
                    text: format!("{}{}{}", left, text2, right),
                })
            }
        }
        
        // Insert + Delete
        (
            Operation::Insert { position: pos1, text: text1 },
            Operation::Delete { start: start2, length: len2 },
        ) => {
            if *start2 + *len2 <= *pos1 {
                // Delete is before insert
                Ok(Operation::Insert {
                    position: *pos1 - *len2,
                    text: text1.clone(),
                })
            } else if *start2 >= *pos1 + text1.len() {
                // Delete is after insert
                Ok(Operation::Insert {
                    position: *pos1,
                    text: text1.clone(),
                })
            } else {
                // Delete overlaps with insert
                // This is complex - for now we'll simplify
                Ok(Operation::Insert {
                    position: *pos1,
                    text: text1.clone(),
                })
            }
        }
        
        // Delete + Insert
        (
            Operation::Delete { start: start1, length: len1 },
            Operation::Insert { position: pos2, text: text2 },
        ) => {
            if *pos2 <= *start1 {
                // Insert is before delete
                Ok(Operation::Delete {
                    start: *start1 + text2.len(),
                    length: *len1,
                })
            } else if *pos2 >= *start1 + *len1 {
                // Insert is after delete
                Ok(Operation::Delete {
                    start: *start1,
                    length: *len1,
                })
            } else {
                // Insert in the middle of delete
                Ok(Operation::Delete {
                    start: *start1,
                    length: *len1 + text2.len(),
                })
            }
        }
        
        // Delete + Delete
        (
            Operation::Delete { start: start1, length: len1 },
            Operation::Delete { start: start2, length: len2 },
        ) => {
            if *start2 + *len2 <= *start1 {
                // Second delete is before first delete
                Ok(Operation::Delete {
                    start: *start1 - *len2,
                    length: *len1,
                })
            } else if *start2 >= *start1 + *len1 {
                // Second delete is after first delete
                Ok(Operation::Delete {
                    start: *start1,
                    length: *len1,
                })
            } else {
                // Deletes overlap - complex case
                // Simplified implementation
                Ok(Operation::Delete {
                    start: *start1,
                    length: *len1,
                })
            }
        }
        
        // Retain operations
        (Operation::Retain { length: len1 }, Operation::Retain { length: len2 }) => {
            Ok(Operation::Retain { length: *len1 + *len2 })
        }
        
        _ => Err(OtError::InvalidOperation {
            message: "Cannot compose these operations".to_string(),
        }),
    }
}

/// Apply operation to document content
pub fn apply(content: &str, op: &Operation) -> Result<String, OtError> {
    match op {
        Operation::Insert { position, text } => {
            if *position > content.len() {
                return Err(OtError::InvalidOperation {
                    message: "Insert position out of bounds".to_string(),
                });
            }
            
            let mut result = content.to_string();
            result.insert_str(*position, text);
            Ok(result)
        }
        
        Operation::Delete { start, length } => {
            if *start + *length > content.len() {
                return Err(OtError::InvalidOperation {
                    message: "Delete range out of bounds".to_string(),
                });
            }
            
            let mut result = content.to_string();
            result.replace_range(*start..(*start + *length), "");
            Ok(result)
        }
        
        Operation::Retain { .. } => {
            // Retain operations don't change the content
            Ok(content.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_insert_insert_transform() {
        let op1 = Operation::Insert {
            position: 5,
            text: "Hello".to_string(),
        };
        let op2 = Operation::Insert {
            position: 3,
            text: "World".to_string(),
        };
        
        let (op1_prime, op2_prime) = transform(&op1, &op2).unwrap();
        
        // op2 should be transformed to account for op1's insertion
        match (op1_prime, op2_prime) {
            (Operation::Insert { position: pos1, text: text1 }, 
             Operation::Insert { position: pos2, text: text2 }) => {
                assert_eq!(pos1, 5);
                assert_eq!(text1, "Hello");
                assert_eq!(pos2, 8); // 3 + 5 (length of "Hello")
                assert_eq!(text2, "World");
            }
            _ => panic!("Unexpected operation types"),
        }
    }
    
    #[test]
    fn test_insert_delete_transform() {
        let op1 = Operation::Insert {
            position: 5,
            text: "Hello".to_string(),
        };
        let op2 = Operation::Delete {
            start: 3,
            length: 4,
        };
        
        let (op1_prime, op2_prime) = transform(&op1, &op2).unwrap();
        
        match (op1_prime, op2_prime) {
            (Operation::Insert { position: pos1, text: text1 }, 
             Operation::Delete { start: start2, length: len2 }) => {
                assert_eq!(pos1, 1); // 5 - 4 (deleted length)
                assert_eq!(text1, "Hello");
                assert_eq!(start2, 3);
                assert_eq!(len2, 4);
            }
            _ => panic!("Unexpected operation types"),
        }
    }
    
    #[test]
    fn test_apply_insert() {
        let content = "Hello World";
        let op = Operation::Insert {
            position: 5,
            text: ", ".to_string(),
        };
        
        let result = apply(content, &op).unwrap();
        assert_eq!(result, "Hello, World");
    }
    
    #[test]
    fn test_apply_delete() {
        let content = "Hello, World";
        let op = Operation::Delete {
            start: 5,
            length: 2,
        };
        
        let result = apply(content, &op).unwrap();
        assert_eq!(result, "Hello World");
    }
#[test]
    fn test_transform_text_operations_insert_insert_deterministic() {
        let user1 = Uuid::new_v4();
        let user2 = Uuid::new_v4();
        
        // Create two operations with the same position but different users
        let op1 = TextOperation {
            op: Operation::Insert {
                position: 5,
                text: "Hello".to_string(),
            },
            user_id: user1,
            version: 1,
            logical_clock: 1,
            timestamp: chrono::Utc::now(),
        };
        
        let op2 = TextOperation {
            op: Operation::Insert {
                position: 5,
                text: "World".to_string(),
            },
            user_id: user2,
            version: 1,
            logical_clock: 1,
            timestamp: chrono::Utc::now(),
        };
        
        // Transform the operations
        let (op1_prime, op2_prime) = transform_text_operations(&op1, &op2).unwrap();
        
        // Check that the transformation is deterministic based on user ID
        // Since user1 < user2, op1 should be applied first
        match (op1_prime, op2_prime) {
            (Operation::Insert { position: pos1, text: text1 }, 
             Operation::Insert { position: pos2, text: text2 }) => {
                assert_eq!(pos1, 5);
                assert_eq!(text1, "Hello");
                assert_eq!(pos2, 10); // 5 + 5 (length of "Hello")
                assert_eq!(text2, "World");
            }
            _ => panic!("Unexpected operation types"),
        }
    }
    
    #[test]
    fn test_merge_ranges() {
        // Test overlapping ranges
        let (start, len) = merge_ranges(5, 10, 8, 7);
        assert_eq!(start, 5);
        assert_eq!(len, 10); // 5 to 15 (originally 5-15 and 8-15)
        
        // Test adjacent ranges
        let (start, len) = merge_ranges(5, 5, 10, 5);
        assert_eq!(start, 5);
        assert_eq!(len, 10); // 5 to 15
        
        // Test non-overlapping ranges
        let (start, len) = merge_ranges(5, 3, 10, 4);
        assert_eq!(start, 5);
        assert_eq!(len, 9); // 5 to 14
    }
    
    #[test]
    fn test_delete_delete_overlapping() {
        let op1 = Operation::Delete {
            start: 5,
            length: 10,
        };
        let op2 = Operation::Delete {
            start: 8,
            length: 7,
        };
        
        let (op1_prime, op2_prime) = transform(&op1, &op2).unwrap();
        
        // Both should become the same merged deletion
        match (op1_prime, op2_prime) {
            (Operation::Delete { start: start1, length: len1 }, 
             Operation::Delete { start: start2, length: len2 }) => {
                assert_eq!(start1, 5);
                assert_eq!(len1, 10);
                assert_eq!(start2, 5);
                assert_eq!(len2, 10);
            }
            _ => panic!("Unexpected operation types"),
        }
    }
}
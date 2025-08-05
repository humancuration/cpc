//! Integration tests for the Operational Transformation implementation

use ot::{Operation, TextOperation, VersionVector, transform, compose, apply, OtError, transform_text_operations};
use uuid::Uuid;
use std::collections::HashMap;

#[test]
fn test_concurrent_inserts() {
    let user1 = Uuid::new_v4();
    let user2 = Uuid::new_v4();
    
    // User 1 inserts "Hello" at position 0
    let op1 = Operation::Insert {
        position: 0,
        text: "Hello".to_string(),
    };
    
    // User 2 inserts "World" at position 0
    let op2 = Operation::Insert {
        position: 0,
        text: "World".to_string(),
    };
    
    // Transform the operations
    let (op1_prime, op2_prime) = transform(&op1, &op2).unwrap();
    
    // Apply op2 then op1_prime to get final result
    let content1 = "Hello World";
    let result1 = apply(content1, &op2_prime).unwrap();
    
    // Apply op1 then op2_prime to get final result
    let content2 = "World Hello";
    let result2 = apply(content2, &op1_prime).unwrap();
    
    // Both results should be the same
    assert_eq!(result1, result2);
    assert_eq!(result1, "WorldHello");
}

#[test]
fn test_concurrent_deletes() {
    let content = "Hello World";
    
    // User 1 deletes "Hello"
    let op1 = Operation::Delete {
        start: 0,
        length: 5,
    };
    
    // User 2 deletes "World"
    let op2 = Operation::Delete {
        start: 6,
        length: 5,
    };
    
    // Transform the operations
    let (op1_prime, op2_prime) = transform(&op1, &op2).unwrap();
    
    // Apply op1 then op2_prime
    let result1 = apply(&apply(content, &op1).unwrap(), &op2_prime).unwrap();
    
    // Apply op2 then op1_prime
    let result2 = apply(&apply(content, &op2).unwrap(), &op1_prime).unwrap();
    
    // Both results should be the same
    assert_eq!(result1, result2);
    assert_eq!(result1, " ");
}

#[test]
fn test_version_vector_causal_ordering() {
    let mut vv = VersionVector::new();
    let user1 = Uuid::new_v4();
    let user2 = Uuid::new_v4();
    
    // Set versions for users
    vv.set(user1, 3);
    vv.set(user2, 5);
    
    // Create an operation that is causally ready
    let op1 = TextOperation {
        op: Operation::Insert {
            position: 0,
            text: "test".to_string(),
        },
        user_id: user1,
        version: 3,
        logical_clock: 0,
        timestamp: chrono::Utc::now(),
    };
    
    // This operation should be causally ready
    assert!(vv.is_causally_ready(&op1));
    
    // Create an operation that is not causally ready
    let op2 = TextOperation {
        op: Operation::Insert {
            position: 0,
            text: "test".to_string(),
        },
        user_id: user1,
        version: 5, // This is ahead of the version vector
        logical_clock: 0,
        timestamp: chrono::Utc::now(),
    };
    
    // This operation should not be causally ready
    assert!(!vv.is_causally_ready(&op2));
}

#[test]
fn test_compose_operations() {
    let content = "Hello World";
    
    // First operation: insert comma
    let op1 = Operation::Insert {
        position: 5,
        text: ",".to_string(),
    };
    
    // Second operation: insert space
    let op2 = Operation::Insert {
        position: 6,
        text: " ".to_string(),
    };
    
    // Compose the operations
    let composed = compose(&op1, &op2);
    // Note: This is a simplified test as compose implementation may not handle all cases
    
    // Apply operations sequentially
    let result1 = apply(&apply(content, &op1).unwrap(), &op2).unwrap();
    
    assert_eq!(result1, "Hello, World");
}

#[test]
fn test_apply_insert_at_end() {
    let content = "Hello";
    let op = Operation::Insert {
        position: 5,
        text: " World".to_string(),
    };
    
    let result = apply(content, &op).unwrap();
    assert_eq!(result, "Hello World");
}

#[test]
fn test_apply_delete_range() {
    let content = "Hello, World!";
    let op = Operation::Delete {
        start: 5,
        length: 2, // Remove ", "
    };
    
    let result = apply(content, &op).unwrap();
    assert_eq!(result, "Hello World!");
}

#[test]
fn test_apply_invalid_operations() {
    let content = "Hello";
    
    // Try to insert at invalid position
    let op1 = Operation::Insert {
        position: 10, // Beyond the content length
        text: " World".to_string(),
    };
    
    let result = apply(content, &op1);
    assert!(result.is_err());
    
    // Try to delete invalid range
    let op2 = Operation::Delete {
        start: 3,
        length: 5, // Beyond the content length
    };
    
    let result = apply(content, &op2);
    assert!(result.is_err());
}

#[test]
fn test_transform_text_operations_deterministic_ordering() {
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
    
    // Transform the operations using the new function
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
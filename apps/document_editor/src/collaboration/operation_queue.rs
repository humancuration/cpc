use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::crdt::operations::DocumentOperation;
use crate::domain::errors::DocumentError;

/// OperationQueue handles persistent storage of operations when offline
pub struct OperationQueue {
    /// In-memory queue of operations
    queue: Arc<Mutex<VecDeque<QueuedOperation>>>,
    /// Maximum number of operations to keep in memory
    max_queue_size: usize,
}

/// A queued operation with metadata
#[derive(Debug, Clone)]
pub struct QueuedOperation {
    pub operation: DocumentOperation,
    pub document_id: Uuid,
    pub timestamp: i64,
    pub attempts: usize,
}

impl OperationQueue {
    /// Create a new operation queue
    pub fn new(max_queue_size: usize) -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            max_queue_size,
        }
    }
    
    /// Add an operation to the queue
    pub fn enqueue(&self, document_id: Uuid, operation: DocumentOperation) -> Result<(), DocumentError> {
        let mut queue = self.queue.lock().unwrap();
        
        let queued_operation = QueuedOperation {
            operation,
            document_id,
            timestamp: chrono::Utc::now().timestamp(),
            attempts: 0,
        };
        
        // Add to the back of the queue
        queue.push_back(queued_operation);
        
        // If we've exceeded the maximum queue size, remove the oldest operation
        if queue.len() > self.max_queue_size {
            queue.pop_front();
        }
        
        Ok(())
    }
    
    /// Get the next operation from the queue
    pub fn dequeue(&self) -> Option<QueuedOperation> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front()
    }
    
    /// Get all operations for a specific document
    pub fn get_operations_for_document(&self, document_id: Uuid) -> Vec<QueuedOperation> {
        let queue = self.queue.lock().unwrap();
        queue.iter()
            .filter(|op| op.document_id == document_id)
            .cloned()
            .collect()
    }
    
    /// Remove operations for a specific document
    pub fn remove_operations_for_document(&self, document_id: Uuid) {
        let mut queue = self.queue.lock().unwrap();
        queue.retain(|op| op.document_id != document_id);
    }
    
    /// Increment the attempt counter for an operation
    pub fn increment_attempts(&self, document_id: Uuid, operation: &DocumentOperation) {
        let mut queue = self.queue.lock().unwrap();
        for queued_op in queue.iter_mut() {
            if queued_op.document_id == document_id && 
               serde_json::to_string(&queued_op.operation).unwrap_or_default() == 
               serde_json::to_string(operation).unwrap_or_default() {
                queued_op.attempts += 1;
                break;
            }
        }
    }
    
    /// Get the current queue size
    pub fn size(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        queue.len()
    }
    
    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        let queue = self.queue.lock().unwrap();
        queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crdt::id::CRDTId;
    use serde_json::json;
    
    #[test]
    fn test_operation_queue_enqueue_dequeue() {
        let queue = OperationQueue::new(10);
        let document_id = Uuid::new_v4();
        
        // Create a test operation
        let id = CRDTId::new(Uuid::new_v4(), 1, 1);
        let operation = DocumentOperation::Insert {
            position: 0,
            value: json!({"text": "Hello, world!"}),
            id,
            parent_id: None,
        };
        
        // Enqueue an operation
        assert!(queue.enqueue(document_id, operation.clone()).is_ok());
        
        // Dequeue the operation
        let dequeued = queue.dequeue();
        assert!(dequeued.is_some());
        let dequeued_op = dequeued.unwrap();
        assert_eq!(dequeued_op.document_id, document_id);
        // Note: We can't directly compare operations because of the CRDTId, but we can check the type
        match (dequeued_op.operation, operation) {
            (DocumentOperation::Insert { position: pos1, .. }, DocumentOperation::Insert { position: pos2, .. }) => {
                assert_eq!(pos1, pos2);
            }
            _ => panic!("Operations don't match"),
        }
    }
    
    #[test]
    fn test_operation_queue_size_limit() {
        let queue = OperationQueue::new(3);
        let document_id = Uuid::new_v4();
        
        // Add more operations than the queue can hold
        for i in 0..5 {
            let id = CRDTId::new(Uuid::new_v4(), i, i as i64);
            let operation = DocumentOperation::Insert {
                position: i,
                value: json!({"text": format!("Operation {}", i)}),
                id,
                parent_id: None,
            };
            
            assert!(queue.enqueue(document_id, operation).is_ok());
        }
        
        // Queue should only contain the last 3 operations
        assert_eq!(queue.size(), 3);
    }
    
    #[test]
    fn test_operation_queue_document_filtering() {
        let queue = OperationQueue::new(10);
        let document_id1 = Uuid::new_v4();
        let document_id2 = Uuid::new_v4();
        
        // Add operations for two different documents
        let id1 = CRDTId::new(Uuid::new_v4(), 1, 1);
        let operation1 = DocumentOperation::Insert {
            position: 0,
            value: json!({"text": "Doc 1"}),
            id: id1,
            parent_id: None,
        };
        
        let id2 = CRDTId::new(Uuid::new_v4(), 1, 1);
        let operation2 = DocumentOperation::Insert {
            position: 0,
            value: json!({"text": "Doc 2"}),
            id: id2,
            parent_id: None,
        };
        
        assert!(queue.enqueue(document_id1, operation1).is_ok());
        assert!(queue.enqueue(document_id2, operation2).is_ok());
        
        // Get operations for document 1
        let doc1_ops = queue.get_operations_for_document(document_id1);
        assert_eq!(doc1_ops.len(), 1);
        assert_eq!(doc1_ops[0].document_id, document_id1);
        
        // Get operations for document 2
        let doc2_ops = queue.get_operations_for_document(document_id2);
        assert_eq!(doc2_ops.len(), 1);
        assert_eq!(doc2_ops[0].document_id, document_id2);
    }
}
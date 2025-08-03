//! Ring Buffer implementation
//!
//! This module provides a fixed-size circular buffer implementation that
//! can be used for efficient streaming data processing.

use std::collections::VecDeque;
use tokio::sync::Mutex;
use crate::error::Result;

/// Ring Buffer implementation
pub struct RingBuffer<T> {
    buffer: Mutex<VecDeque<T>>,
    capacity: usize,
}

impl<T> RingBuffer<T> {
    /// Create a new ring buffer with the specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Mutex::new(VecDeque::with_capacity(capacity)),
            capacity,
        }
    }

    /// Push a value into the ring buffer
    /// If the buffer is full, the oldest value will be removed
    pub async fn push(&self, value: T) {
        let mut buffer = self.buffer.lock().await;
        
        if buffer.len() >= self.capacity {
            buffer.pop_front(); // Remove the oldest element
        }
        
        buffer.push_back(value);
    }

    /// Pop a value from the ring buffer
    pub async fn pop(&self) -> Option<T> {
        let mut buffer = self.buffer.lock().await;
        buffer.pop_front()
    }

    /// Peek at the front value without removing it
    pub async fn front(&self) -> Option<T>
    where
        T: Clone,
    {
        let buffer = self.buffer.lock().await;
        buffer.front().cloned()
    }

    /// Get the current size of the buffer
    pub async fn len(&self) -> usize {
        let buffer = self.buffer.lock().await;
        buffer.len()
    }

    /// Check if the buffer is empty
    pub async fn is_empty(&self) -> bool {
        let buffer = self.buffer.lock().await;
        buffer.is_empty()
    }

    /// Check if the buffer is full
    pub async fn is_full(&self) -> bool {
        let buffer = self.buffer.lock().await;
        buffer.len() >= self.capacity
    }

    /// Clear the buffer
    pub async fn clear(&self) {
        let mut buffer = self.buffer.lock().await;
        buffer.clear();
    }

    /// Get all values in the buffer as a vector
    pub async fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let buffer = self.buffer.lock().await;
        buffer.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ring_buffer_basic() {
        let buffer = RingBuffer::new(3);
        
        buffer.push(1).await;
        buffer.push(2).await;
        buffer.push(3).await;
        
        assert_eq!(buffer.len().await, 3);
        assert!(buffer.is_full().await);
        
        let values = buffer.to_vec().await;
        assert_eq!(values, vec![1, 2, 3]);
    }
    
    #[tokio::test]
    async fn test_ring_buffer_overflow() {
        let buffer = RingBuffer::new(3);
        
        buffer.push(1).await;
        buffer.push(2).await;
        buffer.push(3).await;
        buffer.push(4).await; // This should push out 1
        
        assert_eq!(buffer.len().await, 3);
        
        let values = buffer.to_vec().await;
        assert_eq!(values, vec![2, 3, 4]);
    }
    
    #[tokio::test]
    async fn test_ring_buffer_pop() {
        let buffer = RingBuffer::new(3);
        
        buffer.push(1).await;
        buffer.push(2).await;
        
        let popped = buffer.pop().await;
        assert_eq!(popped, Some(1));
        
        let popped = buffer.pop().await;
        assert_eq!(popped, Some(2));
        
        let popped = buffer.pop().await;
        assert_eq!(popped, None);
    }
    
    #[tokio::test]
    async fn test_ring_buffer_clear() {
        let buffer = RingBuffer::new(3);
        
        buffer.push(1).await;
        buffer.push(2).await;
        
        assert!(!buffer.is_empty().await);
        
        buffer.clear().await;
        assert!(buffer.is_empty().await);
    }
}
//! Network status monitor with observer pattern
//!
//! This module implements a network status monitor that can notify observers
//! when network connectivity changes. It's designed to work with our composite
//! adapter pattern to automatically switch between online and offline implementations.

use std::sync::{Arc, RwLock};
use tokio::sync::watch;

/// Trait for observing network status changes
pub trait NetworkStatusObserver: Send + Sync {
    /// Called when network status changes
    fn on_network_status_changed(&self, is_connected: bool);
}

/// Network status monitor that tracks connectivity and notifies observers
pub struct NetworkStatusMonitor {
    /// Current connection status
    is_connected: Arc<RwLock<bool>>,
    
    /// Channel for notifying about network status changes
    sender: watch::Sender<bool>,
    
    /// Receiver for the current status
    receiver: watch::Receiver<bool>,
}

impl NetworkStatusMonitor {
    /// Create a new NetworkStatusMonitor
    pub fn new() -> Self {
        let (sender, receiver) = watch::channel(true); // Assume connected by default
        Self {
            is_connected: Arc::new(RwLock::new(true)),
            sender,
            receiver,
        }
    }
    
    /// Check if network is available
    pub fn is_connected(&self) -> bool {
        *self.is_connected.read().unwrap()
    }
    
    /// Set network connection status and notify observers
    pub fn set_connected(&self, connected: bool) {
        {
            let mut is_connected = self.is_connected.write().unwrap();
            *is_connected = connected;
        }
        
        // Notify all observers about the change
        let _ = self.sender.send(connected);
    }
    
    /// Subscribe to network status changes
    pub fn subscribe(&self) -> watch::Receiver<bool> {
        self.sender.subscribe()
    }
    
    /// Register an observer to be notified of network status changes
    pub fn register_observer(&self, _observer: Arc<dyn NetworkStatusObserver>) {
        // In a more complex implementation, we would store observers and notify them
        // For now, we're using the watch channel mechanism
    }
}

impl Default for NetworkStatusMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{timeout, Duration};

    #[tokio::test]
    async fn test_network_status_monitor() {
        let monitor = NetworkStatusMonitor::new();
        
        // Initially connected
        assert!(monitor.is_connected());
        
        // Change to disconnected
        monitor.set_connected(false);
        assert!(!monitor.is_connected());
        
        // Change back to connected
        monitor.set_connected(true);
        assert!(monitor.is_connected());
    }
    
    #[tokio::test]
    async fn test_network_status_subscription() {
        let monitor = NetworkStatusMonitor::new();
        let mut receiver = monitor.subscribe();
        
        // Initial value should be true (connected)
        assert!(receiver.borrow().clone());
        
        // Change to disconnected
        monitor.set_connected(false);
        
        // Wait for the change
        timeout(Duration::from_millis(100), receiver.changed())
            .await
            .expect("Timeout waiting for network status change")
            .expect("Failed to receive network status change");
            
        assert!(!receiver.borrow().clone());
        
        // Change back to connected
        monitor.set_connected(true);
        
        // Wait for the change
        timeout(Duration::from_millis(100), receiver.changed())
            .await
            .expect("Timeout waiting for network status change")
            .expect("Failed to receive network status change");
            
        assert!(receiver.borrow().clone());
    }
}
//! Metrics collection for database connection pools

use std::time::Duration;
use std::sync::atomic::{AtomicU64, AtomicU32, Ordering};

/// Metrics collector for database connection pools
pub struct MetricsCollector {
    /// Number of connections acquired
    connections_acquired: AtomicU64,
    
    /// Number of connections returned
    connections_returned: AtomicU64,
    
    /// Number of connections created
    connections_created: AtomicU64,
    
    /// Number of connections dropped
    connections_dropped: AtomicU64,
    
    /// Current number of idle connections
    idle_connections: AtomicU32,
    
    /// Current number of active connections
    active_connections: AtomicU32,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            connections_acquired: AtomicU64::new(0),
            connections_returned: AtomicU64::new(0),
            connections_created: AtomicU64::new(0),
            connections_dropped: AtomicU64::new(0),
            idle_connections: AtomicU32::new(0),
            active_connections: AtomicU32::new(0),
        }
    }

    /// Record that a connection was acquired
    pub fn record_connection_acquired(&self, _duration: Duration) {
        self.connections_acquired.fetch_add(1, Ordering::Relaxed);
        self.active_connections.fetch_add(1, Ordering::Relaxed);
        if self.idle_connections.load(Ordering::Relaxed) > 0 {
            self.idle_connections.fetch_sub(1, Ordering::Relaxed);
        }
    }

    /// Record that a connection was returned
    pub fn record_connection_returned(&self, _duration: Duration) {
        self.connections_returned.fetch_add(1, Ordering::Relaxed);
        if self.active_connections.load(Ordering::Relaxed) > 0 {
            self.active_connections.fetch_sub(1, Ordering::Relaxed);
        }
        self.idle_connections.fetch_add(1, Ordering::Relaxed);
    }

    /// Record that a connection was created
    pub fn record_connection_created(&self, _duration: Duration) {
        self.connections_created.fetch_add(1, Ordering::Relaxed);
    }

    /// Record that a connection was dropped
    pub fn record_connection_dropped(&self) {
        self.connections_dropped.fetch_add(1, Ordering::Relaxed);
        if self.active_connections.load(Ordering::Relaxed) > 0 {
            self.active_connections.fetch_sub(1, Ordering::Relaxed);
        } else if self.idle_connections.load(Ordering::Relaxed) > 0 {
            self.idle_connections.fetch_sub(1, Ordering::Relaxed);
        }
    }

    /// Get the current metrics
    pub fn get_metrics(&self) -> DatabaseMetrics {
        DatabaseMetrics {
            connections_acquired: self.connections_acquired.load(Ordering::Relaxed),
            connections_returned: self.connections_returned.load(Ordering::Relaxed),
            connections_created: self.connections_created.load(Ordering::Relaxed),
            connections_dropped: self.connections_dropped.load(Ordering::Relaxed),
            idle_connections: self.idle_connections.load(Ordering::Relaxed),
            active_connections: self.active_connections.load(Ordering::Relaxed),
        }
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Database metrics structure
#[derive(Debug, Clone)]
pub struct DatabaseMetrics {
    pub connections_acquired: u64,
    pub connections_returned: u64,
    pub connections_created: u64,
    pub connections_dropped: u64,
    pub idle_connections: u32,
    pub active_connections: u32,
}
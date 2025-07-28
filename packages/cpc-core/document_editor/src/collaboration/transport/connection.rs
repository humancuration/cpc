use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use uuid::Uuid;
use crate::collaboration::transport::error::NetworkError;

/// Connection state for a peer
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    ConnectionFailed,
}

/// Connection state machine for managing peer connections
pub struct ConnectionManager {
    connections: Arc<Mutex<HashMap<Uuid, ConnectionInfo>>>,
    max_retries: usize,
    initial_backoff: Duration,
    max_backoff: Duration,
}

/// Information about a connection to a peer
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub state: ConnectionState,
    pub last_attempt: Option<Instant>,
    pub retry_count: usize,
    pub backoff_duration: Duration,
    pub connected_at: Option<Instant>,
}

impl ConnectionManager {
    /// Create a new connection manager
    pub fn new(max_retries: usize, initial_backoff: Duration, max_backoff: Duration) -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
            max_retries,
            initial_backoff,
            max_backoff,
        }
    }

    /// Get the current state of a connection
    pub fn get_connection_state(&self, peer_id: Uuid) -> ConnectionState {
        let connections = self.connections.lock().unwrap();
        connections.get(&peer_id)
            .map(|info| info.state.clone())
            .unwrap_or(ConnectionState::Disconnected)
    }

    /// Set the state of a connection
    pub fn set_connection_state(&self, peer_id: Uuid, state: ConnectionState) {
        let mut connections = self.connections.lock().unwrap();
        let mut info = connections.entry(peer_id).or_insert_with(|| ConnectionInfo {
            state: ConnectionState::Disconnected,
            last_attempt: None,
            retry_count: 0,
            backoff_duration: self.initial_backoff,
            connected_at: None,
        });
        
        match &state {
            ConnectionState::Connecting => {
                info.last_attempt = Some(Instant::now());
            }
            ConnectionState::Connected => {
                info.connected_at = Some(Instant::now());
                info.retry_count = 0;
                info.backoff_duration = self.initial_backoff;
            }
            ConnectionState::ConnectionFailed => {
                info.retry_count += 1;
                // Exponential backoff with capped maximum
                let next_backoff = std::cmp::min(
                    info.backoff_duration * 2,
                    self.max_backoff
                );
                info.backoff_duration = next_backoff;
            }
            _ => {}
        }
        
        info.state = state;
    }

    /// Check if we should attempt to reconnect to a peer
    pub fn should_reconnect(&self, peer_id: Uuid) -> bool {
        let connections = self.connections.lock().unwrap();
        if let Some(info) = connections.get(&peer_id) {
            if info.state == ConnectionState::ConnectionFailed && info.retry_count < self.max_retries {
                if let Some(last_attempt) = info.last_attempt {
                    let elapsed = last_attempt.elapsed();
                    elapsed >= info.backoff_duration
                } else {
                    true
                }
            } else {
                false
            }
        } else {
            true // No connection info means we should try to connect
        }
    }

    /// Get connection info for a peer
    pub fn get_connection_info(&self, peer_id: Uuid) -> Option<ConnectionInfo> {
        let connections = self.connections.lock().unwrap();
        connections.get(&peer_id).cloned()
    }

    /// Reset connection state for a peer
    pub fn reset_connection(&self, peer_id: Uuid) {
        let mut connections = self.connections.lock().unwrap();
        connections.remove(&peer_id);
    }

    /// Get all connections that are in a specific state
    pub fn get_connections_in_state(&self, state: ConnectionState) -> Vec<Uuid> {
        let connections = self.connections.lock().unwrap();
        connections.iter()
            .filter(|(_, info)| info.state == state)
            .map(|(peer_id, _)| *peer_id)
            .collect()
    }

    /// Get the number of active connections
    pub fn active_connection_count(&self) -> usize {
        let connections = self.connections.lock().unwrap();
        connections.iter()
            .filter(|(_, info)| info.state == ConnectionState::Connected)
            .count()
    }
}

/// Connection statistics for monitoring
#[derive(Debug, Clone)]
pub struct ConnectionStats {
    pub total_connections: usize,
    pub active_connections: usize,
    pub failed_connections: usize,
    pub reconnecting_connections: usize,
    pub average_connection_time: Option<Duration>,
}

impl ConnectionManager {
    /// Get connection statistics
    pub fn get_stats(&self) -> ConnectionStats {
        let connections = self.connections.lock().unwrap();
        let total = connections.len();
        let mut active = 0;
        let mut failed = 0;
        let mut reconnecting = 0;
        let mut total_connection_time = Duration::new(0, 0);
        let mut connected_count = 0;

        for info in connections.values() {
            match info.state {
                ConnectionState::Connected => {
                    active += 1;
                    if let Some(connected_at) = info.connected_at {
                        total_connection_time += connected_at.elapsed();
                        connected_count += 1;
                    }
                }
                ConnectionState::ConnectionFailed => failed += 1,
                ConnectionState::Reconnecting => reconnecting += 1,
                _ => {}
            }
        }

        let average_connection_time = if connected_count > 0 {
            Some(total_connection_time / connected_count as u32)
        } else {
            None
        };

        ConnectionStats {
            total_connections: total,
            active_connections: active,
            failed_connections: failed,
            reconnecting_connections: reconnecting,
            average_connection_time,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_connection_state_transitions() {
        let manager = ConnectionManager::new(5, Duration::from_millis(100), Duration::from_secs(5));
        let peer_id = Uuid::new_v4();

        // Initial state should be disconnected
        assert_eq!(manager.get_connection_state(peer_id), ConnectionState::Disconnected);

        // Transition to connecting
        manager.set_connection_state(peer_id, ConnectionState::Connecting);
        assert_eq!(manager.get_connection_state(peer_id), ConnectionState::Connecting);

        // Transition to connected
        manager.set_connection_state(peer_id, ConnectionState::Connected);
        assert_eq!(manager.get_connection_state(peer_id), ConnectionState::Connected);
    }

    #[test]
    fn test_exponential_backoff() {
        let manager = ConnectionManager::new(5, Duration::from_millis(100), Duration::from_secs(1));
        let peer_id = Uuid::new_v4();

        // Simulate multiple connection failures
        for i in 0..4 {
            manager.set_connection_state(peer_id, ConnectionState::ConnectionFailed);
            let info = manager.get_connection_info(peer_id).unwrap();
            assert_eq!(info.retry_count, i + 1);
            
            // Backoff should double each time (up to max)
            let expected_backoff = if i < 3 {
                Duration::from_millis(100 * (2u32.pow(i as u32)))
            } else {
                Duration::from_millis(800) // Max backoff
            };
            assert_eq!(info.backoff_duration, expected_backoff);
        }
    }

    #[test]
    fn test_should_reconnect() {
        let manager = ConnectionManager::new(3, Duration::from_millis(100), Duration::from_secs(1));
        let peer_id = Uuid::new_v4();

        // Should reconnect when no connection info exists
        assert!(manager.should_reconnect(peer_id));

        // Set up a failed connection
        manager.set_connection_state(peer_id, ConnectionState::ConnectionFailed);
        
        // Should reconnect immediately after first failure (no backoff yet)
        assert!(manager.should_reconnect(peer_id));
    }
}
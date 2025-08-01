// DEPRECATED - Refactored to use collaboration_engine
// This file has been deprecated as part of the refactor to use the collaboration_engine package.
// The new implementation can be found in the application/collaboration_service.rs file.
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::collaboration::transport::error::NetworkError;

/// TURN server configuration
#[derive(Debug, Clone)]
pub struct TurnServerConfig {
    pub address: SocketAddr,
    pub username: String,
    pub password: String,
    pub realm: String,
}

/// TURN allocation information
#[derive(Debug, Clone)]
pub struct TurnAllocation {
    pub relay_address: SocketAddr,
    pub allocated_at: Instant,
    pub expires_at: Instant,
    pub permissions: Vec<SocketAddr>,
}

/// TURN client for fallback NAT traversal
pub struct TurnClient {
    servers: Vec<TurnServerConfig>,
    allocations: Arc<Mutex<HashMap<Uuid, TurnAllocation>>>,
    socket: UdpSocket,
    default_allocation_ttl: Duration,
}

impl TurnClient {
    /// Create a new TURN client
    pub fn new(
        servers: Vec<TurnServerConfig>,
        local_addr: SocketAddr,
        default_allocation_ttl: Duration,
    ) -> Result<Self, NetworkError> {
        let socket = UdpSocket::bind(local_addr)
            .map_err(|e| NetworkError::TurnError(format!("Failed to bind UDP socket: {}", e)))?;
        
        // Set timeout for TURN requests
        socket.set_read_timeout(Some(Duration::from_secs(10)))
            .map_err(|e| NetworkError::TurnError(format!("Failed to set socket timeout: {}", e)))?;
        
        Ok(Self {
            servers,
            allocations: Arc::new(Mutex::new(HashMap::new())),
            socket,
            default_allocation_ttl,
        })
    }

    /// Request a TURN allocation
    pub fn allocate(&self, document_id: Uuid) -> Result<SocketAddr, NetworkError> {
        // Try each TURN server until one responds
        for server_config in &self.servers {
            match self.allocate_from_server(document_id, server_config) {
                Ok(relay_address) => return Ok(relay_address),
                Err(e) => {
                    // Log error and try next server
                    tracing::warn!("TURN allocation failed for server {}: {}", server_config.address, e);
                    continue;
                }
            }
        }
        
        Err(NetworkError::NatTraversalFailed("All TURN servers failed".to_string()))
    }

    /// Request a TURN allocation from a specific server
    fn allocate_from_server(
        &self,
        document_id: Uuid,
        server_config: &TurnServerConfig,
    ) -> Result<SocketAddr, NetworkError> {
        // In a real implementation, this would:
        // 1. Perform TURN allocation handshake
        // 2. Authenticate with the server
        // 3. Request a relay address
        // 4. Return the relay address for use
        
        // For this implementation, we'll simulate the process
        let relay_address = SocketAddr::from(([127, 0, 0, 1], 50000 + (document_id.as_u128() % 10000) as u16));
        
        // Store allocation information
        let allocation = TurnAllocation {
            relay_address,
            allocated_at: Instant::now(),
            expires_at: Instant::now() + self.default_allocation_ttl,
            permissions: Vec::new(),
        };
        
        let mut allocations = self.allocations.lock().unwrap();
        allocations.insert(document_id, allocation);
        
        Ok(relay_address)
    }

    /// Create a permission for a peer to send data through the allocation
    pub fn create_permission(&self, document_id: Uuid, peer_addr: SocketAddr) -> Result<(), NetworkError> {
        let mut allocations = self.allocations.lock().unwrap();
        if let Some(allocation) = allocations.get_mut(&document_id) {
            // Check if allocation is still valid
            if allocation.expires_at > Instant::now() {
                // Add permission if it doesn't already exist
                if !allocation.permissions.contains(&peer_addr) {
                    allocation.permissions.push(peer_addr);
                }
                Ok(())
            } else {
                Err(NetworkError::TurnError("Allocation has expired".to_string()))
            }
        } else {
            Err(NetworkError::TurnError("No allocation found for document".to_string()))
        }
    }

    /// Send data through a TURN allocation
    pub fn send_data(&self, document_id: Uuid, peer_addr: SocketAddr, data: &[u8]) -> Result<(), NetworkError> {
        let allocations = self.allocations.lock().unwrap();
        if let Some(allocation) = allocations.get(&document_id) {
            // Check if allocation is still valid
            if allocation.expires_at > Instant::now() {
                // Check if we have permission for this peer
                if allocation.permissions.contains(&peer_addr) {
                    // In a real implementation, this would wrap the data in a TURN
                    // Send Indication or ChannelData message and send it to the TURN server
                    // For now, we'll just simulate the operation
                    tracing::debug!("Sending {} bytes through TURN relay to {}", data.len(), peer_addr);
                    Ok(())
                } else {
                    Err(NetworkError::TurnError("No permission for peer address".to_string()))
                }
            } else {
                Err(NetworkError::TurnError("Allocation has expired".to_string()))
            }
        } else {
            Err(NetworkError::TurnError("No allocation found for document".to_string()))
        }
    }

    /// Refresh an existing allocation
    pub fn refresh_allocation(&self, document_id: Uuid) -> Result<(), NetworkError> {
        let mut allocations = self.allocations.lock().unwrap();
        if let Some(allocation) = allocations.get_mut(&document_id) {
            // Check if allocation is still valid
            if allocation.expires_at > Instant::now() {
                // Extend the expiration time
                allocation.expires_at = Instant::now() + self.default_allocation_ttl;
                Ok(())
            } else {
                Err(NetworkError::TurnError("Allocation has expired".to_string()))
            }
        } else {
            Err(NetworkError::TurnError("No allocation found for document".to_string()))
        }
    }

    /// Check if an allocation exists and is still valid
    pub fn is_allocation_valid(&self, document_id: Uuid) -> bool {
        let allocations = self.allocations.lock().unwrap();
        if let Some(allocation) = allocations.get(&document_id) {
            allocation.expires_at > Instant::now()
        } else {
            false
        }
    }

    /// Get the relay address for an allocation
    pub fn get_relay_address(&self, document_id: Uuid) -> Option<SocketAddr> {
        let allocations = self.allocations.lock().unwrap();
        allocations.get(&document_id)
            .and_then(|allocation| {
                if allocation.expires_at > Instant::now() {
                    Some(allocation.relay_address)
                } else {
                    None
                }
            })
    }

    /// Clean up expired allocations
    pub fn cleanup_expired_allocations(&self) {
        let mut allocations = self.allocations.lock().unwrap();
        let now = Instant::now();
        allocations.retain(|_, allocation| allocation.expires_at > now);
    }

    /// Get statistics about current allocations
    pub fn get_allocation_stats(&self) -> (usize, usize) {
        let allocations = self.allocations.lock().unwrap();
        let total = allocations.len();
        let valid = allocations.values()
            .filter(|allocation| allocation.expires_at > Instant::now())
            .count();
        (total, valid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_turn_client_creation() {
        let servers = vec![
            TurnServerConfig {
                address: SocketAddr::from((Ipv4Addr::new(127, 0, 0, 1), 3478)),
                username: "test".to_string(),
                password: "test".to_string(),
                realm: "test".to_string(),
            }
        ];
        let local_addr = SocketAddr::from((Ipv4Addr::new(127, 0, 0, 1), 0));
        
        let client = TurnClient::new(servers, local_addr, Duration::from_secs(600));
        assert!(client.is_ok());
    }

    #[test]
    fn test_allocation_lifecycle() {
        let servers = vec![
            TurnServerConfig {
                address: SocketAddr::from((Ipv4Addr::new(127, 0, 0, 1), 3478)),
                username: "test".to_string(),
                password: "test".to_string(),
                realm: "test".to_string(),
            }
        ];
        let local_addr = SocketAddr::from((Ipv4Addr::new(127, 0, 0, 1), 0));
        let client = TurnClient::new(servers, local_addr, Duration::from_secs(600)).unwrap();
        let document_id = Uuid::new_v4();
        
        // Allocate
        let relay_addr = client.allocate(document_id);
        assert!(relay_addr.is_ok());
        
        // Check allocation is valid
        assert!(client.is_allocation_valid(document_id));
        
        // Get relay address
        let addr = client.get_relay_address(document_id);
        assert!(addr.is_some());
    }
}
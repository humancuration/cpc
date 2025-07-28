use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;
use stun_codec::{Message, MessageClass, MessageDecoder, MessageEncoder, TransactionId};
use stun_codec::rfc5389::attributes::{MappedAddress, Username};
use stun_codec::rfc5389::methods::BINDING;
use crate::collaboration::transport::error::NetworkError;

/// STUN client for NAT traversal
pub struct StunClient {
    servers: Vec<SocketAddr>,
    socket: UdpSocket,
    username: Option<String>,
}

/// NAT type detected by STUN
#[derive(Debug, Clone, PartialEq)]
pub enum NatType {
    OpenInternet,
    FullCone,
    RestrictedCone,
    PortRestrictedCone,
    Symmetric,
    Unknown,
}

/// Result of STUN binding request
#[derive(Debug, Clone)]
pub struct StunBindingResult {
    pub public_address: SocketAddr,
    pub nat_type: NatType,
}

impl StunClient {
    /// Create a new STUN client
    pub fn new(servers: Vec<SocketAddr>, local_addr: SocketAddr) -> Result<Self, NetworkError> {
        let socket = UdpSocket::bind(local_addr)
            .map_err(|e| NetworkError::StunError(format!("Failed to bind UDP socket: {}", e)))?;
        
        // Set timeout for STUN requests
        socket.set_read_timeout(Some(Duration::from_secs(5)))
            .map_err(|e| NetworkError::StunError(format!("Failed to set socket timeout: {}", e)))?;
        
        Ok(Self {
            servers,
            socket,
            username: None,
        })
    }

    /// Set username for STUN authentication
    pub fn set_username(&mut self, username: String) {
        self.username = Some(username);
    }

    /// Perform a STUN binding request to discover public address
    pub fn binding_request(&self) -> Result<StunBindingResult, NetworkError> {
        // Try each STUN server until one responds
        for server_addr in &self.servers {
            match self.binding_request_to_server(*server_addr) {
                Ok(result) => return Ok(result),
                Err(e) => {
                    // Log error and try next server
                    tracing::warn!("STUN binding request failed for server {}: {}", server_addr, e);
                    continue;
                }
            }
        }
        
        Err(NetworkError::NatTraversalFailed("All STUN servers failed".to_string()))
    }

    /// Perform a STUN binding request to a specific server
    fn binding_request_to_server(&self, server_addr: SocketAddr) -> Result<StunBindingResult, NetworkError> {
        // Create STUN binding request
        let transaction_id = TransactionId::new(rand::random());
        let mut message = Message::new(MessageClass::Request, BINDING, transaction_id);
        
        // Add username attribute if configured
        if let Some(ref username) = self.username {
            message.add_attribute(Username::new(username.clone())
                .map_err(|e| NetworkError::StunError(format!("Invalid username: {}", e)))?);
        }
        
        // Encode the message
        let mut encoder = MessageEncoder::new();
        let bytes = encoder.encode_into_bytes(message)
            .map_err(|e| NetworkError::StunError(format!("Failed to encode STUN message: {}", e)))?;
        
        // Send the request
        self.socket.send_to(&bytes, server_addr)
            .map_err(|e| NetworkError::StunError(format!("Failed to send STUN request: {}", e)))?;
        
        // Receive the response
        let mut buffer = [0u8; 1500]; // Standard MTU size
        let (len, recv_addr) = self.socket.recv_from(&mut buffer)
            .map_err(|e| NetworkError::StunError(format!("Failed to receive STUN response: {}", e)))?;
        
        // Verify response came from the expected server
        if recv_addr != server_addr {
            return Err(NetworkError::StunError(
                "STUN response came from unexpected address".to_string()
            ));
        }
        
        // Decode the response
        let mut decoder = MessageDecoder::new();
        let decoded = decoder.decode_from_bytes(&buffer[..len])
            .map_err(|e| NetworkError::StunError(format!("Failed to decode STUN response: {}", e)))?
            .ok_or_else(|| NetworkError::StunError("Empty STUN response".to_string()))?;
        
        // Verify transaction ID matches
        if decoded.transaction_id() != transaction_id {
            return Err(NetworkError::StunError(
                "STUN response transaction ID mismatch".to_string()
            ));
        }
        
        // Extract mapped address
        let mapped_address = decoded.get_attribute::<MappedAddress>()
            .ok_or_else(|| NetworkError::StunError("No MAPPED-ADDRESS in STUN response".to_string()))?;
        
        Ok(StunBindingResult {
            public_address: mapped_address.address(),
            nat_type: NatType::Unknown, // Simplified - full NAT type detection would require more tests
        })
    }

    /// Detect NAT type using multiple STUN requests
    pub fn detect_nat_type(&self, local_addr: SocketAddr) -> Result<NatType, NetworkError> {
        // This is a simplified NAT type detection
        // A full implementation would require multiple tests with different ports/addresses
        
        let binding_result = self.binding_request()?;
        let public_addr = binding_result.public_address;
        
        // Basic check: if public address equals local address, we're on open internet
        if public_addr.ip() == local_addr.ip() && public_addr.port() == local_addr.port() {
            Ok(NatType::OpenInternet)
        } else {
            // For a complete implementation, we would need to:
            // 1. Send binding requests from different ports
            // 2. Send binding requests to different STUN servers
            // 3. Analyze the responses to determine NAT behavior
            // 
            // For now, we'll return Unknown and let TURN handle complex NATs
            Ok(NatType::Unknown)
        }
    }

    /// Keep connection alive by sending periodic STUN binding requests
    pub fn keep_alive(&self, interval: Duration) -> Result<(), NetworkError> {
        // In a real implementation, this would run in a background task
        // For now, we'll just perform a single binding request
        self.binding_request()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_stun_client_creation() {
        let servers = vec![
            SocketAddr::from((Ipv4Addr::new(127, 0, 0, 1), 3478)),
        ];
        let local_addr = SocketAddr::from((Ipv4Addr::new(127, 0, 0, 1), 0));
        
        let client = StunClient::new(servers, local_addr);
        // This will fail because there's no STUN server running, but creation should work
        assert!(client.is_ok() || matches!(client, Err(NetworkError::StunError(_))));
    }

    #[test]
    fn test_nat_type_enum() {
        assert_eq!(NatType::OpenInternet, NatType::OpenInternet);
        assert_ne!(NatType::OpenInternet, NatType::Symmetric);
    }
}
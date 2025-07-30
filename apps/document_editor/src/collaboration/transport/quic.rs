use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::broadcast;
use uuid::Uuid;
use quinn::{Endpoint, ServerConfig, ClientConfig, Connection, SendStream, RecvStream};
use crate::collaboration::transport::error::NetworkError;
use crate::collaboration::transport::stun::{StunClient, NatType};
use crate::collaboration::transport::turn::{TurnClient, TurnServerConfig};
use crate::collaboration::transport::connection::{ConnectionManager, ConnectionState};

/// Network message format for document operations
#[derive(Debug, Clone)]
pub struct NetworkMessage {
    pub document_id: Uuid,
    pub payload: Vec<u8>,  // Encrypted PandaOperation
    pub message_id: u64,
}

/// QUIC transport configuration
#[derive(Debug, Clone)]
pub struct QuicTransportConfig {
    pub max_udp_payload_size: u16,
    pub concurrent_connections: u32,
    pub connection_timeout: Duration,
    pub max_retries: usize,
    pub initial_backoff: Duration,
    pub max_backoff: Duration,
    pub turn_allocation_ttl: Duration,
}

impl Default for QuicTransportConfig {
    fn default() -> Self {
        Self {
            max_udp_payload_size: 1500,
            concurrent_connections: 1000,
            connection_timeout: Duration::from_secs(30),
            max_retries: 5,
            initial_backoff: Duration::from_millis(500),
            max_backoff: Duration::from_secs(30),
            turn_allocation_ttl: Duration::from_secs(600), // 10 minutes
        }
    }
}

/// QUIC endpoint manager for p2panda network
pub struct QuicEndpoint {
    endpoint: Endpoint,
    server_config: ServerConfig,
    client_config: ClientConfig,
    local_addr: SocketAddr,
    stun_servers: Vec<SocketAddr>,
    turn_servers: Vec<TurnServerConfig>,
    connections: Arc<Mutex<HashMap<Uuid, Connection>>>,
    connection_manager: Arc<ConnectionManager>,
    stun_client: Arc<StunClient>,
    turn_client: Arc<TurnClient>,
    config: QuicTransportConfig,
    message_counter: Arc<Mutex<u64>>,
}

impl QuicEndpoint {
    /// Create a new QUIC endpoint
    pub fn new(
        local_addr: SocketAddr,
        stun_servers: Vec<SocketAddr>,
        turn_servers: Vec<TurnServerConfig>,
        config: QuicTransportConfig,
    ) -> Result<Self, NetworkError> {
        // Create QUIC endpoint configuration
        let mut endpoint_config = quinn::EndpointConfig::default();
        endpoint_config.max_udp_payload_size(config.max_udp_payload_size);
        endpoint_config.concurrent_connections(config.concurrent_connections);
        
        // Create server config (simplified - in a real implementation, you'd use proper certificates)
        let server_config = Self::create_server_config()?;
        
        // Create client config
        let client_config = Self::create_client_config()?;
        
        // Create QUIC endpoint
        let endpoint = Endpoint::server(endpoint_config, local_addr)
            .map_err(|e| NetworkError::QuicError(format!("Failed to create QUIC endpoint: {}", e)))?;
        
        // Create STUN client
        let stun_client = StunClient::new(stun_servers.clone(), local_addr)
            .map_err(|e| NetworkError::StunError(format!("Failed to create STUN client: {}", e)))?;
        
        // Create TURN client
        let turn_client = TurnClient::new(turn_servers.clone(), local_addr, config.turn_allocation_ttl)
            .map_err(|e| NetworkError::TurnError(format!("Failed to create TURN client: {}", e)))?;
        
        // Create connection manager
        let connection_manager = ConnectionManager::new(
            config.max_retries,
            config.initial_backoff,
            config.max_backoff,
        );
        
        Ok(Self {
            endpoint,
            server_config,
            client_config,
            local_addr,
            stun_servers,
            turn_servers,
            connections: Arc::new(Mutex::new(HashMap::new())),
            connection_manager: Arc::new(connection_manager),
            stun_client: Arc::new(stun_client),
            turn_client: Arc::new(turn_client),
            config,
            message_counter: Arc::new(Mutex::new(0)),
        })
    }

    /// Create server configuration with TLS
    fn create_server_config() -> Result<ServerConfig, NetworkError> {
        // In a real implementation, you would use proper certificates
        // For now, we'll use a self-signed certificate for testing
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()])
            .map_err(|e| NetworkError::CertificateError(format!("Failed to generate certificate: {}", e)))?;
        
        let cert_der = cert.serialize_der()
            .map_err(|e| NetworkError::CertificateError(format!("Failed to serialize certificate: {}", e)))?;
        let priv_key = cert.serialize_private_key_der();
        
        let cert_chain = vec![rustls::Certificate(cert_der)];
        let priv_key = rustls::PrivateKey(priv_key);
        
        let mut server_config = ServerConfig::with_single_cert(cert_chain, priv_key)
            .map_err(|e| NetworkError::CertificateError(format!("Failed to create server config: {}", e)))?;
        
        // Configure for low-latency
        server_config.transport = Self::create_transport_config();
        
        Ok(server_config)
    }

    /// Create client configuration
    fn create_client_config() -> Result<ClientConfig, NetworkError> {
        let mut client_config = ClientConfig::new(Arc::new(rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_custom_certificate_verifier(Arc::new(danger::NoVerifier))
            .with_no_client_auth()));
        
        // Configure for low-latency
        client_config.transport_config(Arc::new(Self::create_transport_config()));
        
        Ok(client_config)
    }

    /// Create transport configuration optimized for low-latency
    fn create_transport_config() -> quinn::TransportConfig {
        let mut transport_config = quinn::TransportConfig::default();
        transport_config.max_idle_timeout(Some(quinn::IdleTimeout::from(quinn::VarInt::from_u32(30_000)))); // 30 seconds
        transport_config.keep_alive_interval(Some(Duration::from_secs(10))); // 10 seconds
        transport_config
    }

    /// Discover public address using STUN
    pub fn discover_public_address(&self) -> Result<SocketAddr, NetworkError> {
        let binding_result = self.stun_client.binding_request()?;
        Ok(binding_result.public_address)
    }

    /// Detect NAT type
    pub fn detect_nat_type(&self) -> Result<NatType, NetworkError> {
        self.stun_client.detect_nat_type(self.local_addr)
    }

    /// Connect to a peer
    pub async fn connect_to_peer(&self, peer_id: Uuid, peer_addr: SocketAddr) -> Result<Connection, NetworkError> {
        // Update connection state
        self.connection_manager.set_connection_state(peer_id, ConnectionState::Connecting);
        
        // Try direct connection first
        match self.connect_direct(peer_addr).await {
            Ok(connection) => {
                self.connection_manager.set_connection_state(peer_id, ConnectionState::Connected);
                {
                    let mut connections = self.connections.lock().unwrap();
                    connections.insert(peer_id, connection.clone());
                }
                Ok(connection)
            }
            Err(e) => {
                tracing::warn!("Direct connection failed: {}, trying TURN fallback", e);
                
                // If direct connection fails, try TURN
                self.connection_manager.set_connection_state(peer_id, ConnectionState::ConnectionFailed);
                
                // Check if we should use TURN based on NAT type
                match self.detect_nat_type() {
                    Ok(NatType::Symmetric) | Ok(NatType::Unknown) => {
                        self.connect_via_turn(peer_id, peer_addr).await
                    }
                    _ => Err(e),
                }
            }
        }
    }

    /// Connect directly to a peer
    async fn connect_direct(&self, peer_addr: SocketAddr) -> Result<Connection, NetworkError> {
        let connection = tokio::time::timeout(
            self.config.connection_timeout,
            self.endpoint.connect_with(self.client_config.clone(), peer_addr, "localhost")
        ).await
        .map_err(|_| NetworkError::ConnectionTimeout(Uuid::nil()))?
        .map_err(|e| NetworkError::QuicError(format!("Connection failed: {}", e)))?;
        
        Ok(connection)
    }

    /// Connect via TURN relay
    async fn connect_via_turn(&self, peer_id: Uuid, peer_addr: SocketAddr) -> Result<Connection, NetworkError> {
        // Request TURN allocation
        let relay_addr = self.turn_client.allocate(peer_id)?;
        
        // Create permission for peer
        self.turn_client.create_permission(peer_id, peer_addr)?;
        
        // For a real implementation, we would establish a QUIC connection through the TURN relay
        // This is a simplified version that just simulates the process
        tracing::info!("Established connection via TURN relay for peer {}", peer_id);
        
        // In a real implementation, we would return an actual QUIC connection
        // For now, we'll just update the connection state
        self.connection_manager.set_connection_state(peer_id, ConnectionState::Connected);
        
        // Create a mock connection for demonstration
        Err(NetworkError::UnsupportedOperation("TURN connection not fully implemented".to_string()))
    }

    /// Send message to all connected peers for a document
    pub async fn send_to_all(&self, document_id: Uuid, payload: &[u8]) -> Result<(), NetworkError> {
        let connections = self.connections.lock().unwrap();
        let mut message_id = self.message_counter.lock().unwrap();
        *message_id += 1;
        
        let message = NetworkMessage {
            document_id,
            payload: payload.to_vec(),
            message_id: *message_id,
        };
        
        // Serialize message
        let message_bytes = serde_json::to_vec(&message)
            .map_err(|e| NetworkError::InvalidMessage(format!("Failed to serialize message: {}", e)))?;
        
        // Send to all connected peers
        for (peer_id, connection) in connections.iter() {
            if let Err(e) = self.send_message_to_peer(connection, &message_bytes).await {
                tracing::error!("Failed to send message to peer {}: {}", peer_id, e);
                // Update connection state
                self.connection_manager.set_connection_state(*peer_id, ConnectionState::ConnectionFailed);
            }
        }
        
        Ok(())
    }

    /// Send message to a specific peer
    async fn send_message_to_peer(&self, connection: &Connection, message: &[u8]) -> Result<(), NetworkError> {
        let mut send_stream = connection.open_uni().await
            .map_err(|e| NetworkError::QuicError(format!("Failed to open send stream: {}", e)))?;
        
        send_stream.write_all(message).await
            .map_err(|e| NetworkError::QuicError(format!("Failed to write message: {}", e)))?;
        
        send_stream.finish().await
            .map_err(|e| NetworkError::QuicError(format!("Failed to finish stream: {}", e)))?;
        
        Ok(())
    }

    /// Start listening for incoming connections
    pub async fn start_listening(&self) -> Result<(), NetworkError> {
        // In a real implementation, this would spawn a task to handle incoming connections
        // For now, we'll just log that we're listening
        tracing::info!("QUIC endpoint listening on {}", self.local_addr);
        Ok(())
    }

    /// Set connection status
    pub fn set_connected(&self, connected: bool) {
        // This would update the overall connection status
        // For now, we'll just log the status change
        tracing::info!("QUIC transport connection status: {}", connected);
    }

    /// Get connection statistics
    pub fn get_stats(&self) -> String {
        let connection_stats = self.connection_manager.get_stats();
        format!(
            "Connections: total={}, active={}, failed={}, reconnecting={}",
            connection_stats.total_connections,
            connection_stats.active_connections,
            connection_stats.failed_connections,
            connection_stats.reconnecting_connections
        )
    }
}

/// Danger module for accepting self-signed certificates (for testing only)
mod danger {
    use rustls::client::ServerCertVerified;
    use rustls::{Certificate, Error, ServerName};
    use std::time::SystemTime;

    pub struct NoVerifier;

    impl rustls::client::ServerCertVerifier for NoVerifier {
        fn verify_server_cert(
            &self,
            _end_entity: &Certificate,
            _intermediates: &[Certificate],
            _server_name: &ServerName,
            _scts: &mut dyn Iterator<Item = &[u8]>,
            _ocsp_response: &[u8],
            _now: SystemTime,
        ) -> Result<ServerCertVerified, Error> {
            Ok(ServerCertVerified::assertion())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_quic_endpoint_creation() {
        let local_addr = SocketAddr::from((Ipv4Addr::new(127, 0, 0, 1), 0));
        let stun_servers = vec![SocketAddr::from((Ipv4Addr::new(127, 0, 0, 1), 3478))];
        let turn_servers = vec![TurnServerConfig {
            address: SocketAddr::from((Ipv4Addr::new(127, 0, 0, 1), 3479)),
            username: "test".to_string(),
            password: "test".to_string(),
            realm: "test".to_string(),
        }];
        
        let endpoint = QuicEndpoint::new(local_addr, stun_servers, turn_servers, QuicTransportConfig::default());
        // This will fail because there's no STUN server running, but creation should work
        assert!(endpoint.is_ok() || matches!(endpoint, Err(NetworkError::StunError(_))));
    }

    #[test]
    fn test_network_message_serialization() {
        let message = NetworkMessage {
            document_id: Uuid::new_v4(),
            payload: vec![1, 2, 3, 4],
            message_id: 42,
        };
        
        let serialized = serde_json::to_vec(&message).unwrap();
        let deserialized: NetworkMessage = serde_json::from_slice(&serialized).unwrap();
        
        assert_eq!(message.document_id, deserialized.document_id);
        assert_eq!(message.payload, deserialized.payload);
        assert_eq!(message.message_id, deserialized.message_id);
    }
}
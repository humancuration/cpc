#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, SocketAddr};
    use std::time::Duration;
    use uuid::Uuid;

    #[test]
    fn test_quic_transport_config_default() {
        let config = QuicTransportConfig::default();
        
        assert_eq!(config.max_udp_payload_size, 1500);
        assert_eq!(config.concurrent_connections, 1000);
        assert_eq!(config.connection_timeout, Duration::from_secs(30));
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.initial_backoff, Duration::from_millis(500));
        assert_eq!(config.max_backoff, Duration::from_secs(30));
        assert_eq!(config.turn_allocation_ttl, Duration::from_secs(600));
    }

    #[test]
    fn test_network_message_serialization() {
        let message = NetworkMessage {
            document_id: Uuid::new_v4(),
            payload: vec![1, 2, 3, 4, 5],
            message_id: 42,
        };
        
        let serialized = serde_json::to_vec(&message).unwrap();
        let deserialized: NetworkMessage = serde_json::from_slice(&serialized).unwrap();
        
        assert_eq!(message.document_id, deserialized.document_id);
        assert_eq!(message.payload, deserialized.payload);
        assert_eq!(message.message_id, deserialized.message_id);
    }

    #[test]
    fn test_turn_server_config() {
        let config = TurnServerConfig {
            address: SocketAddr::from((Ipv4Addr::new(127, 0, 0, 1), 3478)),
            username: "test_user".to_string(),
            password: "test_password".to_string(),
            realm: "test_realm".to_string(),
        };
        
        assert_eq!(config.address, SocketAddr::from((Ipv4Addr::new(127, 0, 0, 1), 3478)));
        assert_eq!(config.username, "test_user");
        assert_eq!(config.password, "test_password");
        assert_eq!(config.realm, "test_realm");
    }

    #[test]
    fn test_nat_type_enum() {
        assert_eq!(NatType::OpenInternet, NatType::OpenInternet);
        assert_ne!(NatType::OpenInternet, NatType::Symmetric);
    }

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
    fn test_stun_binding_result() {
        let public_addr = SocketAddr::from((Ipv4Addr::new(192, 168, 1, 100), 12345));
        let result = StunBindingResult {
            public_address: public_addr,
            nat_type: NatType::Unknown,
        };
        
        assert_eq!(result.public_address, public_addr);
        assert_eq!(result.nat_type, NatType::Unknown);
    }

    #[test]
    fn test_turn_allocation() {
        let relay_addr = SocketAddr::from((Ipv4Addr::new(192, 168, 1, 100), 12345));
        let allocation = TurnAllocation {
            relay_address: relay_addr,
            allocated_at: std::time::Instant::now(),
            expires_at: std::time::Instant::now() + Duration::from_secs(600),
            permissions: vec![],
        };
        
        assert_eq!(allocation.relay_address, relay_addr);
    }

    #[test]
    fn test_network_error_variants() {
        let quic_error = NetworkError::QuicError("test error".to_string());
        match quic_error {
            NetworkError::QuicError(msg) => assert_eq!(msg, "test error"),
            _ => panic!("Wrong error variant"),
        }
        
        let stun_error = NetworkError::StunError("test error".to_string());
        match stun_error {
            NetworkError::StunError(msg) => assert_eq!(msg, "test error"),
            _ => panic!("Wrong error variant"),
        }
        
        let turn_error = NetworkError::TurnError("test error".to_string());
        match turn_error {
            NetworkError::TurnError(msg) => assert_eq!(msg, "test error"),
            _ => panic!("Wrong error variant"),
        }
    }
}
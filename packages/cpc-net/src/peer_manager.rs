//! Peer manager for handling NAT traversal and direct connections
//!
//! Manages peer-to-peer connections, NAT traversal, and connection lifecycle
//! for efficient chunk distribution and retrieval in the distributed file hosting system.

use libp2p_core::{PeerId, Multiaddr, Transport, upgrade};
use libp2p_core::muxing::StreamMuxerBox;
use libp2p_core::transport::{Boxed, MemoryTransport};
use libp2p_tcp::tokio::Transport as TcpTransport;
use libp2p_quic::tokio::Transport as QuicTransport;
use libp2p_identify::{Identify, IdentifyConfig, IdentifyEvent};
use libp2p_ping::{Ping, PingConfig, PingEvent};
use libp2p_swarm::{NetworkBehaviour, Swarm, SwarmBuilder, SwarmEvent};
use libp2p_swarm::behaviour::toggle::Toggle;
use libp2p_kad::{Kademlia, KademliaEvent};
use futures::StreamExt;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tracing::{info, warn, error};

/// Network behaviour for the peer manager
#[derive(NetworkBehaviour)]
pub struct PeerManagerBehaviour {
    /// Kademlia DHT for peer discovery
    pub kademlia: Kademlia<libp2p_kad::store::MemoryStore>,
    
    /// Identify protocol for peer information exchange
    pub identify: Identify,
    
    /// Ping protocol for connection health checks
    pub ping: Ping,
}

/// Peer connection information
#[derive(Debug, Clone)]
pub struct PeerConnection {
    pub peer_id: PeerId,
    pub multiaddr: Multiaddr,
    pub last_seen: Instant,
    pub rtt: Option<Duration>,
    pub is_relayed: bool,
}

/// Peer manager for handling P2P connections
pub struct PeerManager {
    swarm: Swarm<PeerManagerBehaviour>,
    connected_peers: HashMap<PeerId, PeerConnection>,
    bootstrap_nodes: Vec<(PeerId, Multiaddr)>,
    event_sender: mpsc::UnboundedSender<PeerManagerEvent>,
}

/// Events emitted by the peer manager
#[derive(Debug)]
pub enum PeerManagerEvent {
    PeerConnected(PeerId, Multiaddr),
    PeerDisconnected(PeerId),
    PeerUpdated(PeerId, PeerConnection),
    ChunkRequestReceived(PeerId, blake3::Hash),
    ChunkAvailable(PeerId, blake3::Hash),
}

impl PeerManager {
    /// Create a new peer manager
    pub fn new(
        local_key: libp2p_core::identity::Keypair,
        listen_addr: Multiaddr,
        bootstrap_nodes: Vec<(PeerId, Multiaddr)>,
    ) -> Result<(Self, mpsc::UnboundedReceiver<PeerManagerEvent>), PeerManagerError> {
        let local_peer_id = PeerId::from(local_key.public());
        
        // Build transport
        let transport = Self::build_transport(local_key)?;
        
        // Build network behaviour
        let behaviour = PeerManagerBehaviour {
            kademlia: Self::build_kademlia(local_peer_id),
            identify: Identify::new(IdentifyConfig::new(
                "/cpc/1.0.0".to_string(),
                local_key.public(),
            )),
            ping: Ping::new(PingConfig::new().with_timeout(Duration::from_secs(10))),
        };
        
        // Build swarm
        let swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, local_peer_id).build();
        
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        
        let mut manager = Self {
            swarm,
            connected_peers: HashMap::new(),
            bootstrap_nodes,
            event_sender,
        };
        
        // Start listening
        manager.swarm.listen_on(listen_addr)?;
        
        Ok((manager, event_receiver))
    }
    
    /// Build the transport stack for NAT traversal
    fn build_transport(
        keypair: libp2p_core::identity::Keypair,
    ) -> Result<Boxed<(PeerId, StreamMuxerBox)>, PeerManagerError> {
        // TCP transport with Noise encryption and Yamux multiplexing
        let tcp_transport = TcpTransport::new(libp2p_tcp::Config::default().nodelay(true))
            .upgrade(upgrade::Version::V1)
            .authenticate(libp2p_noise::NoiseAuthenticated::xx(&keypair)?)
            .multiplex(libp2p_yamux::YamuxConfig::default())
            .map(|(peer, muxer), _| (peer, StreamMuxerBox::new(muxer)))
            .boxed();
        
        // QUIC transport for NAT traversal
        let quic_transport = QuicTransport::new(libp2p_quic::Config::new(&keypair))
            .map(|(peer, muxer), _| (peer, StreamMuxerBox::new(muxer)))
            .boxed();
        
        // Combine transports
        Ok(libp2p_core::transport::OrTransport::new(quic_transport, tcp_transport).boxed())
    }
    
    /// Build Kademlia DHT
    fn build_kademlia(local_peer_id: PeerId) -> Kademlia<libp2p_kad::store::MemoryStore> {
        let store = libp2p_kad::store::MemoryStore::new(local_peer_id);
        let mut kademlia = Kademlia::new(local_peer_id, store);
        
        // Configure Kademlia
        kademlia.set_mode(Some(libp2p_kad::Mode::Server));
        
        kademlia
    }
    
    /// Bootstrap with initial peers
    pub fn bootstrap(&mut self) {
        for (peer_id, addr) in &self.bootstrap_nodes {
            self.swarm.behaviour_mut().kademlia.add_address(peer_id, addr.clone());
        }
        
        // Start bootstrap process
        if let Err(e) = self.swarm.behaviour_mut().kademlia.bootstrap() {
            warn!("Failed to start bootstrap: {}", e);
        }
    }
    
    /// Get list of connected peers
    pub fn get_connected_peers(&self) -> Vec<PeerConnection> {
        self.connected_peers.values().cloned().collect()
    }
    
    /// Get peers that have a specific chunk
    pub fn get_chunk_peers(&self, chunk_hash: &blake3::Hash) -> Vec<PeerId> {
        self.connected_peers.keys()
            .filter(|peer_id| {
                // In a real implementation, this would check the DHT
                // For now, assume all connected peers might have the chunk
                true
            })
            .cloned()
            .collect()
    }
    
    /// Connect to a specific peer
    pub fn connect_to_peer(&mut self, peer_id: PeerId, addr: Multiaddr) {
        self.swarm.behaviour_mut().kademlia.add_address(&peer_id, addr);
    }
    
    /// Handle swarm events
    pub async fn handle_event(&mut self, event: SwarmEvent<PeerManagerBehaviourEvent>) {
        match event {
            SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                let connection = PeerConnection {
                    peer_id,
                    multiaddr: endpoint.get_remote_address().clone(),
                    last_seen: Instant::now(),
                    rtt: None,
                    is_relayed: endpoint.is_relayed(),
                };
                
                self.connected_peers.insert(peer_id, connection.clone());
                
                let _ = self.event_sender.send(PeerManagerEvent::PeerConnected(
                    peer_id,
                    endpoint.get_remote_address().clone(),
                ));
                
                info!("Connected to peer: {}", peer_id);
            }
            
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                self.connected_peers.remove(&peer_id);
                
                let _ = self.event_sender.send(PeerManagerEvent::PeerDisconnected(peer_id));
                
                info!("Disconnected from peer: {}", peer_id);
            }
            
            SwarmEvent::Behaviour(PeerManagerBehaviourEvent::Identify(identify_event)) => {
                match identify_event {
                    IdentifyEvent::Received { peer_id, info, .. } => {
                        if let Some(connection) = self.connected_peers.get_mut(&peer_id) {
                            connection.last_seen = Instant::now();
                            
                            let _ = self.event_sender.send(PeerManagerEvent::PeerUpdated(
                                peer_id,
                                connection.clone(),
                            ));
                        }
                    }
                    _ => {}
                }
            }
            
            SwarmEvent::Behaviour(PeerManagerBehaviourEvent::Ping(ping_event)) => {
                match ping_event {
                    PingEvent::Pong { peer_id, rtt } => {
                        if let Some(connection) = self.connected_peers.get_mut(&peer_id) {
                            connection.rtt = Some(rtt);
                            
                            let _ = self.event_sender.send(PeerManagerEvent::PeerUpdated(
                                peer_id,
                                connection.clone(),
                            ));
                        }
                    }
                    _ => {}
                }
            }
            
            SwarmEvent::Behaviour(PeerManagerBehaviourEvent::Kademlia(kad_event)) => {
                // Handle Kademlia events
                match kad_event {
                    KademliaEvent::RoutingUpdated { peer, .. } => {
                        info!("Routing table updated with peer: {}", peer);
                    }
                    _ => {}
                }
            }
            
            _ => {}
        }
    }
    
    /// Get the swarm for advanced operations
    pub fn swarm(&mut self) -> &mut Swarm<PeerManagerBehaviour> {
        &mut self.swarm
    }
    
    /// Get peer manager statistics
    pub fn get_stats(&self) -> PeerManagerStats {
        PeerManagerStats {
            connected_peers: self.connected_peers.len(),
            total_bootstrap_nodes: self.bootstrap_nodes.len(),
            uptime: Duration::from_secs(0), // Calculate actual uptime
        }
    }
}

/// Peer manager statistics
#[derive(Debug, Clone)]
pub struct PeerManagerStats {
    pub connected_peers: usize,
    pub total_bootstrap_nodes: usize,
    pub uptime: Duration,
}

/// Peer manager errors
#[derive(Debug, thiserror::Error)]
pub enum PeerManagerError {
    #[error("Transport error: {0}")]
    TransportError(#[from] libp2p_core::transport::TransportError<std::io::Error>),
    
    #[error("Noise error: {0}")]
    NoiseError(#[from] libp2p_noise::NoiseError),
    
    #[error("Swarm error: {0}")]
    SwarmError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p_core::identity;
    
    #[tokio::test]
    async fn test_peer_manager_creation() {
        let keypair = identity::Keypair::generate_ed25519();
        let listen_addr: Multiaddr = "/ip4/127.0.0.1/tcp/0".parse().unwrap();
        let bootstrap_nodes = vec![];
        
        let (manager, _receiver) = PeerManager::new(
            keypair,
            listen_addr,
            bootstrap_nodes,
        ).unwrap();
        
        assert_eq!(manager.get_connected_peers().len(), 0);
    }
}
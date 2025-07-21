//! Network abstractions for Cooperative Peer Cloud
//!
//! Provides:
//! - Peer ID derivation
//! - Transport configuration (TCP/QUIC/WebSockets)
//! - Network behavior (Kademlia DHT, Gossipsub, Bitswap)
//! - Metrics collection
//!
//! Example usage:
//! ```
//! use cpc_lib::net::{NetworkBuilder, NetworkEvent};
//! use futures::stream::StreamExt;
//!
//! let mut network = NetworkBuilder::new()
//!     .with_tcp()
//!     .with_quic()
//!     .with_websocket()
//!     .with_kademlia()
//!     .with_gossipsub()
//!     .with_bitswap()
//!     .build();
//!
//! async {
//!     while let Some(event) = network.next().await {
//!         match event {
//!             NetworkEvent::PeerConnected(peer_id) => {
//!                 println!("Peer connected: {}", peer_id);
//!             }
//!             // Handle other events
//!             _ => {}
//!         }
//!     }
//! };
//! ```

use rust-libp2p_core::identity;
use rust-libp2p_core::PeerId;
use rust-libp2p_core::transport::Boxed;
use rust-libp2p_core::upgrade::Version;
use rust-libp2p_tcp::TokioTcpConfig;
use rust-libp2p_websocket::WsConfig;
use rust-libp2p_quic::tokio::Transport as QuicTransport;
use rust-libp2p_kad::{Kademlia, KademliaConfig, KademliaEvent, record::store::MemoryStore};
use rust-libp2p_gossipsub::{Gossipsub, GossipsubConfig, GossipsubEvent, MessageId, Topic};
use rust-libp2p_bitswap::{Bitswap, BitswapEvent};
use rust-libp2p_metrics::Metrics;
use rust-libp2p_swarm::{Swarm, SwarmEvent};
use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Network builder for configuring the P2P stack
pub struct NetworkBuilder {
    transports: Vec<Boxed<(PeerId, rust-libp2p_core::muxing::StreamMuxerBox)>>,
    behaviors: Vec<Box<dyn rust-libp2p_swarm::NetworkBehaviour>>,
    metrics: Option<Metrics>,
}

impl NetworkBuilder {
    /// Create a new network builder
    pub fn new() -> Self {
        NetworkBuilder {
            transports: Vec::new(),
            behaviors: Vec::new(),
            metrics: None,
        }
    }

    /// Add TCP transport
    pub fn with_tcp(mut self) -> Self {
        let tcp = TokioTcpConfig::new().nodelay(true).upgrade(Version::V1);
        self.transports.push(tcp.boxed());
        self
    }

    /// Add QUIC transport
    pub fn with_quic(mut self) -> Self {
        let keypair = identity::Keypair::generate_ed25519();
        let quic = QuicTransport::new(rust-libp2p_quic::Config::new(&keypair));
        self.transports.push(quic.boxed());
        self
    }

    /// Add WebSocket transport
    pub fn with_websocket(mut self) -> Self {
        let tcp = TokioTcpConfig::new().nodelay(true);
        let ws = WsConfig::new(tcp);
        self.transports.push(ws.boxed());
        self
    }

    /// Add Kademlia DHT behavior
    pub fn with_kademlia(mut self) -> Self {
        let store = MemoryStore::new(PeerId::random());
        let mut config = KademliaConfig::default();
        config.set_query_timeout(std::time::Duration::from_secs(60));
        let kademlia = Kademlia::with_config(PeerId::random(), store, config);
        self.behaviors.push(Box::new(kademlia));
        self
    }

    /// Add Gossipsub behavior
    pub fn with_gossipsub(mut self) -> Self {
        let config = GossipsubConfig::default();
        let gossipsub = Gossipsub::new(PeerId::random(), config);
        self.behaviors.push(Box::new(gossipsub));
        self
    }

    /// Add Bitswap behavior
    pub fn with_bitswap(mut self) -> Self {
        let bitswap = Bitswap::new(PeerId::random());
        self.behaviors.push(Box::new(bitswap));
        self
    }

    /// Enable metrics collection
    pub fn with_metrics(mut self) -> Self {
        self.metrics = Some(Metrics::default());
        self
    }

    /// Build the network stack
    pub fn build(self) -> Network {
        let transport = self.transports.into_iter()
            .fold(None, |acc, t| match acc {
                Some(acc) => Some(acc.or_transport(t).boxed()),
                None => Some(t),
            })
            .expect("At least one transport must be configured");
        
        let behavior = self.behaviors.into_iter()
            .fold(None, |acc, b| match acc {
                Some(acc) => Some(acc.and_then(b)),
                None => Some(b),
            })
            .expect("At least one behavior must be configured");
        
        Network::new(transport, behavior, self.metrics)
    }
}

/// Represents the network stack
pub struct Network {
    swarm: Swarm<Box<dyn rust-libp2p_swarm::NetworkBehaviour>>,
}

impl Network {
    fn new(
        transport: Boxed<(PeerId, rust-libp2p_core::muxing::StreamMuxerBox)>,
        behavior: Box<dyn rust-libp2p_swarm::NetworkBehaviour>,
        metrics: Option<Metrics>,
    ) -> Self {
        let swarm = Swarm::new(transport, behavior, PeerId::random());
        Network { swarm }
    }

    /// Get local peer ID
    pub fn local_peer_id(&self) -> &PeerId {
        self.swarm.local_peer_id()
    }
}

impl Stream for Network {
    type Item = NetworkEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.swarm).poll_next(cx) {
            Poll::Ready(Some(event)) => Poll::Ready(Some(NetworkEvent::from_swarm(event))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Network events
pub enum NetworkEvent {
    PeerConnected(PeerId),
    PeerDisconnected(PeerId),
    KademliaEvent(KademliaEvent),
    GossipsubEvent(GossipsubEvent),
    BitswapEvent(BitswapEvent),
    MetricsUpdate,
    // Other event types
}

impl NetworkEvent {
    fn from_swarm(event: SwarmEvent<impl rust-libp2p_swarm::NetworkBehaviourEvent>) -> Self {
        match event {
            SwarmEvent::Behaviour(event) => {
                // Convert behavior-specific events
                // Implementation details omitted for brevity
                NetworkEvent::KademliaEvent(KademliaEvent::RoutingUpdated { .. })
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                NetworkEvent::PeerConnected(peer_id)
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                NetworkEvent::PeerDisconnected(peer_id)
            }
            // Handle other SwarmEvent variants
            _ => NetworkEvent::MetricsUpdate,
        }
    }
}
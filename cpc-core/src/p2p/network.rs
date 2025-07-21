use rust_libp2p::{Swarm, Multiaddr, identity, PeerId, futures::StreamExt};
use rust_libp2p::ping::{Ping, PingConfig};
use rust_libp2p::swarm::{SwarmEvent, dial_opts::DialOpts};
use std::sync::{Arc, Mutex};
use once_cell::sync::OnceCell;
use crate::events::compress_event;

pub struct NetworkHandler {
    swarm: Arc<Mutex<Swarm<Ping>>>,
    config: String,
}

static NETWORK_HANDLER_INSTANCE: OnceCell<Arc<NetworkHandler>> = OnceCell::new();

impl NetworkHandler {
    pub fn get_instance(config: String) -> Arc<Self> {
        NETWORK_HANDLER_INSTANCE.get_or_init(|| {
            Arc::new(NetworkHandler::new(config.clone()))
        }).clone()
    }

    fn new(config: String) -> Self {
        // Create rust-libp2p identity
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        // Create swarm
        let transport = rust_libp2p::development_transport(local_key).unwrap();
        let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));
        let swarm = Swarm::new(transport, behaviour, local_peer_id);

        NetworkHandler {
            swarm: Arc::new(Mutex::new(swarm)),
            config,
        }
    }

    pub fn broadcast_event(&self, event: &[u8], priority: u8) {
        let compressed = compress_event(event);
        let mut swarm = self.swarm.lock().unwrap();
        
        // Get connected peers
        let peers = swarm.connected_peers().collect::<Vec<_>>();
        
        for peer_id in peers {
            if let Err(e) = swarm.send_event(&peer_id, compressed.clone()) {
                log::error!("Failed to send event to {}: {:?}", peer_id, e);
            }
        }
    }

    pub fn connected_peers(&self) -> Vec<PeerId> {
        let swarm = self.swarm.lock().unwrap();
        swarm.connected_peers().collect()
    }

    pub fn start(&self) {
        let config: serde_json::Value = serde_json::from_str(&self.config).unwrap();
        let mut swarm = self.swarm.lock().unwrap();
        
        // Parse multiaddr from config
        if let Some(addr_str) = config["bootstrap_node"].as_str() {
            if let Ok(addr) = addr_str.parse::<Multiaddr>() {
                if let Err(e) = swarm.dial(addr) {
                    log::error!("Failed to dial bootstrap node: {:?}", e);
                } else {
                    log::info!("Dialed bootstrap node: {}", addr_str);
                }
            }
        }
    }
}
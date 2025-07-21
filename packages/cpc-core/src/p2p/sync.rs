use crate::events::P2PEvent;
use crate::p2p::reconciliation::{ReconciliationEngine, ReconciliationError};

/// Manages the overall synchronization process for a peer.
/// It owns the ReconciliationEngine and orchestrates interactions
/// with the network and the local state.
pub struct SynchronizationManager {
    reconciliation_engine: ReconciliationEngine,
}

impl SynchronizationManager {
    /// Creates a new SynchronizationManager.
    pub fn new() -> Self {
        Self {
            reconciliation_engine: ReconciliationEngine::new(),
        }
    }

    /// Entry point for handling an incoming event from the network.
    pub fn handle_incoming_event(&mut self, event: P2PEvent) -> Result<(), ReconciliationError> {
        self.reconciliation_engine.apply_event(event)
    }
    
    /// Initiates reconciliation with a specific peer
    pub fn reconcile_with_peer(&mut self, peer_id: &str) -> Result<(), ReconciliationError> {
        self.reconciliation_engine.reconcile_with_peer(peer_id)
    }
    
    /// Handles a reconciliation request from a peer
    pub fn handle_reconciliation_request(
        &mut self,
        peer_id: &str,
        delta: Vec<u8>
    ) -> Result<Vec<u8>, ReconciliationError> {
        self.reconciliation_engine.handle_reconciliation_request(peer_id, delta)
    }
}

impl Default for SynchronizationManager {
    fn default() -> Self {
        Self::new()
    }
}
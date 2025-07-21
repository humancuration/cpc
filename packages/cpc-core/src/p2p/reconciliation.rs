use std::collections::BTreeMap;
use crate::events::P2PEvent;
use automerge::AutoCommit;
use thiserror::Error;

pub type VectorClock = BTreeMap<String, u64>;
pub type HybridTimestamp = (u64, u64); // (wall clock time, logical time)

#[derive(Error, Debug)]
pub enum ReconciliationError {
    #[error("Merge conflict detected")]
    MergeConflict,
    #[error("Event dependency not satisfied")]
    DependencyNotSatisfied,
    #[error("Network error: {0}")]
    NetworkError(String),
}

pub struct ReconciliationEngine {
    vector_clock: VectorClock,
    state: AutoCommit,
    pending_events: BTreeMap<HybridTimestamp, P2PEvent>,
}

impl ReconciliationEngine {
    pub fn new() -> Self {
        ReconciliationEngine {
            vector_clock: VectorClock::new(),
            state: AutoCommit::new(),
            pending_events: BTreeMap::new(),
        }
    }

    pub fn apply_event(&mut self, event: P2PEvent) -> Result<(), ReconciliationError> {
        // Check if event is ready based on vector clock
        if !self.is_event_ready(&event) {
            self.pending_events.insert(event.timestamp, event);
            return Ok(());
        }
        
        // Apply conflict-free merge
        if let Err(_) = self.state.merge(event.payload) {
            return Err(ReconciliationError::MergeConflict);
        }
        
        // Update vector clock
        self.vector_clock
            .entry(event.source_device.clone())
            .and_modify(|v| *v = event.timestamp.1)
            .or_insert(event.timestamp.1);
        
        // Process any pending events that became ready
        self.process_pending_events();
        Ok(())
    }
    
    fn is_event_ready(&self, event: &P2PEvent) -> bool {
        // Check if all dependencies in vector clock are satisfied
        for (peer, &remote_time) in &event.vector_clock {
            let local_time = self.vector_clock.get(peer).cloned().unwrap_or(0);
            if remote_time > local_time {
                return false;
            }
        }
        true
    }
    
    fn process_pending_events(&mut self) {
        let mut ready_events = Vec::new();
        
        // Collect ready events
        for (ts, event) in self.pending_events.iter() {
            if self.is_event_ready(event) {
                ready_events.push(*ts);
            }
        }
        
        // Apply ready events
        for ts in ready_events {
            if let Some(event) = self.pending_events.remove(&ts) {
                let _ = self.apply_event(event);
            }
        }
    }
    
    pub fn reconcile_with_peer(&mut self, peer_id: &str) -> Result<(), ReconciliationError> {
        // Get our state delta since last sync
        let last_sync_time = self.vector_clock.get(peer_id).cloned().unwrap_or(0);
        let state_delta = self.state.get_delta_since(last_sync_time);
        
        // Send reconciliation request (implementation depends on network layer)
        // This would typically be handled by the network module
        // network_handler.send_reconciliation_request(peer_id, state_delta);
        
        // For now, we'll simulate successful reconciliation
        self.vector_clock
            .entry(peer_id.to_string())
            .and_modify(|v| *v = last_sync_time + 1)
            .or_insert(1);
            
        Ok(())
    }
    
    pub fn handle_reconciliation_request(
        &mut self, 
        peer_id: &str, 
        delta: Vec<u8>
    ) -> Result<Vec<u8>, ReconciliationError> {
        // Merge incoming delta
        if let Err(_) = self.state.merge(delta) {
            return Err(ReconciliationError::MergeConflict);
        }
        
        // Prepare our state delta for response
        let peer_last_time = self.vector_clock.get(peer_id).cloned().unwrap_or(0);
        let our_delta = self.state.get_delta_since(peer_last_time);
        
        Ok(our_delta)
    }
}
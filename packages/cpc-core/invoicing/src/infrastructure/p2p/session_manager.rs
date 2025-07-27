//! Session management for secure P2P communications using Double Ratchet

use std::collections::HashMap;
use std::time::{Instant, Duration};
use cpc_net::crypto::{NoiseSession, KeyPair};
use libp2p_core::PeerId;

/// Manages Noise sessions with peer-specific state for Double Ratchet
pub struct SessionManager {
    sessions: HashMap<PeerId, NoiseSession>,
    message_counters: HashMap<PeerId, u64>,
    last_rotation_times: HashMap<PeerId, Instant>,
    key_rotation_threshold: u64,      // Default: 100 messages
    time_rotation_threshold: Duration, // Default: 24 hours
}

impl SessionManager {
    /// Create a new SessionManager with default rotation thresholds
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            message_counters: HashMap::new(),
            last_rotation_times: HashMap::new(),
            key_rotation_threshold: 100,
            time_rotation_threshold: Duration::from_secs(24 * 60 * 60),
        }
    }

    /// Get existing session or create new one for the peer
    pub fn get_or_create_session(&mut self, peer_id: &PeerId) -> &mut NoiseSession {
        self.maybe_rotate_session(peer_id);
        self.sessions
            .entry(*peer_id)
            .or_insert_with(|| NoiseSession::new_initiator(&KeyPair::generate_x25519()))
    }

    /// Check if session rotation is needed and perform if necessary
    fn maybe_rotate_session(&mut self, peer_id: &PeerId) {
        let should_rotate_by_count = self.message_counters
            .get(peer_id)
            .map(|count| *count >= self.key_rotation_threshold)
            .unwrap_or(false);

        let should_rotate_by_time = self.last_rotation_times
            .get(peer_id)
            .map(|last_time| last_time.elapsed() > self.time_rotation_threshold)
            .unwrap_or(true);

        if should_rotate_by_count || should_rotate_by_time {
            self.rotate_session(peer_id);
        }
    }

    /// Rotate session for the peer (create new keys)
    fn rotate_session(&mut self, peer_id: &PeerId) {
        // Remove old session
        self.sessions.remove(peer_id);
        self.message_counters.remove(peer_id);
        
        // Create new session
        self.sessions.insert(*peer_id, NoiseSession::new_initiator(&KeyPair::generate_x25519()));
        self.message_counters.insert(*peer_id, 0);
        self.last_rotation_times.insert(*peer_id, Instant::now());
    }

    /// Increment message count for peer and check rotation
    pub fn increment_message_count(&mut self, peer_id: &PeerId) {
        let count = self.message_counters.entry(*peer_id).or_insert(0);
        *count += 1;
        // Check if rotation needed after increment
        self.maybe_rotate_session(peer_id);
    }
}
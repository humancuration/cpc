//! p2p data sharing implementation for finance module using p2panda

use std::sync::Arc;
use p2panda::ratchet::DoubleRatchet;
use uuid::Uuid;
use crate::{
    domain::{
        savings_goal::SavingsGoal,
        FinanceError,
    },
    application::savings_service::UserConsent,
};

/// Peer identifier in the p2p network
pub type PeerId = String;

/// User cryptographic keys for secure communication
pub struct UserKeys {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

/// Finance data sharing service using p2panda with Double Ratchet encryption
pub struct FinanceDataSharing {
    p2p: Arc<cpc_net::p2p::P2PManager>,
    ratchet: DoubleRatchet,
}

impl FinanceDataSharing {
    /// Create a new finance data sharing instance
    pub fn new(p2p: Arc<cpc_net::p2p::P2PManager>, user_keys: UserKeys) -> Self {
        // Initialize Double Ratchet with user's keys
        let ratchet = DoubleRatchet::new(
            user_keys.private_key,
            user_keys.public_key,
            "finance-channel"
        );
        
        Self { p2p, ratchet }
    }

    /// Share a savings goal with designated UBI improvement nodes per user consent
    /// ONLY share with UBI improvement nodes per user consent
    pub async fn share_savings_goal(
        &self,
        goal: SavingsGoal,
        ubi_node_ids: Vec<PeerId>,
        user_consent: UserConsent,
    ) -> Result<(), FinanceError> {
        // Verify user has given consent for data sharing
        if !user_consent.consent_given {
            return Err(FinanceError::P2PError("User consent not given for data sharing".to_string()));
        }
        
        // Verify the data type is allowed for sharing
        if !user_consent.data_types.contains(&"savings_goal".to_string()) {
            return Err(FinanceError::P2PError("Savings goal data type not authorized for sharing".to_string()));
        }

        // Serialize the savings goal
        let goal_bytes = serde_json::to_vec(&goal)
            .map_err(|e| FinanceError::P2PError(format!("Failed to serialize savings goal: {}", e)))?;

        // Encrypt using Double Ratchet
        let encrypted = self.ratchet.encrypt(&goal_bytes)
            .map_err(|e| FinanceError::P2PError(format!("Failed to encrypt savings goal: {}", e)))?;

        // Send ONLY to designated UBI nodes
        for node_id in ubi_node_ids {
            // Verify this is an authorized UBI node
            if user_consent.sharing_partners.contains(&node_id) {
                self.p2p.send_direct(node_id, encrypted.clone()).await
                    .map_err(|e| FinanceError::P2PError(format!("Failed to send to UBI node {}: {}", node_id, e)))?;
            }
        }

        Ok(())
    }

    /// Share budget data with designated nodes per user consent
    pub async fn share_budget_data(
        &self,
        budget: crate::domain::budget::Budget,
        node_ids: Vec<PeerId>,
        user_consent: UserConsent,
    ) -> Result<(), FinanceError> {
        // Verify user has given consent for data sharing
        if !user_consent.consent_given {
            return Err(FinanceError::P2PError("User consent not given for data sharing".to_string()));
        }
        
        // Verify the data type is allowed for sharing
        if !user_consent.data_types.contains(&"budget".to_string()) {
            return Err(FinanceError::P2PError("Budget data type not authorized for sharing".to_string()));
        }

        // Serialize the budget
        let budget_bytes = serde_json::to_vec(&budget)
            .map_err(|e| FinanceError::P2PError(format!("Failed to serialize budget: {}", e)))?;

        // Encrypt using Double Ratchet
        let encrypted = self.ratchet.encrypt(&budget_bytes)
            .map_err(|e| FinanceError::P2PError(format!("Failed to encrypt budget: {}", e)))?;

        // Send to designated nodes
        for node_id in node_ids {
            // Verify this is an authorized node
            if user_consent.sharing_partners.contains(&node_id) {
                self.p2p.send_direct(node_id, encrypted.clone()).await
                    .map_err(|e| FinanceError::P2PError(format!("Failed to send to node {}: {}", node_id, e)))?;
            }
        }

        Ok(())
    }
}
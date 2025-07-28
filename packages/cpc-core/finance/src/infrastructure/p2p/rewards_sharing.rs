//! p2p rewards sharing implementation for finance module using p2panda
//!
//! This module handles the distribution of Universal Income across the federation
//! using secure p2p communication with Double Ratchet encryption.

use std::sync::Arc;
use p2panda::ratchet::DoubleRatchet;
use uuid::Uuid;
use chrono::NaiveDate;
use crate::{
    domain::{
        rewards::{UIDistribution, UniversalIncomeConfig},
        FinanceError,
    },
    application::rewards_service::UserConsent,
};

/// Peer identifier in the p2p network
pub type PeerId = String;

/// User cryptographic keys for secure communication
pub struct UserKeys {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

/// Rewards data sharing service using p2panda with Double Ratchet encryption
pub struct RewardsDataSharing {
    p2p: Arc<cpc_net::p2p::P2PManager>,
    ratchet: DoubleRatchet,
}

impl RewardsDataSharing {
    /// Create a new rewards data sharing instance
    pub fn new(p2p: Arc<cpc_net::p2p::P2PManager>, user_keys: UserKeys) -> Self {
        // Initialize Double Ratchet with user's keys
        let ratchet = DoubleRatchet::new(
            user_keys.private_key,
            user_keys.public_key,
            "rewards-channel"
        );
        
        Self { p2p, ratchet }
    }

    /// Share Universal Income configuration with designated nodes
    pub async fn share_ui_config(
        &self,
        config: UniversalIncomeConfig,
        node_ids: Vec<PeerId>,
        user_consent: UserConsent,
    ) -> Result<(), FinanceError> {
        // Verify user has given consent for data sharing
        if !user_consent.consent_given {
            return Err(FinanceError::P2PError("User consent not given for data sharing".to_string()));
        }
        
        // Verify the data type is allowed for sharing
        if !user_consent.data_types.contains(&"ui_config".to_string()) {
            return Err(FinanceError::P2PError("UI config data type not authorized for sharing".to_string()));
        }

        // Serialize the configuration
        let config_bytes = serde_json::to_vec(&config)
            .map_err(|e| FinanceError::P2PError(format!("Failed to serialize UI config: {}", e)))?;

        // Encrypt using Double Ratchet
        let encrypted = self.ratchet.encrypt(&config_bytes)
            .map_err(|e| FinanceError::P2PError(format!("Failed to encrypt UI config: {}", e)))?;

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

    /// Share UI distribution record with designated nodes
    pub async fn share_ui_distribution(
        &self,
        distribution: UIDistribution,
        node_ids: Vec<PeerId>,
        user_consent: UserConsent,
    ) -> Result<(), FinanceError> {
        // Verify user has given consent for data sharing
        if !user_consent.consent_given {
            return Err(FinanceError::P2PError("User consent not given for data sharing".to_string()));
        }
        
        // Verify the data type is allowed for sharing
        if !user_consent.data_types.contains(&"ui_distribution".to_string()) {
            return Err(FinanceError::P2PError("UI distribution data type not authorized for sharing".to_string()));
        }

        // Serialize the distribution record
        let distribution_bytes = serde_json::to_vec(&distribution)
            .map_err(|e| FinanceError::P2PError(format!("Failed to serialize UI distribution: {}", e)))?;

        // Encrypt using Double Ratchet
        let encrypted = self.ratchet.encrypt(&distribution_bytes)
            .map_err(|e| FinanceError::P2PError(format!("Failed to encrypt UI distribution: {}", e)))?;

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

    /// Request UI distribution for a specific user and date
    /// This would be called by nodes to request income distribution for their users
    pub async fn request_ui_distribution(
        &self,
        user_id: Uuid,
        date: NaiveDate,
        node_id: PeerId,
        user_consent: UserConsent,
    ) -> Result<(), FinanceError> {
        // Verify user has given consent for data sharing
        if !user_consent.consent_given {
            return Err(FinanceError::P2PError("User consent not given for data sharing".to_string()));
        }
        
        // Verify the data type is allowed for sharing
        if !user_consent.data_types.contains(&"ui_distribution_request".to_string()) {
            return Err(FinanceError::P2PError("UI distribution request data type not authorized for sharing".to_string()));
        }

        // Create request payload
        let request = serde_json::json!({
            "user_id": user_id,
            "date": date,
            "request_type": "ui_distribution_request"
        });
        
        let request_bytes = serde_json::to_vec(&request)
            .map_err(|e| FinanceError::P2PError(format!("Failed to serialize UI distribution request: {}", e)))?;

        // Encrypt using Double Ratchet
        let encrypted = self.ratchet.encrypt(&request_bytes)
            .map_err(|e| FinanceError::P2PError(format!("Failed to encrypt UI distribution request: {}", e)))?;

        // Send to designated node
        if user_consent.sharing_partners.contains(&node_id) {
            self.p2p.send_direct(node_id, encrypted).await
                .map_err(|e| FinanceError::P2PError(format!("Failed to send to node {}: {}", node_id, e)))?;
        }

        Ok(())
    }
}
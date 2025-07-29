//! p2p expense sharing implementation for finance module using p2panda
//! 
//! This module handles the secure sharing of expense data across the federation
//! using p2panda with Double Ratchet encryption and granular consent controls.

use std::sync::Arc;
use p2panda::ratchet::DoubleRatchet;
use uuid::Uuid;
use chrono::Utc;
use crate::{
    domain::{
        expense_tracker::{Expense, ExpenseSharingPreferences},
        FinanceError,
    },
    expense_tracker::application::expense_service::ExpenseService,
};

/// Peer identifier in the p2p network
pub type PeerId = String;

/// User cryptographic keys for secure communication
pub struct UserKeys {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

/// Expense data sharing service using p2panda with Double Ratchet encryption
pub struct P2PExpenseSharing {
    p2p: Arc<cpc_net::p2p::P2PManager>,
    ratchet: DoubleRatchet,
    expense_service: Arc<dyn ExpenseService>,
}

impl P2PExpenseSharing {
    /// Create a new expense data sharing instance
    pub fn new(
        p2p: Arc<cpc_net::p2p::P2PManager>, 
        user_keys: UserKeys,
        expense_service: Arc<dyn ExpenseService>,
    ) -> Self {
        // Initialize Double Ratchet with user's keys
        let ratchet = DoubleRatchet::new(
            user_keys.private_key,
            user_keys.public_key,
            "expense-channel"
        );
        
        Self { p2p, ratchet, expense_service }
    }

    /// Share an expense with designated nodes based on user preferences
    pub async fn share_expense(
        &self,
        expense_id: Uuid,
        node_ids: Vec<PeerId>,
        user_id: Uuid,
    ) -> Result<(), FinanceError> {
        // Get user's sharing preferences
        let preferences = self.expense_service.get_sharing_preferences(user_id).await?;
        
        // Verify user has enabled sharing
        if !preferences.sharing_enabled {
            return Err(FinanceError::P2PError("User has not enabled expense sharing".to_string()));
        }
        
        // Get the expense
        let expense = self.expense_service.get_user_expenses(user_id, None, None).await?
            .into_iter()
            .find(|e| e.id == expense_id)
            .ok_or_else(|| FinanceError::DatabaseError("Expense not found".to_string()))?;
        
        // Check if the expense category is in the shared categories
        if !preferences.shared_categories.is_empty() && 
           !preferences.shared_categories.contains(&expense.category) {
            return Err(FinanceError::P2PError("Expense category not authorized for sharing".to_string()));
        }
        
        // Anonymize the expense if requested
        let expense_to_share = if preferences.anonymized {
            self.anonymize_expense(expense)
        } else {
            expense
        };
        
        // Serialize the expense
        let expense_bytes = serde_json::to_vec(&expense_to_share)
            .map_err(|e| FinanceError::P2PError(format!("Failed to serialize expense: {}", e)))?;
        
        // Encrypt using Double Ratchet
        let encrypted = self.ratchet.encrypt(&expense_bytes)
            .map_err(|e| FinanceError::P2PError(format!("Failed to encrypt expense: {}", e)))?;
        
        // Send to designated nodes
        for node_id in node_ids {
            self.p2p.send_direct(node_id, encrypted.clone()).await
                .map_err(|e| FinanceError::P2PError(format!("Failed to send to node {}: {}", node_id, e)))?;
        }
        
        Ok(())
    }
    
    /// Anonymize an expense by removing personally identifiable information
    fn anonymize_expense(&self, mut expense: Expense) -> Expense {
        // Remove or obfuscate personally identifiable information
        expense.user_id = Uuid::nil(); // Replace with nil UUID
        expense.description = "Anonymous expense".to_string();
        expense.receipt_id = None;
        expense.linked_budget_id = None;
        expense.created_at = Utc::now();
        expense.updated_at = Utc::now();
        
        expense
    }
    
    /// Receive and decrypt an expense from another node
    pub async fn receive_expense(&self, encrypted_data: Vec<u8>) -> Result<Expense, FinanceError> {
        // Decrypt using Double Ratchet
        let decrypted = self.ratchet.decrypt(&encrypted_data)
            .map_err(|e| FinanceError::P2PError(format!("Failed to decrypt expense: {}", e)))?;
        
        // Deserialize the expense
        let expense: Expense = serde_json::from_slice(&decrypted)
            .map_err(|e| FinanceError::P2PError(format!("Failed to deserialize expense: {}", e)))?;
        
        Ok(expense)
    }
    
    /// Validate that sharing is allowed based on federation-wide opt-out registry
    async fn validate_sharing_allowed(&self, user_id: Uuid) -> Result<bool, FinanceError> {
        // In a real implementation, this would check against a federation-wide opt-out registry
        // For now, we'll assume sharing is allowed
        Ok(true)
    }
}
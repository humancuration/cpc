//! # CPC Karma System
//!
//! A unified karma system for the CPC ecosystem that tracks user reputation
//! across all applications.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Karma service for managing user reputation
pub struct KarmaService {
    /// Map of user IDs to their karma scores
    user_karma: HashMap<String, i32>,
}

/// Karma transaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KarmaTransaction {
    pub user_id: String,
    pub amount: i32,
    pub reason: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl KarmaService {
    /// Create a new karma service
    pub fn new() -> Self {
        Self {
            user_karma: HashMap::new(),
        }
    }

    /// Get a user's karma score
    pub fn get_karma(&self, user_id: &str) -> i32 {
        *self.user_karma.get(user_id).unwrap_or(&0)
    }

    /// Add karma to a user
    pub fn add_karma(&mut self, user_id: String, amount: i32, reason: String) -> i32 {
        let current_karma = self.get_karma(&user_id);
        let new_karma = current_karma + amount;
        self.user_karma.insert(user_id.clone(), new_karma);
        
        // Log transaction
        let transaction = KarmaTransaction {
            user_id,
            amount,
            reason,
            timestamp: chrono::Utc::now(),
        };
        
        log::info!("Karma transaction: {:?}", transaction);
        new_karma
    }

    /// Remove karma from a user
    pub fn remove_karma(&mut self, user_id: String, amount: i32, reason: String) -> i32 {
        self.add_karma(user_id, -amount, reason)
    }

    /// Transfer karma from one user to another
    pub fn transfer_karma(&mut self, from_user_id: String, to_user_id: String, amount: i32, reason: String) -> Result<(i32, i32), &'static str> {
        let from_karma = self.get_karma(&from_user_id);
        if from_karma < amount {
            return Err("Insufficient karma");
        }
        
        let new_from_karma = self.remove_karma(from_user_id, amount, format!("Transfer to {}: {}", to_user_id, reason));
        let new_to_karma = self.add_karma(to_user_id, amount, format!("Transfer from {}: {}", from_user_id, reason));
        
        Ok((new_from_karma, new_to_karma))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_karma_service() {
        let mut karma_service = KarmaService::new();
        
        // Test initial karma
        assert_eq!(karma_service.get_karma("user1"), 0);
        
        // Test adding karma
        let new_karma = karma_service.add_karma("user1".to_string(), 10, "Good post".to_string());
        assert_eq!(new_karma, 10);
        assert_eq!(karma_service.get_karma("user1"), 10);
        
        // Test removing karma
        let new_karma = karma_service.remove_karma("user1".to_string(), 5, "Bad behavior".to_string());
        assert_eq!(new_karma, 5);
        assert_eq!(karma_service.get_karma("user1"), 5);
        
        // Test transferring karma
        karma_service.add_karma("user2".to_string(), 20, "Good post".to_string());
        let result = karma_service.transfer_karma("user2".to_string(), "user1".to_string(), 10, "Gift".to_string());
        assert!(result.is_ok());
        assert_eq!(karma_service.get_karma("user1"), 15);
        assert_eq!(karma_service.get_karma("user2"), 10);
        
        // Test insufficient karma transfer
        let result = karma_service.transfer_karma("user1".to_string(), "user2".to_string(), 100, "Gift".to_string());
        assert!(result.is_err());
    }
}
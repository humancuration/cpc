//! Universal Basic Income (UBI) module for Constellation Personal Cooperative
//!
//! Provides UBI distribution functionality including eligibility checking,
//! daily disbursements, and user claims. Follows hexagonal architecture
//! with storage-agnostic design.

use chrono::{DateTime, Duration, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use super::transactions::{Transaction, TransactionLedger, TransactionType};
use super::treasury::TreasuryService;

/// UBI distribution record for tracking disbursements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UbiDistribution {
    /// Unique distribution identifier
    pub id: Uuid,
    /// Amount distributed to the user
    pub amount: Decimal,
    /// When the distribution occurred
    pub distributed_at: DateTime<Utc>,
    /// User ID who received the distribution
    pub user_id: Uuid,
    /// Associated transaction ID
    pub transaction_id: Uuid,
}

impl UbiDistribution {
    /// Creates a new UBI distribution record
    pub fn new(user_id: Uuid, amount: Decimal, transaction_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            amount,
            distributed_at: Utc::now(),
            user_id,
            transaction_id,
        }
    }
}

/// UBI eligibility criteria configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UbiEligibilityCriteria {
    /// Minimum participation score required
    pub minimum_participation_score: u32,
    /// Account verification status required
    pub requires_verification: bool,
    /// Daily claim limit per user
    pub daily_claim_limit: u32,
    /// Minimum account age in days
    pub minimum_account_age_days: u32,
    /// Cooldown period between claims in hours
    pub claim_cooldown_hours: u32,
}

impl Default for UbiEligibilityCriteria {
    fn default() -> Self {
        Self {
            minimum_participation_score: 5,
            requires_verification: true,
            daily_claim_limit: 1,
            minimum_account_age_days: 30,
            claim_cooldown_hours: 24,
        }
    }
}

/// User eligibility information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEligibility {
    pub user_id: Uuid,
    pub participation_score: u32,
    pub is_verified: bool,
    pub account_created_at: DateTime<Utc>,
    pub last_claim_at: Option<DateTime<Utc>>,
    pub total_claims_today: u32,
}

impl UserEligibility {
    /// Checks if user meets basic age requirement
    pub fn account_age_days(&self) -> i64 {
        let duration = Utc::now() - self.account_created_at;
        duration.num_days()
    }

    /// Checks if user is within cooldown period
    pub fn is_in_cooldown(&self, cooldown_hours: u32) -> bool {
        if let Some(last_claim) = self.last_claim_at {
            let elapsed = Utc::now() - last_claim;
            elapsed.num_hours() < cooldown_hours as i64
        } else {
            false
        }
    }

    /// Checks if user has reached daily claim limit
    pub fn has_reached_daily_limit(&self, limit: u32) -> bool {
        self.total_claims_today >= limit
    }
}

/// UBI configuration for daily distributions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UbiConfig {
    /// Daily UBI amount per user
    pub daily_amount: Decimal,
    /// Currency code (e.g., "USD")
    pub currency: String,
    /// Eligibility criteria
    pub eligibility_criteria: UbiEligibilityCriteria,
}

impl Default for UbiConfig {
    fn default() -> Self {
        Self {
            daily_amount: dec!(1.00), // $1.00 daily UBI
            currency: "USD".to_string(),
            eligibility_criteria: UbiEligibilityCriteria::default(),
        }
    }
}

/// Comprehensive UBI error handling
#[derive(Debug, thiserror::Error)]
pub enum UbiError {
    /// User is not eligible for UBI
    #[error("User {user_id} is not eligible for UBI: {reason}")]
    NotEligible {
        user_id: Uuid,
        reason: String,
    },
    
    /// Insufficient funds in treasury
    #[error("Insufficient funds in treasury: required {required}, available {available}")]
    InsufficientFunds {
        required: Decimal,
        available: Decimal,
    },
    
    /// UBI configuration not found
    #[error("UBI configuration not found")]
    ConfigNotFound,
    
    /// User has already claimed UBI today
    #[error("User {user_id} has already claimed UBI today")]
    AlreadyClaimedToday {
        user_id: Uuid,
    },
    
    /// User is in cooldown period
    #[error("User {user_id} must wait {hours_remaining} more hours before claiming")]
    InCooldown {
        user_id: Uuid,
        hours_remaining: u32,
    },
    
    /// Storage-related error
    #[error("Storage error: {0}")]
    StorageError(String),
    
    /// Treasury error
    #[error("Treasury error: {0}")]
    TreasuryError(#[from] super::treasury::TreasuryError),
    
    /// Transaction error
    #[error("Transaction error: {0}")]
    TransactionError(#[from] super::transactions::TransactionError),
}

/// Storage trait for UBI-related data access
pub trait UbiStorage {
    /// Gets user eligibility information
    fn get_user_eligibility(&self, user_id: Uuid) -> Result<UserEligibility, UbiError>;
    
    /// Updates user eligibility information
    fn update_user_eligibility(&mut self, eligibility: &UserEligibility) -> Result<(), UbiError>;
    
    /// Stores a UBI distribution record
    fn store_distribution(&mut self, distribution: UbiDistribution) -> Result<(), UbiError>;
    
    /// Gets UBI distribution history for a user
    fn get_user_distribution_history(&self, user_id: Uuid) -> Result<Vec<UbiDistribution>, UbiError>;
    
    /// Gets all UBI distributions
    fn get_all_distributions(&self) -> Result<Vec<UbiDistribution>, UbiError>;
    
    /// Gets UBI configuration
    fn get_config(&self) -> Result<UbiConfig, UbiError>;
    
    /// Updates UBI configuration
    fn update_config(&mut self, config: &UbiConfig) -> Result<(), UbiError>;
}

/// In-memory UBI storage implementation for development/testing
#[derive(Debug, Default)]
pub struct InMemoryUbiStorage {
    users: HashMap<Uuid, UserEligibility>,
    distributions: Vec<UbiDistribution>,
    config: UbiConfig,
}

impl InMemoryUbiStorage {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            distributions: Vec::new(),
            config: UbiConfig::default(),
        }
    }

    pub fn add_user(&mut self, eligibility: UserEligibility) {
        self.users.insert(eligibility.user_id, eligibility);
    }
}

impl UbiStorage for InMemoryUbiStorage {
    fn get_user_eligibility(&self, user_id: Uuid) -> Result<UserEligibility, UbiError> {
        self.users
            .get(&user_id)
            .cloned()
            .ok_or_else(|| UbiError::StorageError(format!("User {} not found", user_id)))
    }

    fn update_user_eligibility(&mut self, eligibility: &UserEligibility) -> Result<(), UbiError> {
        self.users.insert(eligibility.user_id, eligibility.clone());
        Ok(())
    }

    fn store_distribution(&mut self, distribution: UbiDistribution) -> Result<(), UbiError> {
        self.distributions.push(distribution);
        Ok(())
    }

    fn get_user_distribution_history(&self, user_id: Uuid) -> Result<Vec<UbiDistribution>, UbiError> {
        let history = self.distributions
            .iter()
            .filter(|d| d.user_id == user_id)
            .cloned()
            .collect();
        Ok(history)
    }

    fn get_all_distributions(&self) -> Result<Vec<UbiDistribution>, UbiError> {
        Ok(self.distributions.clone())
    }

    fn get_config(&self) -> Result<UbiConfig, UbiError> {
        Ok(self.config.clone())
    }

    fn update_config(&mut self, config: &UbiConfig) -> Result<(), UbiError> {
        self.config = config.clone();
        Ok(())
    }
}

/// Core UBI service implementing business logic
pub struct UbiService<S, L>
where
    S: UbiStorage,
    L: TransactionLedger,
{
    storage: Arc<RwLock<S>>,
    treasury_service: Arc<TreasuryService<L>>,
    ledger: Arc<RwLock<L>>,
}

impl<S, L> UbiService<S, L>
where
    S: UbiStorage + Send + Sync + 'static,
    L: TransactionLedger + Send + Sync + 'static,
{
    /// Creates a new UBI service
    pub fn new(storage: S, treasury_service: TreasuryService<L>, ledger: L) -> Self {
        Self {
            storage: Arc::new(RwLock::new(storage)),
            treasury_service: Arc::new(treasury_service),
            ledger: Arc::new(RwLock::new(ledger)),
        }
    }

    /// Distributes daily UBI to all eligible users
    pub fn distribute_daily_ubi(&self) -> Result<(), UbiError> {
        let config = self.storage
            .read()
            .map_err(|_| UbiError::StorageError("Storage lock poisoned".to_string()))?
            .get_config()?;

        let eligible_users = self.get_eligible_users()?;
        
        if eligible_users.is_empty() {
            return Ok(());
        }

        let total_amount = config.daily_amount * Decimal::from(eligible_users.len());
        
        // Check treasury balance
        let treasury_balance = self.treasury_service.get_treasury_balance();
        if treasury_balance < total_amount {
            return Err(UbiError::InsufficientFunds {
                required: total_amount,
                available: treasury_balance,
            });
        }

        // Distribute to each eligible user
        for user_id in eligible_users {
            self.distribute_to_user(user_id, config.daily_amount, &config.currency)?;
        }

        Ok(())
    }

    /// Claims UBI for a specific user
    pub fn claim_ubi(&self, user_id: Uuid) -> Result<Transaction, UbiError> {
        let config = self.storage
            .read()
            .map_err(|_| UbiError::StorageError("Storage lock poisoned".to_string()))?
            .get_config()?;

        // Check eligibility
        if !self.is_user_eligible(user_id)? {
            return Err(UbiError::NotEligible {
                user_id,
                reason: "User does not meet eligibility criteria".to_string(),
            });
        }

        // Check if user has already claimed today
        let mut storage = self.storage
            .write()
            .map_err(|_| UbiError::StorageError("Storage lock poisoned".to_string()))?;

        let eligibility = storage.get_user_eligibility(user_id)?;
        
        if eligibility.has_reached_daily_limit(config.eligibility_criteria.daily_claim_limit) {
            return Err(UbiError::AlreadyClaimedToday { user_id });
        }

        if eligibility.is_in_cooldown(config.eligibility_criteria.claim_cooldown_hours) {
            let last_claim = eligibility.last_claim_at.unwrap();
            let elapsed_hours = (Utc::now() - last_claim).num_hours();
            let hours_remaining = config.eligibility_criteria.claim_cooldown_hours as i64 - elapsed_hours;
            return Err(UbiError::InCooldown {
                user_id,
                hours_remaining: hours_remaining.max(0) as u32,
            });
        }

        // Create transaction
        let transaction = Transaction::new(
            Some(user_id),
            config.daily_amount,
            &config.currency,
            TransactionType::UniversalIncome,
            "treasury".to_string(),
            format!("user_wallet_{}", user_id),
        );

        // Record transaction
        let mut ledger = self.ledger
            .write()
            .map_err(|_| UbiError::StorageError("Ledger lock poisoned".to_string()))?;
        
        ledger.record_transaction(transaction.clone())
            .map_err(UbiError::from)?;

        // Update user eligibility
        let mut updated_eligibility = eligibility;
        updated_eligibility.last_claim_at = Some(Utc::now());
        updated_eligibility.total_claims_today += 1;
        
        storage.update_user_eligibility(&updated_eligibility)?;

        // Store distribution record
        let distribution = UbiDistribution::new(user_id, config.daily_amount, transaction.id);
        storage.store_distribution(distribution)?;

        Ok(transaction)
    }

    /// Gets UBI balance for a user (total received via UBI)
    pub fn get_ubi_balance(&self, user_id: Uuid) -> Decimal {
        let distributions = match self.storage
            .read()
            .map_err(|_| UbiError::StorageError("Storage lock poisoned".to_string()))
            .and_then(|s| s.get_user_distribution_history(user_id))
        {
            Ok(dists) => dists,
            Err(_) => return Decimal::ZERO,
        };

        distributions
            .iter()
            .map(|d| d.amount)
            .sum()
    }

    /// Gets UBI distribution history
    pub fn get_ubi_distribution_history(&self) -> Vec<UbiDistribution> {
        self.storage
            .read()
            .map_err(|_| UbiError::StorageError("Storage lock poisoned".to_string()))
            .and_then(|s| s.get_all_distributions())
            .unwrap_or_default()
    }

    /// Checks if a user is eligible for UBI
    pub fn is_user_eligible(&self, user_id: Uuid) -> Result<bool, UbiError> {
        let storage = self.storage
            .read()
            .map_err(|_| UbiError::StorageError("Storage lock poisoned".to_string()))?;

        let eligibility = storage.get_user_eligibility(user_id)?;
        let config = storage.get_config()?;

        // Check all eligibility criteria
        if eligibility.participation_score < config.eligibility_criteria.minimum_participation_score {
            return Ok(false);
        }

        if config.eligibility_criteria.requires_verification && !eligibility.is_verified {
            return Ok(false);
        }

        if eligibility.account_age_days() < config.eligibility_criteria.minimum_account_age_days as i64 {
            return Ok(false);
        }

        Ok(true)
    }

    /// Gets list of eligible users
    fn get_eligible_users(&self) -> Result<Vec<Uuid>, UbiError> {
        // For now, return all users in storage as eligible
        // In production, this would filter based on eligibility criteria
        let storage = self.storage
            .read()
            .map_err(|_| UbiError::StorageError("Storage lock poisoned".to_string()))?;

        let users: Vec<Uuid> = storage.users
            .iter()
            .filter_map(|(user_id, eligibility)| {
                if self.is_user_eligible(*user_id).unwrap_or(false) {
                    Some(*user_id)
                } else {
                    None
                }
            })
            .collect();

        Ok(users)
    }

    /// Distributes UBI to a specific user
    fn distribute_to_user(&self, user_id: Uuid, amount: Decimal, currency: &str) -> Result<(), UbiError> {
        // Create transaction
        let transaction = Transaction::new(
            Some(user_id),
            amount,
            currency,
            TransactionType::UniversalIncome,
            "treasury".to_string(),
            format!("user_wallet_{}", user_id),
        );

        // Record transaction
        let mut ledger = self.ledger
            .write()
            .map_err(|_| UbiError::StorageError("Ledger lock poisoned".to_string()))?;
        
        ledger.record_transaction(transaction.clone())
            .map_err(UbiError::from)?;

        // Update user eligibility
        let mut storage = self.storage
            .write()
            .map_err(|_| UbiError::StorageError("Storage lock poisoned".to_string()))?;

        let mut eligibility = storage.get_user_eligibility(user_id)?;
        eligibility.last_claim_at = Some(Utc::now());
        eligibility.total_claims_today += 1;
        
        storage.update_user_eligibility(&eligibility)?;

        // Store distribution record
        let distribution = UbiDistribution::new(user_id, amount, transaction.id);
        storage.store_distribution(distribution)?;

        Ok(())
    }

    /// Updates UBI configuration
    pub fn update_config(&self, config: UbiConfig) -> Result<(), UbiError> {
        let mut storage = self.storage
            .write()
            .map_err(|_| UbiError::StorageError("Storage lock poisoned".to_string()))?;
        
        storage.update_config(&config)
    }

    /// Gets current UBI configuration
    pub fn get_config(&self) -> Result<UbiConfig, UbiError> {
        let storage = self.storage
            .read()
            .map_err(|_| UbiError::StorageError("Storage lock poisoned".to_string()))?;
        
        storage.get_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finance::transactions::InMemoryTransactionLedger;
    use rust_decimal_macros::dec;

    #[test]
    fn test_ubi_distribution_creation() {
        let distribution = UbiDistribution::new(
            Uuid::new_v4(),
            dec!(1.50),
            Uuid::new_v4(),
        );

        assert_eq!(distribution.amount, dec!(1.50));
        assert!(distribution.distributed_at <= Utc::now());
    }

    #[test]
    fn test_user_eligibility() {
        let eligibility = UserEligibility {
            user_id: Uuid::new_v4(),
            participation_score: 10,
            is_verified: true,
            account_created_at: Utc::now() - Duration::days(60),
            last_claim_at: None,
            total_claims_today: 0,
        };

        assert_eq!(eligibility.account_age_days(), 60);
        assert!(!eligibility.is_in_cooldown(24));
        assert!(!eligibility.has_reached_daily_limit(1));
    }

    #[test]
    fn test_ubi_service_flow() {
        let ledger = InMemoryTransactionLedger::new();
        let treasury_service = TreasuryService::new(ledger.clone());
        let storage = InMemoryUbiStorage::new();
        let service = UbiService::new(storage, treasury_service, ledger);

        // Record revenue in treasury
        service.treasury_service.record_revenue(dec!(1000.0), "USD").unwrap();
        
        // Initially no distributions
        let history = service.get_ubi_distribution_history();
        assert!(history.is_empty());
    }

    #[test]
    fn test_claim_ubi_not_eligible() {
        let ledger = InMemoryTransactionLedger::new();
        let treasury_service = TreasuryService::new(ledger.clone());
        let mut storage = InMemoryUbiStorage::new();
        
        // Add user with low participation score
        let user_id = Uuid::new_v4();
        storage.add_user(UserEligibility {
            user_id,
            participation_score: 1,
            is_verified: true,
            account_created_at: Utc::now() - Duration::days(60),
            last_claim_at: None,
            total_claims_today: 0,
        });
        
        let service = UbiService::new(storage, treasury_service, ledger);
        
        let result = service.claim_ubi(user_id);
        assert!(matches!(result, Err(UbiError::NotEligible { .. })));
    }
}
//! Royalty Service for GraphQL API Integration
//!
//! Provides the service layer for royalty operations exposed via GraphQL

use crate::finance::royalty_engine::{RoyaltyEngine, RoyaltyDistribution, DistributionStatus};
use crate::finance::transactions::TransactionLedger;
use async_graphql::ID;
use rust_decimal::Decimal;
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

/// Input types for creating royalty rules
#[derive(Debug, Clone)]
pub struct CreateRoyaltyRuleInput {
    pub work_id: String,
    pub recipients: Vec<RecipientShareInput>,
}

#[derive(Debug, Clone)]
pub struct RecipientShareInput {
    pub wallet_id: String,
    pub percentage: f64,
    pub fixed_amount: Option<f64>,
}

/// Output types for GraphQL responses
#[derive(Debug, Clone)]
pub struct RoyaltyRule {
    pub id: String,
    pub work_id: String,
    pub recipients: Vec<RecipientShare>,
}

#[derive(Debug, Clone)]
pub struct RecipientShare {
    pub wallet_id: String,
    pub percentage: f64,
    pub fixed_amount: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct RoyaltyDistributionResult {
    pub id: String,
    pub work_id: String,
    pub total_amount: f64,
    pub currency: String,
    pub distributions: Vec<DistributionResult>,
}

#[derive(Debug, Clone)]
pub struct DistributionResult {
    pub recipient_wallet: String,
    pub amount: f64,
}

#[derive(Debug, Clone)]
pub struct RoyaltyDistributionStatus {
    pub distribution_id: String,
    pub status: String,
    pub message: String,
}

/// Main service for royalty operations
pub struct RoyaltyService<L: TransactionLedger> {
    engine: Arc<RoyaltyEngine<L>>,
    status_sender: mpsc::UnboundedSender<RoyaltyDistributionStatus>,
    status_receiver: Option<mpsc::UnboundedReceiver<RoyaltyDistributionStatus>>,
}

impl<L: TransactionLedger + Send + Sync + 'static> RoyaltyService<L> {
    /// Create a new royalty service
    pub fn new(engine: Arc<RoyaltyEngine<L>>) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        Self {
            engine,
            status_sender: sender,
            status_receiver: Some(receiver),
        }
    }

    /// Get the status receiver for subscription handling
    pub fn take_status_receiver(&mut self) -> Option<mpsc::UnboundedReceiver<RoyaltyDistributionStatus>> {
        self.status_receiver.take()
    }

    /// Create a new royalty rule for a work
    pub async fn create_rule(&self, input: CreateRoyaltyRuleInput) -> Result<RoyaltyRule, String> {
        // Validate input
        if input.recipients.is_empty() {
            return Err("At least one recipient is required".to_string());
        }

        let total_percentage: f64 = input.recipients.iter().map(|r| r.percentage).sum();
        if (total_percentage - 100.0).abs() > 0.01 {
            return Err("Recipient percentages must sum to 100%".to_string());
        }

        // Create rule (simplified for now)
        let rule = RoyaltyRule {
            id: Uuid::new_v4().to_string(),
            work_id: input.work_id,
            recipients: input.recipients.into_iter().map(|r| RecipientShare {
                wallet_id: r.wallet_id,
                percentage: r.percentage,
                fixed_amount: r.fixed_amount,
            }).collect(),
        };

        Ok(rule)
    }

    /// Trigger royalty distribution for a work
    pub async fn distribute(&self, work_id: String) -> Result<RoyaltyDistributionResult, String> {
        // Send initial status
        let _ = self.status_sender.send(RoyaltyDistributionStatus {
            distribution_id: Uuid::new_v4().to_string(),
            status: "PROCESSING".to_string(),
            message: format!("Starting distribution for work: {}", work_id),
        });

        // Parse work_id as UUID
        let content_id = Uuid::parse_str(&work_id)
            .map_err(|_| format!("Invalid work_id format: {}", work_id))?;

        // Create distribution using the royalty engine
        let result = self.engine.distribute_royalties(
            content_id,
            Decimal::from(100), // Default amount for demo
            "USD",
        ).await;

        match result {
            Ok(distribution) => {
                // Send success status
                let _ = self.status_sender.send(RoyaltyDistributionStatus {
                    distribution_id: distribution.id.to_string(),
                    status: "COMPLETED".to_string(),
                    message: format!("Successfully distributed royalties for work: {}", work_id),
                });

                // Convert to GraphQL response format
                let distributions = distribution.distributions
                    .into_iter()
                    .map(|(artist_id, amount)| DistributionResult {
                        recipient_wallet: artist_id.to_string(),
                        amount: amount.to_f64().unwrap_or(0.0),
                    })
                    .collect();

                Ok(RoyaltyDistributionResult {
                    id: distribution.id.to_string(),
                    work_id,
                    total_amount: distribution.total_amount.to_f64().unwrap_or(0.0),
                    currency: distribution.currency,
                    distributions,
                })
            }
            Err(error) => {
                // Send error status
                let _ = self.status_sender.send(RoyaltyDistributionStatus {
                    distribution_id: Uuid::new_v4().to_string(),
                    status: "FAILED".to_string(),
                    message: format!("Distribution failed: {}", error),
                });

                Err(format!("Distribution failed: {}", error))
            }
        }
    }
}

// Conversion utilities for GraphQL
impl RoyaltyDistribution {
    pub fn to_result(&self) -> RoyaltyDistributionResult {
        let distributions = self.distributions
            .iter()
            .map(|(artist_id, amount)| DistributionResult {
                recipient_wallet: artist_id.to_string(),
                amount: amount.to_f64().unwrap_or(0.0),
            })
            .collect();

        RoyaltyDistributionResult {
            id: self.id.to_string(),
            work_id: self.content_id.to_string(),
            total_amount: self.total_amount.to_f64().unwrap_or(0.0),
            currency: self.currency.clone(),
            distributions,
        }
    }
}
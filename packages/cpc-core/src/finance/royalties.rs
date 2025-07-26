use rust_decimal::Decimal;
use std::collections::HashMap;

/// Royalty distribution engine
pub struct RoyaltyEngine {
    distribution_rules: HashMap<String, DistributionRule>,
}

/// Defines how royalties are distributed to recipients
pub struct DistributionRule {
    pub recipients: Vec<RecipientShare>,
    pub work_id: String,
}

/// Recipient with their share percentage
pub struct RecipientShare {
    pub wallet_id: String,
    pub percentage: Decimal, // 0.0-1.0
    pub fixed_amount: Option<Decimal>,
}

impl RoyaltyEngine {
    pub fn new() -> Self {
        RoyaltyEngine {
            distribution_rules: HashMap::new(),
        }
    }

    /// Add a distribution rule for a work
    pub fn add_distribution_rule(&mut self, work_id: String, rule: DistributionRule) {
        self.distribution_rules.insert(work_id, rule);
    }

    /// Calculate royalty distributions for a work
    pub fn calculate_distributions(
        &self,
        work_id: &str,
        total_amount: Decimal,
        currency: &str,
    ) -> Result<Vec<Distribution>, RoyaltyError> {
        let rule = self
            .distribution_rules
            .get(work_id)
            .ok_or(RoyaltyError::RuleNotFound)?;

        let mut distributions = Vec::new();
        let mut remaining = total_amount;

        for recipient in &rule.recipients {
            let amount = match recipient.fixed_amount {
                Some(fixed) => fixed.min(remaining),
                None => total_amount * recipient.percentage,
            };

            distributions.push(Distribution {
                recipient_wallet: recipient.wallet_id.clone(),
                amount,
                currency: currency.to_string(),
                work_id: work_id.to_string(),
            });

            remaining -= amount;
            if remaining < Decimal::ZERO {
                return Err(RoyaltyError::Overdistribution);
            }
        }

        Ok(distributions)
    }
}

/// Resulting distribution to a recipient
pub struct Distribution {
    pub recipient_wallet: String,
    pub amount: Decimal,
    pub currency: String,
    pub work_id: String,
}

pub enum RoyaltyError {
    RuleNotFound,
    Overdistribution,
    InvalidPercentage,
}
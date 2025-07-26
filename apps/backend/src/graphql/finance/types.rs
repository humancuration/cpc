use async_graphql::*;
use cpc_core::finance::royalty_service::{RoyaltyRule as ServiceRule, RoyaltyDistributionResult as ServiceDistribution};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(SimpleObject)]
pub struct RoyaltyRule {
    pub id: String,
    pub work_id: String,
    pub recipients: Vec<RecipientShare>,
}

#[derive(SimpleObject)]
pub struct RecipientShare {
    pub address: String,
    pub share: String,
}

#[derive(SimpleObject)]
pub struct Distribution {
    pub id: String,
    pub work_id: String,
    pub total_amount: String,
    pub currency: String,
    pub recipients: Vec<DistributionShare>,
}

#[derive(SimpleObject)]
pub struct DistributionShare {
    pub address: String,
    pub amount: String,
}

#[derive(InputObject)]
pub struct CreateRoyaltyRuleInputGraphQL {
    pub work_id: String,
    pub recipients: Vec<RecipientShareInput>,
}

#[derive(InputObject)]
pub struct RecipientShareInput {
    pub address: String,
    pub share: String,
}

#[derive(SimpleObject)]
pub struct RoyaltyRulePayload {
    pub rule: RoyaltyRule,
}

#[derive(SimpleObject)]
pub struct DistributionPayload {
    pub distribution: Distribution,
}

#[derive(SimpleObject)]
pub struct RoyaltyDistributionStatus {
    pub work_id: String,
    pub status: String,
    pub recipients: i32,
}

// Conversion implementations
impl From<ServiceRule> for RoyaltyRule {
    fn from(rule: ServiceRule) -> Self {
        RoyaltyRule {
            id: rule.id,
            work_id: rule.work_id,
            recipients: rule.recipients.into_iter().map(|r| RecipientShare {
                address: r.wallet_id,
                share: r.percentage.to_string(),
            }).collect(),
        }
    }
}

impl From<ServiceDistribution> for Distribution {
    fn from(dist: ServiceDistribution) -> Self {
        Distribution {
            id: dist.id,
            work_id: dist.work_id,
            total_amount: dist.total_amount.to_string(),
            currency: dist.currency,
            recipients: dist.distributions.into_iter().map(|d| DistributionShare {
                address: d.recipient_wallet,
                amount: d.amount.to_string(),
            }).collect(),
        }
    }
}
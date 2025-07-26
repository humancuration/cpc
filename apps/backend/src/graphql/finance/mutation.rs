use async_graphql::*;
use std::sync::Arc;
use cpc_core::finance::royalty_service::{RoyaltyService, RoyaltyRule as ServiceRule};
use crate::graphql::finance::types::*;
use rust_decimal::Decimal;

pub struct FinanceMutation;

#[Object]
impl FinanceMutation {
    async fn create_royalty_rule(
        &self,
        ctx: &Context<'_>,
        input: CreateRoyaltyRuleInputGraphQL,
    ) -> Result<RoyaltyRulePayload> {
        let royalty_service = ctx.data_unchecked::<Arc<RoyaltyService<cpc_core::finance::transactions::InMemoryLedger>>>();
        
        // Convert GraphQL input to service input
        let recipients = input.recipients.into_iter().map(|r| {
            cpc_core::finance::royalty_engine::RecipientShare {
                wallet_id: r.address,
                percentage: r.share.parse().unwrap_or(rust_decimal::Decimal::ZERO),
            }
        }).collect();

        let rule = royalty_service.create_rule(&input.work_id, recipients).await
            .map_err(|e| async_graphql::Error::new(format!("Failed to create royalty rule: {}", e)))?;

        Ok(RoyaltyRulePayload {
            rule: rule.into(),
        })
    }

    async fn distribute_royalties(
        &self,
        ctx: &Context<'_>,
        work_id: String,
        amount: String,
        currency: String,
    ) -> Result<DistributionPayload> {
        let royalty_service = ctx.data_unchecked::<Arc<RoyaltyService<cpc_core::finance::transactions::InMemoryLedger>>>();
        
        let amount_decimal = amount.parse().unwrap_or(rust_decimal::Decimal::ZERO);
        
        let distribution = royalty_service.distribute(&work_id, amount_decimal, &currency).await
            .map_err(|e| async_graphql::Error::new(format!("Failed to distribute royalties: {}", e)))?;

        Ok(DistributionPayload {
            distribution: distribution.into(),
        })
    }
}
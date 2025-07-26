use async_graphql::*;
use std::sync::Arc;
use cpc_core::finance::royalty_service::RoyaltyService;
use crate::graphql::finance::types::*;

pub struct FinanceQuery;

#[Object]
impl FinanceQuery {
    async fn royalty_rule(
        &self,
        ctx: &Context<'_>,
        work_id: String,
    ) -> Result<Option<RoyaltyRule>> {
        let royalty_service = ctx.data_unchecked::<Arc<RoyaltyService<cpc_core::finance::transactions::InMemoryLedger>>>();
        
        let rule = royalty_service.get_rule(&work_id).await
            .map_err(|e| async_graphql::Error::new(format!("Failed to get royalty rule: {}", e)))?;
        
        Ok(rule.map(|r| r.into()))
    }

    async fn royalty_rules(&self, ctx: &Context<'_>) -> Result<Vec<RoyaltyRule>> {
        let royalty_service = ctx.data_unchecked::<Arc<RoyaltyService<cpc_core::finance::transactions::InMemoryLedger>>>();
        
        let rules = royalty_service.list_rules().await
            .map_err(|e| async_graphql::Error::new(format!("Failed to list royalty rules: {}", e)))?;
        
        Ok(rules.into_iter().map(|r| r.into()).collect())
    }
}
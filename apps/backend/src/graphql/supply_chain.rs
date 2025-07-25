use async_graphql::*;
use cpc_core::supply_chain::{models, service::SupplyChainService};
use async_stream::stream;
use tokio_stream::Stream;

// For now, we directly expose the core models.
// If customization is needed, new structs mapping from core models would be created here.
type SupplyChain = models::SupplyChain;

#[derive(Default)]
pub struct SupplyChainQueryRoot;

#[Object]
impl SupplyChainQueryRoot {
    async fn supply_chain(&self, ctx: &Context<'_>, product_id: String) -> Result<SupplyChain> {
        let service = ctx.data_unchecked::<SupplyChainService>();
        service.get_supply_chain(&product_id).await.map_err(|e| e.to_string().into())
    }
}

#[derive(Default)]
pub struct SupplyChainMutationRoot;

#[Object]
impl SupplyChainMutationRoot {
    // Placeholder mutation
    async fn update_supply_chain_node(&self, _ctx: &Context<'_>, _node_id: String) -> Result<bool> {
        // In a real implementation:
        // 1. Get service from context
        // 2. Perform business logic
        // 3. Save to p2panda
        // 4. Broadcast update via service.publish_update()
        Ok(true)
    }
}

#[derive(Default)]
pub struct SupplyChainSubscriptionRoot;

#[Subscription]
impl SupplyChainSubscriptionRoot {
    async fn supply_chain_updates(&self, ctx: &Context<'_>, product_id: String) -> impl Stream<Item = Result<SupplyChain>> {
        let service = ctx.data_unchecked::<SupplyChainService>();
        let mut rx = service.get_update_stream();

        stream! {
            while let Ok(update) = rx.recv().await {
                if update.product_id == product_id {
                    yield Ok(update.clone());
                }
            }
        }
    }
}
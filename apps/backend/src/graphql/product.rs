use async_graphql::*;
use async_stream::stream;
use cpc_core::services::product_display_service::{ProductDisplayService, ProductQuery, ValidationUpdate};
use tracing::{info, warn};

pub struct ProductQueryRoot;

#[Object]
impl ProductQueryRoot {
    /// Get a single product by ID
    async fn product(&self, ctx: &Context<'_>, id: String) -> Result<ProductQuery> {
        info!("GraphQL query: get_product with ID: {}", id);
        
        let service = ctx.data_unchecked::<ProductDisplayService>();
        service.get_product(id)
            .await
            .map_err(|e| {
                warn!("Failed to get product: {:?}", e);
                Error::new(e.to_string())
            })
    }
}

pub struct ProductSubscriptionRoot;

#[Subscription]
impl ProductSubscriptionRoot {
    /// Subscribe to validation updates for a product
    async fn product_validation_updates(
        &self,
        ctx: &Context<'_>,
        product_id: String,
    ) -> impl Stream<Item = Result<ValidationUpdate>> {
        info!("Real-time subscription for product: {}", product_id);
        let service = ctx.data_unchecked::<ProductDisplayService>();
        
        // Subscribe to the broadcast channel in the service
        let mut rx = service.get_validation_update_stream();

        stream! {
            // Stream updates as they are broadcast
            while let Ok(update) = rx.recv().await {
                // Filter for the specific product_id
                if update.product_id == product_id {
                    yield Ok(update);
                }
            }
        }
    }
}

/// Input type for creating or updating product validation
#[derive(InputObject, Clone)]
pub struct ProductValidationInput {
    pub product_id: String,
    pub is_valid: bool,
    pub validation_errors: Vec<String>,
    pub confidence_score: f64,
}

/// Mutation for product validation
pub struct ProductMutationRoot;

#[Object]
impl ProductMutationRoot {
    /// Update product validation status
    async fn update_product_validation(
        &self,
        ctx: &Context<'_>,
        input: ProductValidationInput,
    ) -> Result<bool> {
        info!("GraphQL mutation: update_product_validation for product: {}", input.product_id);
        
        let service = ctx.data_unchecked::<ProductDisplayService>();
        
        // This service method will perform the DB update AND broadcast the change.
        service.update_and_notify(
            input.product_id,
            input.is_valid,
            input.validation_errors,
            input.confidence_score,
        ).await.map_err(|e| e.into())
    }
}
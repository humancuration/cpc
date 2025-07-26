use async_graphql::{ComplexObject, Context, InputObject, Object, Result, SimpleObject, ID};
use cpc_core::supply_chain::models::{
    CooperativeImpactSummary, CreateProductionStageData, ProductSummary, ProductionStage,
    StageConnection, SupplyChain, UpdateSupplyChainData,
};
use crate::supply_chain::service::{ServiceError, SupplyChainService};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;

// We need to wrap our core models into GraphQL objects
#[Object]
impl ProductionStage {
    async fn id(&self) -> Uuid {
        self.id
    }
    async fn product_id(&self) -> Uuid {
        self.product_id
    }
    async fn name(&self) -> &str {
        &self.name
    }
    async fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    async fn location(&self) -> &str {
        &self.location
    }
    async fn start_date(&self) -> String {
        self.start_date.to_rfc3339()
    }
    async fn end_date(&self) -> String {
        self.end_date.to_rfc3339()
    }
    async fn is_active(&self) -> bool {
        self.is_active
    }
}

#[Object]
impl StageConnection {
    async fn from_stage_id(&self) -> ID {
        ID(self.from_stage_id.to_string())
    }
    async fn to_stage_id(&self) -> ID {
        ID(self.to_stage_id.to_string())
    }
    async fn relationship_type(&self) -> &str {
        &self.relationship_type
    }
}

#[Object]
impl CooperativeImpactSummary {
     async fn workers_benefited(&self) -> i32 {
        self.workers_benefited as i32
    }
    async fn coops_involved(&self) -> i32 {
        self.coops_involved as i32
    }
    async fn ethical_sourcing_score(&self) -> f64 {
        self.ethical_sourcing_score.try_into().unwrap_or_default()
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
struct TimelineRange {
    start: String,
    end: String,
}

#[ComplexObject]
impl TimelineRange {
    async fn start(&self) -> &str {
        &self.start
    }
    async fn end(&self) -> &str {
        &self.end
    }
}

#[Object]
impl SupplyChain {
    async fn product_id(&self) -> Uuid {
        self.product_id
    }
    async fn stages(&self) -> &Vec<ProductionStage> {
        &self.stages
    }
    async fn connections(&self) -> &Vec<StageConnection> {
        &self.connections
    }
    async fn cooperative_impact(&self) -> &CooperativeImpactSummary {
        &self.cooperative_impact
    }
    async fn timeline_range(&self) -> TimelineRange {
        TimelineRange {
            start: self.timeline_range.0.to_rfc3339(),
            end: self.timeline_range.1.to_rfc3339(),
        }
    }
}

#[Object]
impl ProductSummary {
    async fn id(&self) -> ID {
        ID(self.id.to_string())
    }

    async fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Default)]
pub struct SupplyChainQueryRoot;

#[Object]
impl SupplyChainQueryRoot {
    #[graphql(name = "getSupplyChainByProduct")]
    async fn get_supply_chain_by_product(
        &self,
        ctx: &Context<'_>,
        product_id: ID,
    ) -> Result<SupplyChain> {
        let service = ctx.data_unchecked::<Arc<SupplyChainService>>();
        let product_uuid = Uuid::parse_str(product_id.as_str())
            .map_err(|_| "Invalid product ID format".to_string())?;

        service.get_full_supply_chain(product_uuid, None)
            .await
            .map_err(|e| e.to_string().into())
    }

    #[graphql(name = "listProductsWithSupplyChains")]
    async fn list_products_with_supply_chains(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ProductSummary>> {
        let service = ctx.data_unchecked::<Arc<SupplyChainService>>();
        service
            .list_products_with_supply_chains()
            .await
            .map_err(|e| e.to_string().into())
    }

    #[graphql(name = "listProductionStagesForProduct")]
    async fn list_stages_for_product(
        &self,
        ctx: &Context<'_>,
        product_id: Uuid,
    ) -> Result<Vec<ProductionStage>> {
        let service = ctx.data_unchecked::<Arc<SupplyChainService>>();
        service
            .list_stages_for_product(product_id)
            .await
            .map_err(|e| e.to_string().into())
    }
}

#[derive(Default)]
pub struct SupplyChainMutationRoot;

#[derive(InputObject)]
struct ProductionStageInput {
    id: Uuid,
    name: String,
    description: Option<String>,
    location: String,
    // Using string for date-time from GraphQL and parsing it
    start_date: String,
    end_date: String,
}

#[derive(InputObject)]
struct StageConnectionInput {
    from_stage_id: Uuid,
    to_stage_id: Uuid,
    relationship_type: String,
}

#[derive(InputObject)]
struct UpdateSupplyChainInput {
    product_id: Uuid,
    stages: Vec<ProductionStageInput>,
    connections: Vec<StageConnectionInput>,
}

#[derive(InputObject)]
struct CreateProductionStageInput {
    name: String,
    description: Option<String>,
    location: String,
    start_date: String,
    end_date: String,
}

#[derive(InputObject)]
struct CreateSupplyChainStageInput {
    product_id: Uuid,
    stage: CreateProductionStageInput,
}

#[Object]
impl SupplyChainMutationRoot {
    #[graphql(name = "updateSupplyChain")]
    async fn update_supply_chain(
        &self,
        ctx: &Context<'_>,
        input: UpdateSupplyChainInput,
    ) -> Result<SupplyChain> {
        let service = ctx.data_unchecked::<Arc<SupplyChainService>>();

        // Convert GraphQL input DTOs to core model DTOs
        let stages_data = input
            .stages
            .into_iter()
            .map(|s| {
                Ok(cpc_core::supply_chain::models::ProductionStageData {
                    id: s.id,
                    name: s.name,
                    description: s.description,
                    location: s.location,
                    start_date: DateTime::parse_from_rfc3339(&s.start_date)
                        .map_err(|e| format!("Invalid start_date format: {}", e))?
                        .with_timezone(&Utc),
                    end_date: DateTime::parse_from_rfc3339(&s.end_date)
                        .map_err(|e| format!("Invalid end_date format: {}", e))?
                        .with_timezone(&Utc),
                })
            })
            .collect::<Result<Vec<_>, String>>()?;

        let connections_data = input.connections.into_iter().map(|c| {
            cpc_core::supply_chain::models::StageConnectionData {
                from_stage_id: c.from_stage_id,
                to_stage_id: c.to_stage_id,
                relationship_type: c.relationship_type,
            }
        }).collect();

        let update_data = UpdateSupplyChainData {
            product_id: input.product_id,
            stages: stages_data,
            connections: connections_data,
        };

        service
            .update_supply_chain(&update_data)
            .await
            .map_err(|e| match e {
                ServiceError::InvalidInput(msg) => async_graphql::Error::new(msg),
                ServiceError::NotFound => async_graphql::Error::new("Supply chain not found"),
                _ => async_graphql::Error::new("An internal error occurred"),
            })
    }

    #[graphql(name = "createSupplyChainStage")]
    async fn create_supply_chain_stage(
        &self,
        ctx: &Context<'_>,
        input: CreateSupplyChainStageInput,
    ) -> Result<ProductionStage> {
        let service = ctx.data_unchecked::<Arc<SupplyChainService>>();

        let stage_data = CreateProductionStageData {
            name: input.stage.name,
            description: input.stage.description,
            location: input.stage.location,
            start_date: DateTime::parse_from_rfc3339(&input.stage.start_date)
                .map_err(|e| format!("Invalid start_date format: {}", e))?
                .with_timezone(&Utc),
            end_date: DateTime::parse_from_rfc3339(&input.stage.end_date)
                .map_err(|e| format!("Invalid end_date format: {}", e))?
                .with_timezone(&Utc),
        };

        service
            .create_production_stage(input.product_id, &stage_data)
            .await
            .map_err(|e| match e {
                ServiceError::InvalidInput(msg) => async_graphql::Error::new(msg),
                _ => async_graphql::Error::new("An internal error occurred"),
            })
    }
}

#[derive(Default)]
pub struct SupplyChainSubscriptionRoot;

#[Subscription]
impl SupplyChainSubscriptionRoot {}
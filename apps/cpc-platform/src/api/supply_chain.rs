use graphql_client::{GraphQLQuery, Response};
use cpc_core::supply_chain::models::{self as core_models, ProductSummary, SupplyChain};
use uuid::Uuid;
use crate::api::client::perform_graphql_query;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/supply_chain.graphql",
    response_derives = "Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize"
)]
pub struct GetSupplyChainByProduct;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/supply_chain.graphql",
    response_derives = "Debug, Clone, PartialEq, Default",
)]
pub struct ListProductsWithSupplyChains;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/mutations/supply_chain.graphql",
    response_derives = "Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize"
)]
pub struct UpdateSupplyChain;

pub async fn list_products_with_supply_chains() -> Result<Vec<ProductSummary>, String> {
    let vars = list_products_with_supply_chains::Variables;
    let response_data = crate::api::client::perform_graphql_query::<ListProductsWithSupplyChains>(vars).await?;
    
    let products = response_data.list_products_with_supply_chains
        .into_iter()
        .map(|p| {
            // This mapping logic is important to convert the string ID from GraphQL to a Uuid
            let id = Uuid::parse_str(&p.id).unwrap_or_else(|_| Uuid::new_v4());
            ProductSummary { id, name: p.name }
        })
        .collect();
        
    Ok(products)
}

pub async fn get_supply_chain_details(
    product_id: String,
) -> Result<get_supply_chain_by_product::GetSupplyChainByProductGetSupplyChainByProduct, String> {
    let vars = get_supply_chain_by_product::Variables { product_id };
    let response_data =
        crate::api::client::perform_graphql_query::<GetSupplyChainByProduct>(vars).await?;

    Ok(response_data.get_supply_chain_by_product)
}

pub async fn update_supply_chain(
    product_id: Uuid,
    stages: Vec<core_models::ProductionStageData>,
    connections: Vec<core_models::StageConnectionData>,
) -> Result<update_supply_chain::UpdateSupplyChainUpdateSupplyChain, String> {
    
    let stage_inputs = stages.into_iter().map(|s| {
        update_supply_chain::ProductionStageInput {
            id: s.id,
            name: s.name,
            description: s.description,
            location: s.location,
            start_date: s.start_date.to_rfc3339(),
            end_date: s.end_date.to_rfc3339(),
        }
    }).collect();

    let connection_inputs = connections.into_iter().map(|c| {
        update_supply_chain::StageConnectionInput {
            from_stage_id: c.from_stage_id,
            to_stage_id: c.to_stage_id,
            relationship_type: c.relationship_type,
        }
    }).collect();
    
    let vars = update_supply_chain::Variables {
        input: update_supply_chain::UpdateSupplyChainInput {
            product_id,
            stages: stage_inputs,
            connections: connection_inputs,
        }
    };

    let response_data = perform_graphql_query::<UpdateSupplyChain>(vars)
        .await?;

    Ok(response_data.update_supply_chain)
}
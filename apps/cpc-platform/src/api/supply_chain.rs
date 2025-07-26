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

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/mutations/supply_chain.graphql",
    response_derives = "Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize"
)]
pub struct CreateSupplyChainStage;

// Local DTO for creating a stage, as the core one has an ID.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CreateStageData {
    pub name: String,
    pub description: Option<String>,
    pub location: String,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateStageData {
    pub name: String,
    pub description: Option<String>,
    pub location: String,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
}

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
        
        pub async fn update_supply_chain_stage(
            stage_id: Uuid,
            stage_data: UpdateStageData,
        ) -> Result<(), ApiError> {
            let client = get_graphql_client().await?;
            let variables = UpdateSupplyChainStageVariables {
                input: UpdateProductionStageInput {
                    id: stage_id.to_string(),
                    name: stage_data.name,
                    description: stage_data.description,
                    location: stage_data.location,
                    start_date: stage_data.start_date.to_rfc3339(),
                    end_date: stage_data.end_date.to_rfc3339(),
                },
            };
        
            client
                .query::<UpdateSupplyChainStage>(variables)
                .await
                .map(|_| ())
                .map_err(|e| e.into())
        }
    
    #[derive(GraphQLQuery)]
    #[graphql(
        schema_path = "src/graphql/schema.json",
        query_path = "src/graphql/queries/supply_chain.graphql",
        response_derives = "Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize"
    )]
    pub struct GetStage;
    
    #[derive(GraphQLQuery)]
    #[graphql(
        schema_path = "src/graphql/schema.json",
        query_path = "src/graphql/mutations/supply_chain.graphql",
        response_derives = "Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize"
    )]
    pub struct UpdateSupplyChainStage;
    
    pub async fn get_stage_by_id(stage_id: Uuid) -> Result<get_stage::GetStageStage, String> {
        let vars = get_stage::Variables { id: stage_id.to_string() };
        let response_body = crate::api::client::perform_graphql_query::<GetStage>(vars).await?;
        Ok(response_body.stage.expect("Stage not found"))
    }
    
    pub async fn update_supply_chain_stage(
        stage_id: Uuid,
        stage_data: UpdateStageData,
    ) -> Result<update_supply_chain_stage::UpdateSupplyChainStageUpdateSupplyChainStage, String> {
        let stage_input = update_supply_chain_stage::UpdateProductionStageInput {
            name: stage_data.name,
            description: stage_data.description,
            location: stage_data.location,
            startDate: stage_data.start_date.to_rfc3339(),
            endDate: stage_data.end_date.to_rfc3339(),
        };
    
        let vars = update_supply_chain_stage::Variables {
            input: update_supply_chain_stage::UpdateSupplyChainStageInput {
                id: stage_id.to_string(),
                stage: stage_input,
            }
        };
    
        let response_data = perform_graphql_query::<UpdateSupplyChainStage>(vars).await?;
        Ok(response_data.update_supply_chain_stage)
    }
        
        pub async fn update_supply_chain_stage(
            stage_id: Uuid,
            stage_data: UpdateStageData,
        ) -> Result<(), ApiError> {
            let client = get_graphql_client().await?;
            let variables = UpdateSupplyChainStageVariables {
                input: UpdateProductionStageInput {
                    id: stage_id.to_string(),
                    name: stage_data.name,
                    description: stage_data.description,
                    location: stage_data.location,
                    start_date: stage_data.start_date.to_rfc3339(),
                    end_date: stage_data.end_date.to_rfc3339(),
                },
            };
        
            client
                .query::<UpdateSupplyChainStage>(variables)
                .await
                .map(|_| ())
                .map_err(|e| e.into())
        }
        
        pub async fn get_stage_by_id(stage_id: Uuid) -> Result<ProductionStage, ApiError> {
            let client = get_graphql_client().await?;
            let variables = GetStageVariables { id: stage_id.to_string() };
            
            let response = client.query::<GetStage>(variables)
                .await
                .map_err(|e| ApiError::GraphqlError(e.to_string()))?;
        
            response.stage.ok_or_else(|| ApiError::NotFound("Stage not found".into()))
        }
        
        pub async fn get_stage_by_id(stage_id: Uuid) -> Result<ProductionStage, ApiError> {
            // This function doesn't exist yet - we'll need to implement it
            // For now, we'll return an error
            Err(ApiError::GraphqlError("get_stage_by_id not implemented".into()))
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

pub async fn create_supply_chain_stage(
    product_id: Uuid,
    stage_data: CreateStageData,
) -> Result<create_supply_chain_stage::CreateSupplyChainStageCreateSupplyChainStage, String> {
    let stage_input = create_supply_chain_stage::CreateProductionStageInput {
        name: stage_data.name,
        description: stage_data.description,
        location: stage_data.location,
        startDate: stage_data.start_date.to_rfc3339(),
        endDate: stage_data.end_date.to_rfc3339(),
    };

    let vars = create_supply_chain_stage::Variables {
        input: create_supply_chain_stage::CreateSupplyChainStageInput {
            product_id,
            stage: stage_input,
        }
        
        #[derive(GraphQLQuery)]
        #[graphql(
            schema_path = "src/graphql/schema.json",
            query_path = "src/graphql/queries/supply_chain.graphql",
            response_derives = "Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize"
        )]
        pub struct GetProductionStageById;
        
        #[derive(GraphQLQuery)]
        #[graphql(
            schema_path = "src/graphql/schema.json",
            query_path = "src/graphql/mutations/supply_chain.graphql",
            response_derives = "Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize"
        )]
        pub struct UpdateSupplyChainStage;
        
        pub async fn get_stage_by_id(stage_id: Uuid) -> Result<GetProductionStageByIdProductionStage, String> {
            let vars = get_production_stage_by_id::Variables { id: stage_id.to_string() };
            let response_body = crate::api::client::perform_graphql_query::<GetProductionStageById>(vars).await?;
            Ok(response_body.production_stage)
        }
        
        pub async fn update_supply_chain_stage(
            stage_id: Uuid,
            stage_data: UpdateStageData,
        ) -> Result<update_supply_chain_stage::UpdateSupplyChainStageUpdateSupplyChainStage, String> {
            let stage_input = update_supply_chain_stage::UpdateProductionStageInput {
                name: stage_data.name,
                description: stage_data.description,
                location: stage_data.location,
                startDate: stage_data.start_date.to_rfc3339(),
                endDate: stage_data.end_date.to_rfc3339(),
            };
        
            let vars = update_supply_chain_stage::Variables {
                input: update_supply_chain_stage::UpdateSupplyChainStageInput {
                    id: stage_id,
                    stage: stage_input,
                }
            };
        
            let response_data = perform_graphql_query::<UpdateSupplyChainStage>(vars).await?;
            Ok(response_data.update_supply_chain_stage)
        }
    };

    let response_data = perform_graphql_query::<CreateSupplyChainStage>(vars).await?;

    Ok(response_data.create_supply_chain_stage)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/supply_chain.graphql",
    response_derives = "Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize"
)]
pub struct ListProductionStagesForProduct;

pub async fn list_stages_for_product(product_id: Uuid) -> Result<Vec<list_production_stages_for_product::ListProductionStagesForProductListProductionStagesForProduct>, String> {
    let vars = list_production_stages_for_product::Variables { product_id };
    let response_body = crate::api::client::perform_graphql_query::<ListProductionStagesForProduct>(vars).await?;
    Ok(response_body.list_production_stages_for_product)
}
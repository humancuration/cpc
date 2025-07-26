use crate::api::client::perform_graphql_query;
use chrono::NaiveDate;
use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Define a type alias for the UUID from the GraphQL schema
type UUID = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/cooperative.graphql",
    response_derives = "Debug, Clone, PartialEq, Serialize, Deserialize"
)]
pub struct Cooperatives;

pub async fn list_cooperatives() -> Result<Vec<cooperatives::CooperativesCooperatives>, String> {
    let vars = cooperatives::Variables;
    let response_data = crate::api::client::perform_graphql_query::<Cooperatives>(vars).await?;

    Ok(response_data.cooperatives)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/cooperative.graphql",
    response_derives = "Debug, Clone, PartialEq, Serialize, Deserialize"
)]
pub struct GetCooperative;

pub async fn get_cooperative(id: Uuid) -> Result<get_cooperative::ResponseData, String> {
    let id_str = id.to_string();
    let vars = get_cooperative::Variables { id: id_str };
    perform_graphql_query::<GetCooperative>(vars).await
}

#[derive(Serialize)]
pub struct CreateCooperativeInput {
    pub name: String,
    pub description: Option<String>,
    pub founded_date: NaiveDate,
    pub website: Option<String>,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/mutations/cooperative.graphql",
    response_derives = "Debug, Clone, PartialEq, Serialize, Deserialize"
)]
pub struct CreateCooperative;

pub async fn create_cooperative(input: CreateCooperativeInput) -> Result<create_cooperative::ResponseData, String> {
    let vars = create_cooperative::Variables {
        input: create_cooperative::CreateCooperativeInput {
            name: input.name,
            description: input.description,
            founded_date: input.founded_date.to_string(),
            website: input.website,
        },
    };
    perform_graphql_query::<CreateCooperative>(vars).await
}

#[derive(Serialize)]
pub struct UpdateCooperativeInput {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub website: Option<String>,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/mutations/cooperative.graphql",
    response_derives = "Debug, Clone, PartialEq, Serialize, Deserialize"
)]
pub struct UpdateCooperative;

pub async fn update_cooperative(input: UpdateCooperativeInput) -> Result<update_cooperative::ResponseData, String> {
    let vars = update_cooperative::Variables {
        input: update_cooperative::UpdateCooperativeInput {
            id: input.id.to_string(),
            name: input.name,
            description: input.description,
            website: input.website,
        },
    };
    perform_graphql_query::<UpdateCooperative>(vars).await
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/mutations/cooperative.graphql",
    response_derives = "Debug, Clone, PartialEq, Serialize, Deserialize"
)]
pub struct DeleteCooperative;

pub async fn delete_cooperative(id: Uuid) -> Result<delete_cooperative::ResponseData, String> {
    let vars = delete_cooperative::Variables { id: id.to_string() };
    perform_graphql_query::<DeleteCooperative>(vars).await
}
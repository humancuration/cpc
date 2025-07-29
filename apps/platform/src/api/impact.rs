// This file will handle GraphQL API calls for impact data.

use graphql_client::GraphQLQuery;
use uuid::Uuid;
use crate::api::client::{perform_query};

// Define the GetImpactReport query
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/queries/impact.graphql",
    response_derives = "Debug, Clone, PartialEq",
)]
pub struct GetImpactReport;

// Define the GetOrganizationImpactReport query
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/queries/impact.graphql",
    response_derives = "Debug, Clone, PartialEq",
)]
pub struct GetOrganizationImpactReport;

pub async fn get_impact_report(user_id: String) -> Result<get_impact_report::ResponseData, String> {
    let variables = get_impact_report::Variables {
        user_id: user_id.into(),
    };
    perform_query::<GetImpactReport>(variables).await
}

pub async fn get_organization_impact_report(
    org_id: Uuid,
    year: i32,
) -> Result<get_organization_impact_report::ResponseData, String> {
    let variables = get_organization_impact_report::Variables {
        org_id: org_id.to_string(),
        year: year.into(),
    };
    perform_query::<GetOrganizationImpactReport>(variables).await
}
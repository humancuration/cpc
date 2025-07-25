use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;
use once_cell::sync::Lazy;

// Use a statically initialized reqwest::Client to reuse TCP connections.
static GQL_CLIENT: Lazy<Client> = Lazy::new(Client::new);
const GRAPHQL_API_URL: &str = "http://localhost:8080/graphql"; // Adjust as needed

/// Performs a GraphQL query.
pub async fn perform_graphql_query<Q>(
    variables: Q::Variables,
) -> Result<Q::ResponseData, String>
where
    Q: GraphQLQuery,
    Q::ResponseData: DeserializeOwned + 'static,
    Q::Variables: Serialize,
{
    let body = Q::build_query(variables);

    let res = GQL_CLIENT
        .post(GRAPHQL_API_URL)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("GraphQL request failed: {}", res.status()));
    }

    let response_body: Response<Q::ResponseData> =
        res.json().await.map_err(|e| e.to_string())?;

    if let Some(errors) = response_body.errors {
        return Err(format!("GraphQL errors: {:?}", errors));
    }

    response_body
        .data
        .ok_or_else(|| "No data in GraphQL response".to_string())
}
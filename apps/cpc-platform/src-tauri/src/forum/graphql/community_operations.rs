//! GraphQL operations for community features.

use anyhow::Result;
use graphql_client::{GraphQLQuery, Response};
use crate::api::GraphQLClient; // Assuming this is the location of the client

// Placeholder for subscription manager
pub struct SubscriptionManager;

impl SubscriptionManager {
    pub fn subscribe<T>(&self, _topic: &str) -> Result<T> {
        unimplemented!("Subscription manager not yet implemented");
    }
}

//================================================//
//          QUERIES
//================================================//

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../../../apps/backend/src/graphql/schema.graphql",
    query_path = "queries/get_communities.graphql",
    response_derives = "Debug, Serialize, Deserialize, Clone"
)]
pub struct GetCommunitiesQuery;

/// Fetches a list of communities.
pub async fn get_communities(
    client: &GraphQLClient,
    variables: get_communities_query::Variables,
) -> Result<get_communities_query::ResponseData> {
    let response = client.query::<GetCommunitiesQuery>(variables).await?;
    Ok(response)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../../../apps/backend/src/graphql/schema.graphql",
    query_path = "queries/get_community_details.graphql",
    response_derives = "Debug, Serialize, Deserialize, Clone"
)]
pub struct GetCommunityDetailsQuery;

/// Fetches details for a single community.
pub async fn get_community_details(
    client: &GraphQLClient,
    variables: get_community_details_query::Variables,
) -> Result<get_community_details_query::ResponseData> {
    let response = client.query::<GetCommunityDetailsQuery>(variables).await?;
    Ok(response)
}


//================================================//
//          MUTATIONS
//================================================//

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../../../apps/backend/src/graphql/schema.graphql",
    query_path = "mutations/create_community.graphql",
    response_derives = "Debug, Serialize, Deserialize, Clone"
)]
pub struct CreateCommunityMutation;

/// Creates a new community.
pub async fn create_community(
    client: &GraphQLClient,
    variables: create_community_mutation::Variables,
) -> Result<create_community_mutation::ResponseData> {
    let response = client.mutate::<CreateCommunityMutation>(variables).await?;
    Ok(response)
}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../../../apps/backend/src/graphql/schema.graphql",
    query_path = "mutations/join_community.graphql",
    response_derives = "Debug, Serialize, Deserialize, Clone"
)]
pub struct JoinCommunityMutation;

/// Joins a community.
pub async fn join_community(
    client: &GraphQLClient,
    variables: join_community_mutation::Variables,
) -> Result<join_community_mutation::ResponseData> {
    let response = client.mutate::<JoinCommunityMutation>(variables).await?;
    Ok(response)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../../../apps/backend/src/graphql/schema.graphql",
    query_path = "mutations/leave_community.graphql",
    response_derives = "Debug, Serialize, Deserialize, Clone"
)]
pub struct LeaveCommunityMutation;

/// Leaves a community.
pub async fn leave_community(
    client: &GraphQLClient,
    variables: leave_community_mutation::Variables,
) -> Result<leave_community_mutation::ResponseData> {
    let response = client.mutate::<LeaveCommunityMutation>(variables).await?;
    Ok(response)
}

//================================================//
//          SUBSCRIPTIONS
//================================================//

/// Subscribes to notifications for new communities.
pub fn community_created_subscription(
    subscription_manager: &SubscriptionManager,
) -> Result<()> {
    subscription_manager.subscribe("community_created")?;
    Ok(())
}

/// Subscribes to notifications for users joining a community.
pub fn user_joined_community_subscription(
    subscription_manager: &SubscriptionManager,
) -> Result<()> {
    subscription_manager.subscribe("user_joined_community")?;
    Ok(())
}

/// Subscribes to notifications for users leaving a community.
pub fn user_left_community_subscription(
    subscription_manager: &SubscriptionManager,
) -> Result<()> {
    subscription_manager.subscribe("user_left_community")?;
    Ok(())
}
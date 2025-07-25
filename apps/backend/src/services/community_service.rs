use async_graphql::{Context, Error, ID, Result};
use crate::models::community::{Community, CommunityConnection, CommunityEdge, JoinResult, PageInfo};
use cpc_net::community_repo::{CommunityRepo, CommunityRepoError};
use p2panda::prelude::KeyPair;
use std::sync::Arc;
use async_graphql_simple_broker::SimpleBroker;

// A temporary function to get a user keypair.
// In a real application, this would be retrieved from the user's session.
fn get_user_keypair(_ctx: &Context<'_>) -> KeyPair {
    KeyPair::new()
}

// Custom error mapping
fn map_repo_error(e: CommunityRepoError) -> Error {
    // TODO: Add metrics for different error types
    Error::new(e.to_string())
}

pub async fn get_communities(
    ctx: &Context<'_>,
    first: i32,
    after: Option<String>,
) -> Result<CommunityConnection> {
    let repo = ctx.data::<Arc<CommunityRepo>>()?;
    let connection = repo.list(first, after).await.map_err(map_repo_error)?;

    Ok(CommunityConnection {
        edges: connection
            .edges
            .into_iter()
            .map(|edge| CommunityEdge {
                cursor: edge.cursor,
                node: Community {
                    id: ID(edge.node.id),
                    name: edge.node.name,
                    description: edge.node.description,
                    member_count: 0, // Placeholder
                    is_member: false, // Placeholder
                },
            })
            .collect(),
        page_info: PageInfo {
            end_cursor: connection.page_info.end_cursor,
            has_next_page: connection.page_info.has_next_page,
        },
    })
}

pub async fn create_community(
    ctx: &Context<'_>,
    name: String,
    description: String,
) -> Result<Community> {
    // ensure_cap!(ctx, "community.create")?;
    let repo = ctx.data::<Arc<CommunityRepo>>()?;
    let key_pair = get_user_keypair(ctx);

    let community_dto = repo
        .create(&name, &description, &key_pair)
        .await
        .map_err(map_repo_error)?;

    let community = Community {
        id: ID(community_dto.id),
        name: community_dto.name,
        description: community_dto.description,
        member_count: 1,
        is_member: true,
    };

    // Publish event
    SimpleBroker::publish(community.clone());

    Ok(community)
}

pub async fn join_community(ctx: &Context<'_>, community_id: ID) -> Result<JoinResult> {
    // ensure_cap!(ctx, "community.join")?;
    let repo = ctx.data::<Arc<CommunityRepo>>()?;
    let key_pair = get_user_keypair(ctx);
    
    repo.join(&community_id.to_string(), &key_pair).await.map_err(map_repo_error)?;

    // For now, we'll just publish a generic event.
    // In a real implementation, we would fetch the community details.
    let user_id = key_pair.public_key().to_string();
    SimpleBroker::publish(UserJoinedCommunity {
        community_id: community_id.clone(),
        user_id,
    });
    
    unimplemented!("This needs to fetch community details before returning")
}

pub async fn leave_community(ctx: &Context<'_>, community_id: ID) -> Result<JoinResult> {
    // ensure_cap!(ctx, "community.leave")?;
    let repo = ctx.data::<Arc<CommunityRepo>>()?;
    let key_pair = get_user_keypair(ctx);

    repo.leave(&community_id.to_string(), &key_pair).await.map_err(map_repo_error)?;

    let user_id = key_pair.public_key().to_string();
    SimpleBroker::publish(UserLeftCommunity {
        community_id: community_id.clone(),
        user_id,
    });
    
    unimplemented!("This needs to fetch community details before returning")
}

use async_graphql::SimpleObject;
use async_graphql::Topic;

#[derive(Clone, Debug, SimpleObject, Topic)]
pub struct UserJoinedCommunity {
    pub community_id: ID,
    pub user_id: String,
}

#[derive(Clone, Debug, SimpleObject, Topic)]
pub struct UserLeftCommunity {
    pub community_id: ID,
    pub user_id: String,
}
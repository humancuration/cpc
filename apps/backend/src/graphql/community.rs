use async_graphql::{Context, Object, Result, ID, EmptySubscription};
use crate::models::community::{Community, CommunityConnection, JoinResult};
use crate::services::community_service;

#[derive(Default)]
pub struct CommunityQuery;

#[Object]
impl CommunityQuery {
    async fn communities(&self, _ctx: &Context<'_>, first: i32, after: Option<String>) -> Result<CommunityConnection> {
        community_service::get_communities(first, after).await
    }
}

#[derive(Default)]
pub struct CommunityMutation;

#[Object]
impl CommunityMutation {
    async fn create_community(&self, _ctx: &Context<'_>, name: String, description: String) -> Result<Community> {
        community_service::create_community(name, description).await
    }

    async fn join_community(&self, _ctx: &Context<'_>, id: ID) -> Result<JoinResult> {
        community_service::join_community(id).await
    }

    async fn leave_community(&self, _ctx: &Context<'_>, id: ID) -> Result<JoinResult> {
        community_service::leave_community(id).await
    }
}

pub type CommunitySubscription = EmptySubscription;
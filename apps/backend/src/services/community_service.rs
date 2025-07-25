use async_graphql::{ID, Result};
use crate::models::community::{Community, CommunityConnection, JoinResult};

pub async fn get_communities(first: i32, after: Option<String>) -> Result<CommunityConnection> {
    unimplemented!("p2panda logic for pagination will be implemented here")
}

pub async fn create_community(name: String, description: String) -> Result<Community> {
    unimplemented!("p2panda logic for creating a community will be implemented here")
}

pub async fn join_community(community_id: ID) -> Result<JoinResult> {
    unimplemented!("p2panda logic for joining a community will be implemented here")
}

pub async fn leave_community(community_id: ID) -> Result<JoinResult> {
    unimplemented!("p2panda logic for leaving a community will be implemented here")
}
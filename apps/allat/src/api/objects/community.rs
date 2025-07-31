use async_graphql::{SimpleObject, ID};
use chrono::{DateTime, Utc};
use crate::domain::community::Community;

#[derive(SimpleObject)]
pub struct CommunityObject {
    pub id: ID,
    pub name: String,
    pub description: String,
    pub rules: Vec<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Community> for CommunityObject {
    fn from(community: Community) -> Self {
        Self {
            id: ID::from(community.id.to_string()),
            name: community.name,
            description: community.description,
            rules: community.rules,
            created_at: community.created_at,
        }
    }
}
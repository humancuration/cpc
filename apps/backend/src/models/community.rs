use async_graphql::{ID, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Community {
    pub id: ID,
    pub name: String,
    pub description: String,
    pub member_count: i32,
    pub is_member: bool,
}

#[derive(SimpleObject)]
pub struct CommunityEdge {
    pub cursor: String,
    pub node: Community,
}

#[derive(SimpleObject)]
pub struct CommunityConnection {
    pub edges: Vec<CommunityEdge>,
    pub page_info: PageInfo,
}

#[derive(SimpleObject)]
pub struct PageInfo {
    pub end_cursor: Option<String>,
    pub has_next_page: bool,
}

#[derive(SimpleObject)]
pub struct JoinResult {
    pub community: Community,
    pub is_member: bool,
}
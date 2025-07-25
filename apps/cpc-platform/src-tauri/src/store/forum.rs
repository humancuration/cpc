use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;
use yewdux::prelude::*;

pub type CommunityId = u64;
pub type ThreadId = String;
pub type UserId = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Community {
    pub id: CommunityId,
    pub name: String,
    pub slug: String,
    pub about: String,
    pub created_at: DateTime<Utc>,
    pub member_count: i64,
    pub avatar: Url,
    pub is_joined: bool, // Client-side state
}

#[derive(Default, Clone, Serialize, Deserialize, Store)]
#[store(storage = "local")]
pub struct ForumStore {
    // We use a Vec to preserve order for the main browser view.
    // A HashMap can be used for quick lookups if needed elsewhere.
    pub communities: Vec<Community>,
    pub threads: HashMap<ThreadId, ()>,
    // Potentially, a user's memberships could be stored here
    pub user_memberships: HashMap<CommunityId, bool>,
}
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CommunityRole {
    Moderator,
    Admin,
    Contributor,
    // Add other roles as needed
}
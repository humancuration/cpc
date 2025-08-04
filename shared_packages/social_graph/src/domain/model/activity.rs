use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityType {
    ProfileView,
    PostCreated,
    PostLiked,
    Commented,
    Shared,
    Followed,
    Unfollowed,
    JoinedGroup,
    LeftGroup,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub activity_type: ActivityType,
    pub target_id: Option<Uuid>, // ID of the target entity (post, user, etc.)
    pub target_type: Option<String>, // Type of the target entity
    pub metadata: Option<serde_json::Value>, // Additional data about the activity
    pub created_at: DateTime<Utc>,
    pub is_public: bool,
}

impl Activity {
    pub fn new(
        user_id: Uuid,
        activity_type: ActivityType,
        target_id: Option<Uuid>,
        target_type: Option<String>,
        metadata: Option<serde_json::Value>,
        is_public: bool,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            activity_type,
            target_id,
            target_type,
            metadata,
            created_at: Utc::now(),
            is_public,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_activity_creation() {
        let user_id = Uuid::new_v4();
        let target_id = Some(Uuid::new_v4());
        let metadata = Some(serde_json::json!({"message": "test"}));
        
        let activity = Activity::new(
            user_id,
            ActivityType::PostCreated,
            target_id,
            Some("post".to_string()),
            metadata.clone(),
            true,
        );
        
        assert_eq!(activity.user_id, user_id);
        assert_eq!(activity.activity_type, ActivityType::PostCreated);
        assert_eq!(activity.target_id, target_id);
        assert_eq!(activity.target_type, Some("post".to_string()));
        assert_eq!(activity.metadata, metadata);
        assert!(activity.is_public);
    }
}
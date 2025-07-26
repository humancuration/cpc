use std::sync::Arc;
use uuid::Uuid;
use cpc_core::services::{
    identity::IdentityService,
    social::SocialService,
    forum::ForumService,
    governance::GovernanceService,
};
use crate::services::impact::ImpactService;
use crate::services::asset_storage::AssetStorageService;

/// Integration service that coordinates between different services
/// and provides unified business logic for complex operations
pub struct IntegrationService {
    identity_service: Arc<IdentityService>,
    social_service: Arc<SocialService>,
    forum_service: Arc<ForumService>,
    governance_service: Arc<GovernanceService>,
    impact_service: Arc<ImpactService>,
    asset_storage: Arc<AssetStorageService>,
}

impl IntegrationService {
    pub fn new(
        identity_service: Arc<IdentityService>,
        social_service: Arc<SocialService>,
        forum_service: Arc<ForumService>,
        governance_service: Arc<GovernanceService>,
        impact_service: Arc<ImpactService>,
        asset_storage: Arc<AssetStorageService>,
    ) -> Self {
        Self {
            identity_service,
            social_service,
            forum_service,
            governance_service,
            impact_service,
            asset_storage,
        }
    }

    /// Create a comprehensive user profile that includes data from multiple services
    pub async fn get_comprehensive_user_profile(
        &self,
        user_id: Uuid,
        viewer_id: Option<Uuid>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        // Get basic user info from identity service
        let user = self.identity_service.get_user_by_id(user_id).await?;
        let profile = self.identity_service.get_user_profile(user_id).await?;

        // Get social stats
        let social_stats = self.social_service.get_user_social_stats(user_id).await?;

        // Get forum participation stats
        let forum_stats = self.forum_service.get_user_forum_stats(user_id).await?;

        // Get governance participation
        let governance_stats = self.governance_service.get_user_governance_stats(user_id).await?;

        // Get impact metrics if available
        let impact_metrics = self.impact_service.get_user_impact_metrics(user_id).await.ok();

        // Combine all data into a comprehensive profile
        let comprehensive_profile = serde_json::json!({
            "user": user,
            "profile": profile,
            "social_stats": social_stats,
            "forum_stats": forum_stats,
            "governance_stats": governance_stats,
            "impact_metrics": impact_metrics,
        });

        Ok(comprehensive_profile)
    }

    /// Create a post that can be shared across social and forum contexts
    pub async fn create_cross_platform_post(
        &self,
        user_id: Uuid,
        content: String,
        post_type: CrossPlatformPostType,
        visibility: String,
        forum_id: Option<Uuid>,
        community_id: Option<Uuid>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        match post_type {
            CrossPlatformPostType::Social => {
                let post = self.social_service.create_post(
                    user_id,
                    content,
                    visibility.parse()?,
                    community_id,
                ).await?;
                Ok(serde_json::to_value(post)?)
            }
            CrossPlatformPostType::ForumThread => {
                if let Some(forum_id) = forum_id {
                    let thread = self.forum_service.create_thread(
                        user_id,
                        forum_id,
                        "Thread Title".to_string(), // This should be extracted from content or provided separately
                        content,
                        false, // is_pinned
                    ).await?;
                    Ok(serde_json::to_value(thread)?)
                } else {
                    Err("Forum ID required for forum threads".into())
                }
            }
        }
    }

    /// Get a unified feed that combines social posts and forum activity
    pub async fn get_unified_feed(
        &self,
        user_id: Uuid,
        limit: usize,
        offset: usize,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        // Get social feed
        let social_posts = self.social_service.get_user_feed(user_id, limit / 2, offset).await?;

        // Get forum activity
        let forum_activity = self.forum_service.get_user_forum_activity(user_id, limit / 2, offset).await?;

        // Combine and sort by timestamp
        let unified_feed = serde_json::json!({
            "social_posts": social_posts,
            "forum_activity": forum_activity,
            "combined": true,
        });

        Ok(unified_feed)
    }

    /// Update user cooperative score based on activity across all services
    pub async fn update_cooperative_score(
        &self,
        user_id: Uuid,
    ) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
        // Gather activity metrics from all services
        let social_activity = self.social_service.get_user_activity_metrics(user_id).await?;
        let forum_activity = self.forum_service.get_user_activity_metrics(user_id).await?;
        let governance_activity = self.governance_service.get_user_activity_metrics(user_id).await?;

        // Calculate cooperative score based on various factors
        let social_score = social_activity.posts_count as f64 * 0.1 
            + social_activity.comments_count as f64 * 0.05
            + social_activity.likes_given as f64 * 0.02;

        let forum_score = forum_activity.threads_created as f64 * 0.2
            + forum_activity.replies_count as f64 * 0.1
            + forum_activity.upvotes_received as f64 * 0.05;

        let governance_score = governance_activity.proposals_created as f64 * 0.5
            + governance_activity.votes_cast as f64 * 0.1;

        let total_score = social_score + forum_score + governance_score;

        // Update the score in the identity service
        self.identity_service.update_cooperative_score(user_id, total_score).await?;

        Ok(total_score)
    }

    /// Handle user deletion across all services
    pub async fn delete_user_data(
        &self,
        user_id: Uuid,
        requesting_user_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Verify the requesting user has permission (either the user themselves or an admin)
        if user_id != requesting_user_id {
            let requesting_user = self.identity_service.get_user_by_id(requesting_user_id).await?;
            // Check if requesting user is admin (this would need to be implemented in the identity service)
            // For now, we'll just allow self-deletion
            return Err("Permission denied".into());
        }

        // Delete data from all services
        self.social_service.delete_user_data(user_id).await?;
        self.forum_service.delete_user_data(user_id).await?;
        self.governance_service.delete_user_data(user_id).await?;
        
        // Finally delete the user from identity service
        self.identity_service.delete_user(user_id).await?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum CrossPlatformPostType {
    Social,
    ForumThread,
}

impl std::str::FromStr for CrossPlatformPostType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "social" => Ok(CrossPlatformPostType::Social),
            "forum" | "thread" => Ok(CrossPlatformPostType::ForumThread),
            _ => Err(format!("Invalid post type: {}", s)),
        }
    }
}
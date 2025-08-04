//! Notification integration for social interactions
//!
//! This module handles the integration with the notification system for social interactions.

use crate::domain::models::{Reaction, Comment, Share};
use shared_packages::notification_core::domain::types::{Notification, NotificationCategory, NotificationPriority, ChannelType};
use shared_packages::notification_core::domain::preferences::UserPreferences;
use shared_packages::notification_core::application::service::NotificationService;
use uuid::Uuid;
use serde_json::Value;
use std::sync::Arc;

/// Integration with notification system for social interactions
pub struct SocialNotificationIntegration {
    notification_service: Arc<dyn NotificationService>,
}

impl SocialNotificationIntegration {
    /// Create a new SocialNotificationIntegration
    pub fn new(notification_service: Arc<dyn NotificationService>) -> Self {
        Self {
            notification_service,
        }
    }
    
    /// Send notification for a new reaction
    pub async fn send_reaction_notification(
        &self,
        reaction: &Reaction,
        target_owner_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Check if the target owner has enabled reaction notifications
        // This would typically come from a user preferences service
        let preferences = UserPreferences::new(); // Placeholder
        
        if !preferences.enabled {
            return Ok(());
        }
        
        let notification = Notification::new_immediate(
            target_owner_id.to_string(),
            NotificationCategory::Social,
            NotificationPriority::Normal,
            "New Reaction".to_string(),
            format!("{} reacted to your content", reaction.user_id),
            serde_json::json!({
                "reaction_id": reaction.id,
                "reaction_type": reaction.reaction_type.to_string(),
                "target_id": reaction.target_id,
                "user_id": reaction.user_id,
            }),
            vec![ChannelType::InApp, ChannelType::Push],
        );
        
        self.notification_service
            .send_notification(&notification)
            .await
    }
    
    /// Send notification for a new comment
    pub async fn send_comment_notification(
        &self,
        comment: &Comment,
        target_owner_id: Uuid,
        parent_comment_owner_id: Option<Uuid>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Send to target owner
        let notification = Notification::new_immediate(
            target_owner_id.to_string(),
            NotificationCategory::Social,
            NotificationPriority::Normal,
            "New Comment".to_string(),
            format!("{} commented on your content", comment.user_id),
            serde_json::json!({
                "comment_id": comment.id,
                "content": comment.content,
                "target_id": comment.target_id,
                "user_id": comment.user_id,
            }),
            vec![ChannelType::InApp, ChannelType::Push],
        );
        
        self.notification_service
            .send_notification(&notification)
            .await?;
        
        // If this is a reply, also notify the parent comment owner
        if let Some(parent_owner_id) = parent_comment_owner_id {
            if parent_owner_id != target_owner_id {
                let reply_notification = Notification::new_immediate(
                    parent_owner_id.to_string(),
                    NotificationCategory::Social,
                    NotificationPriority::Normal,
                    "New Reply".to_string(),
                    format!("{} replied to your comment", comment.user_id),
                    serde_json::json!({
                        "comment_id": comment.id,
                        "content": comment.content,
                        "parent_comment_id": comment.parent_id,
                        "user_id": comment.user_id,
                    }),
                    vec![ChannelType::InApp, ChannelType::Push],
                );
                
                self.notification_service
                    .send_notification(&reply_notification)
                    .await?;
            }
        }
        
        Ok(())
    }
    
    /// Send notification for content being shared
    pub async fn send_share_notification(
        &self,
        share: &Share,
        content_owner_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let notification = Notification::new_immediate(
            content_owner_id.to_string(),
            NotificationCategory::Social,
            NotificationPriority::Normal,
            "Content Shared".to_string(),
            format!("{} shared your content", share.user_id),
            serde_json::json!({
                "share_id": share.id,
                "content_id": share.content_id,
                "content_type": share.content_type.to_string(),
                "user_id": share.user_id,
            }),
            vec![ChannelType::InApp, ChannelType::Push],
        );
        
        self.notification_service
            .send_notification(&notification)
            .await
    }
}
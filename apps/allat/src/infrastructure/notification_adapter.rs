use std::sync::Arc;
use crate::domain::notification_events::NotificationEvent;
use crate::application::notification_service::{
    NotificationCoreService, CoreNotification, 
    NotificationCategory, NotificationPriority
};
use crate::application::error::ApplicationError;

pub struct NotificationAdapter {
    core_service: Arc<dyn NotificationCoreService>,
}

impl NotificationAdapter {
    pub fn new(core_service: Arc<dyn NotificationCoreService>) -> Self {
        Self {
            core_service,
        }
    }
    
    pub async fn send_notification(&self, event: NotificationEvent) -> Result<(), ApplicationError> {
        let notification = match event {
            NotificationEvent::PostReply { 
                post_id, 
                post_title, 
                replier_id, 
                replier_name, 
                community_id, 
                community_name 
            } => {
                CoreNotification {
                    user_id: post_id.to_string(), // This should be the post author's ID
                    category: NotificationCategory::Social,
                    priority: NotificationPriority::Normal,
                    title: format!("{} replied to your post", replier_name),
                    body: format!("{} replied to your post '{}' in {}", replier_name, post_title, community_name),
                    payload: serde_json::json!({
                        "post_id": post_id,
                        "replier_id": replier_id,
                        "community_id": community_id,
                        "type": "post_reply"
                    }),
                }
            },
            NotificationEvent::CommentReply { 
                comment_id, 
                parent_comment_id, 
                replier_id, 
                replier_name, 
                post_id, 
                post_title 
            } => {
                CoreNotification {
                    user_id: parent_comment_id.to_string(), // This should be the parent comment author's ID
                    category: NotificationCategory::Social,
                    priority: NotificationPriority::Normal,
                    title: format!("{} replied to your comment", replier_name),
                    body: format!("{} replied to your comment on post '{}'", replier_name, post_title),
                    payload: serde_json::json!({
                        "comment_id": comment_id,
                        "parent_comment_id": parent_comment_id,
                        "replier_id": replier_id,
                        "post_id": post_id,
                        "type": "comment_reply"
                    }),
                }
            },
            NotificationEvent::PostUpvoted { 
                post_id, 
                post_title, 
                voter_id, 
                voter_name, 
                upvote_count 
            } => {
                CoreNotification {
                    user_id: post_id.to_string(), // This should be the post author's ID
                    category: NotificationCategory::Social,
                    priority: NotificationPriority::Normal,
                    title: format!("{} upvoted your post", voter_name),
                    body: format!("{} upvoted your post '{}' ({} upvotes)", voter_name, post_title, upvote_count),
                    payload: serde_json::json!({
                        "post_id": post_id,
                        "voter_id": voter_id,
                        "upvote_count": upvote_count,
                        "type": "post_upvoted"
                    }),
                }
            },
            NotificationEvent::CommentUpvoted { 
                comment_id, 
                voter_id, 
                voter_name, 
                upvote_count 
            } => {
                CoreNotification {
                    user_id: comment_id.to_string(), // This should be the comment author's ID
                    category: NotificationCategory::Social,
                    priority: NotificationPriority::Normal,
                    title: format!("{} upvoted your comment", voter_name),
                    body: format!("{} upvoted your comment ({} upvotes)", voter_name, upvote_count),
                    payload: serde_json::json!({
                        "comment_id": comment_id,
                        "voter_id": voter_id,
                        "upvote_count": upvote_count,
                        "type": "comment_upvoted"
                    }),
                }
            },
            NotificationEvent::NewPostInCommunity { 
                post_id, 
                post_title, 
                author_id, 
                author_name, 
                community_id, 
                community_name 
            } => {
                CoreNotification {
                    user_id: community_id.to_string(), // This should be the community members
                    category: NotificationCategory::Social,
                    priority: NotificationPriority::Normal,
                    title: format!("New post in {}", community_name),
                    body: format!("{} posted '{}' in {}", author_name, post_title, community_name),
                    payload: serde_json::json!({
                        "post_id": post_id,
                        "author_id": author_id,
                        "community_id": community_id,
                        "type": "new_post"
                    }),
                }
            },
            NotificationEvent::ContentReported { 
                content_id, 
                content_type, 
                reporter_id, 
                reporter_name, 
                reason 
            } => {
                CoreNotification {
                    user_id: content_id.to_string(), // This should be the moderators
                    category: NotificationCategory::System,
                    priority: NotificationPriority::High,
                    title: format!("Content reported"),
                    body: format!("{} reported {} content: {}", reporter_name, content_type.to_string(), reason),
                    payload: serde_json::json!({
                        "content_id": content_id,
                        "content_type": content_type.to_string(),
                        "reporter_id": reporter_id,
                        "reason": reason,
                        "type": "content_reported"
                    }),
                }
            },
        };
        
        self.core_service.send_notification(notification).await
    }
}
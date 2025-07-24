//! GraphQL subscription system for real-time social features
//!
//! Provides WebSocket-based subscriptions for:
//! - New comments on experiences
//! - Friend request updates
//! - Experience sharing events

use async_graphql::*;
use futures_util::Stream;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
use uuid::Uuid;
use crate::services::social::{SocialEvent, SocialFeaturesService};

/// Subscription root for social features
pub struct SocialSubscription;

#[Subscription]
impl SocialSubscription {
    /// Subscribe to new comments on a specific experience
    async fn new_comments(
        &self,
        ctx: &Context<'_>,
        experience_id: Uuid,
    ) -> Result<Pin<Box<dyn Stream<Item = CommentSubscription> + Send>>> {
        let service = ctx.data::<Arc<SocialFeaturesService>>()?;
        let mut rx = service.subscribe_events();
        
        let stream = async_stream::stream! {
            while let Ok(event) = rx.recv().await {
                if let SocialEvent::NewComment { experience_id: event_exp_id, comment } = event {
                    if event_exp_id == experience_id {
                        yield CommentSubscription {
                            id: comment.id,
                            experience_id: comment.experience_id,
                            author_id: comment.author_id,
                            content: comment.content,
                            created_at: comment.created_at,
                        };
                    }
                }
            }
        };
        
        Ok(Box::pin(stream))
    }

    /// Subscribe to friend request updates for a user
    async fn friend_request_updates(
        &self,
        ctx: &Context<'_>,
        user_id: Uuid,
    ) -> Result<Pin<Box<dyn Stream<Item = FriendRequestSubscription> + Send>>> {
        let service = ctx.data::<Arc<SocialFeaturesService>>()?;
        let mut rx = service.subscribe_events();
        
        let stream = async_stream::stream! {
            while let Ok(event) = rx.recv().await {
                if let SocialEvent::FriendRequest { from_user_id, to_user_id, status } = event {
                    if from_user_id == user_id || to_user_id == user_id {
                        yield FriendRequestSubscription {
                            from_user_id,
                            to_user_id,
                            status: match status {
                                crate::services::social::InvitationStatus::Pending => "PENDING".to_string(),
                                crate::services::social::InvitationStatus::Accepted => "ACCEPTED".to_string(),
                                crate::services::social::InvitationStatus::Rejected => "REJECTED".to_string(),
                            },
                        };
                    }
                }
            }
        };
        
        Ok(Box::pin(stream))
    }

    /// Subscribe to experience sharing events
    async fn experience_sharing_updates(
        &self,
        ctx: &Context<'_>,
        user_id: Option<Uuid>,
    ) -> Result<Pin<Box<dyn Stream<Item = ExperienceSharingSubscription> + Send>>> {
        let service = ctx.data::<Arc<SocialFeaturesService>>()?;
        let mut rx = service.subscribe_events();
        
        let stream = async_stream::stream! {
            while let Ok(event) = rx.recv().await {
                if let SocialEvent::ExperienceShared { experience_id, owner_id, visibility } = event {
                    // Only send to relevant users based on visibility
                    let should_send = match user_id {
                        Some(uid) => match visibility {
                            cpc_core::models::social::post::Visibility::Public => true,
                            cpc_core::models::social::post::Visibility::Friends => {
                                // TODO: Check if user is a friend
                                true
                            }
                            cpc_core::models::social::post::Visibility::Private => owner_id == uid,
                        },
                        None => visibility == cpc_core::models::social::post::Visibility::Public,
                    };
                    
                    if should_send {
                        yield ExperienceSharingSubscription {
                            experience_id,
                            owner_id,
                            visibility: match visibility {
                                cpc_core::models::social::post::Visibility::Public => "PUBLIC".to_string(),
                                cpc_core::models::social::post::Visibility::Friends => "FRIENDS".to_string(),
                                cpc_core::models::social::post::Visibility::Private => "PRIVATE".to_string(),
                            },
                        };
                    }
                }
            }
        };
        
        Ok(Box::pin(stream))
    }
}

/// Subscription payload for new comments
#[derive(SimpleObject, Clone)]
pub struct CommentSubscription {
    pub id: Uuid,
    pub experience_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Subscription payload for friend request updates
#[derive(SimpleObject, Clone)]
pub struct FriendRequestSubscription {
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub status: String,
}

/// Subscription payload for experience sharing events
#[derive(SimpleObject, Clone)]
pub struct ExperienceSharingSubscription {
    pub experience_id: Uuid,
    pub owner_id: Uuid,
    pub visibility: String,
}
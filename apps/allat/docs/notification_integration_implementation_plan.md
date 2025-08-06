# Notification System Integration Implementation Plan

## Overview

This document details the implementation plan for integrating the Allat app with the CPC notification system. The implementation will follow hexagonal architecture principles and leverage the existing `notification_core` shared package.

## Implementation Steps

### 1. Domain Layer Implementation

#### 1.1. Create Notification Events
Create `src/domain/notification_events.rs`:

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationEvent {
    PostReply {
        post_id: Uuid,
        post_title: String,
        replier_id: Uuid,
        replier_name: String,
        community_id: Uuid,
        community_name: String,
    },
    CommentReply {
        comment_id: Uuid,
        parent_comment_id: Uuid,
        replier_id: Uuid,
        replier_name: String,
        post_id: Uuid,
        post_title: String,
    },
    PostUpvoted {
        post_id: Uuid,
        post_title: String,
        voter_id: Uuid,
        voter_name: String,
        upvote_count: u32,
    },
    CommentUpvoted {
        comment_id: Uuid,
        voter_id: Uuid,
        voter_name: String,
        upvote_count: u32,
    },
    NewPostInCommunity {
        post_id: Uuid,
        post_title: String,
        author_id: Uuid,
        author_name: String,
        community_id: Uuid,
        community_name: String,
    },
    ContentReported {
        content_id: Uuid,
        content_type: ContentType,
        reporter_id: Uuid,
        reporter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Post,
    Comment,
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            ContentType::Post => "post".to_string(),
            ContentType::Comment => "comment".to_string(),
        }
    }
}
```

#### 1.2. Update Domain Module
Update `src/domain/mod.rs`:

```rust
// Add after existing imports
pub mod notification_events;

// Add to exports
pub use notification_events::*;
```

### 2. Application Layer Implementation

#### 2.1. Create Notification Service
Create `src/application/notification_service.rs`:

```rust
use async_trait::async_trait;
use std::sync::Arc;
use crate::domain::notification_events::NotificationEvent;
use crate::application::error::ApplicationError;

#[async_trait]
pub trait NotificationService: Send + Sync {
    async fn handle_event(&self, event: NotificationEvent) -> Result<(), ApplicationError>;
}

pub struct NotificationServiceImpl {
    notification_core_service: Arc<dyn NotificationCoreService>,
    // We might need user repository to get user details
    // user_repo: Arc<dyn UserRepository>,
}

impl NotificationServiceImpl {
    pub fn new(
        notification_core_service: Arc<dyn NotificationCoreService>,
    ) -> Self {
        Self {
            notification_core_service,
        }
    }
}

#[async_trait]
impl NotificationService for NotificationServiceImpl {
    async fn handle_event(&self, event: NotificationEvent) -> Result<(), ApplicationError> {
        // This will be implemented in the infrastructure layer
        todo!("Implement notification event handling")
    }
}

// We'll need to define the trait for the notification core service
// This should match the interface from notification_core
#[async_trait]
pub trait NotificationCoreService: Send + Sync {
    async fn send_notification(&self, notification: CoreNotification) -> Result<(), ApplicationError>;
}

// We'll also need to define the core notification structure
// This should match the structure from notification_core
#[derive(Debug, Clone)]
pub struct CoreNotification {
    pub user_id: String,
    pub category: NotificationCategory,
    pub priority: NotificationPriority,
    pub title: String,
    pub body: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone)]
pub enum NotificationCategory {
    Social,
    System,
    Marketing,
}

#[derive(Debug, Clone)]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Urgent,
}
```

#### 2.2. Update Application Module
Update `src/application/mod.rs`:

```rust
// Add after existing imports
pub mod notification_service;

// Add to exports
pub use notification_service::{NotificationService, NotificationServiceImpl};
```

### 3. Infrastructure Layer Implementation

#### 3.1. Create Notification Adapter
Create `src/infrastructure/notification_adapter.rs`:

```rust
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
```

#### 3.2. Create Notification Core Adapter
Create `src/infrastructure/notification_core_adapter.rs`:

```rust
use async_trait::async_trait;
use std::sync::Arc;
use notification_core::application::service::NotificationService as CoreNotificationService;
use notification_core::domain::types::{
    Notification as CoreNotificationType, 
    NotificationCategory as CoreNotificationCategory,
    NotificationPriority as CoreNotificationPriority,
    ChannelType
};
use crate::application::notification_service::{
    NotificationCoreService, CoreNotification, 
    NotificationCategory, NotificationPriority,
    ApplicationError
};

pub struct NotificationCoreAdapter {
    core_service: Arc<dyn CoreNotificationService>,
}

impl NotificationCoreAdapter {
    pub fn new(core_service: Arc<dyn CoreNotificationService>) -> Self {
        Self {
            core_service,
        }
    }
    
    fn map_category(category: NotificationCategory) -> CoreNotificationCategory {
        match category {
            NotificationCategory::Social => CoreNotificationCategory::Social,
            NotificationCategory::System => CoreNotificationCategory::System,
            NotificationCategory::Marketing => CoreNotificationCategory::Marketing,
        }
    }
    
    fn map_priority(priority: NotificationPriority) -> CoreNotificationPriority {
        match priority {
            NotificationPriority::Low => CoreNotificationPriority::Low,
            NotificationPriority::Normal => CoreNotificationPriority::Normal,
            NotificationPriority::High => CoreNotificationPriority::High,
            NotificationPriority::Urgent => CoreNotificationPriority::Urgent,
        }
    }
}

#[async_trait]
impl NotificationCoreService for NotificationCoreAdapter {
    async fn send_notification(&self, notification: CoreNotification) -> Result<(), ApplicationError> {
        let core_notification = CoreNotificationType::new_immediate(
            notification.user_id,
            Self::map_category(notification.category),
            Self::map_priority(notification.priority),
            notification.title,
            notification.body,
            notification.payload,
        );
        
        self.core_service.send(core_notification).await
            .map_err(|e| ApplicationError::ServiceError(format!("Failed to send notification: {}", e)))
            .map(|_| ())
    }
}
```

#### 3.3. Update Event Handlers in Services
Modify existing services to publish notification events. Update `src/application/comment_service.rs`:

```rust
// Add to imports
use crate::domain::notification_events::NotificationEvent;
use crate::application::notification_service::NotificationService;

// Add to CommentServiceImpl struct
pub struct CommentServiceImpl {
    comment_repo: Arc<dyn CommentRepository>,
    post_repo: Arc<dyn PostRepository>,
    notification_service: Option<Arc<dyn NotificationService>>, // Make optional for now
}

// Update constructor
impl CommentServiceImpl {
    pub fn new(
        comment_repo: Arc<dyn CommentRepository>,
        post_repo: Arc<dyn PostRepository>,
        notification_service: Option<Arc<dyn NotificationService>>,
    ) -> Self {
        Self { comment_repo, post_repo, notification_service }
    }
    
    // ... existing methods
}

// Update create_comment method
impl CommentServiceImpl {
    async fn create_comment(&self, input: CreateCommentInput) -> Result<Comment, ApplicationError> {
        // ... existing validation code
        
        // Create comment
        let comment = Comment::new(
            input.post_id,
            input.user_id,
            input.content,
            input.parent_id,
        );
        
        self.comment_repo.create(&comment).await?;
        
        // Send notification if service is available
        if let Some(ref notification_service) = self.notification_service {
            if let Some(parent_id) = input.parent_id {
                // This is a reply to a comment
                // We would need to fetch the parent comment and post details
                // For now, we'll create a simplified event
                let event = NotificationEvent::CommentReply {
                    comment_id: comment.id,
                    parent_comment_id: parent_id,
                    replier_id: comment.user_id,
                    replier_name: "User".to_string(), // We'd need to fetch the actual username
                    post_id: comment.post_id,
                    post_title: "Post".to_string(), // We'd need to fetch the actual post title
                };
                
                // In a real implementation, we'd handle errors appropriately
                let _ = notification_service.handle_event(event).await;
            } else {
                // This is a comment on a post
                // We would need to fetch the post details
                let event = NotificationEvent::PostReply {
                    post_id: comment.post_id,
                    post_title: "Post".to_string(), // We'd need to fetch the actual post title
                    replier_id: comment.user_id,
                    replier_name: "User".to_string(), // We'd need to fetch the actual username
                    community_id: Uuid::new_v4(), // We'd need to fetch the actual community ID
                    community_name: "Community".to_string(), // We'd need to fetch the actual community name
                };
                
                // In a real implementation, we'd handle errors appropriately
                let _ = notification_service.handle_event(event).await;
            }
        }
        
        Ok(comment)
    }
}
```

Update `src/application/post_service.rs`:

```rust
// Add to imports
use crate::domain::notification_events::NotificationEvent;
use crate::application::notification_service::NotificationService;

// Add to PostServiceImpl struct
pub struct PostServiceImpl {
    post_repo: Arc<dyn PostRepository>,
    community_repo: Arc<dyn CommunityRepository>,
    notification_service: Option<Arc<dyn NotificationService>>, // Make optional for now
}

// Update constructor
impl PostServiceImpl {
    pub fn new(
        post_repo: Arc<dyn PostRepository>,
        community_repo: Arc<dyn CommunityRepository>,
        notification_service: Option<Arc<dyn NotificationService>>,
    ) -> Self {
        Self { post_repo, community_repo, notification_service }
    }
}

// Update create_post method
impl PostServiceImpl {
    async fn create_post(&self, input: CreatePostInput) -> Result<Post, ApplicationError> {
        // ... existing validation code
        
        // Create post
        let post = Post::new(
            input.community_id,
            input.user_id,
            input.title,
            input.content,
            None, // Posts don't have parent_id, that's for comments
            input.media_assets,
        );
        
        self.post_repo.create(&post).await?;
        
        // Send notification if service is available
        if let Some(ref notification_service) = self.notification_service {
            let event = NotificationEvent::NewPostInCommunity {
                post_id: post.id,
                post_title: post.title.clone(),
                author_id: post.user_id,
                author_name: "User".to_string(), // We'd need to fetch the actual username
                community_id: post.community_id,
                community_name: "Community".to_string(), // We'd need to fetch the actual community name
            };
            
            // In a real implementation, we'd handle errors appropriately
            let _ = notification_service.handle_event(event).await;
        }
        
        Ok(post)
    }
}
```

### 4. Update Main Application to Register Services

Update `src/main.rs`:

```rust
// Add to imports
use crate::application::notification_service::{NotificationServiceImpl, NotificationService};
use crate::infrastructure::notification_core_adapter::NotificationCoreAdapter;
use notification_core::NotificationService as CoreNotificationService;

// In main function, after other service initializations
// Initialize the core notification service
// This would typically come from dependency injection or a service locator
// For now, we'll create a placeholder
let core_notification_service: Arc<dyn CoreNotificationService> = Arc::new(create_core_notification_service().await);

// Create the adapter
let notification_core_adapter: Arc<dyn crate::application::notification_service::NotificationCoreService> = 
    Arc::new(NotificationCoreAdapter::new(core_notification_service.clone()));

// Create the notification service
let notification_service: Arc<dyn NotificationService> = Arc::new(
    NotificationServiceImpl::new(notification_core_adapter)
);

// Update service initializations to pass notification service
let comment_service: Arc<dyn CommentService> = Arc::new(CommentServiceImpl::new(
    comment_repo.clone(),
    post_repo.clone(),
    Some(notification_service.clone()),
));

let post_service: Arc<dyn PostService> = Arc::new(PostServiceImpl::new(
    post_repo.clone(),
    community_repo.clone(),
    Some(notification_service.clone()),
));

// Helper function to create core notification service
async fn create_core_notification_service() -> impl CoreNotificationService {
    // In a real implementation, this would initialize the actual core notification service
    // with all its channels and configuration
    todo!("Implement core notification service initialization")
}
```

### 5. API Layer Updates

We don't need major API layer changes since notifications are sent automatically based on events. However, we might want to add a way for users to manage their notification preferences.

#### 5.1. Add Notification Preferences to User Object
Update `src/api/objects/user.rs`:

```rust
// Add notification preferences field to UserObject
#[Object]
impl UserObject {
    // ... existing methods
    
    async fn notification_preferences(&self, ctx: &Context<'_>) -> Result<NotificationPreferencesObject> {
        // In a real implementation, we would fetch the user's notification preferences
        // from the notification core service
        Ok(NotificationPreferencesObject {
            email_notifications: true,
            push_notifications: true,
            social_notifications: true,
        })
    }
}

// Create NotificationPreferencesObject
#[derive(SimpleObject)]
pub struct NotificationPreferencesObject {
    email_notifications: bool,
    push_notifications: bool,
    social_notifications: bool,
}
```

### 6. WebSocket Integration

Update `src/api/subscriptions.rs` to handle real-time notification delivery:

```rust
// Add to imports
use crate::application::notification_service::NotificationService;

// Add subscription for notifications
pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn notifications(&self, ctx: &Context<'_>, user_id: Uuid) -> Result<impl Stream<Item = NotificationEventObject>> {
        // In a real implementation, we would subscribe to the notification service
        // and stream notifications to the client
        todo!("Implement notification streaming")
    }
}

// Create NotificationEventObject for GraphQL
pub struct NotificationEventObject(crate::domain::notification_events::NotificationEvent);

#[Object]
impl NotificationEventObject {
    async fn event_type(&self) -> &str {
        match &self.0 {
            crate::domain::notification_events::NotificationEvent::PostReply { .. } => "POST_REPLY",
            crate::domain::notification_events::NotificationEvent::CommentReply { .. } => "COMMENT_REPLY",
            crate::domain::notification_events::NotificationEvent::PostUpvoted { .. } => "POST_UPVOTED",
            crate::domain::notification_events::NotificationEvent::CommentUpvoted { .. } => "COMMENT_UPVOTED",
            crate::domain::notification_events::NotificationEvent::NewPostInCommunity { .. } => "NEW_POST",
            crate::domain::notification_events::NotificationEvent::ContentReported { .. } => "CONTENT_REPORTED",
        }
    }
    
    // Add other fields as needed
}
```

### 7. Testing

#### 7.1. Unit Tests for Notification Service
Create `tests/notification_service_test.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::notification_service::NotificationServiceImpl;
    use crate::domain::notification_events::NotificationEvent;
    use std::sync::Arc;
    
    // Mock notification core service
    struct MockNotificationCoreService;
    
    #[async_trait::async_trait]
    impl crate::application::notification_service::NotificationCoreService for MockNotificationCoreService {
        async fn send_notification(&self, _notification: crate::application::notification_service::CoreNotification) -> Result<(), crate::application::error::ApplicationError> {
            // Mock implementation
            Ok(())
        }
    }
    
    #[tokio::test]
    async fn test_handle_post_reply_notification() {
        let core_service = Arc::new(MockNotificationCoreService);
        let service = NotificationServiceImpl::new(core_service);
        
        let event = NotificationEvent::PostReply {
            post_id: Uuid::new_v4(),
            post_title: "Test Post".to_string(),
            replier_id: Uuid::new_v4(),
            replier_name: "Test User".to_string(),
            community_id: Uuid::new_v4(),
            community_name: "Test Community".to_string(),
        };
        
        // This test would verify that the notification is properly formatted and sent
        // In a real implementation, we would mock the core service and verify the call
        let result = service.handle_event(event).await;
        // Assertions would go here
    }
}
```

#### 7.2. Integration Tests
Update `tests/integration_test.rs` to include notification functionality tests.

### 8. Documentation Updates

Update `docs/allat_architecture.md` to reflect the new notification functionality:

1. Add notification integration to the Integration Points section
2. Update the TODO list to mark notification integration as complete
3. Add details about the notification implementation to the Infrastructure Layer section

## Dependencies

This implementation depends on:
1. The existing `notification_core` shared package
2. The event bus system (if used for real-time delivery)
3. The existing repository implementations
4. The GraphQL API layer

## Timeline

Estimated implementation time: 3-4 days

1. Day 1: Domain layer and basic infrastructure
2. Day 2: Notification service implementation and adapter
3. Day 3: Integration with existing services and API layer
4. Day 4: Testing and documentation

## Rollback Plan

If issues are encountered:
1. Remove the new service registrations
2. Revert code changes to service layers
3. Remove notification-related dependencies
4. Update documentation to reflect rollback

## Security Considerations

1. Ensure that only authorized users can trigger notifications
2. Validate all notification data to prevent injection attacks
3. Respect user privacy settings when sending notifications
4. Implement rate limiting to prevent notification spam
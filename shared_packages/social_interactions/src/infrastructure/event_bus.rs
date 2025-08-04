//! Event bus for social interactions
//!
//! This module provides an event bus for real-time updates of social interactions.

use crate::domain::models::{Reaction, Comment, Share};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Event types for social interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocialEvent {
    /// A new reaction was added
    ReactionAdded {
        reaction: Reaction,
        target_owner_id: Uuid,
    },
    /// A reaction was removed
    ReactionRemoved {
        reaction_id: Uuid,
        user_id: Uuid,
        target_owner_id: Uuid,
    },
    /// A new comment was added
    CommentAdded {
        comment: Comment,
        target_owner_id: Uuid,
        parent_comment_owner_id: Option<Uuid>,
    },
    /// A comment was updated
    CommentUpdated {
        comment: Comment,
        target_owner_id: Uuid,
    },
    /// A comment was deleted
    CommentDeleted {
        comment_id: Uuid,
        user_id: Uuid,
        target_owner_id: Uuid,
    },
    /// Content was shared
    ContentShared {
        share: Share,
        content_owner_id: Uuid,
    },
    /// Collaborative document updated event
    DocumentUpdated {
        document_id: Uuid,
        user_id: Uuid,
    },
    /// Project task moved between columns or positions
    TaskMoved {
        task_id: Uuid,
        new_column_id: Uuid,
    },
    /// Whiteboard modified
    WhiteboardModified {
        whiteboard_id: Uuid,
    },
    /// Meeting started
    MeetingStarted {
        meeting_id: Uuid,
    },
    /// Volunteer: An opportunity was created/published
    OpportunityCreated {
        opportunity_id: Uuid,
        org_id: Uuid,
        created_by: Uuid,
    },
    /// Volunteer: A user submitted an application
    ApplicationSubmitted {
        application_id: Uuid,
        opportunity_id: Uuid,
        applicant_id: Uuid,
    },
    /// Volunteer: A contribution was logged
    ContributionLogged {
        contribution_id: Uuid,
        opportunity_id: Uuid,
        contributor_id: Uuid,
    }
}

/// Trait for event handlers that can process social events
#[async_trait]
pub trait SocialEventHandler: Send + Sync {
    /// Handle a social event
    async fn handle_event(&self, event: SocialEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Event bus for social interactions
pub struct SocialEventBus {
    handlers: Arc<Mutex<HashMap<Uuid, Weak<dyn SocialEventHandler>>>>,
}

impl SocialEventBus {
    /// Create a new SocialEventBus
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Register an event handler
    pub fn register_handler(&self, handler_id: Uuid, handler: Arc<dyn SocialEventHandler>) {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.insert(handler_id, Arc::downgrade(&handler));
    }
    
    /// Unregister an event handler
    pub fn unregister_handler(&self, handler_id: Uuid) {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.remove(&handler_id);
    }
    
    /// Publish an event to all registered handlers
    pub async fn publish_event(&self, event: SocialEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let handlers = self.handlers.lock().unwrap();
        let mut dead_handlers = Vec::new();
        
        for (handler_id, handler_weak) in handlers.iter() {
            if let Some(handler) = handler_weak.upgrade() {
                if let Err(e) = handler.handle_event(event.clone()).await {
                    eprintln!("Error in handler {}: {}", handler_id, e);
                }
            } else {
                dead_handlers.push(*handler_id);
            }
        }
        
        // Clean up dead handlers
        drop(handlers);
        if !dead_handlers.is_empty() {
            let mut handlers = self.handlers.lock().unwrap();
            for handler_id in dead_handlers {
                handlers.remove(&handler_id);
            }
        }
        
        Ok(())
    }
    
    /// Get the number of registered handlers
    pub fn handler_count(&self) -> usize {
        let handlers = self.handlers.lock().unwrap();
        handlers.len()
    }
}

impl Default for SocialEventBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Example implementation of a SocialEventHandler for logging
pub struct LoggingEventHandler {
    name: String,
}

impl LoggingEventHandler {
    /// Create a new LoggingEventHandler
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[async_trait]
impl SocialEventHandler for LoggingEventHandler {
    async fn handle_event(&self, event: SocialEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match event {
            SocialEvent::ReactionAdded { reaction, target_owner_id } => {
                println!("[{}] Reaction added: {:?} by {} to {} (owner: {})",
                    self.name, reaction.reaction_type, reaction.user_id, reaction.target_id, target_owner_id);
            }
            SocialEvent::ReactionRemoved { reaction_id, user_id, target_owner_id } => {
                println!("[{}] Reaction removed: {} by {} (owner: {})",
                    self.name, reaction_id, user_id, target_owner_id);
            }
            SocialEvent::CommentAdded { comment, target_owner_id, parent_comment_owner_id } => {
                println!("[{}] Comment added: {} by {} to {} (owner: {}, parent: {:?})",
                    self.name, comment.id, comment.user_id, comment.target_id, target_owner_id, parent_comment_owner_id);
            }
            SocialEvent::CommentUpdated { comment, target_owner_id } => {
                println!("[{}] Comment updated: {} by {} to {} (owner: {})",
                    self.name, comment.id, comment.user_id, comment.target_id, target_owner_id);
            }
            SocialEvent::CommentDeleted { comment_id, user_id, target_owner_id } => {
                println!("[{}] Comment deleted: {} by {} (owner: {})",
                    self.name, comment_id, user_id, target_owner_id);
            }
            SocialEvent::ContentShared { share, content_owner_id } => {
                println!("[{}] Content shared: {} by {} (content: {}, owner: {})",
                    self.name, share.id, share.user_id, share.content_id, content_owner_id);
            }
            SocialEvent::DocumentUpdated { document_id, user_id } => {
                println!("[{}] Document updated: {} by {}", self.name, document_id, user_id);
            }
            SocialEvent::TaskMoved { task_id, new_column_id } => {
                println!("[{}] Task moved: {} to column {}", self.name, task_id, new_column_id);
            }
            SocialEvent::WhiteboardModified { whiteboard_id } => {
                println!("[{}] Whiteboard modified: {}", self.name, whiteboard_id);
            }
            SocialEvent::MeetingStarted { meeting_id } => {
                println!("[{}] Meeting started: {}", self.name, meeting_id);
            }
            SocialEvent::OpportunityCreated { opportunity_id, org_id, created_by } => {
                println!("[{}] Opportunity created: {} by {} (org {})", self.name, opportunity_id, created_by, org_id);
            }
            SocialEvent::ApplicationSubmitted { application_id, opportunity_id, applicant_id } => {
                println!("[{}] Application submitted: {} for {} by {}", self.name, application_id, opportunity_id, applicant_id);
            }
            SocialEvent::ContributionLogged { contribution_id, opportunity_id, contributor_id } => {
                println!("[{}] Contribution logged: {} for {} by {}", self.name, contribution_id, opportunity_id, contributor_id);
            }
        }
        Ok(())
    }
}

/// Example implementation of a SocialEventHandler that forwards to a notification service
pub struct NotificationEventHandler {
    notification_service: Arc<dyn crate::application::notification_integration::SocialNotificationIntegration>,
}

impl NotificationEventHandler {
    /// Create a new NotificationEventHandler
    pub fn new(notification_service: Arc<dyn crate::application::notification_integration::SocialNotificationIntegration>) -> Self {
        Self { notification_service }
    }
}

#[async_trait]
impl SocialEventHandler for NotificationEventHandler {
    async fn handle_event(&self, event: SocialEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match event {
            SocialEvent::ReactionAdded { reaction, target_owner_id } => {
                self.notification_service.send_reaction_notification(&reaction, target_owner_id).await?;
            }
            SocialEvent::CommentAdded { comment, target_owner_id, parent_comment_owner_id } => {
                self.notification_service.send_comment_notification(&comment, target_owner_id, parent_comment_owner_id).await?;
            }
            SocialEvent::ContentShared { share, content_owner_id } => {
                self.notification_service.send_share_notification(&share, content_owner_id).await?;
            }
            // Collaboration events currently don't trigger notifications here.
            SocialEvent::DocumentUpdated { .. }
            | SocialEvent::TaskMoved { .. }
            | SocialEvent::WhiteboardModified { .. }
            | SocialEvent::MeetingStarted { .. }
            | SocialEvent::OpportunityCreated { .. }
            | SocialEvent::ApplicationSubmitted { .. }
            | SocialEvent::ContributionLogged { .. } => { }
        }
        Ok(())
    }
}
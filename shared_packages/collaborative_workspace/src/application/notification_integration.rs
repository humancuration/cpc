//! Notification integration for collaborative workspace events
//!
//! Maps collaboration events to user-facing notifications using notification_core.
//!
//! - DocumentUpdated → notify document owner
//! - TaskMoved → notify task creator
//! - MeetingStarted → notify invited users (stub list)
//!
//! This mirrors the pattern used in social_interactions::application::notification_integration.

use std::sync::Arc;
use uuid::Uuid;

use serde::{Deserialize, Serialize};
use shared_packages::notification_core::application::service::NotificationService;
use shared_packages::notification_core::domain::preferences::UserPreferences;
use shared_packages::notification_core::domain::types::{
    ChannelType, Notification, NotificationCategory, NotificationPriority,
};

use crate::application::document_service::DocumentUpdated;
use crate::application::meeting_service::MeetingStarted;
use crate::application::project_service::TaskMoved;

/// Abstraction to resolve recipients and fetch any details we need for notifications.
///
/// In a full implementation this would query repositories to resolve owners/creators/invitees.
/// We keep a minimal trait here to avoid coupling to infra in the application module.
#[async_trait::async_trait]
pub trait CollaborationNotificationContext: Send + Sync {
    /// Given a document id, returns its owner id.
    async fn get_document_owner(&self, document_id: Uuid) -> Result<Uuid, String>;
    /// Given a task id, returns its creator id.
    async fn get_task_creator(&self, task_id: Uuid) -> Result<Uuid, String>;
    /// Given a meeting id, returns the invited users (excluding owner if desired).
    async fn get_meeting_invitees(&self, meeting_id: Uuid) -> Result<Vec<Uuid>, String>;
}

/// Service that bridges collaboration events to NotificationService
pub struct CollaborationNotificationIntegration<C: CollaborationNotificationContext> {
    notification_service: Arc<dyn NotificationService>,
    ctx: Arc<C>,
}

impl<C: CollaborationNotificationContext> CollaborationNotificationIntegration<C> {
    pub fn new(notification_service: Arc<dyn NotificationService>, ctx: Arc<C>) -> Self {
        Self {
            notification_service,
            ctx,
        }
    }

    /// Handle a DocumentUpdated event by notifying the document owner.
    pub async fn handle_document_updated(
        &self,
        evt: &DocumentUpdated,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let owner_id = self.ctx.get_document_owner(evt.document_id).await?;
        let preferences = UserPreferences::new(); // TODO: fetch real user prefs

        if !preferences.enabled {
            return Ok(());
        }

        // Use Social category for now until collaborative categories are added to notification_core.
        let notification = Notification::new_immediate(
            owner_id.to_string(),
            NotificationCategory::Social,
            NotificationPriority::Normal,
            "Document Updated".to_string(),
            "Your document was updated".to_string(),
            serde_json::json!({
                "document_id": evt.document_id,
                "updated_by": evt.updated_by,
                "updated_at": evt.updated_at,
                "state_hash": evt.state_hash,
            }),
            vec![ChannelType::InApp, ChannelType::Push],
        );

        self.notification_service
            .send_notification(&notification)
            .await?;
        Ok(())
    }

    /// Handle a TaskMoved event by notifying the task creator.
    pub async fn handle_task_moved(
        &self,
        evt: &TaskMoved,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let creator_id = self.ctx.get_task_creator(evt.task_id).await?;
        let preferences = UserPreferences::new();

        if !preferences.enabled {
            return Ok(());
        }

        let notification = Notification::new_immediate(
            creator_id.to_string(),
            NotificationCategory::Social,
            NotificationPriority::Normal,
            "Task Moved".to_string(),
            "Your task was moved".to_string(),
            serde_json::json!({
                "task_id": evt.task_id,
                "from_column_id": evt.from_column_id,
                "to_column_id": evt.to_column_id,
                "new_position": evt.new_position,
                "moved_by": evt.moved_by,
                "moved_at": evt.moved_at,
            }),
            vec![ChannelType::InApp, ChannelType::Push],
        );

        self.notification_service
            .send_notification(&notification)
            .await?;
        Ok(())
    }

    /// Handle a MeetingStarted event by notifying invited users.
    pub async fn handle_meeting_started(
        &self,
        evt: &MeetingStarted,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let invitees = self.ctx.get_meeting_invitees(evt.meeting_id).await?;

        for user_id in invitees {
            let preferences = UserPreferences::new();
            if !preferences.enabled {
                continue;
            }

            let notification = Notification::new_immediate(
                user_id.to_string(),
                NotificationCategory::Social,
                NotificationPriority::Normal,
                "Meeting Started".to_string(),
                "You are invited to a meeting that just started".to_string(),
                serde_json::json!({
                    "meeting_id": evt.meeting_id,
                    "owner_id": evt.owner_id,
                    "started_at": evt.started_at,
                }),
                vec![ChannelType::InApp, ChannelType::Push],
            );
            // Fire and continue; if one fails we log in real impl
            let _ = self.notification_service.send_notification(&notification).await;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct StubNotificationService;
    #[async_trait]
    impl NotificationService for StubNotificationService {
        async fn send_notification(
            &self,
            _notification: &Notification,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            Ok(())
        }
    }

    struct StubCtx;
    #[async_trait]
    impl CollaborationNotificationContext for StubCtx {
        async fn get_document_owner(&self, _document_id: Uuid) -> Result<Uuid, String> {
            Ok(Uuid::nil())
        }
        async fn get_task_creator(&self, _task_id: Uuid) -> Result<Uuid, String> {
            Ok(Uuid::nil())
        }
        async fn get_meeting_invitees(&self, _meeting_id: Uuid) -> Result<Vec<Uuid>, String> {
            Ok(vec![Uuid::nil()])
        }
    }

    #[tokio::test]
    async fn sends_all_three_notification_types() {
        let svc = CollaborationNotificationIntegration::new(
            Arc::new(StubNotificationService),
            Arc::new(StubCtx),
        );

        let doc_evt = DocumentUpdated {
            document_id: Uuid::new_v4(),
            updated_by: Uuid::new_v4(),
            updated_at: chrono::Utc::now(),
            state_hash: None,
        };
        svc.handle_document_updated(&doc_evt).await.unwrap();

        let task_evt = TaskMoved {
            task_id: Uuid::new_v4(),
            from_column_id: Uuid::new_v4(),
            to_column_id: Uuid::new_v4(),
            new_position: 1,
            moved_by: Uuid::new_v4(),
            moved_at: chrono::Utc::now(),
        };
        svc.handle_task_moved(&task_evt).await.unwrap();

        let meeting_evt = MeetingStarted {
            meeting_id: Uuid::new_v4(),
            owner_id: Uuid::new_v4(),
            started_at: chrono::Utc::now(),
        };
        svc.handle_meeting_started(&meeting_evt).await.unwrap();
    }
}
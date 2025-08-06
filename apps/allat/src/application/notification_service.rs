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
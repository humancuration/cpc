//! Backend notification service for real-time updates
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Notification types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    JobCompleted {
        job_id: Uuid,
        job_type: String,
        success: bool,
    },
    JobFailed {
        job_id: Uuid,
        job_type: String,
        error_message: String,
    },
    JobStarted {
        job_id: Uuid,
        job_type: String,
    },
    InvoiceIssued {
        invoice_id: Uuid,
        due_date: DateTime<Utc>,
    },
    PaymentReminder {
        invoice_id: Uuid,
        days_until_due: i32,
    },
    InvoicePaid {
        invoice_id: Uuid,
        payment_method: String,
        amount: f64,
        currency: String,
    },
}

/// Notification payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notification_type: NotificationType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: serde_json::Value,
}

/// Notification service for managing real-time updates
#[derive(Debug, Clone)]
pub struct NotificationService {
    /// Broadcast channels for different users
    channels: Arc<RwLock<HashMap<Uuid, broadcast::Sender<Notification>>>>,
}

impl NotificationService {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Get or create a broadcast channel for a user
    pub async fn get_channel(&self, user_id: Uuid) -> broadcast::Sender<Notification> {
        let mut channels = self.channels.write().await;
        
        if let Some(sender) = channels.get(&user_id) {
            sender.clone()
        } else {
            let (sender, _) = broadcast::channel(100);
            channels.insert(user_id, sender.clone());
            sender
        }
    }
    
    /// Send a notification to a specific user
    pub async fn send_notification(
        &self,
        user_id: Uuid,
        notification_type: NotificationType,
        metadata: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let channel = self.get_channel(user_id).await;
        
        let notification = Notification {
            id: Uuid::new_v4(),
            user_id,
            notification_type,
            timestamp: chrono::Utc::now(),
            metadata,
        };
        
        // Send to the channel
        if let Err(e) = channel.send(notification.clone()) {
            tracing::warn!("Failed to send notification to user {}: {}", user_id, e);
        }
        
        // Also store in database for persistence
        self.store_notification(&notification).await?;
        
        Ok(())
    }
    
    /// Store notification in database
    async fn store_notification(
        &self,
        notification: &Notification,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // This would typically use the database connection
        // For now, we'll just log it
        tracing::info!(
            "Storing notification: {:?} for user {}",
            notification.notification_type,
            notification.user_id
        );
        
        Ok(())
    }
    
    /// Subscribe to notifications for a specific user
    pub async fn subscribe(
        &self,
        user_id: Uuid,
    ) -> broadcast::Receiver<Notification> {
        let channel = self.get_channel(user_id).await;
        channel.subscribe()
    }
}

impl Default for NotificationService {
    fn default() -> Self {
        Self::new()
    }
}
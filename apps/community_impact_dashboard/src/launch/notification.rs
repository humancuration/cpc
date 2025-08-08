//! Community notification system
//!
//! This module provides functionality for notifying community members
//! about the dashboard launch and related activities.

use tracing::info;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Community notification system
pub struct CommunityNotifier {
    notifications: Vec<CommunityNotification>,
}

impl CommunityNotifier {
    /// Create a new community notifier
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
        }
    }
    
    /// Send a notification to the community
    pub fn send_notification(
        &mut self,
        notification_type: NotificationType,
        title: &str,
        message: &str,
        recipients: Option<Vec<String>>, // None for all community members
    ) -> Result<Uuid, NotificationError> {
        let notification_id = Uuid::new_v4();
        
        let notification = CommunityNotification {
            id: notification_id,
            notification_type,
            title: title.to_string(),
            message: message.to_string(),
            recipients: recipients.unwrap_or_else(Vec::new),
            sent_at: Utc::now(),
            read_by: Vec::new(),
        };
        
        self.notifications.push(notification);
        info!("Sent {} notification: {}", notification_type, title);
        
        // In a real implementation, this would integrate with the social_interactions
        // package to actually send notifications to users
        
        Ok(notification_id)
    }
    
    /// Mark a notification as read by a user
    pub fn mark_as_read(&mut self, notification_id: Uuid, user_id: &str) -> bool {
        if let Some(notification) = self.notifications.iter_mut()
            .find(|n| n.id == notification_id) {
            if !notification.read_by.contains(&user_id.to_string()) {
                notification.read_by.push(user_id.to_string());
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    
    /// Get unread notifications for a user
    pub fn get_unread_notifications(&self, user_id: &str) -> Vec<&CommunityNotification> {
        self.notifications.iter()
            .filter(|notification| {
                // If recipients is empty, it's for all users
                // Otherwise, check if user is in recipients
                (notification.recipients.is_empty() || 
                 notification.recipients.contains(&user_id.to_string())) &&
                !notification.read_by.contains(&user_id.to_string())
            })
            .collect()
    }
    
    /// Get notification statistics
    pub fn get_notification_stats(&self) -> NotificationStats {
        let total_sent = self.notifications.len();
        let mut read_count = 0;
        let mut unread_count = 0;
        
        for notification in &self.notifications {
            read_count += notification.read_by.len();
            // This is a simplification - in reality we'd need to track per-user
            unread_count += if notification.read_by.is_empty() { 1 } else { 0 };
        }
        
        NotificationStats {
            total_sent,
            total_reads: read_count,
            unread_notifications: unread_count,
        }
    }
}

impl Default for CommunityNotifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Types of notifications that can be sent
#[derive(Debug, Clone)]
pub enum NotificationType {
    /// Pre-launch announcement
    PreLaunch,
    
    /// Launch announcement
    Launch,
    
    /// Feature update
    FeatureUpdate,
    
    /// Community celebration
    Celebration,
    
    /// Important information
    Important,
    
    /// Feedback request
    FeedbackRequest,
}

impl std::fmt::Display for NotificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationType::PreLaunch => write!(f, "Pre-Launch"),
            NotificationType::Launch => write!(f, "Launch"),
            NotificationType::FeatureUpdate => write!(f, "Feature Update"),
            NotificationType::Celebration => write!(f, "Celebration"),
            NotificationType::Important => write!(f, "Important"),
            NotificationType::FeedbackRequest => write!(f, "Feedback Request"),
        }
    }
}

/// Community notification structure
#[derive(Debug, Clone)]
pub struct CommunityNotification {
    /// Unique identifier for this notification
    pub id: Uuid,
    
    /// Type of notification
    pub notification_type: NotificationType,
    
    /// Notification title
    pub title: String,
    
    /// Notification message
    pub message: String,
    
    /// Intended recipients (empty for all community members)
    pub recipients: Vec<String>,
    
    /// When the notification was sent
    pub sent_at: DateTime<Utc>,
    
    /// Users who have read this notification
    pub read_by: Vec<String>,
}

/// Notification statistics
#[derive(Debug, Clone)]
pub struct NotificationStats {
    /// Total notifications sent
    pub total_sent: usize,
    
    /// Total times notifications have been read
    pub total_reads: usize,
    
    /// Number of unread notifications
    pub unread_notifications: usize,
}

/// Error type for notification operations
#[derive(Debug)]
pub enum NotificationError {
    /// Failed to send notification
    SendFailed(String),
}

impl std::fmt::Display for NotificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationError::SendFailed(msg) => write!(f, "Failed to send notification: {}", msg),
        }
    }
}

impl std::error::Error for NotificationError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_send_notification() {
        let mut notifier = CommunityNotifier::new();
        let result = notifier.send_notification(
            NotificationType::Launch,
            "Dashboard Launch",
            "The Unified Community Impact Dashboard is now live!",
            None,
        );
        
        assert!(result.is_ok());
        assert_eq!(notifier.notifications.len(), 1);
    }
    
    #[test]
    fn test_mark_as_read() {
        let mut notifier = CommunityNotifier::new();
        let notification_id = notifier.send_notification(
            NotificationType::Launch,
            "Dashboard Launch",
            "The Unified Community Impact Dashboard is now live!",
            None,
        ).unwrap();
        
        assert!(notifier.mark_as_read(notification_id, "user123"));
        assert!(!notifier.mark_as_read(notification_id, "user123")); // Already read
    }
    
    #[test]
    fn test_get_unread_notifications() {
        let mut notifier = CommunityNotifier::new();
        let notification_id = notifier.send_notification(
            NotificationType::Launch,
            "Dashboard Launch",
            "The Unified Community Impact Dashboard is now live!",
            Some(vec!["user123".to_string(), "user456".to_string()]),
        ).unwrap();
        
        let unread = notifier.get_unread_notifications("user123");
        assert_eq!(unread.len(), 1);
        
        notifier.mark_as_read(notification_id, "user123");
        
        let unread = notifier.get_unread_notifications("user123");
        assert_eq!(unread.len(), 0);
    }
}
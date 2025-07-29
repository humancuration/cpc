//! Notification types, channels
//! 
//! This module defines the core notification structures and channel types.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::fmt;

/// Notification category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationCategory {
    /// System notifications
    System,
    /// Transaction-related notifications
    Transaction,
    /// Calendar event notifications
    Calendar,
    /// Health-related notifications
    Health,
    /// Social notifications
    Social,
    /// Security notifications
    Security,
    /// Marketing notifications
    Marketing,
}

impl fmt::Display for NotificationCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotificationCategory::System => write!(f, "System"),
            NotificationCategory::Transaction => write!(f, "Transaction"),
            NotificationCategory::Calendar => write!(f, "Calendar"),
            NotificationCategory::Health => write!(f, "Health"),
            NotificationCategory::Social => write!(f, "Social"),
            NotificationCategory::Security => write!(f, "Security"),
            NotificationCategory::Marketing => write!(f, "Marketing"),
        }
    }
}

/// Notification priority
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationPriority {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    High,
    /// Urgent priority
    Urgent,
}

impl fmt::Display for NotificationPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotificationPriority::Low => write!(f, "Low"),
            NotificationPriority::Normal => write!(f, "Normal"),
            NotificationPriority::High => write!(f, "High"),
            NotificationPriority::Urgent => write!(f, "Urgent"),
        }
    }
}

/// Channel type for delivery
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChannelType {
    /// Push notification (mobile)
    Push,
    /// Email notification
    Email,
    /// In-app notification
    InApp,
    /// SMS notification
    Sms,
    /// Social media notification
    Social,
}

impl fmt::Display for ChannelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChannelType::Push => write!(f, "Push"),
            ChannelType::Email => write!(f, "Email"),
            ChannelType::InApp => write!(f, "InApp"),
            ChannelType::Sms => write!(f, "Sms"),
            ChannelType::Social => write!(f, "Social"),
        }
    }
}

/// Core notification structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Unique identifier for the notification
    pub id: Uuid,
    
    /// User ID this notification is for
    pub user_id: String,
    
    /// Category of the notification
    pub category: NotificationCategory,
    
    /// Priority of the notification
    pub priority: NotificationPriority,
    
    /// Title of the notification
    pub title: String,
    
    /// Body/content of the notification
    pub body: String,
    
    /// Additional payload data
    pub payload: serde_json::Value,
    
    /// Channels to deliver this notification through
    pub delivery_channels: Vec<ChannelType>,
    
    /// Scheduled time for delivery (None means immediate)
    pub scheduled_time: Option<DateTime<Utc>>,
    
    /// Timestamp when the notification was created
    pub created_at: DateTime<Utc>,
    
    /// Timestamp when the notification was delivered
    pub delivered_at: Option<DateTime<Utc>>,
    
    /// Whether the notification has been read
    pub read: bool,
}

impl Notification {
    /// Create a new notification
    pub fn new(
        user_id: String,
        category: NotificationCategory,
        priority: NotificationPriority,
        title: String,
        body: String,
        payload: serde_json::Value,
        delivery_channels: Vec<ChannelType>,
        scheduled_time: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            category,
            priority,
            title,
            body,
            payload,
            delivery_channels,
            scheduled_time,
            created_at: Utc::now(),
            delivered_at: None,
            read: false,
        }
    }
    
    /// Create a new immediate notification
    pub fn new_immediate(
        user_id: String,
        category: NotificationCategory,
        priority: NotificationPriority,
        title: String,
        body: String,
        payload: serde_json::Value,
        delivery_channels: Vec<ChannelType>,
    ) -> Self {
        Self::new(
            user_id,
            category,
            priority,
            title,
            body,
            payload,
            delivery_channels,
            None,
        )
    }
    
    /// Create a new scheduled notification
    pub fn new_scheduled(
        user_id: String,
        category: NotificationCategory,
        priority: NotificationPriority,
        title: String,
        body: String,
        payload: serde_json::Value,
        delivery_channels: Vec<ChannelType>,
        scheduled_time: DateTime<Utc>,
    ) -> Self {
        Self::new(
            user_id,
            category,
            priority,
            title,
            body,
            payload,
            delivery_channels,
            Some(scheduled_time),
        )
    }
    
    /// Mark the notification as delivered
    pub fn mark_delivered(&mut self) {
        self.delivered_at = Some(Utc::now());
    }
    
    /// Mark the notification as read
    pub fn mark_read(&mut self) {
        self.read = true;
    }
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Notification({}, {}, {})",
            self.category, self.title, self.id
        )
    }
}
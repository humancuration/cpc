//! User preferences
//! 
//! This module handles user notification preferences and settings.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::domain::types::{NotificationCategory, ChannelType, NotificationPriority};

/// User notification preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    /// Whether notifications are enabled globally
    pub enabled: bool,
    
    /// Default priority for notifications
    pub default_priority: NotificationPriority,
    
    /// Per-category preferences
    pub category_preferences: HashMap<NotificationCategory, CategoryPreference>,
    
    /// Per-channel preferences
    pub channel_preferences: HashMap<ChannelType, ChannelPreference>,
    
    /// Quiet hours (start and end times in 24-hour format)
    pub quiet_hours: Option<(u8, u8)>,
    
    /// Timezone for scheduling
    pub timezone: String,
}

impl UserPreferences {
    /// Create new user preferences with default settings
    pub fn new() -> Self {
        Self {
            enabled: true,
            default_priority: NotificationPriority::Normal,
            category_preferences: HashMap::new(),
            channel_preferences: HashMap::new(),
            quiet_hours: None,
            timezone: "UTC".to_string(),
        }
    }
    
    /// Check if a notification should be sent based on preferences
    pub fn should_send_notification(
        &self,
        category: &NotificationCategory,
        channel: &ChannelType,
        priority: &NotificationPriority,
    ) -> bool {
        // Check if notifications are globally enabled
        if !self.enabled {
            return false;
        }
        
        // Check quiet hours
        if let Some((start, end)) = self.quiet_hours {
            let current_hour = chrono::Utc::now().hour() as u8;
            if current_hour >= start && current_hour < end {
                // Only allow high priority notifications during quiet hours
                if *priority != NotificationPriority::High && *priority != NotificationPriority::Urgent {
                    return false;
                }
            }
        }
        
        // Check category preferences
        if let Some(category_pref) = self.category_preferences.get(category) {
            if !category_pref.enabled {
                return false;
            }
            
            // Check if the priority meets the minimum requirement
            if Self::priority_value(priority) < Self::priority_value(&category_pref.min_priority) {
                return false;
            }
        }
        
        // Check channel preferences
        if let Some(channel_pref) = self.channel_preferences.get(channel) {
            if !channel_pref.enabled {
                return false;
            }
        }
        
        true
    }
    
    /// Get the priority value for comparison
    fn priority_value(priority: &NotificationPriority) -> u8 {
        match priority {
            NotificationPriority::Low => 1,
            NotificationPriority::Normal => 2,
            NotificationPriority::High => 3,
            NotificationPriority::Urgent => 4,
        }
    }
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self::new()
    }
}

/// Preferences for a specific notification category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryPreference {
    /// Whether this category is enabled
    pub enabled: bool,
    
    /// Minimum priority required for notifications in this category
    pub min_priority: NotificationPriority,
    
    /// Custom delivery channels for this category (overrides global settings)
    pub custom_channels: Option<Vec<ChannelType>>,
}

impl CategoryPreference {
    /// Create a new category preference
    pub fn new(enabled: bool, min_priority: NotificationPriority) -> Self {
        Self {
            enabled,
            min_priority,
            custom_channels: None,
        }
    }
}

/// Preferences for a specific delivery channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPreference {
    /// Whether this channel is enabled
    pub enabled: bool,
    
    /// Whether this channel requires confirmation
    pub requires_confirmation: bool,
}

impl ChannelPreference {
    /// Create a new channel preference
    pub fn new(enabled: bool, requires_confirmation: bool) -> Self {
        Self {
            enabled,
            requires_confirmation,
        }
    }
}
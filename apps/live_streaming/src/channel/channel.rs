//! Channel model and related structures

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Represents a streaming channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    /// Unique identifier for the channel
    pub id: Uuid,
    
    /// User ID of the channel owner
    pub owner_id: Uuid,
    
    /// Channel name/display name
    pub name: String,
    
    /// Channel description
    pub description: Option<String>,
    
    /// Channel profile image URL
    pub profile_image_url: Option<String>,
    
    /// Channel banner image URL
    pub banner_image_url: Option<String>,
    
    /// When the channel was created
    pub created_at: DateTime<Utc>,
    
    /// When the channel was last updated
    pub updated_at: DateTime<Utc>,
    
    /// Channel settings
    pub settings: ChannelSettings,
    
    /// Channel statistics
    pub stats: ChannelStats,
    
    /// Custom emotes for this channel
    pub emotes: Vec<CustomEmote>,
}

/// Channel settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSettings {
    /// Whether the channel is live notifications are enabled
    pub notifications_enabled: bool,
    
    /// Whether the channel is in mature mode
    pub mature_content: bool,
    
    /// Language of the channel
    pub language: String,
    
    /// Channel categories/tags
    pub categories: Vec<String>,
    
    /// Whether chat is enabled
    pub chat_enabled: bool,
    
    /// Whether followers-only chat is enabled
    pub followers_only_chat: bool,
    
    /// Minimum following duration for followers-only chat (in minutes)
    pub followers_only_duration: Option<u32>,
    
    /// Whether subscriber-only chat is enabled
    pub subscribers_only_chat: bool,
    
    /// Whether slow mode is enabled
    pub slow_mode: bool,
    
    /// Slow mode delay in seconds
    pub slow_mode_delay: Option<u32>,
}

impl Default for ChannelSettings {
    fn default() -> Self {
        Self {
            notifications_enabled: true,
            mature_content: false,
            language: "en".to_string(),
            categories: Vec::new(),
            chat_enabled: true,
            followers_only_chat: false,
            followers_only_duration: None,
            subscribers_only_chat: false,
            slow_mode: false,
            slow_mode_delay: None,
        }
    }
}

/// Channel statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelStats {
    /// Total number of followers
    pub follower_count: u64,
    
    /// Total number of views
    pub view_count: u64,
    
    /// Current number of viewers (if live)
    pub current_viewers: Option<u32>,
    
    /// Total number of streams
    pub stream_count: u64,
}

impl Default for ChannelStats {
    fn default() -> Self {
        Self {
            follower_count: 0,
            view_count: 0,
            current_viewers: None,
            stream_count: 0,
        }
    }
}

/// Custom emote for a channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEmote {
    /// Emote ID
    pub id: Uuid,
    
    /// Emote name
    pub name: String,
    
    /// Emote image URL
    pub image_url: String,
    
    /// Whether the emote is subscriber-only
    pub subscriber_only: bool,
    
    /// Tier required to use the emote (if subscriber-only)
    pub tier_required: Option<u8>,
}

impl Channel {
    /// Create a new channel
    pub fn new(owner_id: Uuid, name: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            owner_id,
            name,
            description,
            profile_image_url: None,
            banner_image_url: None,
            created_at: now,
            updated_at: now,
            settings: ChannelSettings::default(),
            stats: ChannelStats::default(),
            emotes: Vec::new(),
        }
    }
    
    /// Update channel information
    pub fn update_info(&mut self, name: Option<String>, description: Option<String>) {
        if let Some(name) = name {
            self.name = name;
        }
        
        if let Some(description) = description {
            self.description = Some(description);
        }
        
        self.updated_at = Utc::now();
    }
    
    /// Update channel settings
    pub fn update_settings(&mut self, settings: ChannelSettings) {
        self.settings = settings;
        self.updated_at = Utc::now();
    }
    
    /// Add a custom emote to the channel
    pub fn add_emote(&mut self, emote: CustomEmote) {
        self.emotes.push(emote);
        self.updated_at = Utc::now();
    }
    
    /// Remove a custom emote from the channel
    pub fn remove_emote(&mut self, emote_id: Uuid) -> Option<CustomEmote> {
        if let Some(pos) = self.emotes.iter().position(|e| e.id == emote_id) {
            let emote = self.emotes.remove(pos);
            self.updated_at = Utc::now();
            Some(emote)
        } else {
            None
        }
    }
    
    /// Update channel statistics
    pub fn update_stats(&mut self, stats: ChannelStats) {
        self.stats = stats;
        self.updated_at = Utc::now();
    }
}
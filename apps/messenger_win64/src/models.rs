//! Additional models for the Messenger application

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Extended conversation settings for groups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSettings {
    /// Name of the group
    pub name: Option<String>,
    
    /// Description of the group
    pub description: Option<String>,
    
    /// Whether new members require approval to join
    pub require_approval: bool,
    
    /// Whether message history is visible to new members
    pub message_history_visibility: MessageHistoryVisibility,
}

/// Visibility settings for message history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageHistoryVisibility {
    /// All messages are visible
    All,
    
    /// Only messages after joining are visible
    MemberJoinDate,
    
    /// No message history is visible
    None,
}

/// Reference to a thumbnail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThumbnailReference {
    /// Storage location of the thumbnail
    pub storage_location: String,
    
    /// Width of the thumbnail
    pub width: u32,
    
    /// Height of the thumbnail
    pub height: u32,
}
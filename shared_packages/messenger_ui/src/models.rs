//! UI models for the CPC Messenger application

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// UI representation of a message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIMessage {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub sent_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub reactions: Vec<UIReaction>,
    pub is_editing: bool,
    pub is_deleted: bool,
}

/// UI representation of a reaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIReaction {
    pub id: Uuid,
    pub message_id: Uuid,
    pub user_id: Uuid,
    pub reaction_type: String,
    pub created_at: DateTime<Utc>,
    pub user_display_name: String,
    pub user_avatar_url: Option<String>,
}

/// UI state for message input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageInputState {
    pub content: String,
    pub is_sending: bool,
    pub error: Option<String>,
}

/// UI state for reaction picker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionPickerState {
    pub is_open: bool,
    pub selected_message_id: Option<Uuid>,
    pub available_reactions: Vec<String>,
}
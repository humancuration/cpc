//! Core business models for the Messenger application

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// A conversation between users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    /// Unique identifier for the conversation
    pub id: Uuid,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Whether this is a group conversation
    pub is_group: bool,
    
    /// Participants in the conversation
    pub participants: Vec<Participant>,
    
    /// Conversation settings
    pub settings: ConversationSettings,
    
    /// For group conversations, the name of the group
    pub group_name: Option<String>,
}

impl Conversation {
    /// Create a new 1:1 conversation
    pub fn new_1to1(participants: Vec<Participant>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            created_at: now,
            is_group: false,
            participants,
            settings: ConversationSettings::default(),
            group_name: None,
        }
    }
    
    /// Create a new group conversation
    pub fn new_group(participants: Vec<Participant>, group_name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            created_at: now,
            is_group: true,
            participants,
            settings: ConversationSettings::default(),
            group_name: Some(group_name),
        }
    }
    
    /// Add a participant to the conversation
    pub fn add_participant(&mut self, participant: Participant) {
        self.participants.push(participant);
        self.updated_at();
    }
    
    /// Remove a participant from the conversation
    pub fn remove_participant(&mut self, user_id: Uuid) {
        self.participants.retain(|p| p.user_id != user_id);
        self.updated_at();
    }
    
    /// Update the last activity timestamp
    fn updated_at(&mut self) {
        // In a real implementation, we would update a last_activity field
        // For now, we'll just update the updated_at field on settings
        self.settings.updated_at = Utc::now();
    }
}

/// Settings for a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSettings {
    /// Whether notifications are enabled
    pub notifications_enabled: bool,
    
    /// Whether the conversation is muted
    pub is_muted: bool,
    
    /// Custom notification sound
    pub notification_sound: Option<String>,
    
    /// Whether message previews are enabled
    pub message_previews: bool,
    
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

impl Default for ConversationSettings {
    fn default() -> Self {
        Self {
            notifications_enabled: true,
            is_muted: false,
            notification_sound: None,
            message_previews: true,
            updated_at: Utc::now(),
        }
    }
}

/// A participant in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    /// User identifier
    pub user_id: Uuid,
    
    /// When the participant joined the conversation
    pub joined_at: DateTime<Utc>,
    
    /// ID of the last message read by this participant
    pub last_read_message_id: Option<Uuid>,
    
    /// Participant permissions
    pub permissions: ParticipantPermissions,
}

impl Participant {
    /// Create a new participant
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            joined_at: Utc::now(),
            last_read_message_id: None,
            permissions: ParticipantPermissions::default(),
        }
    }
    
    /// Mark a message as read by this participant
    pub fn mark_message_read(&mut self, message_id: Uuid) {
        self.last_read_message_id = Some(message_id);
    }
}

/// Permissions for a participant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantPermissions {
    /// Whether the participant can send messages
    pub can_send_messages: bool,
    
    /// Whether the participant can add/remove other participants
    pub can_manage_participants: bool,
    
    /// Whether the participant can change conversation settings
    pub can_change_settings: bool,
    
    /// Whether the participant can delete messages
    pub can_delete_messages: bool,
}

impl Default for ParticipantPermissions {
    fn default() -> Self {
        Self {
            can_send_messages: true,
            can_manage_participants: false,
            can_change_settings: false,
            can_delete_messages: false,
        }
    }
}

/// A message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique identifier for the message
    pub id: Uuid,
    
    /// Conversation this message belongs to
    pub conversation_id: Uuid,
    
    /// User who sent the message
    pub sender_id: Uuid,
    
    /// Content of the message
    pub content: MessageContent,
    
    /// When the message was sent
    pub sent_at: DateTime<Utc>,
    
    /// Delivery status of the message
    pub delivery_status: DeliveryStatus,
}

impl Message {
    /// Create a new text message
    pub fn new_text(conversation_id: Uuid, sender_id: Uuid, text: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            conversation_id,
            sender_id,
            content: MessageContent::Text(text),
            sent_at: Utc::now(),
            delivery_status: DeliveryStatus::Pending,
        }
    }
    
    /// Create a new media message
    pub fn new_media(conversation_id: Uuid, sender_id: Uuid, media: MediaReference) -> Self {
        Self {
            id: Uuid::new_v4(),
            conversation_id,
            sender_id,
            content: MessageContent::Media(media),
            sent_at: Utc::now(),
            delivery_status: DeliveryStatus::Pending,
        }
    }
    
    /// Mark the message as sent
    pub fn mark_sent(&mut self) {
        self.delivery_status = DeliveryStatus::Sent(Utc::now());
    }
    
    /// Mark the message as delivered
    pub fn mark_delivered(&mut self) {
        self.delivery_status = DeliveryStatus::Delivered(Utc::now());
    }
    
    /// Mark the message as read
    pub fn mark_read(&mut self) {
        self.delivery_status = DeliveryStatus::Read(Utc::now());
    }
}

/// Content of a message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageContent {
    /// Text message
    Text(String),
    
    /// Media message (image, document, etc.)
    Media(MediaReference),
    
    /// System message (e.g., user joined/left)
    System(SystemMessage),
}

/// Reference to media content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaReference {
    /// Unique identifier for the media
    pub id: Uuid,
    
    /// Type of media
    pub media_type: MediaType,
    
    /// Storage location of the media
    pub storage_location: String,
    
    /// Optional thumbnail reference
    pub thumbnail: Option<ThumbnailReference>,
    
    /// File size in bytes
    pub size_bytes: u64,
    
    /// Original filename
    pub filename: Option<String>,
}

/// Type of media
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaType {
    /// Image
    Image,
    
    /// Document
    Document,
    
    /// Audio
    Audio,
    
    /// Video
    Video,
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

/// System message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemMessage {
    /// User joined the conversation
    UserJoined { user_id: Uuid },
    
    /// User left the conversation
    UserLeft { user_id: Uuid },
    
    /// Conversation name changed
    NameChanged { new_name: String, changed_by: Uuid },
    
    /// User was added to the conversation
    UserAdded { user_id: Uuid, added_by: Uuid },
    
    /// User was removed from the conversation
    UserRemoved { user_id: Uuid, removed_by: Uuid },
}

/// Delivery status of a message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeliveryStatus {
    /// Message is pending sending
    Pending,
    
    /// Message has been sent to the server
    Sent(DateTime<Utc>),
    
    /// Message has been delivered to all recipients
    Delivered(DateTime<Utc>),
    
    /// Message has been read by at least one recipient
    Read(DateTime<Utc>),
}

/// Update to a message's delivery status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageStatusUpdate {
    /// Message identifier
    pub message_id: Uuid,
    
    /// New delivery status
    pub new_status: DeliveryStatus,
    
    /// Timestamp of the status update
}

/// A stream chat message with Twitch-style features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamMessage {
    /// The base message from the messenger system
    pub base_message: Message,
    
    /// Emotes used in the message
    pub emotes: Vec<Emote>,
    
    /// Badges the user has
    pub badges: Vec<Badge>,
    
    /// Whether this is a moderator message
    pub is_moderator: bool,
    
    /// Whether this is a subscriber message
    pub is_subscriber: bool,
}

/// Represents an emote in chat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emote {
    /// Emote ID
    pub id: Uuid,
    
    /// Emote name (e.g., "Kappa")
    pub name: String,
    
    /// Position in the message where the emote appears
    pub positions: (usize, usize),
}

/// Represents a badge in chat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badge {
    /// Badge ID
    pub id: Uuid,
    
    /// Badge name (e.g., "moderator", "subscriber")
    pub name: String,
    
    /// Badge version (for tiered badges)
    pub version: Option<String>,
}
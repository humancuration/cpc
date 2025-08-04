//! State management for the CPC Messenger UI

use std::collections::HashMap;
use uuid::Uuid;
use crate::models::{UIMessage, UIReaction, MessageInputState};

/// Application state for the messenger UI
#[derive(Debug, Clone)]
pub struct MessengerState {
    /// All messages, indexed by ID
    pub messages: HashMap<Uuid, UIMessage>,
    
    /// Current message input state
    pub message_input: MessageInputState,
    
    /// Currently selected conversation ID
    pub current_conversation_id: Option<Uuid>,
    
    /// Loading state
    pub is_loading: bool,
    
    /// Error state
    pub error: Option<String>,
}

impl MessengerState {
    /// Create a new, empty state
    pub fn new() -> Self {
        Self {
            messages: HashMap::new(),
            message_input: MessageInputState {
                content: String::new(),
                is_sending: false,
                error: None,
            },
            current_conversation_id: None,
            is_loading: false,
            error: None,
        }
    }
    
    /// Add or update a message in the state
    pub fn upsert_message(&mut self, message: UIMessage) {
        self.messages.insert(message.id, message);
    }
    
    /// Remove a message from the state
    pub fn remove_message(&mut self, message_id: Uuid) {
        self.messages.remove(&message_id);
    }
    
    /// Add a reaction to a message
    pub fn add_reaction(&mut self, message_id: Uuid, reaction: UIReaction) {
        if let Some(message) = self.messages.get_mut(&message_id) {
            message.reactions.push(reaction);
        }
    }
    
    /// Remove a reaction from a message
    pub fn remove_reaction(&mut self, message_id: Uuid, reaction_id: Uuid) {
        if let Some(message) = self.messages.get_mut(&message_id) {
            message.reactions.retain(|r| r.id != reaction_id);
        }
    }
    
    /// Update message input content
    pub fn update_message_input(&mut self, content: String) {
        self.message_input.content = content;
    }
    
    /// Set message input sending state
    pub fn set_message_input_sending(&mut self, is_sending: bool) {
        self.message_input.is_sending = is_sending;
    }
    
    /// Set message input error
    pub fn set_message_input_error(&mut self, error: Option<String>) {
        self.message_input.error = error;
    }
    
    /// Set current conversation ID
    pub fn set_current_conversation_id(&mut self, conversation_id: Option<Uuid>) {
        self.current_conversation_id = conversation_id;
    }
    
    /// Set loading state
    pub fn set_loading(&mut self, is_loading: bool) {
        self.is_loading = is_loading;
    }
    
    /// Set error state
    pub fn set_error(&mut self, error: Option<String>) {
        self.error = error;
    }
    
    /// Get messages for the current conversation
    pub fn get_current_conversation_messages(&self) -> Vec<UIMessage> {
        if let Some(conversation_id) = self.current_conversation_id {
            self.messages
                .values()
                .filter(|message| message.conversation_id == conversation_id)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Apply an optimistic update for a message
    pub fn apply_optimistic_update(&mut self, message_id: Uuid, update: MessageUpdate) {
        if let Some(message) = self.messages.get_mut(&message_id) {
            match update {
                MessageUpdate::Content(new_content) => {
                    message.content = new_content;
                    message.updated_at = Some(chrono::Utc::now());
                }
                MessageUpdate::Deleted => {
                    message.is_deleted = true;
                    message.content = String::new();
                    message.updated_at = Some(chrono::Utc::now());
                }
            }
        }
    }
}

impl Default for MessengerState {
    fn default() -> Self {
        Self::new()
    }
}

/// Types of message updates for optimistic updates
#[derive(Debug, Clone)]
pub enum MessageUpdate {
    /// Update message content
    Content(String),
    
    /// Mark message as deleted
    Deleted,
}

/// State manager for handling state updates
pub struct StateManager {
    state: MessengerState,
}

impl StateManager {
    /// Create a new state manager
    pub fn new() -> Self {
        Self {
            state: MessengerState::new(),
        }
    }
    
    /// Get a reference to the current state
    pub fn get_state(&self) -> &MessengerState {
        &self.state
    }
    
    /// Get a mutable reference to the current state
    pub fn get_state_mut(&mut self) -> &mut MessengerState {
        &mut self.state
    }
    
    /// Update the state with a function
    pub fn update_state<F>(&mut self, updater: F) 
    where 
        F: FnOnce(&mut MessengerState)
    {
        updater(&mut self.state);
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}
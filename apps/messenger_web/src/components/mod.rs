//! UI components for the Messenger web application

// Re-export the UI components from the messenger_win64 crate
// In a real implementation, these would be in a shared crate
pub use cpc_messenger::ui::reactions::{ReactionPicker, ReactionList};
pub use cpc_messenger::ui::threads::{ThreadView, ThreadCreateButton};
pub use cpc_messenger::ui::groups::{GroupSettingsModal, ParticipantManagement};
pub use cpc_messenger::ui::media::{MediaUpload, MediaPreview};
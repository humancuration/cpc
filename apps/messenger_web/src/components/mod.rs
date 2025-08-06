//! UI components for the Messenger web application
//!
//! This module combines components from the messenger backend and the shared web_core package

// Re-export the UI components from the messenger_win64 crate
pub use cpc_messenger::ui::reactions::{ReactionPicker, ReactionList};
pub use cpc_messenger::ui::threads::{ThreadView, ThreadCreateButton};
pub use cpc_messenger::ui::groups::{GroupSettingsModal, ParticipantManagement};
pub use cpc_messenger::ui::media::{MediaUpload, MediaPreview};

// Re-export components from the shared web_core package
pub use web_core::components::{Button, Modal, ButtonVariant};
//! Presentation layer for the document editor

pub mod editor;
pub mod toolbar;
pub mod preview;
pub mod visibility_settings;
pub mod conflict_resolution;
pub mod conflict_resolution_test;
pub mod presence_indicators;
pub mod presence_indicators_test;
pub mod editor_with_presence;
pub mod editor_with_presence_test;
pub mod position_translator;
pub mod presence_state;
pub mod presence_batcher;
pub mod viewport_sync;
#[cfg(test)]
pub mod position_translator_test;
#[cfg(test)]
pub mod coordinate_translation_tests;
#[cfg(test)]
pub mod viewport_sync_test;
#[cfg(test)]
pub mod integration_tests;

// Re-export components for convenience
pub use editor::DocumentEditor;
pub use toolbar::Toolbar;
pub use preview::DocumentPreview;
pub use visibility_settings::VisibilitySettings;
pub use conflict_resolution::ConflictResolutionDialog;
pub use presence_indicators::{PresenceSidebar, CursorOverlay, StatusIndicator, AvatarBadge};
pub use editor_with_presence::DocumentEditorWithPresence;
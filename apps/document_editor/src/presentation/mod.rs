//! Presentation layer for the document editor

pub mod editor;
pub mod toolbar;
pub mod preview;
pub mod visibility_settings;
pub mod conflict_resolution;
pub mod conflict_resolution_test;
pub mod presence_indicators;
pub mod presence_indicators_test;

// Re-export components for convenience
pub use editor::DocumentEditor;
pub use toolbar::Toolbar;
pub use preview::DocumentPreview;
pub use visibility_settings::VisibilitySettings;
pub use conflict_resolution::ConflictResolutionDialog;
pub use presence_indicators::{PresenceSidebar, CursorOverlay, StatusIndicator, AvatarBadge};
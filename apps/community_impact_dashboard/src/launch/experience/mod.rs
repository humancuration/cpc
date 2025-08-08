//! Community launch experience module for the Unified Community Impact Dashboard
//!
//! This module provides community-specific welcome experiences, values-centered
//! launch announcements, community storytelling integration, launch celebration
//! features, and community ownership transfer mechanisms.

/// Community-specific welcome experiences
pub mod welcome;

/// Values-centered launch announcements
pub mod announcements;

/// Community storytelling integration
pub mod storytelling;

/// Launch celebration features
pub mod celebration;

/// Community ownership transfer mechanisms
pub mod ownership;

// Re-export key components
pub use welcome::WelcomeExperience;
pub use announcements::LaunchAnnouncement;
pub use storytelling::StoryIntegration;
pub use celebration::LaunchCelebration;
pub use ownership::OwnershipTransfer;
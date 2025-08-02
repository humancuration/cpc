//! Domain layer for social integration

pub mod post;
pub mod social_event;
pub mod feed_preferences;

pub use post::{UnifiedPost, AppSource, PostMetadata};
pub use social_event::SocialEvent;
pub use feed_preferences::{FeedPreferences, FeedPreferencesRepository, FeedAlgorithmType};
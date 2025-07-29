//! Event system for the consent manager.

pub mod pubsub;
pub mod listener;
#[cfg(feature = "bevy-integration")]
pub mod bevy;
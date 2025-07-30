//! Music Player Module
//!
//! This module provides a full-featured music streaming platform with social features
//! including timestamped comments, visualizers, and offline playback.
//! It follows a hexagonal architecture pattern with clear separation of concerns.

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod web;
pub mod module_registry;

// Re-export key components for easier access
pub use domain::models::{Track, TimestampedComment, TrackInteraction, VisualizerPreset, Playlist, PlaylistTrack};
pub use application::streaming_service::StreamingService;
pub use application::social_service::SocialService;
pub use application::visualizer_service::VisualizerService;
pub use application::cache_service::CacheService;
pub use web::module::MusicPlayerModule;
pub use web::modular_module::ModularMusicPlayer;
pub use module_registry::create_module;
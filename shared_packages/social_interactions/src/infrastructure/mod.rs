//! Infrastructure layer for social interactions
//!
//! This module contains the adapters for external systems.

pub mod postgres_repository;
pub mod in_memory_repository;
pub mod event_bus;

pub use postgres_repository::PostgresReactionRepository;
pub use in_memory_repository::InMemoryReactionRepository;
pub use event_bus::SocialEventBus;
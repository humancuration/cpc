//! Domain models for the event bus system
//!
//! This module contains the core business logic and entities for event management.

pub mod event;
pub mod subscription;
pub mod errors;

pub use errors::EventError;
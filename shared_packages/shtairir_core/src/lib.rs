//! Shtairir Core Infrastructure
//!
//! This module provides the core infrastructure for Shtairir integration across the CPC platform.
//! It includes adapter registry, common abstractions, type mapping, error handling, and event bus.

pub mod abstractions;
pub mod error;
pub mod event_bus;
pub mod registry;
pub mod types;

pub use abstractions::*;
pub use error::*;
pub use event_bus::*;
pub use registry::*;
pub use types::*;
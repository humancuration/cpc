//! Domain entities and business logic for the CRM module.
//!
//! This module contains the core business entities and their associated logic.
//! These entities should be pure Rust structs/enums with no external dependencies.

pub mod contact;
pub mod interaction;
pub mod pipeline;
pub mod deal;
pub mod primitives;

// Re-export key types for convenience
pub use contact::{Contact, ContactType, ConsentSettings, DataSharingLevel};
pub use interaction::{Interaction, InteractionType};
pub use pipeline::{Pipeline, PipelineStage};
pub use deal::{Deal, DealNote};
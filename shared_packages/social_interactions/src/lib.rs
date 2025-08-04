//! Social Interactions Module
//!
//! This module provides core social interaction features including:
//! - Reactions (like, heart, celebrate, etc.)
//! - Threaded comments with rich text formatting
//! - Content sharing between users
//! - Integration with notification system
//!
//! This module follows hexagonal architecture principles with clear separation
//! of domain, application, and infrastructure layers.

pub mod domain;
pub mod application;
pub mod infrastructure;

#[cfg(test)]
mod tests;

// Re-export key types
pub use domain::{Reaction, Comment, Share, ReactionType, TargetType, ContentType};
pub use application::{ReactionService, CommentService, ShareService};
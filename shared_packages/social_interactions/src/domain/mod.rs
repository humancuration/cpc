//! Domain layer for social interactions
//!
//! This module contains the core business entities and traits for social interactions.

pub mod models;
pub mod repository;
pub mod service;

pub use models::{Reaction, Comment, Share, ReactionType, TargetType, ContentType};
pub use repository::{ReactionRepository, CommentRepository, ShareRepository};
pub use service::{ReactionService, CommentService, ShareService};
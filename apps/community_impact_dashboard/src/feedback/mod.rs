//! Feedback module for the Unified Community Impact Dashboard
//!
//! This module contains components and services for collecting community feedback
//! and measuring understanding of interconnected impact.

pub mod collector;

pub use collector::{FeedbackCollector, UserFeedback, FeedbackCategory, FeedbackStats};
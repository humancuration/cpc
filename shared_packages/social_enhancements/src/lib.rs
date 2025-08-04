//! Social Enhancements Module
//!
//! This module provides enhancements to the social features, including:
//! - Automatic posting of volunteer activities and skill exchanges
//! - Achievement system for volunteer milestones and skill mastery
//! - Group challenges for community engagement

pub mod models;
pub mod services;
pub mod achievements;

#[cfg(test)]
mod achievement_service_test;

// Re-export key types
pub use models::{Achievement, AchievementType, GroupChallenge, ChallengeProgress};
pub use services::{SocialEnhancementService, SocialEnhancementServiceImpl};
pub use achievements::{AchievementService, AchievementServiceImpl};
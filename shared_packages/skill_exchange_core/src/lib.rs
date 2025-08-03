//! Skill Exchange Core Module
//!
//! This module provides functionality for the skill marketplace, including:
//! - Skill listing management
//! - Claiming system
//! - Completion workflow
//! - Integration with wallet, notification, and social systems

pub mod models;
pub mod repositories;
pub mod services;

#[cfg(test)]
mod lib_test;

// Re-export key types
pub use models::{SkillListing, SkillClaim, SkillExchangeCompletion, SkillRating};
pub use services::{SkillExchangeService, SkillExchangeServiceImpl};
pub use repositories::SkillExchangeRepository;
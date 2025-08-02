//! Skill endorsement management module

pub mod models;
pub mod service;

#[cfg(test)]
mod tests;

pub use models::SkillEndorsement;
pub use service::{EndorsementService, EndorsementServiceError};
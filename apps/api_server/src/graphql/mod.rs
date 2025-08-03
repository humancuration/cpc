//! GraphQL module for the API server

pub mod volunteer;
pub mod skill_exchange;

#[cfg(test)]
mod integration_tests;

// Re-export key types
pub use volunteer::{VolunteerMutation, VolunteerQuery};
pub use skill_exchange::{SkillExchangeMutation, SkillExchangeQuery};
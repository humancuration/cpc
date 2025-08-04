//! GraphQL module for the API server

pub mod volunteer;
pub mod skill_exchange;
pub mod social_interactions;
pub mod collaborative_workspace;

#[cfg(test)]
mod integration_tests;

#[cfg(test)]
mod volunteer_test;

#[cfg(test)]
mod skill_exchange_test;

#[cfg(test)]
mod collaborative_workspace_test;

// Re-export key types
pub use volunteer::{VolunteerMutation, VolunteerQuery};
pub use skill_exchange::{SkillExchangeMutation, SkillExchangeQuery};
pub use social_interactions::{SocialInteractionMutations, SocialInteractionQueries};
pub use collaborative_workspace::{CollaborativeWorkspaceMutations, CollaborativeWorkspaceQueries};
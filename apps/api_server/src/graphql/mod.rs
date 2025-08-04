#![allow(clippy::unused_async)]
//! GraphQL module for the API server
//!
//! Deterministic schema construction for CI:
//! See docs/adr/0009-bootstrap-stub-toggles.md — all schema-affecting stubs must be OFF.
//! The CI helper below composes the schema roots and returns an async-graphql Schema
//! identical to production defaults (no stubs), suitable for Schema::sdl() snapshotting.

pub mod volunteer;
pub mod skill_exchange;
pub mod social_interactions;
pub mod collaborative_workspace;
pub mod volunteer_coordination; // New: Volunteer Coordination GraphQL (ADR 0008)

// Minimal bootstrap module namespace for DI
pub mod bootstrap {
    pub mod volunteer;
}

#[cfg(test)]
mod integration_tests;

#[cfg(test)]
mod volunteer_test;

#[cfg(test)]
mod skill_exchange_test;

#[cfg(test)]
mod collaborative_workspace_test;

#[cfg(test)]
mod volunteer_coordination_test;

// Re-export key types
pub use volunteer::{VolunteerMutation, VolunteerQuery};
pub use skill_exchange::{SkillExchangeMutation, SkillExchangeQuery};
pub use social_interactions::{SocialInteractionMutations, SocialInteractionQueries};
pub use collaborative_workspace::{CollaborativeWorkspaceMutations, CollaborativeWorkspaceQueries};
pub use volunteer_coordination::{VolunteerCoordinationMutations, VolunteerCoordinationQueries};

//// CI helper to build a deterministic schema for snapshotting.
/// Stub toggles MUST be disabled (e.g., VOLUNTEER_REPUTATION unset).
/// See docs/adr/0009-bootstrap-stub-toggles.md
///
/// Note: This builder mirrors production roots with schema‑affecting stubs OFF; used by tools/ci check-schema. See docs/adr/0009-bootstrap-stub-toggles.md.
pub mod ci_schema {
    use async_graphql::{EmptySubscription, Schema};
    use super::{
        VolunteerQuery, VolunteerMutation,
        SkillExchangeQuery, SkillExchangeMutation,
        SocialInteractionQueries, SocialInteractionMutations,
        collaborative_workspace::{CollaborativeWorkspaceQueries, CollaborativeWorkspaceMutations},
        volunteer_coordination::{VolunteerCoordinationQueries, VolunteerCoordinationMutations},
    };

    /// Build a composed schema that includes all public roots we expose in production.
    /// Note: This returns a unified schema of multiple domains under a single root by
    /// nesting the domain-specific roots under a top-level Root objects.
    /// If your project uses a different composition, adjust here to match production.
    #[derive(Default)]
    struct RootQuery;
    #[async_graphql::Object]
    impl RootQuery {
        async fn volunteer(&self) -> VolunteerQuery { VolunteerQuery }
        async fn skill_exchange(&self) -> SkillExchangeQuery { SkillExchangeQuery }
        async fn social(&self) -> SocialInteractionQueries { SocialInteractionQueries::default() }
        async fn collaborative_workspace(&self) -> CollaborativeWorkspaceQueries { CollaborativeWorkspaceQueries::default() }
        async fn volunteer_coordination(&self) -> VolunteerCoordinationQueries { VolunteerCoordinationQueries::default() }
    }

    #[derive(Default)]
    struct RootMutation;
    #[async_graphql::Object]
    impl RootMutation {
        async fn volunteer(&self) -> VolunteerMutation { VolunteerMutation }
        async fn skill_exchange(&self) -> SkillExchangeMutation { SkillExchangeMutation }
        async fn social(&self) -> SocialInteractionMutations { SocialInteractionMutations::default() }
        async fn collaborative_workspace(&self) -> CollaborativeWorkspaceMutations { CollaborativeWorkspaceMutations::default() }
        async fn volunteer_coordination(&self) -> VolunteerCoordinationMutations { VolunteerCoordinationMutations::default() }
    }

    pub type CiSchema = Schema<RootQuery, RootMutation, EmptySubscription>;

    /// Build schema for CI with all schema-affecting stubs OFF.
    /// Caller is responsible for ensuring env toggles are cleared.
    pub fn build_schema_for_ci() -> CiSchema {
        Schema::build(RootQuery::default(), RootMutation::default(), EmptySubscription).finish()
    }
}

// Optional helper to build schema with volunteer service composed (useful for tests)
#[cfg(test)]
pub mod test_helpers {
    use async_graphql::{EmptySubscription, Schema};
    use std::sync::Arc;
    use crate::graphql::volunteer_coordination::{VolunteerCoordinationMutations, VolunteerCoordinationQueries};
    use shared_packages::volunteer_coordination::application::volunteer_service::VolunteerServiceImpl;
    use shared_packages::volunteer_coordination::domain::repository::{ApplicationRepository, ContributionRepository, OpportunityRepository};
    use crate::graphql::bootstrap::volunteer::build_volunteer_service;

    pub type VcSchema = Schema<VolunteerCoordinationQueries, VolunteerCoordinationMutations, EmptySubscription>;

    pub fn build_vc_schema_with_service(
        user_id: uuid::Uuid,
        opp_repo: Arc<dyn OpportunityRepository>,
        app_repo: Arc<dyn ApplicationRepository>,
        contrib_repo: Arc<dyn ContributionRepository>,
    ) -> VcSchema {
        let svc: Arc<VolunteerServiceImpl> = build_volunteer_service(opp_repo, app_repo, contrib_repo);
        Schema::build(VolunteerCoordinationQueries::default(), VolunteerCoordinationMutations::default(), EmptySubscription)
            .data(user_id)
            .data::<Arc<VolunteerServiceImpl>>(svc)
            .finish()
    }
}
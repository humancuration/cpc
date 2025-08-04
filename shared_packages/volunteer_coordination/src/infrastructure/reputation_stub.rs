//! Reputation stub adapter
//! Minimal, deterministic verification rules for local/dev usage.
//!
//! TODO(ADR-0008): Replace with real scoring heuristics and external reputation integration.

use async_trait::async_trait;
use uuid::Uuid;

use crate::application::reputation_port::{ReputationError, ReputationPort};
use crate::domain::models::{ContributionId, ContributionKind};

#[derive(Debug, Default, Clone)]
pub struct ReputationStub;

impl ReputationStub {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ReputationPort for ReputationStub {
    async fn verify_contribution(
        &self,
        _contribution_id: ContributionId,
        kind: ContributionKind,
        amount_hours: Option<f32>,
        _contributor_id: Uuid,
    ) -> Result<bool, ReputationError> {
        // Simple deterministic rule:
        // - Hours: require >= 1.0 hour to be considered verified.
        // - Other kinds: verified = true.
        // TODO(ADR-0008): incorporate trust graph, prior history, organizer reviews, and cross-app signals.
        let result = match kind {
            ContributionKind::Hours => amount_hours.unwrap_or(0.0) >= 1.0,
            _ => true,
        };
        Ok(result)
    }
}
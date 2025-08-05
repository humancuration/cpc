//! Mock implementations for testing

use async_trait::async_trait;
use uuid::Uuid;
use crate::{
    domain::{
        model::Visibility,
        service::consent_service::{ConsentService, ConsentError},
    },
};

/// Mock implementation of the ConsentService for testing
pub struct MockConsentService {
    /// List of allowed (viewer, owner, visibility) combinations
    allowed: Vec<(Uuid, Uuid, Visibility)>,
}

impl MockConsentService {
    /// Create a new MockConsentService
    pub fn new(allowed: Vec<(Uuid, Uuid, Visibility)>) -> Self {
        Self { allowed }
    }
}

#[async_trait]
impl ConsentService for MockConsentService {
    async fn can_view_content(
        &self,
        viewer: Uuid,
        owner: Uuid,
        vis: Visibility,
    ) -> Result<bool, ConsentError> {
        Ok(self.allowed.contains(&(viewer, owner, vis)))
    }
}
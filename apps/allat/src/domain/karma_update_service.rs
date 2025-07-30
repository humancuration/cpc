use crate::domain::auth_service::AuthService;
use crate::domain::vote::{VoteEvent, VoteType};
use crate::domain::auth::AuthError;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait KarmaUpdateService: Send + Sync {
    async fn handle_vote_event(&self, event: VoteEvent) -> Result<(), AuthError>;
}

pub struct AllatKarmaUpdateService {
    auth_service: Arc<dyn AuthService>,
}

impl AllatKarmaUpdateService {
    pub fn new(auth_service: Arc<dyn AuthService>) -> Self {
        Self { auth_service }
    }
}

#[async_trait]
impl KarmaUpdateService for AllatKarmaUpdateService {
    async fn handle_vote_event(&self, event: VoteEvent) -> Result<(), AuthError> {
        self.auth_service
            .handle_vote_event(event)
            .await
    }
}
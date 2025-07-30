//! User preferences service for managing currency and other settings

use crate::domain::primitives::Currency;
use uuid::Uuid;
use std::sync::Arc;
use async_trait::async_trait;
use crate::application::savings_service::DataSharingRepository;
use crate::domain::FinanceError;

#[async_trait]
pub trait UserPreferences {
    async fn get_preferred_currency(&self, user_id: Uuid) -> Result<Currency, String>;
    async fn set_preferred_currency(&self, user_id: Uuid, currency: Currency) -> Result<(), String>;
}

pub struct UserPreferencesService {
    preferences_repo: Arc<dyn DataSharingRepository>,
}

impl UserPreferencesService {
    pub fn new(preferences_repo: Arc<dyn DataSharingRepository>) -> Self {
        Self { preferences_repo }
    }
}

#[async_trait]
impl UserPreferences for UserPreferencesService {
    async fn get_preferred_currency(&self, user_id: Uuid) -> Result<Currency, String> {
        self.preferences_repo.get_user_currency(user_id)
            .await
            .map_err(|e| format!("Failed to get user currency for user {}: {}", user_id, e))
    }

    async fn set_preferred_currency(&self, user_id: Uuid, currency: Currency) -> Result<(), String> {
        self.preferences_repo.update_user_currency(user_id, currency)
            .await
            .map_err(|e| format!("Failed to update user currency for user {}: {}", user_id, e))
    }
}
//! Savings service for managing savings goals and progress

use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::{
    savings_goal::{SavingsGoal, SavingsProgress},
    primitives::Money,
    FinanceError,
};

#[async_trait]
pub trait SavingsRepository {
    async fn save(&self, goal: &SavingsGoal) -> Result<(), FinanceError>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<SavingsGoal>, FinanceError>;
    async fn find_active_by_user_id(&self, user_id: Uuid) -> Result<Vec<SavingsGoal>, FinanceError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<SavingsGoal>, FinanceError>;
    async fn delete(&self, id: Uuid) -> Result<(), FinanceError>;
}

#[async_trait]
pub trait DataSharingRepository {
    async fn save(&self, preference: &DataSharingPreference) -> Result<(), FinanceError>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<DataSharingPreference>, FinanceError>;
    async fn create_default(&self, user_id: Uuid) -> Result<DataSharingPreference, FinanceError>;
}

#[async_trait]
pub trait UserConsentStore {
    async fn get_user_consent(&self, user_id: Uuid) -> Result<Option<UserConsent>, FinanceError>;
    async fn update_user_consent(&self, user_id: Uuid, consent: UserConsent) -> Result<(), FinanceError>;
}

#[async_trait]
pub trait SavingsService {
    async fn create_goal(&self, user_id: Uuid, name: String, target_amount: Money, target_date: DateTime<Utc>) -> Result<SavingsGoal, FinanceError>;
    async fn get_user_goals(&self, user_id: Uuid) -> Result<Vec<SavingsGoal>, FinanceError>;
    async fn get_active_goals(&self, user_id: Uuid) -> Result<Vec<SavingsGoal>, FinanceError>;
    async fn get_goal_progress(&self, goal_id: Uuid) -> Result<SavingsProgress, FinanceError>;
    async fn update_goal_progress(&self, goal_id: Uuid, amount: Money) -> Result<SavingsGoal, FinanceError>;
    async fn delete_goal(&self, goal_id: Uuid) -> Result<(), FinanceError>;
    async fn get_data_sharing_preference(&self, user_id: Uuid) -> Result<DataSharingPreference, FinanceError>;
    async fn update_data_sharing_preference(&self, user_id: Uuid, enabled: bool, anonymized: bool) -> Result<DataSharingPreference, FinanceError>;
}

pub struct SavingsServiceImpl {
    savings_repo: std::sync::Arc<dyn SavingsRepository>,
    data_sharing_repo: std::sync::Arc<dyn DataSharingRepository>,
}

impl SavingsServiceImpl {
    pub fn new(
        savings_repo: std::sync::Arc<dyn SavingsRepository>,
        data_sharing_repo: std::sync::Arc<dyn DataSharingRepository>,
    ) -> Self {
        Self {
            savings_repo,
            data_sharing_repo,
        }
    }
}

#[async_trait]
impl SavingsService for SavingsServiceImpl {
    async fn create_goal(&self, user_id: Uuid, name: String, target_amount: Money, target_date: DateTime<Utc>) -> Result<SavingsGoal, FinanceError> {
        let goal = SavingsGoal::new(user_id, name, target_amount, target_date);
        self.savings_repo.save(&goal).await?;
        Ok(goal)
    }

    async fn get_user_goals(&self, user_id: Uuid) -> Result<Vec<SavingsGoal>, FinanceError> {
        self.savings_repo.find_by_user_id(user_id).await
    }

    async fn get_active_goals(&self, user_id: Uuid) -> Result<Vec<SavingsGoal>, FinanceError> {
        self.savings_repo.find_active_by_user_id(user_id).await
    }

    async fn get_goal_progress(&self, goal_id: Uuid) -> Result<SavingsProgress, FinanceError> {
        let goal = self.savings_repo.find_by_id(goal_id).await?
            .ok_or(FinanceError::SavingsGoalNotFound(goal_id))?;
        Ok(SavingsProgress::from_goal(&goal))
    }

    async fn update_goal_progress(&self, goal_id: Uuid, amount: Money) -> Result<SavingsGoal, FinanceError> {
        let mut goal = self.savings_repo.find_by_id(goal_id).await?
            .ok_or(FinanceError::SavingsGoalNotFound(goal_id))?;
        
        let new_amount = goal.current_amount.add(&amount)
            .map_err(|e| FinanceError::InvalidAmount(e.to_string()))?;
        goal.current_amount = new_amount;
        goal.updated_at = Utc::now();
        
        self.savings_repo.save(&goal).await?;
        Ok(goal)
    }

    async fn delete_goal(&self, goal_id: Uuid) -> Result<(), FinanceError> {
        self.savings_repo.delete(goal_id).await
    }

    async fn get_data_sharing_preference(&self, user_id: Uuid) -> Result<DataSharingPreference, FinanceError> {
        match self.data_sharing_repo.find_by_user_id(user_id).await? {
            Some(preference) => Ok(preference),
            None => self.data_sharing_repo.create_default(user_id).await,
        }
    }

    async fn update_data_sharing_preference(&self, user_id: Uuid, enabled: bool, anonymized: bool) -> Result<DataSharingPreference, FinanceError> {
        let mut preference = self.get_data_sharing_preference(user_id).await?;
        
        if enabled {
            preference.enable_sharing();
        } else {
            preference.disable_sharing();
        }
        
        if anonymized {
            preference.enable_anonymization();
        } else {
            preference.disable_anonymization();
        }
        
        self.data_sharing_repo.save(&preference).await?;
        Ok(preference)
    }
}

/// Domain model for data sharing preferences
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct DataSharingPreference {
    pub id: Uuid,
    pub user_id: Uuid,
    pub data_sharing_enabled: bool,
    pub anonymized_data: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DataSharingPreference {
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            data_sharing_enabled: false,
            anonymized_data: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn enable_sharing(&mut self) {
        self.data_sharing_enabled = true;
        self.updated_at = Utc::now();
    }

    pub fn disable_sharing(&mut self) {
        self.data_sharing_enabled = false;
        self.updated_at = Utc::now();
    }

    pub fn enable_anonymization(&mut self) {
        self.anonymized_data = true;
        self.updated_at = Utc::now();
    }

    pub fn disable_anonymization(&mut self) {
        self.anonymized_data = false;
        self.updated_at = Utc::now();
    }
}

/// User consent for data sharing
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct UserConsent {
    pub user_id: Uuid,
    pub consent_given: bool,
    pub consent_date: DateTime<Utc>,
    pub data_types: Vec<String>,
    pub sharing_partners: Vec<String>,
}
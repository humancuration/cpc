//! Tests for the volunteer core module

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{VolunteerActivity, VolunteerVerification, DabloonConversion};
    use crate::services::{VolunteerService, VolunteerServiceImpl};
    use crate::repositories::VolunteerRepository;
    use wallet::domain::primitives::{Money, Currency};
    use notification_core::domain::types::Notification;
    use notification_core::domain::preferences::UserPreferences;
    use social_integration::domain::social_event::SocialEvent;
    use common_utils::error::CommonError;
    use uuid::Uuid;
    use rust_decimal::Decimal;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    // Mock repository for testing
    struct MockVolunteerRepository {
        activities: Arc<Mutex<Vec<VolunteerActivity>>>,
        verifications: Arc<Mutex<Vec<VolunteerVerification>>>,
        conversions: Arc<Mutex<Vec<DabloonConversion>>>,
    }

    impl MockVolunteerRepository {
        fn new() -> Self {
            Self {
                activities: Arc::new(Mutex::new(Vec::new())),
                verifications: Arc::new(Mutex::new(Vec::new())),
                conversions: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    #[async_trait::async_trait]
    impl VolunteerRepository for MockVolunteerRepository {
        async fn save_activity(&self, activity: &VolunteerActivity) -> Result<(), CommonError> {
            let mut activities = self.activities.lock().await;
            activities.push(activity.clone());
            Ok(())
        }

        async fn find_activity_by_id(&self, id: Uuid) -> Result<Option<VolunteerActivity>, CommonError> {
            let activities = self.activities.lock().await;
            Ok(activities.iter().find(|a| a.id == id).cloned())
        }

        async fn find_activities_by_user_id(&self, user_id: Uuid) -> Result<Vec<VolunteerActivity>, CommonError> {
            let activities = self.activities.lock().await;
            Ok(activities.iter().filter(|a| a.user_id == user_id).cloned().collect())
        }

        async fn find_unverified_activities_by_organization(&self, _organization_id: Uuid) -> Result<Vec<VolunteerActivity>, CommonError> {
            let activities = self.activities.lock().await;
            Ok(activities.iter().filter(|a| !a.verified).cloned().collect())
        }

        async fn save_verification(&self, verification: &VolunteerVerification) -> Result<(), CommonError> {
            let mut verifications = self.verifications.lock().await;
            verifications.push(verification.clone());
            Ok(())
        }

        async fn find_verification_by_id(&self, id: Uuid) -> Result<Option<VolunteerVerification>, CommonError> {
            let verifications = self.verifications.lock().await;
            Ok(verifications.iter().find(|v| v.id == id).cloned())
        }

        async fn find_verifications_by_activity_id(&self, _activity_id: Uuid) -> Result<Vec<VolunteerVerification>, CommonError> {
            Ok(vec![])
        }

        async fn save_conversion(&self, conversion: &DabloonConversion) -> Result<(), CommonError> {
            let mut conversions = self.conversions.lock().await;
            conversions.push(conversion.clone());
            Ok(())
        }

        async fn find_conversion_by_id(&self, id: Uuid) -> Result<Option<DabloonConversion>, CommonError> {
            let conversions = self.conversions.lock().await;
            Ok(conversions.iter().find(|c| c.id == id).cloned())
        }

        async fn find_conversions_by_user_id(&self, _user_id: Uuid) -> Result<Vec<DabloonConversion>, CommonError> {
            Ok(vec![])
        }
    }

    // Mock wallet service for testing
    struct MockWalletService;

    #[async_trait::async_trait]
    impl wallet::application::WalletService for MockWalletService {
        async fn get_or_create_wallet(&self, _user_id: Uuid) -> Result<wallet::Wallet, wallet::domain::primitives::FinancialError> {
            Ok(wallet::Wallet::new(Uuid::new_v4()))
        }

        async fn add_dabloons(&self, _user_id: Uuid, amount: Money, _description: Option<String>) -> Result<wallet::Wallet, wallet::domain::primitives::FinancialError> {
            let mut wallet = wallet::Wallet::new(Uuid::new_v4());
            wallet.add_dabloons(amount)?;
            Ok(wallet)
        }

        async fn subtract_dabloons(&self, _user_id: Uuid, amount: Money, _description: Option<String>) -> Result<wallet::Wallet, wallet::domain::primitives::FinancialError> {
            let mut wallet = wallet::Wallet::new(Uuid::new_v4());
            wallet.subtract_dabloons(amount)?;
            Ok(wallet)
        }

        async fn transfer_dabloons(&self, _from_user_id: Uuid, _to_user_id: Uuid, amount: Money, _description: Option<String>) -> Result<(wallet::Wallet, wallet::Wallet), wallet::domain::primitives::FinancialError> {
            let mut from_wallet = wallet::Wallet::new(Uuid::new_v4());
            let mut to_wallet = wallet::Wallet::new(Uuid::new_v4());
            from_wallet.subtract_dabloons(amount.clone())?;
            to_wallet.add_dabloons(amount)?;
            Ok((from_wallet, to_wallet))
        }

        async fn send_tip(&self, _from_user_id: Uuid, _to_user_id: Uuid, _amount: Money, _note: Option<String>) -> Result<(), wallet::domain::primitives::FinancialError> {
            Ok(())
        }

        async fn get_transaction_history(&self, _user_id: Uuid) -> Result<Vec<wallet::WalletTransaction>, wallet::domain::primitives::FinancialError> {
            Ok(vec![])
        }

        async fn distribute_universal_income(&self, _user_id: Uuid, amount: Money, _distribution_date: chrono::NaiveDate) -> Result<wallet::Wallet, wallet::domain::primitives::FinancialError> {
            let mut wallet = wallet::Wallet::new(Uuid::new_v4());
            wallet.add_dabloons(amount)?;
            Ok(wallet)
        }

        fn subscribe_tip_events(&self) -> tokio::sync::broadcast::Receiver<wallet::domain::wallet::TipSentEvent> {
            let (sender, _) = tokio::sync::broadcast::channel(1);
            sender.subscribe()
        }
    }

    // Mock notification service for testing
    struct MockNotificationService;

    #[async_trait::async_trait]
    impl notification_core::application::service::NotificationService for MockNotificationService {
        async fn send_notification(&self, _notification: Notification) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            Ok(())
        }

        async fn send_notification_to_user(&self, _user_id: &str, _title: &str, _body: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            Ok(())
        }

        async fn get_user_preferences(&self, _user_id: &str) -> Result<UserPreferences, Box<dyn std::error::Error + Send + Sync>> {
            Ok(UserPreferences::default())
        }

        async fn update_user_preferences(&self, _user_id: &str, _preferences: UserPreferences) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            Ok(())
        }
    }

    // Mock social service for testing
    struct MockSocialService;

    #[async_trait::async_trait]
    impl social_integration::application::social_integration_service::SocialIntegrationService for MockSocialService {
        async fn handle_social_event(&self, _event: SocialEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_log_volunteer_hours() {
        // Arrange
        let repo = Arc::new(MockVolunteerRepository::new());
        let wallet_service = Arc::new(MockWalletService);
        let notification_service = Arc::new(MockNotificationService);
        let social_service = Arc::new(MockSocialService);
        let service = VolunteerServiceImpl::new(repo.clone(), wallet_service, notification_service, social_service);

        let user_id = Uuid::new_v4();
        let description = "Helped at the food bank".to_string();
        let hours = Decimal::from(5);

        // Act
        let result = service.log_volunteer_hours(user_id, None, description.clone(), hours).await;

        // Assert
        assert!(result.is_ok());
        let activity = result.unwrap();
        assert_eq!(activity.user_id, user_id);
        assert_eq!(activity.description, description);
        assert_eq!(activity.hours, hours);
        assert!(!activity.verified);
    }

    #[tokio::test]
    async fn test_verify_volunteer_hours() {
        // Arrange
        let repo = Arc::new(MockVolunteerRepository::new());
        let wallet_service = Arc::new(MockWalletService);
        let notification_service = Arc::new(MockNotificationService);
        let social_service = Arc::new(MockSocialService);
        let service = VolunteerServiceImpl::new(repo.clone(), wallet_service, notification_service, social_service);

        let user_id = Uuid::new_v4();
        let verifier_id = Uuid::new_v4();
        let description = "Helped at the food bank".to_string();
        let hours = Decimal::from(5);

        // First log the hours
        let activity = service.log_volunteer_hours(user_id, None, description.clone(), hours).await.unwrap();

        // Act
        let result = service.verify_volunteer_hours(activity.id, verifier_id, true, Some("Great work!".to_string())).await;

        // Assert
        assert!(result.is_ok());
        let verified_activity = result.unwrap();
        assert!(verified_activity.verified);
        assert_eq!(verified_activity.verified_by, Some(verifier_id));
    }

    #[tokio::test]
    async fn test_convert_to_dabloons() {
        // Arrange
        let repo = Arc::new(MockVolunteerRepository::new());
        let wallet_service = Arc::new(MockWalletService);
        let notification_service = Arc::new(MockNotificationService);
        let social_service = Arc::new(MockSocialService);
        let service = VolunteerServiceImpl::new(repo.clone(), wallet_service, notification_service, social_service);

        let user_id = Uuid::new_v4();
        let description = "Helped at the food bank".to_string();
        let hours = Decimal::from(5);

        // First log and verify the hours
        let mut activity = service.log_volunteer_hours(user_id, None, description.clone(), hours).await.unwrap();
        activity.verify(Uuid::new_v4());
        repo.save_activity(&activity).await.unwrap();

        // Act
        let result = service.convert_to_dabloons(activity.id, user_id).await;

        // Assert
        assert!(result.is_ok());
        let conversion = result.unwrap();
        assert_eq!(conversion.user_id, user_id);
        assert_eq!(conversion.hours_converted, hours);
        assert_eq!(conversion.dabloons_credited.currency, Currency::Dabloons);
    }
}
//! Tests for the skill exchange core module

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{SkillListing, SkillClaim, SkillExchangeCompletion, SkillRating};
    use crate::services::{SkillExchangeService, SkillExchangeServiceImpl};
    use crate::repositories::SkillExchangeRepository;
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
    struct MockSkillExchangeRepository {
        listings: Arc<Mutex<Vec<SkillListing>>>,
        claims: Arc<Mutex<Vec<SkillClaim>>>,
        completions: Arc<Mutex<Vec<SkillExchangeCompletion>>>,
    }

    impl MockSkillExchangeRepository {
        fn new() -> Self {
            Self {
                listings: Arc::new(Mutex::new(Vec::new())),
                claims: Arc::new(Mutex::new(Vec::new())),
                completions: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    #[async_trait::async_trait]
    impl SkillExchangeRepository for MockSkillExchangeRepository {
        async fn save_listing(&self, listing: &SkillListing) -> Result<(), CommonError> {
            let mut listings = self.listings.lock().await;
            listings.push(listing.clone());
            Ok(())
        }

        async fn find_listing_by_id(&self, id: Uuid) -> Result<Option<SkillListing>, CommonError> {
            let listings = self.listings.lock().await;
            Ok(listings.iter().find(|l| l.id == id).cloned())
        }

        async fn find_active_listings(&self) -> Result<Vec<SkillListing>, CommonError> {
            let listings = self.listings.lock().await;
            Ok(listings.iter().filter(|l| l.is_active).cloned().collect())
        }

        async fn find_listings_by_provider(&self, provider_id: Uuid) -> Result<Vec<SkillListing>, CommonError> {
            let listings = self.listings.lock().await;
            Ok(listings.iter().filter(|l| l.provider_id == provider_id).cloned().collect())
        }

        async fn find_listings_by_category(&self, category: &str) -> Result<Vec<SkillListing>, CommonError> {
            let listings = self.listings.lock().await;
            Ok(listings.iter().filter(|l| l.category == category).cloned().collect())
        }

        async fn search_listings(&self, term: &str) -> Result<Vec<SkillListing>, CommonError> {
            let listings = self.listings.lock().await;
            Ok(listings.iter().filter(|l| l.title.contains(term) || l.description.contains(term)).cloned().collect())
        }

        async fn save_claim(&self, claim: &SkillClaim) -> Result<(), CommonError> {
            let mut claims = self.claims.lock().await;
            claims.push(claim.clone());
            Ok(())
        }

        async fn find_claim_by_id(&self, id: Uuid) -> Result<Option<SkillClaim>, CommonError> {
            let claims = self.claims.lock().await;
            Ok(claims.iter().find(|c| c.id == id).cloned())
        }

        async fn find_claims_by_listing_id(&self, listing_id: Uuid) -> Result<Vec<SkillClaim>, CommonError> {
            let claims = self.claims.lock().await;
            Ok(claims.iter().filter(|c| c.listing_id == listing_id).cloned().collect())
        }

        async fn find_claims_by_claimant_id(&self, claimant_id: Uuid) -> Result<Vec<SkillClaim>, CommonError> {
            let claims = self.claims.lock().await;
            Ok(claims.iter().filter(|c| c.claimant_id == claimant_id).cloned().collect())
        }

        async fn save_completion(&self, completion: &SkillExchangeCompletion) -> Result<(), CommonError> {
            let mut completions = self.completions.lock().await;
            completions.push(completion.clone());
            Ok(())
        }

        async fn find_completion_by_id(&self, id: Uuid) -> Result<Option<SkillExchangeCompletion>, CommonError> {
            let completions = self.completions.lock().await;
            Ok(completions.iter().find(|c| c.id == id).cloned())
        }

        async fn find_completions_by_provider(&self, _provider_id: Uuid) -> Result<Vec<SkillExchangeCompletion>, CommonError> {
            Ok(vec![])
        }

        async fn find_completions_by_claimant(&self, _claimant_id: Uuid) -> Result<Vec<SkillExchangeCompletion>, CommonError> {
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
    async fn test_create_skill_listing() {
        // Arrange
        let repo = Arc::new(MockSkillExchangeRepository::new());
        let wallet_service = Arc::new(MockWalletService);
        let notification_service = Arc::new(MockNotificationService);
        let social_service = Arc::new(MockSocialService);
        let service = SkillExchangeServiceImpl::new(repo.clone(), wallet_service, notification_service, social_service);

        let provider_id = Uuid::new_v4();
        let title = "Web Development".to_string();
        let description = "I can help you build a website".to_string();
        let category = "Technology".to_string();

        // Act
        let result = service.create_listing(provider_id, title.clone(), description.clone(), category.clone(), None).await;

        // Assert
        assert!(result.is_ok());
        let listing = result.unwrap();
        assert_eq!(listing.provider_id, provider_id);
        assert_eq!(listing.title, title);
        assert_eq!(listing.description, description);
        assert_eq!(listing.category, category);
        assert!(listing.is_active);
    }

    #[tokio::test]
    async fn test_claim_skill_listing() {
        // Arrange
        let repo = Arc::new(MockSkillExchangeRepository::new());
        let wallet_service = Arc::new(MockWalletService);
        let notification_service = Arc::new(MockNotificationService);
        let social_service = Arc::new(MockSocialService);
        let service = SkillExchangeServiceImpl::new(repo.clone(), wallet_service, notification_service, social_service);

        let provider_id = Uuid::new_v4();
        let claimant_id = Uuid::new_v4();
        let title = "Web Development".to_string();
        let description = "I can help you build a website".to_string();
        let category = "Technology".to_string();

        // First create a listing
        let listing = service.create_listing(provider_id, title.clone(), description.clone(), category.clone(), None).await.unwrap();

        // Act
        let result = service.claim_listing(listing.id, claimant_id, Some("I need help with my website".to_string())).await;

        // Assert
        assert!(result.is_ok());
        let claim = result.unwrap();
        assert_eq!(claim.listing_id, listing.id);
        assert_eq!(claim.claimant_id, claimant_id);
        assert_eq!(claim.status, crate::models::ClaimStatus::Pending);
    }

    #[tokio::test]
    async fn test_complete_skill_exchange() {
        // Arrange
        let repo = Arc::new(MockSkillExchangeRepository::new());
        let wallet_service = Arc::new(MockWalletService);
        let notification_service = Arc::new(MockNotificationService);
        let social_service = Arc::new(MockSocialService);
        let service = SkillExchangeServiceImpl::new(repo.clone(), wallet_service, notification_service, social_service);

        let provider_id = Uuid::new_v4();
        let claimant_id = Uuid::new_v4();
        let title = "Web Development".to_string();
        let description = "I can help you build a website".to_string();
        let category = "Technology".to_string();

        // First create a listing
        let listing = service.create_listing(provider_id, title.clone(), description.clone(), category.clone(), None).await.unwrap();

        // Then claim it
        let mut claim = service.claim_listing(listing.id, claimant_id, Some("I need help with my website".to_string())).await.unwrap();

        // Accept the claim
        claim.accept();
        repo.save_claim(&claim).await.unwrap();

        // Act
        let result = service.complete_exchange(claim.id, claimant_id, Some(5), Some("Great service!".to_string()), None).await;

        // Assert
        assert!(result.is_ok());
        let completion = result.unwrap();
        assert_eq!(completion.listing_id, listing.id);
        assert_eq!(completion.claim_id, claim.id);
        assert_eq!(completion.provider_id, provider_id);
        assert_eq!(completion.claimant_id, claimant_id);
        assert!(completion.rating.is_some());
    }
}
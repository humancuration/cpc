//! Service layer tests for volunteer functionality

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::VolunteerServiceImpl;
    use crate::models::{VolunteerActivity, DabloonConversion};
    use uuid::Uuid;
    use rust_decimal::Decimal;
    use wallet::domain::primitives::{Money, Currency};
    use wallet::domain::wallet::FinancialError;
    use common_utils::error::CommonError;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    
    // Mock repository implementation
    struct MockVolunteerRepository {
        should_fail: bool,
    }
    
    #[async_trait::async_trait]
    impl crate::repositories::VolunteerRepository for MockVolunteerRepository {
        async fn save_activity(&self, _activity: &VolunteerActivity) -> Result<(), CommonError> {
            if self.should_fail {
                Err(CommonError::ServiceError("Database error".to_string()))
            } else {
                Ok(())
            }
        }
        
        async fn find_activity_by_id(&self, _id: Uuid) -> Result<Option<VolunteerActivity>, CommonError> {
            if self.should_fail {
                Err(CommonError::ServiceError("Database error".to_string()))
            } else {
                // Return a mock activity for testing
                let activity = VolunteerActivity::new(Uuid::new_v4(), None, "Test activity".to_string(), Decimal::from(2));
                Ok(Some(activity))
            }
        }
        
        async fn find_activities_by_user_id(&self, _user_id: Uuid) -> Result<Vec<VolunteerActivity>, CommonError> {
            if self.should_fail {
                Err(CommonError::ServiceError("Database error".to_string()))
            } else {
                let activity = VolunteerActivity::new(_user_id, None, "Test activity".to_string(), Decimal::from(2));
                Ok(vec![activity])
            }
        }
        
        async fn save_verification(&self, _verification: &crate::models::VolunteerVerification) -> Result<(), CommonError> {
            if self.should_fail {
                Err(CommonError::ServiceError("Database error".to_string()))
            } else {
                Ok(())
            }
        }
        
        async fn save_conversion(&self, _conversion: &DabloonConversion) -> Result<(), CommonError> {
            if self.should_fail {
                Err(CommonError::ServiceError("Database error".to_string()))
            } else {
                Ok(())
            }
        }
    }
    
    // Mock wallet service implementation
    struct MockWalletService {
        should_fail: bool,
        insufficient_funds: bool,
    }
    
    #[async_trait::async_trait]
    impl wallet::application::WalletService for MockWalletService {
        async fn get_or_create_wallet(&self, _user_id: Uuid) -> Result<wallet::domain::wallet::Wallet, FinancialError> {
            if self.should_fail {
                Err(FinancialError::ServiceError("Wallet service error".to_string()))
            } else {
                Ok(wallet::domain::wallet::Wallet::new(_user_id))
            }
        }
        
        async fn add_dabloons(&self, _user_id: Uuid, _amount: Money, _description: Option<String>) -> Result<wallet::domain::wallet::Wallet, FinancialError> {
            if self.should_fail {
                Err(FinancialError::ServiceError("Wallet service error".to_string()))
            } else if self.insufficient_funds {
                Err(FinancialError::InsufficientFunds(Currency::Dabloons))
            } else {
                Ok(wallet::domain::wallet::Wallet::new(_user_id))
            }
        }
        
        async fn subtract_dabloons(&self, _user_id: Uuid, _amount: Money, _description: Option<String>) -> Result<wallet::domain::wallet::Wallet, FinancialError> {
            if self.should_fail {
                Err(FinancialError::ServiceError("Wallet service error".to_string()))
            } else if self.insufficient_funds {
                Err(FinancialError::InsufficientFunds(Currency::Dabloons))
            } else {
                Ok(wallet::domain::wallet::Wallet::new(_user_id))
            }
        }
        
        async fn transfer_dabloons(&self, _from_user_id: Uuid, _to_user_id: Uuid, _amount: Money, _description: Option<String>) -> Result<(wallet::domain::wallet::Wallet, wallet::domain::wallet::Wallet), FinancialError> {
            if self.should_fail {
                Err(FinancialError::ServiceError("Wallet service error".to_string()))
            } else if self.insufficient_funds {
                Err(FinancialError::InsufficientFunds(Currency::Dabloons))
            } else {
                Ok((wallet::domain::wallet::Wallet::new(_from_user_id), wallet::domain::wallet::Wallet::new(_to_user_id)))
            }
        }
        
        async fn send_tip(&self, _from_user_id: Uuid, _to_user_id: Uuid, _amount: Money, _note: Option<String>) -> Result<(), FinancialError> {
            if self.should_fail {
                Err(FinancialError::ServiceError("Wallet service error".to_string()))
            } else if self.insufficient_funds {
                Err(FinancialError::InsufficientFunds(Currency::Dabloons))
            } else {
                Ok(())
            }
        }
        
        async fn get_transaction_history(&self, _user_id: Uuid) -> Result<Vec<wallet::domain::wallet::WalletTransaction>, FinancialError> {
            if self.should_fail {
                Err(FinancialError::ServiceError("Wallet service error".to_string()))
            } else {
                Ok(vec![])
            }
        }
        
        async fn distribute_universal_income(&self, _user_id: Uuid, _amount: Money, _distribution_date: chrono::NaiveDate) -> Result<wallet::domain::wallet::Wallet, FinancialError> {
            if self.should_fail {
                Err(FinancialError::ServiceError("Wallet service error".to_string()))
            } else {
                Ok(wallet::domain::wallet::Wallet::new(_user_id))
            }
        }
        
        async fn credit_volunteer_dabloons(&self, _user_id: Uuid, _amount: Money, _hours_converted: Decimal) -> Result<wallet::domain::wallet::Wallet, FinancialError> {
            if self.should_fail {
                Err(FinancialError::ServiceError("Wallet service error".to_string()))
            } else if self.insufficient_funds {
                Err(FinancialError::InsufficientFunds(Currency::Dabloons))
            } else {
                Ok(wallet::domain::wallet::Wallet::new(_user_id))
            }
        }
    }
    
    // Mock notification service implementation
    struct MockNotificationService {
        should_fail: bool,
    }
    
    #[async_trait::async_trait]
    impl notification_core::application::service::NotificationService for MockNotificationService {
        async fn send_notification(&self, _notification: notification_core::domain::types::Notification) -> Result<(), notification_core::domain::error::NotificationError> {
            if self.should_fail {
                Err(notification_core::domain::error::NotificationError::ServiceError("Notification service error".to_string()))
            } else {
                Ok(())
            }
        }
        
        async fn get_user_preferences(&self, _user_id: &str) -> Result<notification_core::domain::preferences::UserPreferences, notification_core::domain::error::NotificationError> {
            if self.should_fail {
                Err(notification_core::domain::error::NotificationError::ServiceError("Notification service error".to_string()))
            } else {
                Ok(notification_core::domain::preferences::UserPreferences::default())
            }
        }
        
        async fn update_user_preferences(&self, _user_id: &str, _preferences: notification_core::domain::preferences::UserPreferences) -> Result<(), notification_core::domain::error::NotificationError> {
            if self.should_fail {
                Err(notification_core::domain::error::NotificationError::ServiceError("Notification service error".to_string()))
            } else {
                Ok(())
            }
        }
    }
    
    // Mock social integration service implementation
    struct MockSocialIntegrationService {
        should_fail: bool,
    }
    
    #[async_trait::async_trait]
    impl social_integration::application::social_integration_service::SocialIntegrationService for MockSocialIntegrationService {
        async fn handle_social_event(&self, _event: social_integration::domain::social_event::SocialEvent) -> Result<(), social_integration::domain::error::SocialIntegrationError> {
            if self.should_fail {
                Err(social_integration::domain::error::SocialIntegrationError::ServiceError("Social integration error".to_string()))
            } else {
                Ok(())
            }
        }
    }
    
    #[tokio::test]
    async fn test_log_volunteer_hours_success() {
        // Setup
        let volunteer_repo = Arc::new(MockVolunteerRepository { should_fail: false });
        let wallet_service = Arc::new(MockWalletService { should_fail: false, insufficient_funds: false });
        let notification_service = Arc::new(MockNotificationService { should_fail: false });
        let social_service = Arc::new(MockSocialIntegrationService { should_fail: false });
        
        let service = VolunteerServiceImpl::new(
            volunteer_repo,
            wallet_service,
            notification_service,
            social_service,
        );
        
        // Execute
        let result = service.log_volunteer_hours(
            Uuid::new_v4(),
            None,
            "Test volunteer work".to_string(),
            Decimal::from(2),
        ).await;
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_log_volunteer_hours_validation_error() {
        // Setup
        let volunteer_repo = Arc::new(MockVolunteerRepository { should_fail: false });
        let wallet_service = Arc::new(MockWalletService { should_fail: false, insufficient_funds: false });
        let notification_service = Arc::new(MockNotificationService { should_fail: false });
        let social_service = Arc::new(MockSocialIntegrationService { should_fail: false });
        
        let service = VolunteerServiceImpl::new(
            volunteer_repo,
            wallet_service,
            notification_service,
            social_service,
        );
        
        // Execute with negative hours
        let result = service.log_volunteer_hours(
            Uuid::new_v4(),
            None,
            "Test volunteer work".to_string(),
            Decimal::from(-1),
        ).await;
        
        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            CommonError::ValidationError(_) => {}, // Expected
            _ => panic!("Expected ValidationError"),
        }
    }
    
    #[tokio::test]
    async fn test_verify_volunteer_hours_success() {
        // Setup
        let volunteer_repo = Arc::new(MockVolunteerRepository { should_fail: false });
        let wallet_service = Arc::new(MockWalletService { should_fail: false, insufficient_funds: false });
        let notification_service = Arc::new(MockNotificationService { should_fail: false });
        let social_service = Arc::new(MockSocialIntegrationService { should_fail: false });
        
        let service = VolunteerServiceImpl::new(
            volunteer_repo,
            wallet_service,
            notification_service,
            social_service,
        );
        
        // Execute
        let result = service.verify_volunteer_hours(
            Uuid::new_v4(),
            Uuid::new_v4(),
            true,
            Some("Looks good".to_string()),
        ).await;
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_convert_to_dabloons_success() {
        // Setup
        let volunteer_repo = Arc::new(MockVolunteerRepository { should_fail: false });
        let wallet_service = Arc::new(MockWalletService { should_fail: false, insufficient_funds: false });
        let notification_service = Arc::new(MockNotificationService { should_fail: false });
        let social_service = Arc::new(MockSocialIntegrationService { should_fail: false });
        
        let service = VolunteerServiceImpl::new(
            volunteer_repo,
            wallet_service,
            notification_service,
            social_service,
        );
        
        // Execute
        let result = service.convert_to_dabloons(
            Uuid::new_v4(),
            Uuid::new_v4(),
        ).await;
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_convert_to_dabloons_insufficient_balance() {
        // Setup
        let volunteer_repo = Arc::new(MockVolunteerRepository { should_fail: false });
        let wallet_service = Arc::new(MockWalletService { should_fail: false, insufficient_funds: true });
        let notification_service = Arc::new(MockNotificationService { should_fail: false });
        let social_service = Arc::new(MockSocialIntegrationService { should_fail: false });
        
        let service = VolunteerServiceImpl::new(
            volunteer_repo,
            wallet_service,
            notification_service,
            social_service,
        );
        
        // Execute
        let result = service.convert_to_dabloons(
            Uuid::new_v4(),
            Uuid::new_v4(),
        ).await;
        
        // Assert
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_get_user_activities_success() {
        // Setup
        let volunteer_repo = Arc::new(MockVolunteerRepository { should_fail: false });
        let wallet_service = Arc::new(MockWalletService { should_fail: false, insufficient_funds: false });
        let notification_service = Arc::new(MockNotificationService { should_fail: false });
        let social_service = Arc::new(MockSocialIntegrationService { should_fail: false });
        
        let service = VolunteerServiceImpl::new(
            volunteer_repo,
            wallet_service,
            notification_service,
            social_service,
        );
        
        // Execute
        let result = service.get_user_activities(Uuid::new_v4()).await;
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_get_verified_user_activities_success() {
        // Setup
        let volunteer_repo = Arc::new(MockVolunteerRepository { should_fail: false });
        let wallet_service = Arc::new(MockWalletService { should_fail: false, insufficient_funds: false });
        let notification_service = Arc::new(MockNotificationService { should_fail: false });
        let social_service = Arc::new(MockSocialIntegrationService { should_fail: false });
        
        let service = VolunteerServiceImpl::new(
            volunteer_repo,
            wallet_service,
            notification_service,
            social_service,
        );
        
        // Execute
        let result = service.get_verified_user_activities(Uuid::new_v4()).await;
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_volunteer_service_event_broadcasting() {
        // Setup
        let volunteer_repo = Arc::new(MockVolunteerRepository { should_fail: false });
        let wallet_service = Arc::new(MockWalletService { should_fail: false, insufficient_funds: false });
        let notification_service = Arc::new(MockNotificationService { should_fail: false });
        let social_service = Arc::new(MockSocialIntegrationService { should_fail: false });
        
        let service = VolunteerServiceImpl::new(
            volunteer_repo,
            wallet_service,
            notification_service,
            social_service,
        );
        
        // Subscribe to events
        let mut receiver = service.subscribe_volunteer_events();
        
        // Execute an operation that should broadcast an event
        let user_id = Uuid::new_v4();
        let result = service.log_volunteer_hours(
            user_id,
            None,
            "Test volunteer work".to_string(),
            Decimal::from(2),
        ).await;
        
        // Assert the operation succeeded
        assert!(result.is_ok());
        
        // Check that an event was broadcast
        let event = receiver.try_recv();
        assert!(event.is_ok());
    }
}
//! Service layer for volunteer functionality

use async_trait::async_trait;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::models::{VolunteerActivity, VolunteerVerification, DabloonConversion, VerificationStatus};
use crate::repositories::VolunteerRepository;
use wallet::application::WalletService;
use wallet::domain::primitives::{Money, Currency};
use notification_core::application::service::NotificationService;
use notification_core::domain::{types::{Notification, NotificationCategory, NotificationPriority, ChannelType}, preferences::UserPreferences};
use cpay_core::models::SkillRate;
use social_integration::domain::social_event::SocialEvent;
use social_integration::application::social_integration_service::SocialIntegrationService;
use common_utils::error::CommonError;
use std::sync::Arc;
use tokio::sync::broadcast;

// Conditionally import common_utils logging or fallback to tracing
#[cfg(feature = "common-utils-integration")]
use common_utils::logging::{info, warn, error, debug};
#[cfg(not(feature = "common-utils-integration"))]
use tracing::{info, warn, error, debug};

/// Service trait for volunteer operations
#[async_trait]
pub trait VolunteerService {
    /// Log volunteer hours for a user
    async fn log_volunteer_hours(
        &self,
        user_id: Uuid,
        organization_id: Option<Uuid>,
        description: String,
        hours: Decimal,
    ) -> Result<VolunteerActivity, CommonError>;
    
    /// Verify volunteer hours
    async fn verify_volunteer_hours(
        &self,
        activity_id: Uuid,
        verifier_id: Uuid,
        approved: bool,
        notes: Option<String>,
    ) -> Result<VolunteerActivity, CommonError>;
    
    /// Convert verified volunteer hours to Dabloons
    async fn convert_to_dabloons(
        &self,
        activity_id: Uuid,
        user_id: Uuid,
    ) -> Result<DabloonConversion, CommonError>;
    
    /// Get all volunteer activities for a user
    async fn get_user_activities(&self, user_id: Uuid) -> Result<Vec<VolunteerActivity>, CommonError>;
    
    /// Get verified volunteer activities for a user
    async fn get_verified_user_activities(&self, user_id: Uuid) -> Result<Vec<VolunteerActivity>, CommonError>;
}

/// Implementation of the VolunteerService
pub struct VolunteerServiceImpl {
    volunteer_repo: Arc<dyn VolunteerRepository>,
    wallet_service: Arc<dyn WalletService>,
    notification_service: Arc<dyn NotificationService>,
    social_service: Arc<dyn SocialIntegrationService>,
    // For broadcasting events
    volunteer_events: broadcast::Sender<VolunteerEvent>,
}

/// Events that can be broadcast by the volunteer service
#[derive(Debug, Clone)]
pub enum VolunteerEvent {
    /// Volunteer hours were logged
    HoursLogged { activity_id: Uuid, user_id: Uuid, hours: Decimal },
    
    /// Volunteer hours were verified
    HoursVerified { activity_id: Uuid, user_id: Uuid, verified: bool },
    
    /// Volunteer hours were converted to Dabloons
    ConvertedToDabloons { activity_id: Uuid, user_id: Uuid, dabloons: Money },
}

impl VolunteerServiceImpl {
    /// Create a new volunteer service
    pub fn new(
        volunteer_repo: Arc<dyn VolunteerRepository>,
        wallet_service: Arc<dyn WalletService>,
        notification_service: Arc<dyn NotificationService>,
        social_service: Arc<dyn SocialIntegrationService>,
    ) -> Self {
        let (volunteer_events, _) = broadcast::channel(100);
        Self {
            volunteer_repo,
            wallet_service,
            notification_service,
            social_service,
            volunteer_events,
        }
    }
    
    /// Subscribe to volunteer events
    pub fn subscribe_volunteer_events(&self) -> broadcast::Receiver<VolunteerEvent> {
        self.volunteer_events.subscribe()
    }
    
    /// Get the skill rate for volunteer hour conversion
    /// In a real implementation, this would be configurable or retrieved from a service
    fn get_skill_rate(&self) -> Decimal {
        // Default rate of 10 Dabloons per hour
        Decimal::from(10)
    }
    
    /// Send a notification to a user
    async fn send_notification(
        &self,
        user_id: Uuid,
        title: String,
        body: String,
        category: NotificationCategory,
    ) -> Result<(), CommonError> {
        let notification = Notification::new_immediate(
            user_id.to_string(),
            category,
            NotificationPriority::Normal,
            title,
            body,
            serde_json::Value::Null,
            vec![ChannelType::InApp, ChannelType::Email],
        );
        
        self.notification_service.send_notification(notification).await
            .map_err(|e| CommonError::ServiceError(format!("Failed to send notification: {}", e)))
    }
    
    /// Create a social event for a volunteer activity
    async fn create_social_event(&self, event: SocialEvent) -> Result<(), CommonError> {
        self.social_service.handle_social_event(event).await
            .map_err(|e| CommonError::ServiceError(format!("Failed to create social event: {}", e)))
    }
}

#[async_trait]
impl VolunteerService for VolunteerServiceImpl {
    async fn log_volunteer_hours(
        &self,
        user_id: Uuid,
        organization_id: Option<Uuid>,
        description: String,
        hours: Decimal,
    ) -> Result<VolunteerActivity, CommonError> {
        // Validate hours
        if hours <= Decimal::ZERO {
            return Err(CommonError::ValidationError("Hours must be positive".to_string()));
        }
        
        // Create the volunteer activity
        let mut activity = VolunteerActivity::new(user_id, organization_id, description, hours);
        
        // Save the activity
        self.volunteer_repo.save_activity(&activity).await?;
        
        // Send notification to organization if provided
        if let Some(org_id) = organization_id {
            // In a real implementation, we would look up the organization admins
            // For now, we'll just log that a notification should be sent
            info!("Notification should be sent to organization {} for volunteer hours", org_id);
        }
        
        // Broadcast the event
        let _ = self.volunteer_events.send(VolunteerEvent::HoursLogged {
            activity_id: activity.id,
            user_id: activity.user_id,
            hours: activity.hours,
        });
        
        Ok(activity)
    }
    
    async fn verify_volunteer_hours(
        &self,
        activity_id: Uuid,
        verifier_id: Uuid,
        approved: bool,
        notes: Option<String>,
    ) -> Result<VolunteerActivity, CommonError> {
        // Find the activity
        let mut activity = self.volunteer_repo.find_activity_by_id(activity_id).await?
            .ok_or_else(|| CommonError::NotFound("Volunteer activity not found".to_string()))?;
        
        // Create a verification record
        let mut verification = VolunteerVerification::new(activity_id, verifier_id);
        
        if approved {
            verification.approve(notes.clone());
            activity.verify(verifier_id);
        } else {
            verification.reject(notes.clone());
        }
        
        // Save the verification
        self.volunteer_repo.save_verification(&verification).await?;
        
        // Update the activity
        self.volunteer_repo.save_activity(&activity).await?;
        
        // Send notification to user
        let status = if approved { "approved" } else { "rejected" };
        self.send_notification(
            activity.user_id,
            "Volunteer Hours Verified".to_string(),
            format!("Your volunteer hours for '{}' have been {}.", activity.description, status),
            NotificationCategory::Social,
        ).await?;
        
        // Create social event if approved
        if approved {
            let event = SocialEvent::Volunteered {
                user_id: activity.user_id,
                opportunity_id: activity.id, // Using activity ID as proxy
                hours_contributed: activity.hours.to_f32().unwrap_or(0.0),
                timestamp: chrono::Utc::now(),
            };
            self.create_social_event(event).await?;
        }
        
        // Broadcast the event
        let _ = self.volunteer_events.send(VolunteerEvent::HoursVerified {
            activity_id: activity.id,
            user_id: activity.user_id,
            verified: approved,
        });
        
        Ok(activity)
    }
    
    async fn convert_to_dabloons(
        &self,
        activity_id: Uuid,
        user_id: Uuid,
    ) -> Result<DabloonConversion, CommonError> {
        // Find the activity
        let mut activity = self.volunteer_repo.find_activity_by_id(activity_id).await?
            .ok_or_else(|| CommonError::NotFound("Volunteer activity not found".to_string()))?;
        
        // Check if the activity belongs to the user
        if activity.user_id != user_id {
            return Err(CommonError::Unauthorized("You can only convert your own volunteer hours".to_string()));
        }
        
        // Check if the activity is verified
        if !activity.verified {
            return Err(CommonError::ValidationError("Volunteer hours must be verified before conversion".to_string()));
        }
        
        // Check if already converted
        if activity.converted_to_dabloons {
            return Err(CommonError::ValidationError("Volunteer hours have already been converted".to_string()));
        }
        
        // Get the skill rate
        let skill_rate = self.get_skill_rate();
        
        // Calculate Dabloons to credit
        let dabloons_amount = activity.hours * skill_rate;
        let dabloons = Money::new(dabloons_amount, Currency::Dabloons);
        
        // Credit Dabloons to user's wallet
        let wallet = self.wallet_service.add_dabloons(
            user_id,
            dabloons.clone(),
            Some(format!("Converted {} volunteer hours", activity.hours)),
        ).await?;
        
        // Create the conversion record
        let conversion = DabloonConversion::new(
            activity_id,
            user_id,
            activity.hours,
            dabloons.clone(),
            wallet.id, // Using wallet ID as proxy for transaction ID
            skill_rate,
        );
        
        // Save the conversion
        self.volunteer_repo.save_conversion(&conversion).await?;
        
        // Mark activity as converted
        activity.mark_as_converted(conversion.id);
        self.volunteer_repo.save_activity(&activity).await?;
        
        // Send notification to user
        self.send_notification(
            user_id,
            "Volunteer Hours Converted".to_string(),
            format!("{} volunteer hours have been converted to {} Dabloons.", activity.hours, dabloons_amount),
            NotificationCategory::Transaction,
        ).await?;
        
        // Broadcast the event
        let _ = self.volunteer_events.send(VolunteerEvent::ConvertedToDabloons {
            activity_id: activity.id,
            user_id: activity.user_id,
            dabloons: dabloons.clone(),
        });
        
        Ok(conversion)
    }
    
    async fn get_user_activities(&self, user_id: Uuid) -> Result<Vec<VolunteerActivity>, CommonError> {
        self.volunteer_repo.find_activities_by_user_id(user_id).await
    }
    
    async fn get_verified_user_activities(&self, user_id: Uuid) -> Result<Vec<VolunteerActivity>, CommonError> {
        let activities = self.volunteer_repo.find_activities_by_user_id(user_id).await?;
        Ok(activities.into_iter().filter(|a| a.verified).collect())
    }
}
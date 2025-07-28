//! Calendar integration service for cross-module event registration
//!
//! This service provides the implementation for the CalendarIntegration gRPC service
//! defined in calendar_integration.proto. It handles event registration from
//! external modules while enforcing privacy and data minimization principles.

use crate::domain::{CalendarEvent, EventType, EventVisibility, PaymentStatus, SalesStage};
use crate::application::{EventRepository, CalendarError};
use crate::infrastructure::sync::P2PSyncManager;
use cpc_protos::calendar_integration::{
    CalendarIntegration, EventRegistrationResponse, CrmEventRequest, InvoiceEventRequest,
    Uuid as ProtoUuid, IntegrationFilter, TimelineEvent, EventType as ProtoEventType,
    EventVisibility as ProtoEventVisibility, PaymentStatus as ProtoPaymentStatus,
    SalesStage as ProtoSalesStage
};
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tracing::{info, error, instrument};

/// Service implementation for CalendarIntegration gRPC service
pub struct CalendarIntegrationService {
    event_repository: Arc<dyn EventRepository>,
    p2p_manager: Arc<dyn P2PSyncManager>,
    consent_checker: Arc<dyn ConsentChecker>,
}

impl CalendarIntegrationService {
    pub fn new(
        event_repository: Arc<dyn EventRepository>,
        p2p_manager: Arc<dyn P2PSyncManager>,
        consent_checker: Arc<dyn ConsentChecker>,
    ) -> Self {
        Self {
            event_repository,
            p2p_manager,
            consent_checker,
        }
    }
}

#[async_trait]
impl CalendarIntegration for CalendarIntegrationService {
    #[instrument(skip(self, request))]
    async fn register_crm_event(
        &self,
        request: CrmEventRequest,
    ) -> Result<EventRegistrationResponse, CalendarError> {
        // 1. Verify consent
        let user_id = self.convert_uuid(&request.user_id)?;
        if !self.consent_checker.has_consent(
            user_id,
            Module::Crm,
            Module::Calendar,
            ConsentPurpose::CrmIntegration,
        ).await? {
            return Err(CalendarError::MissingConsent);
        }

        // 2. Convert and validate CRM event
        let calendar_event = self.convert_crm_event(request, user_id)?;

        // 3. Save event
        self.event_repository.save(&calendar_event).await?;

        // 4. Sync with p2p network
        self.p2p_manager.share_event(&calendar_event, &[]).await?;

        Ok(EventRegistrationResponse {
            event_id: self.to_proto_uuid(calendar_event.id),
            success: true,
            message: "CRM event registered successfully".to_string(),
        })
    }

    #[instrument(skip(self, request))]
    async fn register_invoice_event(
        &self,
        request: InvoiceEventRequest,
    ) -> Result<EventRegistrationResponse, CalendarError> {
        // 1. Verify consent
        let user_id = self.convert_uuid(&request.user_id)?;
        if !self.consent_checker.has_consent(
            user_id,
            Module::Invoicing,
            Module::Calendar,
            ConsentPurpose::InvoicingIntegration,
        ).await? {
            return Err(CalendarError::MissingConsent);
        }

        // 2. Convert and validate invoice event
        let calendar_event = self.convert_invoice_event(request, user_id)?;

        // 3. Save event
        self.event_repository.save(&calendar_event).await?;

        // 4. Sync with p2p network
        self.p2p_manager.share_event(&calendar_event, &[]).await?;

        Ok(EventRegistrationResponse {
            event_id: self.to_proto_uuid(calendar_event.id),
            success: true,
            message: "Invoice event registered successfully".to_string(),
        })
    }

    #[instrument(skip(self, request))]
    async fn get_integrated_timeline(
        &self,
        request: IntegrationFilter,
    ) -> Result<tonic::Response<tonic::Streaming<TimelineEvent>>, CalendarError> {
        // Implementation for streaming timeline events
        unimplemented!("Timeline streaming not yet implemented");
    }
}

impl CalendarIntegrationService {
    /// Convert proto UUID to domain UUID
    fn convert_uuid(&self, proto_uuid: &ProtoUuid) -> Result<Uuid, CalendarError> {
        Uuid::parse_str(&proto_uuid.value)
            .map_err(|_| CalendarError::InvalidUuid(proto_uuid.value.clone()))
    }

    /// Convert domain UUID to proto UUID
    fn to_proto_uuid(&self, uuid: Uuid) -> ProtoUuid {
        ProtoUuid {
            value: uuid.to_string(),
        }
    }

    /// Convert CRM event request to calendar event
    fn convert_crm_event(
        &self,
        request: CrmEventRequest,
        user_id: Uuid,
    ) -> Result<CalendarEvent, CalendarError> {
        match request.event_type {
            Some(cpc_protos::calendar_integration::crm_event_request::EventType::SalesPipeline(opportunity)) => {
                self.convert_sales_pipeline_event(opportunity, user_id)
            }
            Some(cpc_protos::calendar_integration::crm_event_request::EventType::LeadFollowUp(follow_up)) => {
                self.convert_lead_follow_up_event(follow_up, user_id)
            }
            Some(cpc_protos::calendar_integration::crm_event_request::EventType::EmailCampaign(campaign)) => {
                self.convert_email_campaign_event(campaign, user_id)
            }
            None => Err(CalendarError::InvalidEvent("No event type specified".to_string())),
        }
    }

    /// Convert invoice event request to calendar event
    fn convert_invoice_event(
        &self,
        request: InvoiceEventRequest,
        user_id: Uuid,
    ) -> Result<CalendarEvent, CalendarError> {
        match request.event_type {
            Some(cpc_protos::calendar_integration::invoice_event_request::EventType::PaymentDue(payment_due)) => {
                self.convert_payment_due_event(payment_due, user_id)
            }
            Some(cpc_protos::calendar_integration::invoice_event_request::EventType::PaymentStatusChange(status_change)) => {
                self.convert_payment_status_change_event(status_change, user_id)
            }
            None => Err(CalendarError::InvalidEvent("No event type specified".to_string())),
        }
    }

    /// Convert sales pipeline milestone to calendar event
    fn convert_sales_pipeline_event(
        &self,
        event: cpc_protos::calendar_integration::SalesPipelineMilestoneEvent,
        user_id: Uuid,
    ) -> Result<CalendarEvent, CalendarError> {
        let stage = match ProtoSalesStage::from_i32(event.stage) {
            Some(ProtoSalesStage::Lead) => SalesStage::Lead,
            Some(ProtoSalesStage::Qualified) => SalesStage::Qualified,
            Some(ProtoSalesStage::DemoScheduled) => SalesStage::DemoScheduled,
            Some(ProtoSalesStage::ProposalSent) => SalesStage::ProposalSent,
            Some(ProtoSalesStage::Negotiation) => SalesStage::Negotiation,
            Some(ProtoSalesStage::ClosedWon) => SalesStage::ClosedWon,
            Some(ProtoSalesStage::ClosedLost) => SalesStage::ClosedLost,
            None => return Err(CalendarError::InvalidEvent("Invalid sales stage".to_string())),
        };

        // Create a meaningful title based on the stage
        let title = match stage {
            SalesStage::DemoScheduled => "Demo Scheduled".to_string(),
            SalesStage::ProposalSent => "Proposal Sent".to_string(),
            _ => format!("Sales Stage: {:?}", stage),
        };

        Ok(CalendarEvent::new(
            user_id,
            title,
            Some(format!("Opportunity ID: {}", event.opportunity_id.value)),
            Utc::now(), // This would be filled with actual date from CRM
            Utc::now() + chrono::Duration::minutes(30),
            EventType::SalesPipelineMilestone {
                opportunity_id: self.convert_uuid(&event.opportunity_id)?,
                stage,
            },
            EventVisibility::Business,
            None,
            None,
        ))
    }

    /// Convert lead follow-up event to calendar event
    fn convert_lead_follow_up_event(
        &self,
        event: cpc_protos::calendar_integration::LeadFollowUpEvent,
        user_id: Uuid,
    ) -> Result<CalendarEvent, CalendarError> {
        // Determine urgency based on score change
        let title = if event.score_change > 20 {
            "Urgent: Lead Engagement Spike".to_string()
        } else {
            "Lead Follow-Up".to_string()
        };

        Ok(CalendarEvent::new(
            user_id,
            title,
            Some(format!(
                "Lead ID: {} | Score Change: {}",
                event.lead_id.value, event.score_change
            )),
            Utc::now(), // This would be filled with actual follow-up date
            Utc::now() + chrono::Duration::hours(24),
            EventType::LeadFollowUp {
                lead_id: self.convert_uuid(&event.lead_id)?,
                score_change: event.score_change,
                wellness_threshold: event.wellness_threshold.map(|t| t as u8),
            },
            EventVisibility::Business,
            None,
            None,
        ))
    }

    /// Convert email campaign timeline event to calendar event
    fn convert_email_campaign_event(
        &self,
        event: cpc_protos::calendar_integration::EmailCampaignTimelineEvent,
        user_id: Uuid,
    ) -> Result<CalendarEvent, CalendarError> {
        Ok(CalendarEvent::new(
            user_id,
            format!("Email Campaign: {}", event.campaign_name),
            Some(format!(
                "Campaign ID: {} | Recipients: {}",
                event.campaign_id.value, event.total_recipients
            )),
            Utc::now(), // This would be filled with campaign start date
            Utc::now() + chrono::Duration::days(7), // Campaign duration
            EventType::EmailCampaignTimeline {
                campaign_id: self.convert_uuid(&event.campaign_id)?,
                campaign_name: event.campaign_name,
                total_recipients: event.total_recipients,
            },
            EventVisibility::Business,
            None,
            None,
        ))
    }

    /// Convert payment due event to calendar event
    fn convert_payment_due_event(
        &self,
        event: cpc_protos::calendar_integration::PaymentDueEvent,
        user_id: Uuid,
    ) -> Result<CalendarEvent, CalendarError> {
        let status = match ProtoPaymentStatus::from_i32(event.status) {
            Some(ProtoPaymentStatus::Draft) => PaymentStatus::Draft,
            Some(ProtoPaymentStatus::Sent) => PaymentStatus::Sent,
            Some(ProtoPaymentStatus::Viewed) => PaymentStatus::Viewed,
            Some(ProtoPaymentStatus::Paid) => PaymentStatus::Paid,
            Some(ProtoPaymentStatus::Overdue) => PaymentStatus::Overdue,
            Some(ProtoPaymentStatus::Partial) => PaymentStatus::Partial,
            None => return Err(CalendarError::InvalidEvent("Invalid payment status".to_string())),
        };

        Ok(CalendarEvent::new(
            user_id,
            format!("Payment Due: ${:.2}", event.amount),
            Some(format!("Invoice ID: {}", event.invoice_id.value)),
            Utc::now(), // This would be filled with actual due date
            Utc::now() + chrono::Duration::days(30), // Default 30-day duration
            EventType::PaymentDue {
                invoice_id: self.convert_uuid(&event.invoice_id)?,
                amount: event.amount,
                status,
                payment_reminder_id: event.payment_reminder_id.as_ref().map(|id| self.convert_uuid(id)).transpose()?,
            },
            EventVisibility::Business,
            None,
            None,
        ))
    }

    /// Convert payment status change event to calendar event
    fn convert_payment_status_change_event(
        &self,
        event: cpc_protos::calendar_integration::PaymentStatusChangeEvent,
        user_id: Uuid,
    ) -> Result<CalendarEvent, CalendarError> {
        let previous_status = match ProtoPaymentStatus::from_i32(event.previous_status) {
            Some(ProtoPaymentStatus::Draft) => PaymentStatus::Draft,
            Some(ProtoPaymentStatus::Sent) => PaymentStatus::Sent,
            Some(ProtoPaymentStatus::Viewed) => PaymentStatus::Viewed,
            Some(ProtoPaymentStatus::Paid) => PaymentStatus::Paid,
            Some(ProtoPaymentStatus::Overdue) => PaymentStatus::Overdue,
            Some(ProtoPaymentStatus::Partial) => PaymentStatus::Partial,
            None => return Err(CalendarError::InvalidEvent("Invalid previous payment status".to_string())),
        };

        let new_status = match ProtoPaymentStatus::from_i32(event.new_status) {
            Some(ProtoPaymentStatus::Draft) => PaymentStatus::Draft,
            Some(ProtoPaymentStatus::Sent) => PaymentStatus::Sent,
            Some(ProtoPaymentStatus::Viewed) => PaymentStatus::Viewed,
            Some(ProtoPaymentStatus::Paid) => PaymentStatus::Paid,
            Some(ProtoPaymentStatus::Overdue) => PaymentStatus::Overdue,
            Some(ProtoPaymentStatus::Partial) => PaymentStatus::Partial,
            None => return Err(CalendarError::InvalidEvent("Invalid new payment status".to_string())),
        };

        Ok(CalendarEvent::new(
            user_id,
            format!("Payment Status Change: {:?} → {:?}", previous_status, new_status),
            Some(format!("Invoice ID: {}", event.invoice_id.value)),
            event.timestamp.to_std().map_err(|_| CalendarError::InvalidEvent("Invalid timestamp".to_string()))?.into(),
            event.timestamp.to_std().map_err(|_| CalendarError::InvalidEvent("Invalid timestamp".to_string()))?.into() + chrono::Duration::minutes(1),
            EventType::PaymentStatusChange {
                invoice_id: self.convert_uuid(&event.invoice_id)?,
                previous_status,
                new_status,
                timestamp: event.timestamp.to_std().map_err(|_| CalendarError::InvalidEvent("Invalid timestamp".to_string()))?.into(),
            },
            EventVisibility::Business,
            None,
            None,
        ))
    }
}

/// Trait for checking user consent for data sharing
#[async_trait]
pub trait ConsentChecker: Send + Sync {
    async fn has_consent(
        &self,
        user_id: Uuid,
        source_module: Module,
        target_module: Module,
        purpose: ConsentPurpose,
    ) -> Result<bool, CalendarError>;
}

/// Module identifiers for consent management
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Module {
    Calendar,
    Crm,
    Invoicing,
    Health,
    TaskManager,
    // Add other modules as needed
}

/// Purpose of data sharing for consent management
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsentPurpose {
    CrmIntegration,
    InvoicingIntegration,
    Analytics,
    // Add other purposes as needed
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use std::sync::Arc;
    use async_trait::async_trait;

    // Mock implementations for testing
    #[derive(Default)]
    struct MockEventRepository;

    #[async_trait]
    impl EventRepository for MockEventRepository {
        async fn save(&self, _event: &CalendarEvent) -> Result<(), CalendarError> {
            Ok(())
        }
        // Implement other methods as needed
    }

    #[derive(Default)]
    struct MockP2PManager;

    impl P2PSyncManager for MockP2PManager {
        async fn share_event(
            &self,
            _event: &CalendarEvent,
            _participants: &[Participant],
        ) -> Result<(), CalendarError> {
            Ok(())
        }
    }

    #[derive(Default)]
    struct MockConsentChecker {
        has_consent: bool,
    }

    #[async_trait]
    impl ConsentChecker for MockConsentChecker {
        async fn has_consent(
            &self,
            _user_id: Uuid,
            _source_module: Module,
            _target_module: Module,
            _purpose: ConsentPurpose,
        ) -> Result<bool, CalendarError> {
            Ok(self.has_consent)
        }
    }

    #[tokio::test]
    async fn test_sales_pipeline_event_conversion() {
        let repo = Arc::new(MockEventRepository::default());
        let p2p = Arc::new(MockP2PManager::default());
        let consent = Arc::new(MockConsentChecker { has_consent: true });
        
        let service = CalendarIntegrationService::new(repo, p2p, consent);
        let user_id = Uuid::new_v4();
        
        let request = CrmEventRequest {
            event_type: Some(
                cpc_protos::calendar_integration::crm_event_request::EventType::SalesPipeline(
                    cpc_protos::calendar_integration::SalesPipelineMilestoneEvent {
                        opportunity_id: ProtoUuid { value: Uuid::new_v4().to_string() },
                        stage: ProtoSalesStage::DemoScheduled as i32,
                    }
                )
            ),
            user_id: ProtoUuid { value: user_id.to_string() },
        };
        
        let response = service.register_crm_event(request).await.unwrap();
        assert!(response.success);
    }
}
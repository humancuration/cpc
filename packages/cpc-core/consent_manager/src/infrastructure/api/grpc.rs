//! gRPC server implementation for the consent manager.

use tonic::{Request, Response, Status};
use crate::{
    domain::{
        consent::{DataSharingLevel, Domain},
        audit::Actor,
    },
    application::service::ConsentService,
};

// Import the generated protobuf code
use consent_manager_proto::consent_manager_server::{ConsentManager, ConsentManagerServer};
use consent_manager_proto::*;

/// Consent manager gRPC service
pub struct ConsentManagerService {
    consent_service: ConsentService,
}

impl ConsentManagerService {
    /// Create a new consent manager gRPC service
    pub fn new(consent_service: ConsentService) -> Self {
        Self { consent_service }
    }
}

#[tonic::async_trait]
impl ConsentManager for ConsentManagerService {
    async fn get_consent_level(
        &self,
        request: Request<GetConsentLevelRequest>,
    ) -> Result<Response<GetConsentLevelResponse>, Status> {
        let req = request.into_inner();
        
        // Convert domain enum
        let domain = match req.domain() {
            Domain::FinancialData => Domain::FinancialData,
            Domain::HealthData => Domain::HealthData,
            Domain::CalendarData => Domain::CalendarData,
            Domain::CrmData => Domain::CrmData,
            Domain::ScmData => Domain::ScmData,
            Domain::DocumentData => Domain::DocumentData,
            Domain::WebsiteData => Domain::WebsiteData,
            Domain::RecruitmentData => Domain::RecruitmentData,
            Domain::DataLakehouse => Domain::DataLakehouse,
            Domain::ForecastingData => Domain::ForecastingData,
            _ => return Err(Status::invalid_argument("Invalid domain")),
        };
        
        // Get consent level
        let level = self.consent_service
            .get_consent_level(&req.user_id, domain)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        
        // Convert level to proto enum
        let proto_level = match level {
            DataSharingLevel::None => DataSharingLevel::None,
            DataSharingLevel::Minimal => DataSharingLevel::Minimal,
            DataSharingLevel::Standard => DataSharingLevel::Standard,
            DataSharingLevel::Full => DataSharingLevel::Full,
        };
        
        let response = GetConsentLevelResponse {
            level: proto_level.into(),
        };
        
        Ok(Response::new(response))
    }
    
    async fn update_consent_level(
        &self,
        request: Request<UpdateConsentLevelRequest>,
    ) -> Result<Response<UpdateConsentLevelResponse>, Status> {
        let req = request.into_inner();
        
        // Convert domain enum
        let domain = match req.domain() {
            Domain::FinancialData => Domain::FinancialData,
            Domain::HealthData => Domain::HealthData,
            Domain::CalendarData => Domain::CalendarData,
            Domain::CrmData => Domain::CrmData,
            Domain::ScmData => Domain::ScmData,
            Domain::DocumentData => Domain::DocumentData,
            Domain::WebsiteData => Domain::WebsiteData,
            Domain::RecruitmentData => Domain::RecruitmentData,
            Domain::DataLakehouse => Domain::DataLakehouse,
            Domain::ForecastingData => Domain::ForecastingData,
            _ => return Err(Status::invalid_argument("Invalid domain")),
        };
        
        // Convert level enum
        let level = match req.level() {
            DataSharingLevel::None => DataSharingLevel::None,
            DataSharingLevel::Minimal => DataSharingLevel::Minimal,
            DataSharingLevel::Standard => DataSharingLevel::Standard,
            DataSharingLevel::Full => DataSharingLevel::Full,
            _ => return Err(Status::invalid_argument("Invalid level")),
        };
        
        // Convert actor
        let actor = match req.actor_type() {
            ActorType::User => Actor::User(req.actor_id),
            ActorType::Service => Actor::Service(req.actor_id),
            ActorType::Admin => Actor::Admin(req.actor_id),
            _ => return Err(Status::invalid_argument("Invalid actor type")),
        };
        
        // Update consent level
        self.consent_service
            .update_consent_level(&req.user_id, domain, level, actor)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        
        let response = UpdateConsentLevelResponse {
            success: true,
        };
        
        Ok(Response::new(response))
    }
    
    async fn revoke_domain(
        &self,
        request: Request<RevokeDomainRequest>,
    ) -> Result<Response<RevokeDomainResponse>, Status> {
        let req = request.into_inner();
        
        // Convert domain enum
        let domain = match req.domain() {
            Domain::FinancialData => Domain::FinancialData,
            Domain::HealthData => Domain::HealthData,
            Domain::CalendarData => Domain::CalendarData,
            Domain::CrmData => Domain::CrmData,
            Domain::ScmData => Domain::ScmData,
            Domain::DocumentData => Domain::DocumentData,
            Domain::WebsiteData => Domain::WebsiteData,
            Domain::RecruitmentData => Domain::RecruitmentData,
            Domain::DataLakehouse => Domain::DataLakehouse,
            Domain::ForecastingData => Domain::ForecastingData,
            _ => return Err(Status::invalid_argument("Invalid domain")),
        };
        
        // Convert actor
        let actor = match req.actor_type() {
            ActorType::User => Actor::User(req.actor_id),
            ActorType::Service => Actor::Service(req.actor_id),
            ActorType::Admin => Actor::Admin(req.actor_id),
            _ => return Err(Status::invalid_argument("Invalid actor type")),
        };
        
        // Revoke domain
        self.consent_service
            .revoke_domain(&req.user_id, domain, actor)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        
        let response = RevokeDomainResponse {
            success: true,
        };
        
        Ok(Response::new(response))
    }
    
    async fn get_audit_events(
        &self,
        request: Request<GetAuditEventsRequest>,
    ) -> Result<Response<GetAuditEventsResponse>, Status> {
        let req = request.into_inner();
        
        // Get audit events
        let events = self.consent_service
            .get_audit_events(&req.user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        
        // Convert to proto format
        let proto_events: Vec<AuditEvent> = events
            .into_iter()
            .map(|event| {
                let domain = match event.domain {
                    Domain::FinancialData => Domain::FinancialData,
                    Domain::HealthData => Domain::HealthData,
                    Domain::CalendarData => Domain::CalendarData,
                    Domain::CrmData => Domain::CrmData,
                    Domain::ScmData => Domain::ScmData,
                    Domain::DocumentData => Domain::DocumentData,
                    Domain::WebsiteData => Domain::WebsiteData,
                    Domain::RecruitmentData => Domain::RecruitmentData,
                    Domain::DataLakehouse => Domain::DataLakehouse,
                    Domain::ForecastingData => Domain::ForecastingData,
                };
                
                let action = match event.action {
                    crate::domain::audit::ConsentAction::Granted => ConsentAction::Granted,
                    crate::domain::audit::ConsentAction::Revoked => ConsentAction::Revoked,
                    crate::domain::audit::ConsentAction::Modified => ConsentAction::Modified,
                };
                
                let previous_level = event.previous_level.map(|level| {
                    match level {
                        DataSharingLevel::None => DataSharingLevel::None,
                        DataSharingLevel::Minimal => DataSharingLevel::Minimal,
                        DataSharingLevel::Standard => DataSharingLevel::Standard,
                        DataSharingLevel::Full => DataSharingLevel::Full,
                    }
                });
                
                let new_level = match event.new_level {
                    DataSharingLevel::None => DataSharingLevel::None,
                    DataSharingLevel::Minimal => DataSharingLevel::Minimal,
                    DataSharingLevel::Standard => DataSharingLevel::Standard,
                    DataSharingLevel::Full => DataSharingLevel::Full,
                };
                
                let (actor_type, actor_id) = match event.actor {
                    Actor::User(id) => (ActorType::User, id),
                    Actor::Service(name) => (ActorType::Service, name),
                    Actor::Admin(id) => (ActorType::Admin, id),
                };
                
                AuditEvent {
                    id: event.id,
                    user_id: event.user_id,
                    domain: domain.into(),
                    action: action.into(),
                    previous_level: previous_level.map(|level| level.into()),
                    new_level: new_level.into(),
                    actor_type: actor_type.into(),
                    actor_id,
                    timestamp: event.timestamp.timestamp(),
                }
            })
            .collect();
        
        let response = GetAuditEventsResponse {
            events: proto_events,
        };
        
        Ok(Response::new(response))
    }
}

impl ConsentManagerService {
    /// Create a gRPC server instance
    pub fn into_server(self) -> ConsentManagerServer<Self> {
        ConsentManagerServer::new(self)
    }
}
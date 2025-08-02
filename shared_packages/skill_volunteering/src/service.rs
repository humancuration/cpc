//! gRPC service implementation for the Skill Volunteering module.

use crate::{
    opportunity_management::{models::{VolunteerOpportunity as DomainVolunteerOpportunity, ApplicationStatus}, service::{OpportunityService, OpportunityServiceError, UpdateOpportunityData}},
    skill_management::{models::Skill as DomainSkill, service::SkillService},
    user_skill_management::service::{UserSkillService, UserSkillServiceError},
    endorsement_management::{service::{EndorsementService, EndorsementServiceError}, models::SkillEndorsement},
};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::str::FromStr;
use rust_decimal::Decimal;

// Proto-generated code
pub mod proto {
    tonic::include_proto!("skill_volunteering");
}

use proto::{
    skill_volunteering_service_server::{SkillVolunteeringService, SkillVolunteeringServiceServer},
    ApplyRequest, ApplyResponse, CreateOpportunityRequest, CreateOpportunityResponse,
    DeleteOpportunityRequest, DeleteOpportunityResponse, Endorsement as ProtoEndorsement,
    GetEndorsementsRequest, GetEndorsementsResponse, GetOpportunityRequest,
    GetOpportunityResponse, ImpactRecordRequest, ImpactRecordResponse, ListOpportunitiesRequest,
    ListOpportunitiesResponse, ListUserApplicationsRequest, ListUserApplicationsResponse,
    OpportunitiesNearMeRequest, OpportunitiesNearMeResponse, RecordEndorsementRequest,
    RecordEndorsementResponse, UpdateApplicationStatusRequest, UpdateApplicationStatusResponse,
    UpdateOpportunityRequest, UpdateOpportunityResponse, VolunteeringActivityRequest,
    VolunteeringActivityResponse, VolunteerOpportunity as ProtoVolunteerOpportunity,
    OpportunityApplication as ProtoOpportunityApplication,
    AddUserSkillRequest, AddUserSkillResponse, ListUserSkillsRequest, ListUserSkillsResponse,
    RemoveUserSkillRequest, RemoveUserSkillResponse, UserSkill as ProtoUserSkill,
    UserSkillDetails as ProtoUserSkillDetails, Skill as ProtoSkill, ListSkillsRequest, ListSkillsResponse
};
    
    
    /// The main gRPC service for skill volunteering.
    pub struct SkillVolunteeringServiceImpl {
        opp_service: tokio::sync::Mutex<OpportunityService>,
        skill_service: Arc<SkillService>,
        user_skill_service: Arc<UserSkillService>,
        endorsement_service: Arc<EndorsementService>,
    }
    
    impl SkillVolunteeringServiceImpl {
        pub fn new(
            opp_service: OpportunityService,
            skill_service: Arc<SkillService>,
            user_skill_service: Arc<UserSkillService>,
            endorsement_service: Arc<EndorsementService>,
        ) -> Self {
            Self {
                opp_service: tokio::sync::Mutex::new(opp_service),
                skill_service,
                user_skill_service,
                endorsement_service,
            }
        }
    }
    
    #[tonic::async_trait]
    impl SkillVolunteeringService for SkillVolunteeringServiceImpl {
        async fn create_opportunity(
            &self,
            request: Request<CreateOpportunityRequest>,
        ) -> Result<Response<CreateOpportunityResponse>, Status> {
            let req = request.into_inner();
    
            let cause_id = Uuid::from_str(&req.cause_id)
                .map_err(|_| Status::invalid_argument("Invalid cause_id format"))?;
            
            let created_by = Uuid::from_str(&req.created_by)
                .map_err(|_| Status::invalid_argument("Invalid created_by format"))?;
    
            let required_skills = req
                .required_skills
                .into_iter()
                .map(|s| Uuid::from_str(&s))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| Status::invalid_argument("Invalid skill ID format in required_skills"))?;
    
            let deadline = req.deadline.and_then(|ts| {
                chrono::DateTime::from_timestamp(ts.seconds, ts.nanos as u32)
            });
    
            let mut opp_service = self.opp_service.lock().await;
    
            let opportunity = opp_service
                .create_opportunity(
                    cause_id,
                    req.title,
                    req.description,
                    required_skills,
                    Some(req.estimated_hours),
                    deadline,
                    created_by,
                )
                .await
                .map_err(map_error)?;
    
            let response = CreateOpportunityResponse {
                opportunity: Some(domain_opp_to_proto(opportunity)),
            };
    
            Ok(Response::new(response))
        }
    
        async fn apply_for_opportunity(
            &self,
            request: Request<ApplyRequest>,
        ) -> Result<Response<ApplyResponse>, Status> {
            let req = request.into_inner();

            let opportunity_id = Uuid::from_str(&req.opportunity_id)
                .map_err(|_| Status::invalid_argument("Invalid opportunity_id format"))?;
            
            let user_id = Uuid::from_str(&req.user_id)
                .map_err(|_| Status::invalid_argument("Invalid user_id format"))?;
            
            let mut opp_service = self.opp_service.lock().await;

            let application = opp_service
                .apply_for_opportunity(user_id, opportunity_id)
                .await
                .map_err(map_error)?;
            
            let response = ApplyResponse {
                application: Some(domain_app_to_proto(application)),
            };

            Ok(Response::new(response))
        }
    
        async fn list_opportunities(
            &self,
            request: Request<ListOpportunitiesRequest>,
        ) -> Result<Response<ListOpportunitiesResponse>, Status> {
            let req = request.into_inner();

            let cause_id = req
                .cause_id
                .map(|id| Uuid::from_str(&id))
                .transpose()
                .map_err(|_| Status::invalid_argument("Invalid cause_id format"))?;

            let skill_id = req
                .skill_id
                .map(|id| Uuid::from_str(&id))
                .transpose()
                .map_err(|_| Status::invalid_argument("Invalid skill_id format"))?;

            let limit = req.limit.unwrap_or(10) as i64;
            let offset = req.offset.unwrap_or(0) as i64;
            let only_open = req.only_open.unwrap_or(false);

            let mut opp_service = self.opp_service.lock().await;

            let (opportunities, total_count) = opp_service
                .list_opportunities(cause_id, skill_id, only_open, limit, offset)
                .await
                .map_err(map_error)?;
            
            let proto_opportunities = opportunities
                .into_iter()
                .map(domain_opp_to_proto)
                .collect();

            let response = ListOpportunitiesResponse {
                opportunities: proto_opportunities,
                total_count: total_count as i32,
            };

            Ok(Response::new(response))
        }
    
        async fn record_impact(
            &self,
            _request: Request<ImpactRecordRequest>,
        ) -> Result<Response<ImpactRecordResponse>, Status> {
            Err(Status::unimplemented("Not yet implemented"))
        }
    
        async fn get_opportunity(
            &self,
            request: Request<GetOpportunityRequest>,
        ) -> Result<Response<GetOpportunityResponse>, Status> {
            let req = request.into_inner();
    
            let opportunity_id = Uuid::from_str(&req.opportunity_id)
                .map_err(|_| Status::invalid_argument("Invalid opportunity_id format"))?;
    
            let mut opp_service = self.opp_service.lock().await;
    
            let opportunity = opp_service
                .get_opportunity(opportunity_id)
                .await
                .map_err(map_error)?;
    
            let response = GetOpportunityResponse {
                opportunity: Some(domain_opp_to_proto(opportunity)),
            };
    
            Ok(Response::new(response))
        }
    
        async fn update_opportunity(
            &self,
            request: Request<UpdateOpportunityRequest>,
        ) -> Result<Response<UpdateOpportunityResponse>, Status> {
            let req = request.into_inner();
    
            let opportunity_id = Uuid::from_str(&req.opportunity_id)
                .map_err(|_| Status::invalid_argument("Invalid opportunity_id format"))?;
    
            let required_skill_ids = req
                .required_skills
                .map(|skills| {
                    skills
                        .into_iter()
                        .map(|s| Uuid::from_str(&s))
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()
                .map_err(|_| Status::invalid_argument("Invalid skill ID format in required_skills"))?;

            let deadline = req.deadline.map(|ts| {
                chrono::DateTime::from_timestamp(ts.seconds, ts.nanos as u32)
                    .map(Some) // Wrap the DateTime in an Option to match UpdateOpportunityData
                    .ok_or_else(|| Status::invalid_argument("Invalid deadline timestamp"))
            }).transpose()?;
    
            let update_data = UpdateOpportunityData {
                title: req.title,
                description: req.description,
                required_skill_ids,
                estimated_hours: req.estimated_hours,
                deadline,
            };
    
            let mut opp_service = self.opp_service.lock().await;
    
            let updated_opportunity = opp_service
                .update_opportunity(opportunity_id, update_data)
                .await
                .map_err(map_error)?;
    
            let response = UpdateOpportunityResponse {
                opportunity: Some(domain_opp_to_proto(updated_opportunity)),
            };
    
            Ok(Response::new(response))
        }
    
        async fn delete_opportunity(
            &self,
            request: Request<DeleteOpportunityRequest>,
        ) -> Result<Response<DeleteOpportunityResponse>, Status> {
            let req = request.into_inner();

            let opportunity_id = Uuid::from_str(&req.opportunity_id)
                .map_err(|_| Status::invalid_argument("Invalid opportunity_id format"))?;

            let opp_service = self.opp_service.lock().await;

            opp_service
                .delete_opportunity(opportunity_id)
                .await
                .map_err(map_error)?;

            let response = DeleteOpportunityResponse {};

            Ok(Response::new(response))
        }
    
        async fn list_user_applications(
            &self,
            request: Request<ListUserApplicationsRequest>,
        ) -> Result<Response<ListUserApplicationsResponse>, Status> {
            let req = request.into_inner();

            let user_id = Uuid::from_str(&req.user_id)
                .map_err(|_| Status::invalid_argument("Invalid user_id format"))?;

            let limit = req.limit.unwrap_or(10) as i64;
            let offset = req.offset.unwrap_or(0) as i64;

            let opp_service = self.opp_service.lock().await;

            let (applications, total_count) = opp_service
                .list_user_applications(user_id, req.status, limit, offset)
                .await
                .map_err(map_error)?;

            let proto_applications = applications
                .into_iter()
                .map(domain_app_to_proto)
                .collect();

            let response = ListUserApplicationsResponse {
                applications: proto_applications,
                total_count: total_count as i32,
            };

            Ok(Response::new(response))
        }
    
        async fn update_application_status(
            &self,
            request: Request<UpdateApplicationStatusRequest>,
        ) -> Result<Response<UpdateApplicationStatusResponse>, Status> {
            let req = request.into_inner();

            let application_id = Uuid::from_str(&req.application_id)
                .map_err(|_| Status::invalid_argument("Invalid application_id format"))?;

            let new_status = ApplicationStatus::from_str(&req.status)
                .map_err(|_| Status::invalid_argument("Invalid status value"))?;

            let volunteer_hours = req
                .volunteer_hours
                .map(|s| Decimal::from_str(&s))
                .transpose()
                .map_err(|_| Status::invalid_argument("Invalid volunteer_hours format"))?;

            let mut opp_service = self.opp_service.lock().await;

            let updated_application = opp_service
                .update_application_status(application_id, new_status, volunteer_hours)
                .await
                .map_err(map_error)?;

            let response = UpdateApplicationStatusResponse {
                application: Some(domain_app_to_proto(updated_application)),
            };

            Ok(Response::new(response))
        }
    
        async fn add_user_skill(
            &self,
            request: Request<AddUserSkillRequest>,
        ) -> Result<Response<AddUserSkillResponse>, Status> {
            let req = request.into_inner();
    
            let user_id = Uuid::from_str(&req.user_id)
                .map_err(|_| Status::invalid_argument("Invalid user_id format"))?;
            let skill_id = Uuid::from_str(&req.skill_id)
                .map_err(|_| Status::invalid_argument("Invalid skill_id format"))?;
    
            let user_skill = self
                .user_skill_service
                .add_user_skill(user_id, skill_id, &req.skill_level)
                .await
                .map_err(map_user_skill_error)?;
    
            let response = AddUserSkillResponse {
                user_skill: Some(domain_user_skill_to_proto(user_skill)),
            };
    
            Ok(Response::new(response))
        }
        
        fn map_user_skill_error(err: UserSkillServiceError) -> Status {
            match err {
                UserSkillServiceError::InvalidInput(msg) => Status::invalid_argument(msg),
                UserSkillServiceError::NotFound => Status::not_found("User skill association not found"),
                UserSkillServiceError::AlreadyExists => {
                    Status::already_exists("This skill is already associated with the user")
                }
                UserSkillServiceError::Internal(msg) => {
                    Status::internal(format!("Internal server error: {}", msg))
                }
            }
            
            fn map_endorsement_error(err: EndorsementServiceError) -> Status {
                match err {
                    EndorsementServiceError::InvalidInput(msg) => Status::invalid_argument(msg),
                    EndorsementServiceError::NotFound => Status::not_found("Endorsement not found"),
                    EndorsementServiceError::Unauthorized => Status::permission_denied("Unauthorized"),
                    EndorsementServiceError::Internal(msg) => Status::internal(msg),
                }
            }
            
            fn domain_endorsement_to_proto(endorsement: SkillEndorsement) -> ProtoEndorsement {
                ProtoEndorsement {
                    id: endorsement.id.to_string(),
                    opportunity_id: endorsement.opportunity_id.to_string(),
                    skill_id: endorsement.skill_id.to_string(),
                    endorser_id: endorsement.endorser_id.to_string(),
                    recipient_id: endorsement.recipient_id.to_string(),
                    comment: endorsement.comment.unwrap_or_default(),
                    rating: endorsement.rating,
                    created_at: Some(prost_types::Timestamp {
                        seconds: endorsement.created_at.timestamp(),
                        nanos: endorsement.created_at.timestamp_nanos_opt().unwrap_or_default() as i32,
                    }),
                }
            }
        }
        
        fn domain_skill_to_proto(domain_skill: DomainSkill) -> ProtoSkill {
            ProtoSkill {
                id: domain_skill.id.to_string(),
                name: domain_skill.name,
                category: domain_skill.category,
                description: domain_skill.description.unwrap_or_default(),
            }
        }
        
        fn domain_user_skill_to_proto(
            domain_user_skill: crate::user_skill_management::models::UserSkill,
        ) -> ProtoUserSkill {
            ProtoUserSkill {
                user_id: domain_user_skill.user_id.to_string(),
                skill_id: domain_user_skill.skill_id.to_string(),
                skill_level: domain_user_skill.skill_level.to_string(),
                created_at: Some(prost_types::Timestamp {
                    seconds: domain_user_skill.created_at.timestamp(),
                    nanos: domain_user_skill.created_at.timestamp_nanos_opt().unwrap_or_default() as i32,
                }),
                updated_at: Some(prost_types::Timestamp {
                    seconds: domain_user_skill.updated_at.timestamp(),
                    nanos: domain_user_skill.updated_at.timestamp_nanos_opt().unwrap_or_default() as i32,
                }),
            }
        }
        
        fn domain_user_skill_details_to_proto(
            domain_details: crate::user_skill_management::models::UserSkillDetails,
        ) -> ProtoUserSkillDetails {
            ProtoUserSkillDetails {
                user_id: domain_details.user_id.to_string(),
                skill: Some(domain_skill_to_proto(domain_details.skill)),
                skill_level: domain_details.skill_level.to_string(),
                created_at: Some(prost_types::Timestamp {
                    seconds: domain_details.created_at.timestamp(),
                    nanos: domain_details.created_at.timestamp_nanos_opt().unwrap_or_default() as i32,
                }),
            }
        }
    
        async fn list_user_skills(
            &self,
            request: Request<ListUserSkillsRequest>,
        ) -> Result<Response<ListUserSkillsResponse>, Status> {
            let req = request.into_inner();
    
            let user_id = Uuid::from_str(&req.user_id)
                .map_err(|_| Status::invalid_argument("Invalid user_id format"))?;
    
            let user_skills = self
                .user_skill_service
                .list_user_skills(user_id)
                .await
                .map_err(map_user_skill_error)?;
    
            let proto_user_skills = user_skills
                .into_iter()
                .map(domain_user_skill_details_to_proto)
                .collect();
    
            let response = ListUserSkillsResponse {
                user_skills: proto_user_skills,
            };
    
            Ok(Response::new(response))
        }
    
        async fn remove_user_skill(
            &self,
            request: Request<RemoveUserSkillRequest>,
        ) -> Result<Response<RemoveUserSkillResponse>, Status> {
            let req = request.into_inner();
    
            let user_id = Uuid::from_str(&req.user_id)
                .map_err(|_| Status::invalid_argument("Invalid user_id format"))?;
            let skill_id = Uuid::from_str(&req.skill_id)
                .map_err(|_| Status::invalid_argument("Invalid skill_id format"))?;
    
            self.user_skill_service
                .remove_user_skill(user_id, skill_id)
                .await
                .map_err(map_user_skill_error)?;
    
            let response = RemoveUserSkillResponse { success: true };
    
                    Ok(Response::new(response))
                }
            
                async fn list_skills(
                    &self,
                    request: Request<ListSkillsRequest>,
                ) -> Result<Response<ListSkillsResponse>, Status> {
                    let req = request.into_inner();
            
                    let paginated_result = self
                        .skill_service
                        .list_skills(req.category, req.limit, req.offset)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?;
            
                    let proto_skills = paginated_result
                        .skills
                        .into_iter()
                        .map(domain_skill_to_proto)
                        .collect();
            
                    let response = ListSkillsResponse {
                        skills: proto_skills,
                        total_count: paginated_result.total_count as i32,
                    };
            
                    Ok(Response::new(response))
                }
                
                async fn record_endorsement(
                    &self,
                    request: Request<RecordEndorsementRequest>,
                ) -> Result<Response<RecordEndorsementResponse>, Status> {
                    let req = request.into_inner();
                    
                    let opportunity_id = Uuid::from_str(&req.opportunity_id)
                        .map_err(|_| Status::invalid_argument("Invalid opportunity_id format"))?;
                    
                    let skill_id = Uuid::from_str(&req.skill_id)
                        .map_err(|_| Status::invalid_argument("Invalid skill_id format"))?;
                    
                    let recipient_id = Uuid::from_str(&req.recipient_id)
                        .map_err(|_| Status::invalid_argument("Invalid recipient_id format"))?;
                    
                    let rating = req.rating;
                    
                    // Get endorser ID from request metadata (authentication context)
                    let endorser_id = Uuid::from_str(
                        request.metadata()
                            .get("user_id")
                            .ok_or_else(|| Status::unauthenticated("No user ID provided"))?
                            .to_str()
                            .map_err(|_| Status::invalid_argument("Invalid user ID format"))?
                    ).map_err(|_| Status::invalid_argument("Invalid user ID format"))?;
                    
                    let endorsement = self.endorsement_service
                        .record_endorsement(
                            opportunity_id,
                            skill_id,
                            endorser_id,
                            recipient_id,
                            if req.comment.is_empty() { None } else { Some(req.comment) },
                            rating,
                        )
                        .await
                        .map_err(map_endorsement_error)?;
                    
                    let response = RecordEndorsementResponse {
                        endorsement: Some(domain_endorsement_to_proto(endorsement)),
                    };
                    
                    Ok(Response::new(response))
                }
                
                async fn get_endorsements_for_user(
                    &self,
                    request: Request<GetEndorsementsRequest>,
                ) -> Result<Response<GetEndorsementsResponse>, Status> {
                    let req = request.into_inner();
                    
                    let user_id = Uuid::from_str(&req.user_id)
                        .map_err(|_| Status::invalid_argument("Invalid user_id format"))?;
                    
                    let endorsements = self.endorsement_service
                        .get_endorsements_for_user(user_id)
                        .await
                        .map_err(map_endorsement_error)?;
                    
                    let proto_endorsements = endorsements
                        .into_iter()
                        .map(domain_endorsement_to_proto)
                        .collect();
                    
                    let response = GetEndorsementsResponse {
                        endorsements: proto_endorsements,
                    };
                    
                    Ok(Response::new(response))
                }
                
                async fn opportunities_near_me(
                    &self,
                    request: Request<OpportunitiesNearMeRequest>,
                ) -> Result<Response<OpportunitiesNearMeResponse>, Status> {
                    let req = request.into_inner();
                    
                    // TODO: Implement actual location-based filtering
                    // For now, return all opportunities as a placeholder
                    let limit = req.limit.unwrap_or(10) as i64;
                    let offset = req.offset.unwrap_or(0) as i64;
                    
                    let mut opp_service = self.opp_service.lock().await;
                    
                    let (opportunities, total_count) = opp_service
                        .list_opportunities(None, None, true, limit, offset)
                        .await
                        .map_err(map_error)?;
                    
                    let proto_opportunities = opportunities
                        .into_iter()
                        .map(domain_opp_to_proto)
                        .collect();
                    
                    let response = OpportunitiesNearMeResponse {
                        opportunities: proto_opportunities,
                        total_count: total_count as i32,
                    };
                    
                    Ok(Response::new(response))
                }
                
                async fn get_volunteering_activity(
                    &self,
                    request: Request<VolunteeringActivityRequest>,
                ) -> Result<Response<VolunteeringActivityResponse>, Status> {
                    let req = request.into_inner();
                    
                    let user_id = Uuid::from_str(&req.user_id)
                        .map_err(|_| Status::invalid_argument("Invalid user_id format"))?;
                    
                    let limit = req.limit.unwrap_or(10) as i64;
                    let offset = req.offset.unwrap_or(0) as i64;
                    
                    let mut opp_service = self.opp_service.lock().await;
                    
                    // Get opportunities where user has applications
                    let (applications, _) = opp_service
                        .list_user_applications(user_id, None, limit, offset)
                        .await
                        .map_err(map_error)?;
                    
                    // Get the opportunities for these applications
                    let mut opportunities = Vec::new();
                    for app in applications {
                        if let Ok(Some(opp)) = opp_service.get_opportunity(app.opportunity_id).await {
                            opportunities.push(opp);
                        }
                    }
                    
                    let proto_opportunities = opportunities
                        .into_iter()
                        .map(domain_opp_to_proto)
                        .collect();
                    
                    let response = VolunteeringActivityResponse {
                        opportunities: proto_opportunities,
                        total_count: proto_opportunities.len() as i32,
                    };
                    
                    Ok(Response::new(response))
                }
            }
            fn map_error(err: OpportunityServiceError) -> Status {
                match err {
                    OpportunityServiceError::InvalidInput(msg) => Status::invalid_argument(msg),
                    OpportunityServiceError::NotFound => Status::not_found("Resource not found"),
                    OpportunityServiceError::CauseNotFound => Status::not_found("The specified cause was not found"),
                    OpportunityServiceError::CauseServiceError(msg) => Status::failed_precondition(format!("Cause service error: {}", msg)),
                    OpportunityServiceError::Internal(msg) => Status::internal(format!("Internal server error: {}", msg)),
                }
            }

fn domain_app_to_proto(domain_app: crate::opportunity_management::models::OpportunityApplication) -> ProtoOpportunityApplication {
   ProtoOpportunityApplication {
       id: domain_app.id.to_string(),
       opportunity_id: domain_app.opportunity_id.to_string(),
       user_id: domain_app.user_id.to_string(),
       applied_at: Some(prost_types::Timestamp {
           seconds: domain_app.applied_at.timestamp(),
           nanos: domain_app.applied_at.timestamp_nanos_opt().unwrap_or_default() as i32,
       }),
       status: domain_app.status.to_string(),
       volunteer_hours: domain_app.volunteer_hours.map(|d| d.to_string()),
   }
}

fn domain_opp_to_proto(domain_opp: DomainVolunteerOpportunity) -> ProtoVolunteerOpportunity {
    ProtoVolunteerOpportunity {
        id: domain_opp.id.to_string(),
        cause_id: domain_opp.cause_id.to_string(),
        title: domain_opp.title,
        description: domain_opp.description,
        required_skills: domain_opp.required_skills.into_iter().map(|id| id.to_string()).collect(),
        estimated_hours: domain_opp.estimated_hours.unwrap_or(0),
        created_at: Some(prost_types::Timestamp {
            seconds: domain_opp.created_at.timestamp(),
            nanos: domain_opp.created_at.timestamp_nanos_opt().unwrap_or_default() as i32,
        }),
        deadline: domain_opp.deadline.map(|dt| prost_types::Timestamp {
            seconds: dt.timestamp(),
            nanos: dt.timestamp_nanos_opt().unwrap_or_default() as i32,
        }),
        created_by: domain_opp.created_by.to_string(),
    }
}
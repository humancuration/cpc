//! GraphQL tests for Volunteer Coordination (ADR 0008)
//! Mirrors the style used in collaborative_workspace_test.rs: use mock services,
//! build an async-graphql Schema with the resolvers, inject a user ID into context,
//! and assert GraphQL responses.
/* Refactor note: standardized schema construction via graphql::test_helpers::build_vc_schema_with_service
   to ensure VOLUNTEER_REPUTATION toggle consistency. */

use async_graphql::{Request, ID};
use async_trait::async_trait;
use chrono::{TimeZone, Utc};
use std::sync::Arc;
use uuid::Uuid;

use crate::graphql::volunteer_coordination::{
    ApplicationStatus as GqlApplicationStatus,
    ContributionKind as GqlContributionKind,
    CreateOpportunityInput, LogContributionInput, ReviewApplicationInput, SubmitApplicationInput,
    VerifyContributionInput,
    VolunteerCoordinationMutations, VolunteerCoordinationQueries,
    VolunteerApplication as GqlVolunteerApplication,
    VolunteerContribution as GqlVolunteerContribution,
    VolunteerOpportunity as GqlVolunteerOpportunity,
};
use shared_packages::volunteer_coordination::application::volunteer_service::VolunteerServiceImpl;
use shared_packages::volunteer_coordination::domain::models::*;
use shared_packages::volunteer_coordination::domain::service::*;

#[derive(Clone, Default)]
struct MockVolunteerService {
    // Store last created objects so we can return deterministic responses
    last_opportunity: Option<VolunteerOpportunity>,
    last_application: Option<VolunteerApplication>,
    last_contribution: Option<VolunteerContribution>,
}

#[async_trait]
impl VolunteerOpportunityService for MockVolunteerService {
    async fn create_opportunity(
        &self,
        org_id: Uuid,
        created_by: Uuid,
        title: String,
        description: String,
        tags: Vec<String>,
        location: Option<String>,
        starts_at: Option<chrono::DateTime<Utc>>,
        ends_at: Option<chrono::DateTime<Utc>>,
    ) -> Result<VolunteerOpportunity, VolunteerServiceError> {
        let now = Utc.timestamp_opt(0, 0).unwrap();
        let opp = VolunteerOpportunity {
            id: OpportunityId(Uuid::new_v4()),
            org_id,
            created_by,
            title,
            description,
            tags,
            status: OpportunityStatus::Draft,
            location,
            starts_at,
            ends_at,
            created_at: now,
            updated_at: now,
        };
        Ok(opp)
    }

    async fn publish_opportunity(
        &self,
        opportunity_id: OpportunityId,
        _user_id: Uuid,
    ) -> Result<VolunteerOpportunity, VolunteerServiceError> {
        let now = Utc::now();
        Ok(VolunteerOpportunity {
            id: opportunity_id,
            org_id: Uuid::nil(),
            created_by: Uuid::nil(),
            title: "Published".into(),
            description: "desc".into(),
            tags: vec!["rust".into()],
            status: OpportunityStatus::Published,
            location: None,
            starts_at: None,
            ends_at: None,
            created_at: now,
            updated_at: now,
        })
    }

    async fn close_opportunity(
        &self,
        opportunity_id: OpportunityId,
        _user_id: Uuid,
    ) -> Result<VolunteerOpportunity, VolunteerServiceError> {
        let now = Utc::now();
        Ok(VolunteerOpportunity {
            id: opportunity_id,
            org_id: Uuid::nil(),
            created_by: Uuid::nil(),
            title: "Closed".into(),
            description: "desc".into(),
            tags: vec![],
            status: OpportunityStatus::Closed,
            location: None,
            starts_at: None,
            ends_at: None,
            created_at: now,
            updated_at: now,
        })
    }

    async fn get_opportunity(
        &self,
        opportunity_id: OpportunityId,
    ) -> Result<Option<VolunteerOpportunity>, VolunteerServiceError> {
        let now = Utc::now();
        Ok(Some(VolunteerOpportunity {
            id: opportunity_id,
            org_id: Uuid::nil(),
            created_by: Uuid::nil(),
            title: "Fetched".into(),
            description: "desc".into(),
            tags: vec!["tag".into()],
            status: OpportunityStatus::Draft,
            location: None,
            starts_at: None,
            ends_at: None,
            created_at: now,
            updated_at: now,
        }))
    }
}

#[async_trait]
impl VolunteerApplicationService for MockVolunteerService {
    async fn submit_application(
        &self,
        opportunity_id: OpportunityId,
        applicant_id: Uuid,
        motivation: Option<String>,
    ) -> Result<VolunteerApplication, VolunteerServiceError> {
        Ok(VolunteerApplication {
            id: ApplicationId(Uuid::new_v4()),
            opportunity_id,
            applicant_id,
            motivation,
            status: ApplicationStatus::Submitted,
            submitted_at: Utc::now(),
            decided_at: None,
            reviewer_id: None,
        })
    }

    async fn review_application(
        &self,
        application_id: ApplicationId,
        reviewer_id: Uuid,
        status: ApplicationStatus,
    ) -> Result<VolunteerApplication, VolunteerServiceError> {
        Ok(VolunteerApplication {
            id: application_id,
            opportunity_id: OpportunityId(Uuid::new_v4()),
            applicant_id: Uuid::new_v4(),
            motivation: Some("ok".into()),
            status,
            submitted_at: Utc::now(),
            decided_at: Some(Utc::now()),
            reviewer_id: Some(reviewer_id),
        })
    }

    async fn get_application(
        &self,
        application_id: ApplicationId,
    ) -> Result<Option<VolunteerApplication>, VolunteerServiceError> {
        Ok(Some(VolunteerApplication {
            id: application_id,
            opportunity_id: OpportunityId(Uuid::new_v4()),
            applicant_id: Uuid::new_v4(),
            motivation: None,
            status: ApplicationStatus::Submitted,
            submitted_at: Utc::now(),
            decided_at: None,
            reviewer_id: None,
        }))
    }
}

#[async_trait]
impl VolunteerContributionService for MockVolunteerService {
    async fn log_contribution(
        &self,
        opportunity_id: OpportunityId,
        contributor_id: Uuid,
        kind: ContributionKind,
        amount: f32,
        notes: Option<String>,
        occurred_at: chrono::DateTime<Utc>,
    ) -> Result<VolunteerContribution, VolunteerServiceError> {
        Ok(VolunteerContribution {
            id: ContributionId(Uuid::new_v4()),
            opportunity_id,
            contributor_id,
            kind,
            amount,
            notes,
            occurred_at,
            created_at: Utc::now(),
            verified: false,
            verification_ref: None,
        })
    }

    async fn verify_contribution(
        &self,
        contribution_id: ContributionId,
        _verifier_id: Uuid,
        verification_ref: Option<Uuid>,
    ) -> Result<VolunteerContribution, VolunteerServiceError> {
        Ok(VolunteerContribution {
            id: contribution_id,
            opportunity_id: OpportunityId(Uuid::new_v4()),
            contributor_id: Uuid::new_v4(),
            kind: ContributionKind::Hours,
            amount: 1.5,
            notes: Some("notes".into()),
            occurred_at: Utc::now(),
            created_at: Utc::now(),
            verified: verification_ref.is_some(),
            verification_ref,
        })
    }

    async fn get_contribution(
        &self,
        contribution_id: ContributionId,
    ) -> Result<Option<VolunteerContribution>, VolunteerServiceError> {
        Ok(Some(VolunteerContribution {
            id: contribution_id,
            opportunity_id: OpportunityId(Uuid::new_v4()),
            contributor_id: Uuid::new_v4(),
            kind: ContributionKind::Donation,
            amount: 100.0,
            notes: None,
            occurred_at: Utc::now(),
            created_at: Utc::now(),
            verified: false,
            verification_ref: None,
        }))
    }
/*
Refactor note: standardized schema construction via graphql::test_helpers::build_vc_schema_with_service to ensure VOLUNTEER_REPUTATION toggle consistency.
Previous manual schema wiring retained here for reference:

type TestSchema = Schema<VolunteerCoordinationQueries, VolunteerCoordinationMutations, EmptySubscription>;
fn build_test_schema(user_id: Uuid) -> TestSchema { /* ... old manual wiring ... */ }

*/

type TestSchema = crate::graphql::test_helpers::VcSchema;

fn build_test_schema(user_id: Uuid) -> TestSchema {
    use crate::graphql::test_helpers::build_vc_schema_with_service;
    use shared_packages::volunteer_coordination::domain::repository::*;
    // Minimal in-memory repos implementing required traits. Send+Sync satisfied by trait bounds; these ZSTs are thread-safe.
    struct MemOppRepo;
    #[async_trait::async_trait]
    impl OpportunityRepository for MemOppRepo {
        async fn insert(&self, _o: &VolunteerOpportunity) -> Result<(), VolunteerRepositoryError> { Ok(()) }
        async fn update_status(&self, _id: OpportunityId, _s: OpportunityStatus) -> Result<(), VolunteerRepositoryError> { Ok(()) }
        async fn get(&self, id: OpportunityId) -> Result<Option<VolunteerOpportunity>, VolunteerRepositoryError> {
            Ok(Some(VolunteerOpportunity {
                id,
                org_id: uuid::Uuid::nil(),
                created_by: uuid::Uuid::nil(),
                title: "Mem".into(),
                description: "mem".into(),
                tags: vec![],
                status: OpportunityStatus::Published,
                location: None,
                starts_at: None,
                ends_at: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }))
        }
    }
    struct MemAppRepo;
    #[async_trait::async_trait]
    impl ApplicationRepository for MemAppRepo {
        async fn insert(&self, _a: &VolunteerApplication) -> Result<(), VolunteerRepositoryError> { Ok(()) }
        async fn update_status(&self, _id: ApplicationId, _s: ApplicationStatus, _r: Option<uuid::Uuid>, _d: Option<chrono::DateTime<chrono::Utc>>) -> Result<(), VolunteerRepositoryError> { Ok(()) }
        async fn get(&self, id: ApplicationId) -> Result<Option<VolunteerApplication>, VolunteerRepositoryError> {
            Ok(Some(VolunteerApplication {
                id,
                opportunity_id: OpportunityId(uuid::Uuid::new_v4()),
                applicant_id: uuid::Uuid::new_v4(),
                motivation: None,
                status: ApplicationStatus::Submitted,
                submitted_at: chrono::Utc::now(),
                decided_at: None,
                reviewer_id: None,
            }))
        }
    }
    struct MemContribRepo;
    #[async_trait::async_trait]
    impl ContributionRepository for MemContribRepo {
        async fn insert(&self, _c: &VolunteerContribution) -> Result<(), VolunteerRepositoryError> { Ok(()) }
        async fn verify(&self, _id: ContributionId, _v: bool, _r: Option<uuid::Uuid>) -> Result<(), VolunteerRepositoryError> { Ok(()) }
        async fn get(&self, id: ContributionId) -> Result<Option<VolunteerContribution>, VolunteerRepositoryError> {
            Ok(Some(VolunteerContribution {
                id,
                opportunity_id: OpportunityId(uuid::Uuid::new_v4()),
                contributor_id: uuid::Uuid::new_v4(),
                kind: ContributionKind::Hours,
                amount: 1.0,
                notes: None,
                occurred_at: chrono::Utc::now(),
                created_at: chrono::Utc::now(),
                verified: false,
                verification_ref: None,
            }))
        }
    }

    let opp_repo: std::sync::Arc<dyn OpportunityRepository> = std::sync::Arc::new(MemOppRepo);
    let app_repo: std::sync::Arc<dyn ApplicationRepository> = std::sync::Arc::new(MemAppRepo);
    let contrib_repo: std::sync::Arc<dyn ContributionRepository> = std::sync::Arc::new(MemContribRepo);
    build_vc_schema_with_service(user_id, opp_repo, app_repo, contrib_repo)
}
    .finish()
}

#[tokio::test]
async fn opportunity_flow_create_publish_close() {
    let user = Uuid::new_v4();
    let schema = build_test_schema(user);

    // create
    let req = Request::new(
        r#"
        mutation Create($input: CreateOpportunityInput!){
          createOpportunity(input: $input){
            id orgId title description skillsNeeded status createdBy
          }
        }
        "#
    )
    .variables(async_graphql::Variables::from_json(serde_json::json!({
        "input": {
            "orgId": Uuid::new_v4().to_string(),
            "title": "Help Build CPC",
            "description": "Volunteer with Rust skills",
            "skillsNeeded": ["Rust","GraphQL"],
            "location": "Remote",
            "startsAt": null,
            "endsAt": null
        }
    })).unwrap());

    let resp = schema.execute(req).await;
    assert!(resp.errors.is_empty(), "errors: {:?}", resp.errors);
    let data = resp.data.into_json().unwrap();
    let opp_id = data["createOpportunity"]["id"].as_str().unwrap().to_string();

    // publish
    let pub_req = Request::new(
        r#"
        mutation Pub($id: ID!){
          publishOpportunity(id: $id){ id status }
        }
        "#
    ).variables(async_graphql::Variables::from_json(serde_json::json!({ "id": opp_id })).unwrap());
    let resp2 = schema.execute(pub_req).await;
    assert!(resp2.errors.is_empty(), "errors: {:?}", resp2.errors);

    // close
    let close_req = Request::new(
        r#"
        mutation Close($id: ID!){
          closeOpportunity(id: $id){ id status }
        }
        "#
    ).variables(async_graphql::Variables::from_json(serde_json::json!({ "id": opp_id })).unwrap());
    let resp3 = schema.execute(close_req).await;
    assert!(resp3.errors.is_empty(), "errors: {:?}", resp3.errors);
}

#[tokio::test]
async fn application_and_contribution_flow() {
    let user = Uuid::new_v4();
    let schema = build_test_schema(user);

    let opp_id = Uuid::new_v4().to_string();

    // submit application
    let submit = Request::new(
        r#"
        mutation Submit($input: SubmitApplicationInput!){
          submitApplication(input: $input){
            id opportunityId applicantId status createdAt
          }
        }
        "#
    ).variables(async_graphql::Variables::from_json(serde_json::json!({
        "input": { "opportunityId": opp_id, "message": "I'd love to help" }
    })).unwrap());
    let resp = schema.execute(submit).await;
    assert!(resp.errors.is_empty(), "errors: {:?}", resp.errors);
    let app_id = resp.data.into_json().unwrap()["submitApplication"]["id"].as_str().unwrap().to_string();

    // review application
    let review = Request::new(
        r#"
        mutation Review($input: ReviewApplicationInput!){
          reviewApplication(input: $input){ id status reviewedBy reviewedAt }
        }
        "#
    ).variables(async_graphql::Variables::from_json(serde_json::json!({
        "input": { "applicationId": app_id, "status": "ACCEPTED" }
    })).unwrap());
    let resp2 = schema.execute(review).await;
    assert!(resp2.errors.is_empty(), "errors: {:?}", resp2.errors);

    // log contribution
    let logc = Request::new(
        r#"
        mutation Log($input: LogContributionInput!){
          logContribution(input: $input){
            id opportunityId contributorId kind hours notes verified createdAt
          }
        }
        "#
    ).variables(async_graphql::Variables::from_json(serde_json::json!({
        "input": {
          "opportunityId": opp_id,
          "kind": "HOURS",
          "amount": 2.5,
          "notes": "Pair programming",
          "occurredAt": "1970-01-01T00:00:00Z"
        }
    })).unwrap());
    let resp3 = schema.execute(logc).await;
    assert!(resp3.errors.is_empty(), "errors: {:?}", resp3.errors);
    let contrib_id = resp3.data.into_json().unwrap()["logContribution"]["id"].as_str().unwrap().to_string();

    // verify contribution
    let verify = Request::new(
        r#"
        mutation Verify($input: VerifyContributionInput!){
          verifyContribution(input: $input){
            id verified verifiedBy verificationRef
          }
        }
        "#
    ).variables(async_graphql::Variables::from_json(serde_json::json!({
        "input": { "contributionId": contrib_id, "verificationRef": null }
    })).unwrap());
    let resp4 = schema.execute(verify).await;
    assert!(resp4.errors.is_empty(), "errors: {:?}", resp4.errors);
    let data4 = resp4.data.into_json().unwrap();
    let vb = data4["verifyContribution"]["verifiedBy"].as_str();
    let vref = data4["verifyContribution"]["verificationRef"].as_str();
    assert_eq!(vb, vref, "verificationRef should alias verifiedBy");
}

/// Focused tests exercising the ReputationStub via composition helper.
#[tokio::test]
async fn reputation_stub_verification_behavior() {
    use crate::graphql::test_helpers::build_vc_schema_with_service;
    use shared_packages::volunteer_coordination::domain::repository::*;
    use shared_packages::volunteer_coordination::domain::models as dm;

    // Enable stub
    std::env::set_var("VOLUNTEER_REPUTATION", "stub");

    // In-memory repositories to drive service and store last written verification
    #[derive(Default)]
    struct MemOpp;
    #[async_trait::async_trait]
    impl OpportunityRepository for MemOpp {
        async fn insert(&self, _o: &dm::VolunteerOpportunity) -> Result<(), VolunteerRepositoryError> { Ok(()) }
        async fn update_status(&self, _id: dm::OpportunityId, _s: dm::OpportunityStatus) -> Result<(), VolunteerRepositoryError> { Ok(()) }
        async fn get(&self, id: dm::OpportunityId) -> Result<Option<dm::VolunteerOpportunity>, VolunteerRepositoryError> {
            Ok(Some(dm::VolunteerOpportunity {
                id,
                org_id: uuid::Uuid::new_v4(),
                created_by: uuid::Uuid::new_v4(),
                title: "t".into(),
                description: "d".into(),
                tags: vec![],
                status: dm::OpportunityStatus::Published,
                location: None,
                starts_at: None,
                ends_at: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }))
        }
    }

    #[derive(Default)]
    struct MemApp;
    #[async_trait::async_trait]
    impl ApplicationRepository for MemApp {
        async fn insert(&self, _a: &dm::VolunteerApplication) -> Result<(), VolunteerRepositoryError> { Ok(()) }
        async fn update_status(&self, _id: dm::ApplicationId, _s: dm::ApplicationStatus, _r: Option<uuid::Uuid>, _d: Option<chrono::DateTime<chrono::Utc>>) -> Result<(), VolunteerRepositoryError> { Ok(()) }
        async fn get(&self, _id: dm::ApplicationId) -> Result<Option<dm::VolunteerApplication>, VolunteerRepositoryError> {
            Ok(Some(dm::VolunteerApplication {
                id: dm::ApplicationId(uuid::Uuid::new_v4()),
                opportunity_id: dm::OpportunityId(uuid::Uuid::new_v4()),
                applicant_id: uuid::Uuid::new_v4(),
                motivation: None,
                status: dm::ApplicationStatus::Submitted,
                submitted_at: chrono::Utc::now(),
                decided_at: None,
                reviewer_id: None,
            }))
        }
    }

    use std::sync::{Arc, Mutex};
    #[derive(Default)]
    struct MemContrib {
        last: Mutex<Option<dm::VolunteerContribution>>,
    }
    #[async_trait::async_trait]
    impl ContributionRepository for MemContrib {
        async fn insert(&self, c: &dm::VolunteerContribution) -> Result<(), VolunteerRepositoryError> {
            *self.last.lock().unwrap() = Some(c.clone());
            Ok(())
        }
        async fn verify(&self, id: dm::ContributionId, v: bool, r: Option<uuid::Uuid>) -> Result<(), VolunteerRepositoryError> {
            if let Some(mut existing) = self.last.lock().unwrap().clone() {
                if existing.id == id {
                    existing.verified = v;
                    existing.verification_ref = r;
                    *self.last.lock().unwrap() = Some(existing);
                }
            }
            Ok(())
        }
        async fn get(&self, id: dm::ContributionId) -> Result<Option<dm::VolunteerContribution>, VolunteerRepositoryError> {
            if let Some(c) = self.last.lock().unwrap().clone() {
                if c.id == id { return Ok(Some(c)); }
            }
            // Default a simple record if not present
            Ok(Some(dm::VolunteerContribution {
                id,
                opportunity_id: dm::OpportunityId(uuid::Uuid::new_v4()),
                contributor_id: uuid::Uuid::new_v4(),
                kind: dm::ContributionKind::Hours,
                amount: 0.0,
                notes: None,
                occurred_at: chrono::Utc::now(),
                created_at: chrono::Utc::now(),
                verified: false,
                verification_ref: None,
            }))
        }
    }

    let opp_repo: Arc<dyn OpportunityRepository> = Arc::new(MemOpp::default());
    let app_repo: Arc<dyn ApplicationRepository> = Arc::new(MemApp::default());
    let contrib_repo = Arc::new(MemContrib::default());
    let contrib_repo_trait: Arc<dyn ContributionRepository> = contrib_repo.clone();

    let user_id = uuid::Uuid::new_v4();
    let schema = build_vc_schema_with_service(user_id, opp_repo, app_repo, contrib_repo_trait);

    let opp_id = uuid::Uuid::new_v4().to_string();

    // Case A: Hours < 1.0 - expect verified=false
    let log_small = async_graphql::Request::new(r#"
        mutation($input: LogContributionInput!){
          logContribution(input: $input){ id verified verifiedBy verificationRef }
        }
    "#).variables(async_graphql::Variables::from_json(serde_json::json!({
        "input": { "opportunityId": opp_id, "kind": "HOURS", "amount": 0.5, "notes": "half hour", "occurredAt": "1970-01-01T00:00:00Z" }
    })).unwrap());
    let resp_small = schema.execute(log_small).await;
    assert!(resp_small.errors.is_empty(), "errors: {:?}", resp_small.errors);
    let cid_small = resp_small.data.into_json().unwrap()["logContribution"]["id"].as_str().unwrap().to_string();

    let verify_small = async_graphql::Request::new(r#"
        mutation($input: VerifyContributionInput!){
          verifyContribution(input: $input){ id verified verifiedBy verificationRef }
        }
    "#).variables(async_graphql::Variables::from_json(serde_json::json!({
        "input": { "contributionId": cid_small, "verificationRef": null }
    })).unwrap());
    let resp_vs = schema.execute(verify_small).await;
    assert!(resp_vs.errors.is_empty(), "errors: {:?}", resp_vs.errors);
    let data_vs = resp_vs.data.into_json().unwrap();
    assert_eq!(data_vs["verifyContribution"]["verified"].as_bool().unwrap(), false);

    // Case B: Hours ≥ 1.0 - expect verified=true
    let log_big = async_graphql::Request::new(r#"
        mutation($input: LogContributionInput!){
          logContribution(input: $input){ id }
        }
    "#).variables(async_graphql::Variables::from_json(serde_json::json!({
        "input": { "opportunityId": opp_id, "kind": "HOURS", "amount": 1.25, "notes": "good", "occurredAt": "1970-01-01T00:00:00Z" }
    })).unwrap());
    let resp_big = schema.execute(log_big).await;
    assert!(resp_big.errors.is_empty(), "errors: {:?}", resp_big.errors);
    let cid_big = resp_big.data.into_json().unwrap()["logContribution"]["id"].as_str().unwrap().to_string();

    let verify_big = async_graphql::Request::new(r#"
        mutation($input: VerifyContributionInput!){
          verifyContribution(input: $input){ id verified verifiedBy verificationRef }
        }
    "#).variables(async_graphql::Variables::from_json(serde_json::json!({
        "input": { "contributionId": cid_big, "verificationRef": null }
    })).unwrap());
    let resp_vb = schema.execute(verify_big).await;
    assert!(resp_vb.errors.is_empty(), "errors: {:?}", resp_vb.errors);
    let data_vb = resp_vb.data.into_json().unwrap();
    assert_eq!(data_vb["verifyContribution"]["verified"].as_bool().unwrap(), true);
    // alias check
    assert_eq!(
        data_vb["verifyContribution"]["verifiedBy"],
        data_vb["verifyContribution"]["verificationRef"],
        "verificationRef should alias verifiedBy"
    );

    // Case C: Deliverable - expect verified=true
    let log_deliv = async_graphql::Request::new(r#"
        mutation($input: LogContributionInput!){
          logContribution(input: $input){ id }
        }
    "#).variables(async_graphql::Variables::from_json(serde_json::json!({
        "input": { "opportunityId": opp_id, "kind": "DELIVERABLE", "amount": 1.0, "notes": "deliver", "occurredAt": "1970-01-01T00:00:00Z" }
    })).unwrap());
    let resp_deliv = schema.execute(log_deliv).await;
    assert!(resp_deliv.errors.is_empty(), "errors: {:?}", resp_deliv.errors);
    let cid_deliv = resp_deliv.data.into_json().unwrap()["logContribution"]["id"].as_str().unwrap().to_string();

    let verify_deliv = async_graphql::Request::new(r#"
        mutation($input: VerifyContributionInput!){
          verifyContribution(input: $input){ id verified verifiedBy verificationRef }
        }
    "#).variables(async_graphql::Variables::from_json(serde_json::json!({
        "input": { "contributionId": cid_deliv, "verificationRef": null }
    })).unwrap());
    let resp_vd = schema.execute(verify_deliv).await;
    assert!(resp_vd.errors.is_empty(), "errors: {:?}", resp_vd.errors);
    let data_vd = resp_vd.data.into_json().unwrap();
    assert_eq!(data_vd["verifyContribution"]["verified"].as_bool().unwrap(), true);

    // Cleanup env
    std::env::remove_var("VOLUNTEER_REPUTATION");
}
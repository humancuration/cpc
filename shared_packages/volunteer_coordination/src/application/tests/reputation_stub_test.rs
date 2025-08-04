//! Unit tests for reputation stub integration with VolunteerServiceImpl.

use std::sync::Arc;
use chrono::{Utc};
use uuid::Uuid;

use crate::application::reputation_port::ReputationPort;
use crate::application::volunteer_service::VolunteerServiceImpl;
use crate::domain::models::*;
use crate::domain::repository::*;
use crate::infrastructure::reputation_stub::ReputationStub;

// In-memory fakes for repositories
#[derive(Default, Clone)]
struct InMemoryOppRepo {
    store: parking_lot::RwLock<std::collections::HashMap<Uuid, VolunteerOpportunity>>,
}
#[async_trait::async_trait]
impl OpportunityRepository for InMemoryOppRepo {
    async fn insert(&self, o: &VolunteerOpportunity) -> Result<(), VolunteerRepositoryError> {
        self.store.write().insert(o.id.0, o.clone());
        Ok(())
    }
    async fn update_status(&self, id: OpportunityId, status: OpportunityStatus) -> Result<(), VolunteerRepositoryError> {
        if let Some(mut opp) = self.store.write().get_mut(&id.0) {
            opp.status = status;
            opp.updated_at = Utc::now();
            Ok(())
        } else {
            Err(VolunteerRepositoryError::NotFound)
        }
    }
    async fn get(&self, id: OpportunityId) -> Result<Option<VolunteerOpportunity>, VolunteerRepositoryError> {
        Ok(self.store.read().get(&id.0).cloned())
    }
}

#[derive(Default, Clone)]
struct InMemoryAppRepo {
    store: parking_lot::RwLock<std::collections::HashMap<Uuid, VolunteerApplication>>,
}
#[async_trait::async_trait]
impl ApplicationRepository for InMemoryAppRepo {
    async fn insert(&self, a: &VolunteerApplication) -> Result<(), VolunteerRepositoryError> {
        self.store.write().insert(a.id.0, a.clone());
        Ok(())
    }
    async fn update_status(
        &self,
        id: ApplicationId,
        status: ApplicationStatus,
        reviewer_id: Option<Uuid>,
        decided_at: Option<chrono::DateTime<Utc>>,
    ) -> Result<(), VolunteerRepositoryError> {
        if let Some(mut app) = self.store.write().get_mut(&id.0) {
            app.status = status;
            app.reviewer_id = reviewer_id;
            app.decided_at = decided_at;
            Ok(())
        } else {
            Err(VolunteerRepositoryError::NotFound)
        }
    }
    async fn get(&self, id: ApplicationId) -> Result<Option<VolunteerApplication>, VolunteerRepositoryError> {
        Ok(self.store.read().get(&id.0).cloned())
    }
}

#[derive(Default, Clone)]
struct InMemoryContribRepo {
    store: parking_lot::RwLock<std::collections::HashMap<Uuid, VolunteerContribution>>,
    // Simple event collector flag to indicate we "published" something; here we just record updates.
    updates: parking_lot::RwLock<Vec<(Uuid, bool, Option<Uuid>)>>,
}
#[async_trait::async_trait]
impl ContributionRepository for InMemoryContribRepo {
    async fn insert(&self, c: &VolunteerContribution) -> Result<(), VolunteerRepositoryError> {
        self.store.write().insert(c.id.0, c.clone());
        Ok(())
    }
    async fn verify(
        &self,
        id: ContributionId,
        verified: bool,
        verification_ref: Option<Uuid>,
    ) -> Result<(), VolunteerRepositoryError> {
        if let Some(mut existing) = self.store.write().get_mut(&id.0) {
            existing.verified = verified;
            existing.verification_ref = verification_ref;
            self.updates.write().push((id.0, verified, verification_ref));
            Ok(())
        } else {
            Err(VolunteerRepositoryError::NotFound)
        }
    }
    async fn get(&self, id: ContributionId) -> Result<Option<VolunteerContribution>, VolunteerRepositoryError> {
        Ok(self.store.read().get(&id.0).cloned())
    }
}

fn make_service_with_stub() -> VolunteerServiceImpl {
    let opp = Arc::new(InMemoryOppRepo::default());
    let app = Arc::new(InMemoryAppRepo::default());
    let contrib = Arc::new(InMemoryContribRepo::default());
    let rep: Arc<dyn ReputationPort + Send + Sync> = Arc::new(ReputationStub::new());
    VolunteerServiceImpl::new(opp, app, contrib, Some(rep))
}

fn seed_published_opportunity(service: &VolunteerServiceImpl) -> OpportunityId {
    let org_id = Uuid::new_v4();
    let created_by = Uuid::new_v4();
    let opp = VolunteerOpportunity {
        id: OpportunityId(Uuid::new_v4()),
        org_id,
        created_by,
        title: "Test".into(),
        description: "Desc".into(),
        tags: vec![],
        status: OpportunityStatus::Published,
        location: None,
        starts_at: None,
        ends_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    // insert directly into repo
    futures::executor::block_on(service.opportunities.insert(&opp)).unwrap();
    opp.id
}

#[test]
fn hours_less_than_one_should_not_verify() {
    let service = make_service_with_stub();
    let opp_id = seed_published_opportunity(&service);
    let contributor = Uuid::new_v4();

    // Log contribution
    let c = futures::executor::block_on(service.log_contribution(
        opp_id,
        contributor,
        ContributionKind::Hours,
        0.5,
        None,
        Utc::now(),
    )).unwrap();

    // Verify
    let out = futures::executor::block_on(service.verify_contribution(
        c.id,
        Uuid::new_v4(),
        None,
    )).unwrap();

    assert_eq!(out.verified, false);

    // Ensure persisted state updated
    let fetched = futures::executor::block_on(service.get_contribution(c.id)).unwrap().unwrap();
    assert_eq!(fetched.verified, false);
}

#[test]
fn hours_greater_or_equal_one_should_verify() {
    let service = make_service_with_stub();
    let opp_id = seed_published_opportunity(&service);
    let contributor = Uuid::new_v4();

    let c = futures::executor::block_on(service.log_contribution(
        opp_id,
        contributor,
        ContributionKind::Hours,
        1.0,
        None,
        Utc::now(),
    )).unwrap();

    let out = futures::executor::block_on(service.verify_contribution(
        c.id,
        Uuid::new_v4(),
        None,
    )).unwrap();

    assert_eq!(out.verified, true);

    let fetched = futures::executor::block_on(service.get_contribution(c.id)).unwrap().unwrap();
    assert_eq!(fetched.verified, true);
}

#[test]
fn deliverable_should_verify() {
    let service = make_service_with_stub();
    let opp_id = seed_published_opportunity(&service);
    let contributor = Uuid::new_v4();

    let c = futures::executor::block_on(service.log_contribution(
        opp_id,
        contributor,
        ContributionKind::Deliverable,
        1.0,
        None,
        Utc::now(),
    )).unwrap();

    let out = futures::executor::block_on(service.verify_contribution(
        c.id,
        Uuid::new_v4(),
        None,
    )).unwrap();

    assert_eq!(out.verified, true);

    let fetched = futures::executor::block_on(service.get_contribution(c.id)).unwrap().unwrap();
    assert_eq!(fetched.verified, true);
}
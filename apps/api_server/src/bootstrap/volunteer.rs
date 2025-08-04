use std::sync::Arc;
use tracing::info;

use shared_packages::volunteer_coordination::application::volunteer_service::VolunteerServiceImpl;
use shared_packages::volunteer_coordination::domain::repository::{
    ApplicationRepository, ContributionRepository, OpportunityRepository,
};

/// Build VolunteerServiceImpl with optional reputation stub, toggled by env:
/// VOLUNTEER_REPUTATION=stub
pub fn build_volunteer_service(
    opp_repo: Arc<dyn OpportunityRepository>,
    app_repo: Arc<dyn ApplicationRepository>,
    contrib_repo: Arc<dyn ContributionRepository>,
) -> Arc<VolunteerServiceImpl> {
    let use_stub = std::env::var("VOLUNTEER_REPUTATION").ok().as_deref() == Some("stub");
    let reputation = if use_stub {
        info!("Volunteer reputation stub enabled via VOLUNTEER_REPUTATION=stub");
        Some(shared_packages::volunteer_coordination::reputation_stub())
    } else {
        info!("Volunteer reputation integration disabled (no VOLUNTEER_REPUTATION=stub)");
        None
    };
    Arc::new(VolunteerServiceImpl::new(opp_repo, app_repo, contrib_repo, reputation))
}
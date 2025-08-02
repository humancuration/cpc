//! Integration tests for the cooperative fundraising system
//!
//! These tests use a per-test schema and run full SQLx migrations.
//! Each test runs inside a transaction and rolls back at the end for isolation.

use cooperative_fundraising::domain::{
    Campaign, CampaignType, CampaignStatus, MembershipRequirements, DonationDetails, Membership, Contribution
};
use cooperative_fundraising::application::{CampaignService, ApplicationError};
use cooperative_fundraising::infrastructure::postgres::{
    PostgresCampaignRepository, PostgresContributionRepository, PostgresMembershipRepository,
};
use rust_decimal::Decimal;
use uuid::Uuid;

use super::test_setup::{test_pool_with_schema, TestTx};

// Helpers to create entities
fn sample_membership_campaign(owner: Uuid) -> Campaign {
    Campaign::new_membership_campaign(
        "Membership Drive".to_string(),
        "Join our co-op".to_string(),
        owner,
        MembershipRequirements {
            max_participants: None,
            required_actions: vec![],
        },
    )
}

fn sample_donation_campaign(owner: Uuid) -> Campaign {
    Campaign::new_donation_campaign(
        CampaignType::PureDonation,
        "Donation Drive".to_string(),
        "Help external cause".to_string(),
        owner,
        DonationDetails {
            funding_goal: Some(Decimal::new(5000, 0)),
            external_use_case: "Community garden build".to_string(),
            currency: "USD".to_string(),
        },
    )
}

#[tokio::test]
async fn membership_uniqueness_constraint_db_and_error() {
    let pool = test_pool_with_schema().await;
    let mut tx = TestTx::new(&pool).await;

    let campaign_repo = PostgresCampaignRepository::new(pool.clone());
    let membership_repo = PostgresMembershipRepository::new(pool.clone());

    // Create two membership campaigns
    let owner = Uuid::new_v4();
    let mut c1 = sample_membership_campaign(owner);
    let mut c2 = sample_membership_campaign(owner);
    // Persist campaigns
    campaign_repo.save(&c1).await.expect("save c1");
    campaign_repo.save(&c2).await.expect("save c2");

    let user = Uuid::new_v4();
    let m1 = Membership::new(user, c1.id);
    membership_repo.save(&m1).await.expect("first membership should succeed");

    // Attempt to add same user to another membership campaign
    let m2 = Membership::new(user, c2.id);
    let err = membership_repo.save(&m2).await.err().expect("should fail");

    // Should bubble as ApplicationError::RepositoryError(sqlx::Error::Database)
    let msg = format!("{err}");
    assert!(
        msg.contains("User already has a membership share"),
        "expected DB trigger message, got: {msg}"
    );

    // Rollback transaction
    tx.rollback().await;
}

#[tokio::test]
async fn delete_campaign_non_draft_should_fail() {
    let pool = test_pool_with_schema().await;
    let mut tx = TestTx::new(&pool).await;

    let campaign_repo = PostgresCampaignRepository::new(pool.clone());
    let contribution_repo = PostgresContributionRepository::new(pool.clone());
    let membership_repo = PostgresMembershipRepository::new(pool.clone());
    let service = CampaignService::new(
        Box::new(campaign_repo),
        Box::new(contribution_repo),
        Box::new(membership_repo),
    );

    let owner = Uuid::new_v4();
    let mut campaign = sample_donation_campaign(owner);
    // Activate campaign so status != Draft
    // Persist draft first
    service.create_campaign(campaign.clone()).await.expect("persist draft");
    campaign = service.activate_campaign(campaign.id).await.expect("activate");

    let err = service.delete_campaign(campaign.id).await.err().expect("should fail");
    match err {
        ApplicationError::ValidationError(msg) => {
            assert_eq!(msg, "Only DRAFT campaigns can be deleted");
        }
        other => panic!("Expected ValidationError, got {other:?}"),
    }

    tx.rollback().await;
}

#[tokio::test]
async fn delete_campaign_with_contributions_should_fail() {
    let pool = test_pool_with_schema().await;
    let mut tx = TestTx::new(&pool).await;

    let campaign_repo = PostgresCampaignRepository::new(pool.clone());
    let contribution_repo = PostgresContributionRepository::new(pool.clone());
    let membership_repo = PostgresMembershipRepository::new(pool.clone());
    let service = CampaignService::new(
        Box::new(campaign_repo),
        Box::new(contribution_repo),
        Box::new(membership_repo),
    );

    let owner = Uuid::new_v4();
    let campaign = sample_donation_campaign(owner);
    service.create_campaign(campaign.clone()).await.expect("create");

    // Insert a contribution for this campaign
    let contribution = Contribution::new_monetary(
        campaign.id,
        Uuid::new_v4(),
        Uuid::new_v4(),
        Decimal::new(1000, 0),
        "USD".to_string(),
    );
    let contrib_repo = PostgresContributionRepository::new(pool.clone());
    contrib_repo.save(&contribution).await.expect("save contribution");

    let err = service.delete_campaign(campaign.id).await.err().expect("should fail");
    match err {
        ApplicationError::ValidationError(msg) => {
            assert_eq!(msg, "Cannot delete campaign with contributions");
        }
        other => panic!("Expected ValidationError, got {other:?}"),
    }

    tx.rollback().await;
}

#[tokio::test]
async fn delete_campaign_with_membership_shares_should_fail() {
    let pool = test_pool_with_schema().await;
    let mut tx = TestTx::new(&pool).await;

    let campaign_repo = PostgresCampaignRepository::new(pool.clone());
    let contribution_repo = PostgresContributionRepository::new(pool.clone());
    let membership_repo = PostgresMembershipRepository::new(pool.clone());
    let service = CampaignService::new(
        Box::new(campaign_repo),
        Box::new(contribution_repo),
        Box::new(membership_repo.clone()),
    );

    let owner = Uuid::new_v4();
    let campaign = sample_membership_campaign(owner);
    service.create_campaign(campaign.clone()).await.expect("create");

    // Insert a membership share for this campaign
    let m = Membership::new(Uuid::new_v4(), campaign.id);
    membership_repo.save(&m).await.expect("save membership");

    let err = service.delete_campaign(campaign.id).await.err().expect("should fail");
    match err {
        ApplicationError::ValidationError(msg) => {
            assert_eq!(msg, "Cannot delete campaign with membership shares");
        }
        other => panic!("Expected ValidationError, got {other:?}"),
    }

    tx.rollback().await;
}

#[tokio::test]
async fn delete_valid_draft_campaign_without_dependencies_should_soft_delete() {
    let pool = test_pool_with_schema().await;
    let mut tx = TestTx::new(&pool).await;

    let campaign_repo = PostgresCampaignRepository::new(pool.clone());
    let contribution_repo = PostgresContributionRepository::new(pool.clone());
    let membership_repo = PostgresMembershipRepository::new(pool.clone());
    let service = CampaignService::new(
        Box::new(campaign_repo),
        Box::new(contribution_repo),
        Box::new(membership_repo),
    );

    let owner = Uuid::new_v4();
    let campaign = sample_donation_campaign(owner);
    service.create_campaign(campaign.clone()).await.expect("create");

    // No contributions or memberships; status is Draft
    service.delete_campaign(campaign.id).await.expect("soft delete");

    // Verify status is 'cancelled'
    let found = service.get_campaign(campaign.id).await.expect("get").expect("exists");
    assert_eq!(found.status, CampaignStatus::Cancelled);

    tx.rollback().await;
}
//! Unit tests for CampaignService create_campaign validation and transactional behavior

use cooperative_fundraising::application::CampaignService;
use cooperative_fundraising::domain::{
    Campaign, CampaignStatus, CampaignType, DonationDetails, MembershipRequirements,
};
use cooperative_fundraising::infrastructure::postgres::{
    PostgresCampaignRepository, PostgresContributionRepository, PostgresMembershipRepository,
};
use cooperative_fundraising::proto;
use rust_decimal::Decimal;
use uuid::Uuid;

use super::test_setup::{test_pool_with_schema, TestTx};

#[tokio::test]
async fn create_membership_campaign_success() {
    let pool = test_pool_with_schema().await;
    let mut tx = TestTx::new(&pool).await;

    let service = CampaignService::new(
        Box::new(PostgresCampaignRepository::new(pool.clone())),
        Box::new(PostgresContributionRepository::new(pool.clone())),
        Box::new(PostgresMembershipRepository::new(pool.clone())),
    );

    let owner = Uuid::new_v4();
    let campaign = Campaign::new_membership_campaign(
        "Valid Title".to_string(),
        "A valid description with more than twenty chars".to_string(),
        owner,
        MembershipRequirements {
            max_participants: Some(10),
            required_actions: vec![],
        },
    );

    let created = service.create_campaign(campaign.clone()).await.expect("should create");
    assert_eq!(created.title, campaign.title);
    assert_eq!(created.status, CampaignStatus::Draft);
    assert!(created.membership_requirements.is_some());
    assert!(!created.is_donation_campaign());

    // Verify persisted
    let fetched = service.get_campaign(created.id).await.expect("get ok").expect("exists");
    assert_eq!(fetched.id, created.id);

    tx.rollback().await;
}

#[tokio::test]
async fn create_donation_campaign_success() {
    let pool = test_pool_with_schema().await;
    let mut tx = TestTx::new(&pool).await;

    let service = CampaignService::new(
        Box::new(PostgresCampaignRepository::new(pool.clone())),
        Box::new(PostgresContributionRepository::new(pool.clone())),
        Box::new(PostgresMembershipRepository::new(pool.clone())),
    );

    let owner = Uuid::new_v4();
    let campaign = Campaign::new_donation_campaign(
        CampaignType::PureDonation,
        "Valid Title".to_string(),
        "A valid donation description with enough length.".to_string(),
        owner,
        DonationDetails {
            funding_goal: Some(Decimal::new(10000, 2)),
            external_use_case: "Some compliant purpose".to_string(),
            currency: "USD".to_string(),
        },
    );

    let created = service.create_campaign(campaign.clone()).await.expect("should create");
    assert!(created.donation_details.is_some());
    assert!(created.is_donation_campaign());

    let fetched = service.get_campaign(created.id).await.expect("get ok").expect("exists");
    assert_eq!(fetched.id, created.id);

    tx.rollback().await;
}

#[tokio::test]
async fn fail_on_short_title() {
    let pool = test_pool_with_schema().await;
    let mut tx = TestTx::new(&pool).await;

    let service = CampaignService::new(
        Box::new(PostgresCampaignRepository::new(pool.clone())),
        Box::new(PostgresContributionRepository::new(pool.clone())),
        Box::new(PostgresMembershipRepository::new(pool.clone())),
    );

    let owner = Uuid::new_v4();
    let campaign = Campaign::new_membership_campaign(
        "abcd".to_string(), // 4 chars
        "This description is fine and long enough".to_string(),
        owner,
        MembershipRequirements {
            max_participants: Some(5),
            required_actions: vec![],
        },
    );

    let err = service.create_campaign(campaign).await.err().expect("should fail");
    let msg = format!("{err}");
    assert!(msg.contains("Invalid title"));
    assert!(msg.contains("Title must be between 5 and 100 characters"));

    tx.rollback().await;
}

#[tokio::test]
async fn fail_on_long_title() {
    let pool = test_pool_with_schema().await;
    let mut tx = TestTx::new(&pool).await;

    let service = CampaignService::new(
        Box::new(PostgresCampaignRepository::new(pool.clone())),
        Box::new(PostgresContributionRepository::new(pool.clone())),
        Box::new(PostgresMembershipRepository::new(pool.clone())),
    );

    let owner = Uuid::new_v4();
    let long_title = "x".repeat(101);
    let campaign = Campaign::new_membership_campaign(
        long_title,
        "This description is fine and long enough".to_string(),
        owner,
        MembershipRequirements {
            max_participants: Some(5),
            required_actions: vec![],
        },
    );

    let err = service.create_campaign(campaign).await.err().expect("should fail");
    let msg = format!("{err}");
    assert!(msg.contains("Invalid title"));
    assert!(msg.contains("Title must be between 5 and 100 characters"));

    tx.rollback().await;
}

#[tokio::test]
async fn fail_on_short_description() {
    let pool = test_pool_with_schema().await;
    let mut tx = TestTx::new(&pool).await;

    let service = CampaignService::new(
        Box::new(PostgresCampaignRepository::new(pool.clone())),
        Box::new(PostgresContributionRepository::new(pool.clone())),
        Box::new(PostgresMembershipRepository::new(pool.clone())),
    );

    let owner = Uuid::new_v4();
    let campaign = Campaign::new_membership_campaign(
        "Valid Title".to_string(),
        "too short".to_string(), // < 20
        owner,
        MembershipRequirements {
            max_participants: Some(5),
            required_actions: vec![],
        },
    );

    let err = service.create_campaign(campaign).await.err().expect("should fail");
    let msg = format!("{err}");
    assert!(msg.contains("Invalid description"));
    assert!(msg.contains("Description must be between 20 and 1000 characters"));

    tx.rollback().await;
}

#[tokio::test]
async fn fail_on_non_draft_status() {
    let pool = test_pool_with_schema().await;
    let mut tx = TestTx::new(&pool).await;

    let service = CampaignService::new(
        Box::new(PostgresCampaignRepository::new(pool.clone())),
        Box::new(PostgresContributionRepository::new(pool.clone())),
        Box::new(PostgresMembershipRepository::new(pool.clone())),
    );

    let owner = Uuid::new_v4();
    let mut campaign = Campaign::new_membership_campaign(
        "Valid Title".to_string(),
        "Valid description with enough length".to_string(),
        owner,
        MembershipRequirements {
            max_participants: Some(5),
            required_actions: vec![],
        },
    );
    // Force non-draft
    campaign.status = CampaignStatus::Active;

    let err = service.create_campaign(campaign).await.err().expect("should fail");
    let msg = format!("{err}");
    assert!(msg.contains("Only DRAFT status allowed"));

    tx.rollback().await;
}

#[tokio::test]
async fn fail_membership_requires_max_participants_positive() {
    let pool = test_pool_with_schema().await;
    let mut tx = TestTx::new(&pool).await;

    let service = CampaignService::new(
        Box::new(PostgresCampaignRepository::new(pool.clone())),
        Box::new(PostgresContributionRepository::new(pool.clone())),
        Box::new(PostgresMembershipRepository::new(pool.clone())),
    );

    // Case: None
    let owner = Uuid::new_v4();
    let campaign_none = Campaign::new_membership_campaign(
        "Valid Title".to_string(),
        "Valid description with enough length".to_string(),
        owner,
        MembershipRequirements {
            max_participants: None,
            required_actions: vec![],
        },
    );
    let err = service.create_campaign(campaign_none).await.err().expect("should fail");
    let msg = format!("{err}");
    assert!(msg.contains("Invalid membership requirements"));
    assert!(msg.contains("max_participants > 0"));

    // Case: Zero
    let owner2 = Uuid::new_v4();
    let campaign_zero = Campaign::new_membership_campaign(
        "Valid Title".to_string(),
        "Valid description with enough length".to_string(),
        owner2,
        MembershipRequirements {
            max_participants: Some(0),
            required_actions: vec![],
        },
    );
    let err2 = service.create_campaign(campaign_zero).await.err().expect("should fail");
    let msg2 = format!("{err2}");
    assert!(msg2.contains("Invalid membership requirements"));
    assert!(msg2.contains("max_participants > 0"));

    tx.rollback().await;
}

#[tokio::test]
async fn fail_donation_requires_positive_goal_and_use_case() {
    let pool = test_pool_with_schema().await;
    let mut tx = TestTx::new(&pool).await;

    let service = CampaignService::new(
        Box::new(PostgresCampaignRepository::new(pool.clone())),
        Box::new(PostgresContributionRepository::new(pool.clone())),
        Box::new(PostgresMembershipRepository::new(pool.clone())),
    );

    let owner = Uuid::new_v4();

    // Missing funding goal
    let campaign_missing = Campaign::new_donation_campaign(
        CampaignType::PureDonation,
        "Valid Title".to_string(),
        "Valid description with enough length".to_string(),
        owner,
        DonationDetails {
            funding_goal: None,
            external_use_case: "External use".to_string(),
            currency: "USD".to_string(),
        },
    );
    let err_missing = service.create_campaign(campaign_missing).await.err().expect("should fail");
    let msg_missing = format!("{err_missing}");
    assert!(msg_missing.contains("Invalid donation details"));
    assert!(msg_missing.contains("require a funding_goal"));

    // Non-positive funding goal
    let campaign_nonpos = Campaign::new_donation_campaign(
        CampaignType::PureDonation,
        "Valid Title".to_string(),
        "Valid description with enough length".to_string(),
        owner,
        DonationDetails {
            funding_goal: Some(Decimal::ZERO),
            external_use_case: "External use".to_string(),
            currency: "USD".to_string(),
        },
    );
    let err_nonpos = service.create_campaign(campaign_nonpos).await.err().expect("should fail");
    let msg_nonpos = format!("{err_nonpos}");
    assert!(msg_nonpos.contains("Invalid donation details"));
    assert!(msg_nonpos.contains("must be positive"));

    // Empty external use case
    let campaign_empty_use = Campaign::new_donation_campaign(
        CampaignType::PureDonation,
        "Valid Title".to_string(),
        "Valid description with enough length".to_string(),
        owner,
        DonationDetails {
            funding_goal: Some(Decimal::new(1000, 2)),
            external_use_case: "".to_string(),
            currency: "USD".to_string(),
        },
    );
    let err_empty = service.create_campaign(campaign_empty_use).await.err().expect("should fail");
    let msg_empty = format!("{err_empty}");
    assert!(msg_empty.contains("Invalid donation details"));
    assert!(msg_empty.contains("external use case"));

    tx.rollback().await;
}

// Basic check that a failed create does not leave partial rows (transaction rollback).
// Since repository save runs within a transaction and service validates first,
// we can check DB rows absence after a failure by attempting a lookup.
#[tokio::test]
async fn transaction_rollback_verification_on_failure() {
    let pool = test_pool_with_schema().await;
    let mut tx = TestTx::new(&pool).await;

    let service = CampaignService::new(
        Box::new(PostgresCampaignRepository::new(pool.clone())),
        Box::new(PostgresContributionRepository::new(pool.clone())),
        Box::new(PostgresMembershipRepository::new(pool.clone())),
    );

    let owner = Uuid::new_v4();
    // Invalid: donation missing funding goal
    let campaign = Campaign::new_donation_campaign(
        CampaignType::PureDonation,
        "Valid Title".to_string(),
        "Valid description with enough length".to_string(),
        owner,
        DonationDetails {
            funding_goal: None,
            external_use_case: "External use".to_string(),
            currency: "USD".to_string(),
        },
    );

    let id = campaign.id;
    let _ = service.create_campaign(campaign).await.expect_err("should fail");

    // Ensure not persisted
    let found = service.get_campaign(id).await.expect("get ok");
    assert!(found.is_none(), "Campaign should not exist after failed create");

    tx.rollback().await;
}
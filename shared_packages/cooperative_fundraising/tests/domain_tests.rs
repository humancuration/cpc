//! Tests for domain models

use cooperative_fundraising::domain::{
    Campaign, CampaignType, CampaignStatus, MembershipRequirements,
    DonationDetails, Membership, Contribution, VerificationStatus
};
use uuid::Uuid;
use rust_decimal::Decimal;

#[test]
fn test_membership_campaign_creation() {
    let requirements = MembershipRequirements {
        max_participants: Some(100),
        required_actions: vec!["attend_meeting".to_string(), "complete_onboarding".to_string()],
    };
    
    let campaign = Campaign::new_membership_campaign(
        "Test Membership Campaign".to_string(),
        "A test membership campaign".to_string(),
        Uuid::new_v4(),
        requirements,
    );
    
    assert_eq!(campaign.campaign_type, CampaignType::CooperativeMembership);
    assert_eq!(campaign.status, CampaignStatus::Draft);
    assert!(campaign.membership_requirements.is_some());
    assert!(campaign.donation_details.is_none());
    assert!(campaign.is_membership_campaign());
}

#[test]
fn test_donation_campaign_creation() {
    let details = DonationDetails {
        funding_goal: Some(Decimal::new(100000, 2)), // $1000.00
        external_use_case: "Supporting community projects".to_string(),
        currency: "USD".to_string(),
    };
    
    let campaign = Campaign::new_donation_campaign(
        CampaignType::PureDonation,
        "Test Donation Campaign".to_string(),
        "A test donation campaign".to_string(),
        Uuid::new_v4(),
        details,
    );
    
    assert_eq!(campaign.campaign_type, CampaignType::PureDonation);
    assert_eq!(campaign.status, CampaignStatus::Draft);
    assert!(campaign.membership_requirements.is_none());
    assert!(campaign.donation_details.is_some());
    assert!(campaign.is_donation_campaign());
}

#[test]
fn test_campaign_status_transitions() {
    let requirements = MembershipRequirements {
        max_participants: None,
        required_actions: vec![],
    };
    
    let mut campaign = Campaign::new_membership_campaign(
        "Test Campaign".to_string(),
        "A test campaign".to_string(),
        Uuid::new_v4(),
        requirements,
    );
    
    // Activate campaign
    assert!(campaign.activate().is_ok());
    assert_eq!(campaign.status, CampaignStatus::Active);
    
    // Complete campaign
    assert!(campaign.complete().is_ok());
    assert_eq!(campaign.status, CampaignStatus::Completed);
}

#[test]
fn test_membership_creation() {
    let user_id = Uuid::new_v4();
    let campaign_id = Uuid::new_v4();
    
    let membership = Membership::new(user_id, campaign_id);
    
    assert_eq!(membership.user_id, user_id);
    assert_eq!(membership.campaign_id, campaign_id);
    assert!(membership.is_active());
}

#[test]
fn test_monetary_contribution_creation() {
    let campaign_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let cpay_transaction_id = Uuid::new_v4();
    let amount = Decimal::new(10000, 2); // $100.00
    let currency = "USD".to_string();
    
    let contribution = Contribution::new_monetary(
        campaign_id,
        user_id,
        cpay_transaction_id,
        amount,
        currency,
    );
    
    assert_eq!(contribution.campaign_id, campaign_id);
    assert_eq!(contribution.user_id, user_id);
    assert_eq!(contribution.cpay_transaction_id, Some(cpay_transaction_id));
    assert_eq!(contribution.amount, Some(amount));
    assert_eq!(contribution.currency, Some("USD".to_string()));
    assert!(contribution.is_monetary());
    assert!(!contribution.is_volunteer());
}

#[test]
fn test_volunteer_contribution_creation() {
    let campaign_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let opportunity_id = Uuid::new_v4();
    let hours = 5;
    
    let contribution = Contribution::new_volunteer(
        campaign_id,
        user_id,
        opportunity_id,
        hours,
    );
    
    assert_eq!(contribution.campaign_id, campaign_id);
    assert_eq!(contribution.user_id, user_id);
    assert_eq!(contribution.opportunity_id, Some(opportunity_id));
    assert_eq!(contribution.hours, Some(hours));
    assert_eq!(contribution.verification_status, Some(VerificationStatus::Pending));
    assert!(contribution.is_volunteer());
    assert!(!contribution.is_monetary());
}

#[test]
fn test_volunteer_contribution_verification() {
    let campaign_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let opportunity_id = Uuid::new_v4();
    let hours = 5;
    
    let mut contribution = Contribution::new_volunteer(
        campaign_id,
        user_id,
        opportunity_id,
        hours,
    );
    
    // Verify the contribution
    assert!(contribution.verify().is_ok());
    assert_eq!(contribution.verification_status, Some(VerificationStatus::Verified));
    
    // Dispute the contribution
    assert!(contribution.dispute().is_ok());
    assert_eq!(contribution.verification_status, Some(VerificationStatus::Disputed));
    
    // Reject the contribution
    assert!(contribution.reject().is_ok());
    assert_eq!(contribution.verification_status, Some(VerificationStatus::Rejected));
}
//! Integration tests for the royalty distribution system
//!
//! These tests demonstrate the complete flow from content creation to royalty distribution.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use crate::finance::royalty_engine::*;
use crate::finance::transactions::{InMemoryLedger, TransactionLedger};
use crate::finance::treasury::TreasuryService;

#[tokio::test]
async fn test_simple_royalty_distribution() {
    // Setup
    let ledger = Arc::new(RwLock::new(InMemoryLedger::new()));
    let treasury = Arc::new(TreasuryService::new(ledger.clone()));
    let content_service = Arc::new(MockContentService);
    let storage = Arc::new(RwLock::new(InMemoryRoyaltyStorage::default()));
    
    let engine = RoyaltyEngine::new(
        treasury,
        content_service,
        storage,
        RoyaltyConfig {
            min_payout_amount: dec!(0.01),
            max_recursion_depth: 5,
            calculation_scale: 8,
            platform_fee_percentage: dec!(0.05), // 5%
            cooperative_treasury_percentage: dec!(0.10), // 10%
        },
    );

    // Test data
    let content_id = Uuid::new_v4();
    let revenue = dec!(100.00);
    
    // Execute distribution
    let result = engine.distribute_royalties(content_id, revenue, "USD").await;
    
    assert!(result.is_ok());
    let distribution = result.unwrap();
    
    // Verify results
    assert_eq!(distribution.total_amount, dec!(100.00));
    assert_eq!(distribution.platform_fee, dec!(5.00)); // 5% of 100
    assert_eq!(distribution.currency, "USD");
    assert_eq!(distribution.status, DistributionStatus::Completed);
}

#[test]
fn test_license_validation() {
    let mut license = ContentLicense {
        royalty_split: HashMap::new(),
        minimum_upstream_percentage: dec!(0.10),
        allows_remixing: true,
        requires_attribution: true,
    };
    
    // Valid license
    license.royalty_split.insert(Uuid::new_v4(), dec!(0.5));
    license.royalty_split.insert(Uuid::new_v4(), dec!(0.5));
    assert!(license.is_valid());
    
    // Invalid license
    license.royalty_split.clear();
    license.royalty_split.insert(Uuid::new_v4(), dec!(0.6));
    license.royalty_split.insert(Uuid::new_v4(), dec!(0.5));
    assert!(!license.is_valid());
}

#[test]
fn test_royalty_storage_operations() {
    let mut storage = InMemoryRoyaltyStorage::default();
    
    let distribution = RoyaltyDistribution {
        id: Uuid::new_v4(),
        content_id: Uuid::new_v4(),
        total_amount: dec!(1000.00),
        currency: "USD".to_string(),
        distributed_at: chrono::Utc::now(),
        distributions: HashMap::new(),
        transaction_ids: HashMap::new(),
        platform_fee: dec!(50.00),
        cooperative_treasury_amount: dec!(100.00),
        status: DistributionStatus::Completed,
    };
    
    // Test storage
    storage.store_distribution(distribution.clone()).unwrap();
    
    let retrieved = storage.get_distribution(distribution.id);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().total_amount, dec!(1000.00));
}
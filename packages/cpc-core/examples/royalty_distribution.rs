//! Example demonstrating the royalty distribution system
//!
//! This example shows how to set up and use the royalty engine
//! to distribute royalties for content revenue.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use cpc_core::finance::royalty_engine::*;
use cpc_core::finance::transactions::{InMemoryLedger, TransactionLedger};
use cpc_core::finance::treasury::TreasuryService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CPC Royalty Distribution Example ===\n");

    // Setup the royalty engine
    let ledger = Arc::new(RwLock::new(InMemoryLedger::new()));
    let treasury = Arc::new(TreasuryService::new(ledger.clone()));
    let storage = Arc::new(RwLock::new(InMemoryRoyaltyStorage::default()));
    let content_service = Arc::new(MockContentService);

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

    // Example 1: Simple royalty distribution
    println!("1. Simple Royalty Distribution");
    let content_id = Uuid::new_v4();
    let revenue = dec!(1000.00);
    
    let distribution = engine.distribute_royalties(content_id, revenue, "USD").await?;
    
    println!("   Content ID: {:?}", content_id);
    println!("   Total Revenue: ${}", revenue);
    println!("   Platform Fee: ${}", distribution.platform_fee);
    println!("   Total Distributed: ${}", distribution.distributions.values().sum::<Decimal>());
    println!("   Number of Recipients: {}", distribution.distributions.len());
    println!();

    // Example 2: Get artist royalty history
    println!("2. Artist Royalty History");
    let artist_id = *distribution.distributions.keys().next().unwrap();
    let history = engine.get_artist_royalty_history(artist_id);
    
    println!("   Artist ID: {:?}", artist_id);
    println!("   Total Distributions: {}", history.len());
    println!("   Total Revenue: ${}", 
        history.iter().map(|d| d.distributions.get(&artist_id).unwrap_or(&dec!(0))).sum::<Decimal>());
    println!();

    // Example 3: Content revenue tracking
    println!("3. Content Revenue Tracking");
    let total_revenue = engine.get_content_total_revenue(content_id);
    println!("   Content ID: {:?}", content_id);
    println!("   Total Lifetime Revenue: ${}", total_revenue);
    println!();

    Ok(())
}
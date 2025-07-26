//! Integration example for Treasury module usage
//!
//! This module demonstrates how to use the TreasuryService with the existing
//! transaction system and shows integration patterns for different use cases.

use crate::finance::{
    transactions::{InMemoryTransactionLedger, TransactionLedger},
    treasury::{ProfitDistribution, Treasury, TreasuryError, TreasuryService},
};
use rust_decimal::Decimal;
use uuid::Uuid;

/// Example integration service showing treasury usage patterns
pub struct TreasuryIntegrationExample {
    treasury_service: TreasuryService<InMemoryTransactionLedger>,
}

impl TreasuryIntegrationExample {
    /// Creates a new integration example
    pub fn new() -> Self {
        let ledger = InMemoryTransactionLedger::new();
        let treasury_service = TreasuryService::new(ledger);
        
        Self { treasury_service }
    }

    /// Example: Record ad revenue and distribute profits
    pub fn example_revenue_and_distribution(&self) -> Result<(), TreasuryError> {
        // Step 1: Record revenue from advertisements
        self.treasury_service.record_revenue(
            Decimal::from(1000),
            "USD"
        )?;
        
        println!("Revenue recorded. Treasury balance: ${}", 
            self.treasury_service.get_treasury_balance());
        
        // Step 2: Create profit distribution
        let mut distribution = ProfitDistribution::new(Decimal::from(500));
        
        // Add users to distribution
        let user1 = Uuid::new_v4();
        let user2 = Uuid::new_v4();
        
        distribution.add_user_transaction(user1, "dist_tx_1".to_string());
        distribution.add_user_transaction(user2, "dist_tx_2".to_string());
        
        // Step 3: Execute distribution
        self.treasury_service.distribute_profits(distribution)?;
        
        println!("Distribution completed. Treasury balance: ${}", 
            self.treasury_service.get_treasury_balance());
        
        Ok(())
    }

    /// Example: Get treasury analytics
    pub fn example_analytics(&self) -> Result<(), TreasuryError> {
        // Record some revenue
        self.treasury_service.record_revenue(
            Decimal::from(2000),
            "USD"
        )?;
        
        // Get analytics
        let balance = self.treasury_service.get_treasury_balance();
        let total_revenue = self.treasury_service.get_total_revenue();
        
        println!("Treasury Analytics:");
        println!("Current Balance: ${}", balance);
        println!("Total Revenue: ${}", total_revenue);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_example() {
        let example = TreasuryIntegrationExample::new();
        
        // Test basic functionality
        assert!(example.treasury_service.record_revenue(
            Decimal::from(1000),
            "USD"
        ).is_ok());
        
        assert_eq!(
            example.treasury_service.get_treasury_balance(),
            Decimal::from(1000)
        );
        
        assert_eq!(
            example.treasury_service.get_total_revenue(),
            Decimal::from(1000)
        );
    }
}
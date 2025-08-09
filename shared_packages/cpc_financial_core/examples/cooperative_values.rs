//! Example demonstrating cooperative values integration with the CPC Financial Core

use cpc_financial_core::{
    MonetaryAmount, CurrencyCode, RoundingStrategy, 
    FinancialAuditable, AuditLogEntry
};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use chrono::{Utc, DateTime};
use uuid::Uuid;

/// A cooperative transaction that embodies cooperative values
#[derive(Debug, Clone)]
pub struct CooperativeTransaction {
    pub id: Uuid,
    pub description: String,
    pub amount: MonetaryAmount,
    pub timestamp: DateTime<Utc>,
    pub cooperative_principle: String, // e.g., "Transparency", "Mutual Aid", "Democratic Control"
}

impl FinancialAuditable for CooperativeTransaction {
    fn get_description(&self) -> String {
        format!("Cooperative Transaction: {} - Principle: {}", 
                self.description, self.cooperative_principle)
    }
    
    fn get_amount(&self) -> MonetaryAmount {
        self.amount.clone()
    }
    
    fn get_timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

impl CooperativeTransaction {
    pub fn new(description: &str, amount: MonetaryAmount, principle: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            description: description.to_string(),
            amount,
            timestamp: Utc::now(),
            cooperative_principle: principle.to_string(),
        }
    }
    
    /// Demonstrates the "Transparency" cooperative principle through audit trails
    pub fn demonstrate_transparency(&self) -> Vec<AuditLogEntry> {
        let audit_entry = self.create_audit_log();
        vec![audit_entry]
    }
    
    /// Demonstrates "Mutual Aid" by showing how transactions can support community
    pub fn demonstrate_mutual_aid(
        supporter_amount: MonetaryAmount,
        recipient_amount: MonetaryAmount,
    ) -> (MonetaryAmount, MonetaryAmount) {
        // In a real cooperative system, this might involve:
        // - Recording both transactions with clear community benefit
        // - Applying special rounding for community benefit
        // - Creating audit trails showing mutual support
        
        println!("Mutual Aid Transaction:");
        println!("  Community Supporter contributes: {}", supporter_amount);
        println!("  Community Recipient receives: {}", recipient_amount);
        
        (supporter_amount, recipient_amount)
    }
    
    /// Demonstrates "Democratic Control" through transparent financial operations
    pub fn demonstrate_democratic_control(
        &self,
        member_votes: Vec<bool>, // true = approve, false = reject
    ) -> (&str, usize, usize) {
        let approvals = member_votes.iter().filter(|&&vote| vote).count();
        let rejections = member_votes.len() - approvals;
        
        let decision = if approvals > rejections {
            "APPROVED - Democratic consensus reached"
        } else {
            "REJECTED - Democratic consensus not reached"
        };
        
        println!("Democratic Control Decision:");
        println!("  Transaction: {}", self.description);
        println!("  Member votes - Approvals: {}, Rejections: {}", approvals, rejections);
        println!("  Decision: {}", decision);
        
        (decision, approvals, rejections)
    }
}

fn main() {
    println!("=== CPC Financial Core: Cooperative Values Integration ===\n");
    
    // Create a cooperative transaction embodying transparency
    let transaction = CooperativeTransaction::new(
        "Community Garden Funding",
        MonetaryAmount::new(dec!(250.75), CurrencyCode::USD),
        "Transparency"
    );
    
    println!("1. TRANSPARENCY through Audit Trails");
    let audit_logs = transaction.demonstrate_transparency();
    for log in audit_logs {
        println!("   Audit Entry: {} at {} for {}",
                 log.description, log.timestamp, log.amount);
    }
    
    println!("\n2. MUTUAL AID through Community Support");
    let supporter = MonetaryAmount::new(dec!(100.00), CurrencyCode::USD);
    let recipient = MonetaryAmount::new(dec!(100.00), CurrencyCode::USD);
    CooperativeTransaction::demonstrate_mutual_aid(supporter, recipient);
    
    println!("\n3. DEMOCRATIC CONTROL through Member Voting");
    let democratic_transaction = CooperativeTransaction::new(
        "Cooperative Office Space Rental",
        MonetaryAmount::new(dec!(1200.00), CurrencyCode::USD),
        "Democratic Control"
    );
    
    // Simulate member votes (in a real system, these would come from actual members)
    let member_votes = vec![true, true, false, true, true, false, true];
    democratic_transaction.demonstrate_democratic_control(member_votes);
    
    println!("\n4. FAIRNESS through Proper Rounding");
    let community_fund = MonetaryAmount::new(dec!(1000.456), CurrencyCode::USD);
    println!("   Community fund before rounding: {}", community_fund);
    
    // Using Banker's rounding (statistically unbiased)
    let rounded_fund = community_fund.round(RoundingStrategy::Bankers);
    println!("   Community fund after Banker's rounding: {}", rounded_fund);
    
    println!("\n5. EQUITABLE DISTRIBUTION");
    let total_community_fund = MonetaryAmount::new(dec!(5000.00), CurrencyCode::USD);
    let member_count = 12u32;
    
    if let Ok(per_member) = total_community_fund.divide(Decimal::from(member_count)) {
        println!("   Total community fund: {}", total_community_fund);
        println!("   Number of cooperative members: {}", member_count);
        println!("   Equitable distribution per member: {}", per_member);
    }
    
    println!("\n=== Cooperative Values Successfully Integrated ===");
    println!("This financial system embodies:");
    println!("  - Transparency through audit trails");
    println!("  - Mutual aid through community support mechanisms");
    println!("  - Democratic control through member voting");
    println!("  - Fairness through proper rounding strategies");
    println!("  - Equitable distribution of resources");
}
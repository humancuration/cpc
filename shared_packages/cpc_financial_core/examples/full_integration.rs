//! Full integration example showing how all CPC components work together

use cpc_financial_core::{
    MonetaryAmount, CurrencyCode, RoundingStrategy,
    audit::FinancialAuditHook
};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;
use chrono::Utc;

// Note: In a real implementation, these would be the actual crates
// For this example, we'll simulate the integration

/// Simulates a wallet service
pub struct WalletService;

impl WalletService {
    pub async fn transfer_dabloons(
        &self,
        _from_user: Uuid,
        _to_user: Uuid,
        amount: Decimal,
        _description: Option<String>,
    ) -> Result<(), String> {
        println!("Transferring {} DBL from user to user", amount);
        // In a real implementation, this would update wallet balances
        Ok(())
    }
}

/// Simulates a notification service
pub struct NotificationService;

impl NotificationService {
    pub async fn send_payment_notification(
        &self,
        user_id: Uuid,
        message: &str,
    ) -> Result<(), String> {
        println!("Sending notification to user {}: {}", user_id, message);
        Ok(())
    }
}

/// Simulates the transaction engine
pub struct TransactionEngine {
    wallet_service: WalletService,
    notification_service: NotificationService,
    audit_hook: FinancialAuditHook,
}

impl TransactionEngine {
    pub fn new(
        wallet_service: WalletService,
        notification_service: NotificationService,
        audit_hook: FinancialAuditHook,
    ) -> Self {
        Self {
            wallet_service,
            notification_service,
            audit_hook,
        }
    }
    
    pub async fn process_payment(
        &self,
        user_id: Uuid,
        recipient_id: Uuid,
        amount: MonetaryAmount,
        description: Option<String>,
    ) -> Result<Uuid, String> {
        println!("Processing payment:");
        println!("  From: {}", user_id);
        println!("  To: {}", recipient_id);
        println!("  Amount: {}", amount);
        println!("  Description: {:?}", description);
        
        // Record audit log
        let _ = self.audit_hook.record_operation(
            Some(user_id.to_string()),
            "payment_processing",
            serde_json::json!({
                "sender_id": user_id.to_string(),
                "recipient_id": recipient_id.to_string(),
                "amount": amount.value().to_string(),
                "currency": amount.currency().to_string(),
            }),
            serde_json::json!({}),
            serde_json::json!({}),
        ).await;
        
        // Process based on currency
        match amount.currency() {
            CurrencyCode::DBL => {
                self.wallet_service
                    .transfer_dabloons(
                        user_id,
                        recipient_id,
                        amount.value(),
                        description,
                    )
                    .await?;
            },
            _ => {
                // For traditional currencies, integrate with external payment providers
                println!("Processing traditional currency payment through external provider");
            }
        }
        
        // Send notification
        let notification_msg = format!(
            "You sent {} to another user",
            amount
        );
        self.notification_service
            .send_payment_notification(user_id, &notification_msg)
            .await?;
        
        let transaction_id = Uuid::new_v4();
        println!("Payment processed successfully. Transaction ID: {}", transaction_id);
        
        Ok(transaction_id)
    }
    
    pub async fn get_balance(&self, user_id: Uuid) -> Result<MonetaryAmount, String> {
        // Record audit log
        let _ = self.audit_hook.record_operation(
            Some(user_id.to_string()),
            "balance_check",
            serde_json::json!({}),
            serde_json::json!({}),
            serde_json::json!({}),
        ).await;
        
        // In a real implementation, this would fetch from the wallet service
        let balance = MonetaryAmount::new(dec!(1250.75), CurrencyCode::DBL);
        println!("User {} balance: {}", user_id, balance);
        Ok(balance)
    }
}

/// Simulates data processing with memory management
pub struct DataProcessor;

impl DataProcessor {
    pub fn process_financial_data(
        &self,
        data: Vec<MonetaryAmount>,
    ) -> Result<FinancialSummary, String> {
        println!("Processing {} financial records", data.len());
        
        // In a real implementation, this would check memory constraints
        // and use appropriate processing strategy (LoadFull, Downsample, Stream)
        
        let mut total = MonetaryAmount::new(Decimal::ZERO, CurrencyCode::DBL);
        let mut count = 0;
        
        for amount in data {
            total = total.add(&amount)
                .map_err(|e| format!("Calculation error: {}", e))?;
            count += 1;
        }
        
        let average = if count > 0 {
            total.divide(Decimal::from(count))
                .map_err(|e| format!("Division error: {}", e))?
        } else {
            MonetaryAmount::new(Decimal::ZERO, CurrencyCode::DBL)
        };
        
        Ok(FinancialSummary {
            total_value: total,
            average_value: average,
            record_count: count,
        })
    }
}

/// Summary of financial data processing
pub struct FinancialSummary {
    pub total_value: MonetaryAmount,
    pub average_value: MonetaryAmount,
    pub record_count: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CPC Full Integration Example ===\n");
    
    // Initialize services (in a real app, these would come from dependency injection)
    let wallet_service = WalletService;
    let notification_service = NotificationService;
    // Note: In a real implementation, we would need an actual AuditService implementation
    // For this example, we'll skip the audit hook initialization
    
    // Create a mock audit hook (in a real implementation, this would connect to the audit service)
    // let audit_hook = FinancialAuditHook::new(audit_service);
    
    let transaction_engine = TransactionEngine::new(
        wallet_service,
        notification_service,
        // audit_hook, // Skip for this example
        FinancialAuditHook { /* mock */ },
    );
    
    // Create user IDs
    let user1 = Uuid::new_v4();
    let user2 = Uuid::new_v4();
    
    println!("1. USER REGISTRATION AND INITIAL BALANCE");
    let initial_balance = transaction_engine.get_balance(user1).await?;
    println!("   Initial balance: {}\n", initial_balance);
    
    println!("2. PROCESSING PAYMENTS");
    
    // Process a payment in Dabloons
    let payment_amount = MonetaryAmount::new(dec!(150.50), CurrencyCode::DBL);
    let transaction_id = transaction_engine
        .process_payment(
            user1,
            user2,
            payment_amount,
            Some("Community garden donation".to_string()),
        )
        .await?;
    
    println!("   Transaction completed: {}\n", transaction_id);
    
    // Check updated balance
    let updated_balance = transaction_engine.get_balance(user1).await?;
    println!("   Updated balance: {}\n", updated_balance);
    
    println!("3. PROCESSING TRADITIONAL CURRENCY PAYMENT");
    
    // Process a payment in USD
    let usd_amount = MonetaryAmount::new(dec!(25.99), CurrencyCode::USD);
    let usd_transaction_id = transaction_engine
        .process_payment(
            user1,
            user2,
            usd_amount,
            Some("Online purchase".to_string()),
        )
        .await?;
    
    println!("   USD Transaction completed: {}\n", usd_transaction_id);
    
    println!("4. DATA PROCESSING WITH MEMORY MANAGEMENT");
    
    // Create sample financial data
    let financial_data = vec![
        MonetaryAmount::new(dec!(100.50), CurrencyCode::DBL),
        MonetaryAmount::new(dec!(75.25), CurrencyCode::DBL),
        MonetaryAmount::new(dec!(200.00), CurrencyCode::DBL),
        MonetaryAmount::new(dec!(50.75), CurrencyCode::DBL),
        MonetaryAmount::new(dec!(300.25), CurrencyCode::DBL),
    ];
    
    let data_processor = DataProcessor;
    let summary = data_processor.process_financial_data(financial_data)?;
    
    println!("   Processed {} financial records", summary.record_count);
    println!("   Total value: {}", summary.total_value);
    println!("   Average value: {}", summary.average_value);
    
    println!("\n5. ROUNDING EXAMPLES");
    
    let precise_amount = MonetaryAmount::new(dec!(123.456789), CurrencyCode::DBL);
    println!("   Original amount: {}", precise_amount);
    
    let bankers_rounded = precise_amount.round(RoundingStrategy::Bankers);
    println!("   Banker's rounded: {}", bankers_rounded);
    
    let ceiling_rounded = precise_amount.round(RoundingStrategy::Ceiling);
    println!("   Ceiling rounded: {}", ceiling_rounded);
    
    let floor_rounded = precise_amount.round(RoundingStrategy::Floor);
    println!("   Floor rounded: {}", floor_rounded);
    
    println!("\n6. CURRENCY CONVERSION (SIMULATED)");
    
    let usd_amount = MonetaryAmount::new(dec!(100.00), CurrencyCode::USD);
    let eur_amount = usd_amount.convert_to(CurrencyCode::EUR);
    println!("   {} = {}", usd_amount, eur_amount);
    
    println!("\n=== Integration Example Completed Successfully ===");
    println!("\nThis example demonstrated:");
    println!("  - Integration of financial core with payment processing");
    println!("  - Audit trail generation for all operations");
    println!("  - Multi-currency support (Dabloons and traditional currencies)");
    println!("  - Notification service integration");
    println!("  - Data processing with financial calculations");
    println!("  - Proper rounding strategies for financial accuracy");
    println!("  - Currency conversion framework");
    
    Ok(())
}

// Mock implementation to make the example compile
// In a real implementation, this would be provided by the audit_framework crate
pub struct FinancialAuditHook;

impl FinancialAuditHook {
    pub async fn record_operation(
        &self,
        _user_id: Option<String>,
        _operation_type: &str,
        _input_data: serde_json::Value,
        _parameters: serde_json::Value,
        _result: serde_json::Value,
    ) -> Result<(), String> {
        // Mock implementation - in a real system this would record to the audit trail
        Ok(())
    }
}
# Expense Tracker Module - Usage Examples

This document provides practical examples of how to use the Expense Tracker module in a real application context.

## Overview

The following examples demonstrate common usage patterns for the Expense Tracker module, showing how to initialize components, perform operations, and integrate with other services.

## 1. Basic Initialization

```rust
use cpc_core::finance::expense_tracker::bootstrap::bootstrap_expense_tracker;
use cpc_core::finance::application::wallet_service::WalletServiceImpl;
use cpc_core::finance::application::budget_service::BudgetServiceImpl;
use sqlx::PgPool;
use std::sync::Arc;

async fn initialize_expense_tracker() -> Result<(), Box<dyn std::error::Error>> {
    // Assume these are already initialized
    let db_pool: PgPool = /* database connection pool */;
    let wallet_service: Arc<dyn WalletService> = /* wallet service */;
    let budget_service: Arc<dyn BudgetService> = /* budget service */;
    let p2p_manager: Arc<cpc_net::p2p::P2PManager> = /* p2p manager */;
    let user_keys = UserKeys {
        private_key: vec![], // User's private key
        public_key: vec![],  // User's public key
    };
    
    // Bootstrap all expense tracker components
    let (expense_service, ocr_service, p2p_sharing, receipt_scanner_plugin) = 
        bootstrap_expense_tracker(
            db_pool,
            wallet_service,
            budget_service,
            p2p_manager,
            user_keys,
        ).await?;
    
    // Components are now ready to use
    Ok(())
}
```

## 2. Creating an Expense

```rust
use cpc_core::finance::domain::primitives::{Money, Currency};
use cpc_core::finance::domain::expense_tracker::{ExpenseCategory, ExpenseStatus};
use uuid::Uuid;
use chrono::Utc;
use rust_decimal_macros::dec;

async fn create_expense_example(expense_service: Arc<dyn ExpenseService>) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = Uuid::new_v4();
    let amount = Money::new(dec!(25.50), Currency::Dabloons);
    let category = ExpenseCategory::Food;
    let date = Utc::now();
    let description = "Dinner at restaurant".to_string();
    
    // Create the expense
    let expense = expense_service.create_expense(
        user_id,
        amount,
        category,
        date,
        description,
    ).await?;
    
    println!("Created expense with ID: {}", expense.id);
    println!("Expense amount: {} {}", expense.amount.amount, expense.amount.currency);
    println!("Expense category: {:?}", expense.category);
    
    Ok(())
}
```

## 3. Processing a Receipt

```rust
use cpc_core::finance::domain::expense_tracker::ReceiptImageData;

async fn process_receipt_example(
    expense_service: Arc<dyn ExpenseService>,
    ocr_service: &OCRService,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = Uuid::new_v4();
    
    // Save a receipt (in a real app, this would come from camera capture)
    let receipt = expense_service.save_receipt(
        user_id,
        ReceiptImageData::Base64Data("base64_encoded_image_data".to_string()),
    ).await?;
    
    println!("Saved receipt with ID: {}", receipt.id);
    
    // Process the receipt with OCR
    // In a real implementation, this would extract text and data
    ocr_service.process_receipt(receipt.id).await?;
    
    // Create a draft expense from the processed receipt
    let draft_expense = expense_service.create_draft_from_receipt(
        user_id,
        receipt.id,
        "Scanned receipt".to_string(),
    ).await?;
    
    println!("Created draft expense: {}", draft_expense.id);
    
    Ok(())
}
```

## 4. Managing Sharing Preferences

```rust
use cpc_core::finance::domain::expense_tracker::ExpenseCategory;

async fn manage_sharing_preferences(expense_service: Arc<dyn ExpenseService>) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = Uuid::new_v4();
    
    // Get current sharing preferences
    let preferences = expense_service.get_sharing_preferences(user_id).await?;
    println!("Current sharing enabled: {}", preferences.sharing_enabled);
    println!("Anonymization enabled: {}", preferences.anonymized);
    
    // Update sharing preferences
    let updated_preferences = expense_service.update_sharing_preferences(
        user_id,
        true,  // Enable sharing
        true,  // Enable anonymization
        vec![ExpenseCategory::Food, ExpenseCategory::Entertainment], // Share only these categories
    ).await?;
    
    println!("Updated preferences for user: {}", updated_preferences.user_id);
    
    Ok(())
}
```

## 5. Secure p2p Sharing

```rust
async fn share_expense_example(
    expense_service: Arc<dyn ExpenseService>,
    p2p_sharing: &P2PExpenseSharing,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = Uuid::new_v4();
    
    // Create an expense to share
    let amount = Money::new(dec!(15.75), Currency::Dabloons);
    let expense = expense_service.create_expense(
        user_id,
        amount,
        ExpenseCategory::Food,
        Utc::now(),
        "Lunch expense".to_string(),
    ).await?;
    
    // Share the expense with specific recipients
    let recipient_node_ids = vec![
        "node_id_1".to_string(),
        "node_id_2".to_string(),
    ];
    
    p2p_sharing.share_expense(
        expense.id,
        recipient_node_ids,
        user_id,
    ).await?;
    
    println!("Shared expense {} with recipients", expense.id);
    
    Ok(())
}
```

## 6. Querying Expenses

```rust
async fn query_expenses_example(expense_service: Arc<dyn ExpenseService>) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = Uuid::new_v4();
    
    // Get all expenses for a user
    let all_expenses = expense_service.get_user_expenses(user_id, None, None).await?;
    println!("User has {} total expenses", all_expenses.len());
    
    // Get expenses for a specific date range
    let start_date = Utc::now() - chrono::Duration::days(30);
    let end_date = Utc::now();
    
    let recent_expenses = expense_service.get_user_expenses(
        user_id,
        Some(start_date),
        Some(end_date),
    ).await?;
    
    println!("User has {} expenses in the last 30 days", recent_expenses.len());
    
    // Display expense details
    for expense in recent_expenses.iter().take(5) {
        println!(
            "Expense: {} - {} {} on {}",
            expense.description,
            expense.amount.amount,
            expense.amount.currency,
            expense.date.format("%Y-%m-%d")
        );
    }
    
    Ok(())
}
```

## 7. Updating an Expense

```rust
async fn update_expense_example(expense_service: Arc<dyn ExpenseService>) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = Uuid::new_v4();
    
    // Create an expense
    let expense = expense_service.create_expense(
        user_id,
        Money::new(dec!(10.0), Currency::Dabloons),
        ExpenseCategory::Food,
        Utc::now(),
        "Initial expense".to_string(),
    ).await?;
    
    // Update the expense
    let updated_expense = expense_service.update_expense(
        expense.id,
        Some(Money::new(dec!(15.0), Currency::Dabloons)), // New amount
        Some(ExpenseCategory::Entertainment),             // New category
        None,                                             // Keep same date
        Some("Updated expense description".to_string()),  // New description
        Some(ExpenseStatus::Verified),                    // New status
        None,                                             // Keep same budget link
    ).await?;
    
    println!("Updated expense: {}", updated_expense.id);
    println!("New amount: {} {}", updated_expense.amount.amount, updated_expense.amount.currency);
    println!("New category: {:?}", updated_expense.category);
    
    Ok(())
}
```

## 8. Linking to Budget

```rust
async fn link_to_budget_example(
    expense_service: Arc<dyn ExpenseService>,
    budget_service: Arc<dyn BudgetService>,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = Uuid::new_v4();
    
    // Create a budget (this would typically already exist)
    let budget = budget_service.create_budget(
        user_id,
        "Food".to_string(),
        Money::new(dec!(500.0), Currency::Dabloons),
        crate::domain::budget::BudgetPeriod::Monthly,
        Utc::now(),
        Utc::now() + chrono::Duration::days(30),
    ).await?;
    
    // Create an expense
    let expense = expense_service.create_expense(
        user_id,
        Money::new(dec!(25.50), Currency::Dabloons),
        ExpenseCategory::Food,
        Utc::now(),
        "Grocery shopping".to_string(),
    ).await?;
    
    // Link the expense to the budget
    expense_service.link_to_budget(expense.id, budget.id).await?;
    
    println!("Linked expense {} to budget {}", expense.id, budget.id);
    
    // The budget's spent amount should now be updated
    let updated_budget = budget_service.get_budget_by_category(user_id, "Food").await?;
    if let Some(budget) = updated_budget {
        println!("Budget spent amount: {} {}", budget.spent_amount.amount, budget.spent_amount.currency);
    }
    
    Ok(())
}
```

## 9. Working with Receipts

```rust
async fn receipt_workflow_example(
    expense_service: Arc<dyn ExpenseService>,
    ocr_service: &OCRService,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = Uuid::new_v4();
    
    // Simulate capturing a receipt image (in real app, this comes from camera)
    let image_data = ReceiptImageData::Base64Data(
        "base64_encoded_receipt_image_data".to_string()
    );
    
    // Save the receipt
    let mut receipt = expense_service.save_receipt(user_id, image_data).await?;
    println!("Saved receipt: {}", receipt.id);
    
    // Process the receipt with OCR
    ocr_service.process_receipt(receipt.id).await?;
    
    // Get the updated receipt with extracted data
    if let Some(updated_receipt) = expense_service.get_receipt(receipt.id).await? {
        receipt = updated_receipt;
        
        println!("Extracted text: {}", receipt.extracted_text);
        if let Some(merchant) = &receipt.merchant_name {
            println!("Merchant: {}", merchant);
        }
        if let Some(amount) = &receipt.total_amount {
            println!("Total amount: {} {}", amount.amount, amount.currency);
        }
        println!("Processing status: {:?}", receipt.processing_status);
    }
    
    Ok(())
}
```

## 10. Error Handling

```rust
async fn error_handling_example(expense_service: Arc<dyn ExpenseService>) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = Uuid::new_v4();
    
    // Handle potential errors when creating an expense
    match expense_service.create_expense(
        user_id,
        Money::new(dec!(-10.0), Currency::Dabloons), // Invalid negative amount
        ExpenseCategory::Food,
        Utc::now(),
        "Invalid expense".to_string(),
    ).await {
        Ok(expense) => {
            println!("Created expense: {}", expense.id);
        }
        Err(e) => {
            println!("Error creating expense: {:?}", e);
            // Handle specific error types
            match e {
                FinanceError::InvalidAmount(msg) => {
                    println!("Invalid amount error: {}", msg);
                }
                FinanceError::InsufficientFunds(currency) => {
                    println!("Insufficient funds in {}", currency);
                }
                _ => {
                    println!("Other error occurred");
                }
            }
        }
    }
    
    Ok(())
}
```

## 11. Configuration-Based Initialization

```rust
use cpc_core::finance::expense_tracker::bootstrap::{bootstrap_with_config, ExpenseTrackerConfig};

async fn conditional_initialization_example() -> Result<(), Box<dyn std::error::Error>> {
    // Assume these are already initialized
    let db_pool: PgPool = /* database connection pool */;
    let wallet_service: Arc<dyn WalletService> = /* wallet service */;
    let budget_service: Arc<dyn BudgetService> = /* budget service */;
    
    // Configure the expense tracker
    let config = ExpenseTrackerConfig {
        enable_ocr: true,
        enable_p2p_sharing: false, // Disable p2p for this instance
        enable_receipt_scanning: true,
        default_currency: Currency::USD,
    };
    
    // Bootstrap with configuration
    let (expense_service, ocr_service, p2p_sharing, receipt_scanner_plugin) = 
        bootstrap_with_config(
            db_pool,
            wallet_service,
            budget_service,
            None, // No p2p manager since p2p is disabled
            None, // No user keys since p2p is disabled
            config,
        ).await?;
    
    // Some components may be None based on configuration
    if let Some(ocr) = ocr_service {
        println!("OCR service is available");
    } else {
        println!("OCR service is disabled");
    }
    
    if p2p_sharing.is_none() {
        println!("p2p sharing is disabled");
    }
    
    Ok(())
}
```

## 12. Integration with Bevy

```rust
use cpc_core::finance::expense_tracker::infrastructure::bevy::receipt_scanner::{capture_receipt_image, display_scanner_ui};
use bevy::prelude::*;

// Bevy system for handling receipt scanning
fn handle_receipt_scanning(
    // In a real implementation, you would have Bevy resources and components here
) {
    // Display the receipt scanner UI
    display_scanner_ui();
    
    // This would typically be triggered by user input
    // For example, when the user presses a "capture" button
}

// Bevy system for processing captured receipts
async fn process_captured_receipt(
    expense_service: Arc<dyn ExpenseService>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Capture image from camera
    let image_data = capture_receipt_image()?;
    
    // Save the captured receipt
    let user_id = Uuid::new_v4(); // In real app, get from session
    let receipt = expense_service.save_receipt(user_id, image_data).await?;
    
    println!("Captured and saved receipt: {}", receipt.id);
    
    Ok(())
}
```

## Best Practices

1. **Always Handle Errors**: Use proper error handling for all operations
2. **Validate Input**: Check data before creating or updating entities
3. **Use Transactions**: For operations that modify multiple entities
4. **Respect User Preferences**: Check sharing preferences before sharing data
5. **Log Operations**: Use tracing for debugging and monitoring
6. **Secure Sensitive Data**: Encrypt data in transit and at rest
7. **Follow Privacy Guidelines**: Respect user consent and opt-out preferences
8. **Test Thoroughly**: Write unit and integration tests for all functionality

## Conclusion

These examples demonstrate the core functionality of the Expense Tracker module and how to integrate it into a larger application. The module is designed to be flexible and extensible, allowing developers to use only the features they need while maintaining consistency with the overall CPC architecture.
//! Bootstrap file showing how to initialize the expense tracker module
//!
//! This file demonstrates how to wire up all the components of the expense tracker
//! module together in a real application context.

use std::sync::Arc;
use sqlx::PgPool;

use crate::{
    // Existing services that expense tracker integrates with
    application::{
        wallet_service::{WalletServiceImpl, WalletService},
        budget_service::{BudgetServiceImpl, BudgetService},
    },
    
    // Expense tracker components
    expense_tracker::{
        application::expense_service::{ExpenseServiceImpl, ExpenseService},
        infrastructure::{
            database::{
                expense_repository::PostgresExpenseRepository,
                receipt_repository::PostgresReceiptRepository,
                sharing_preference_repository::PostgresExpenseSharingPreferenceRepository,
            },
            ocr::receipt_processor::OCRService,
            p2p::expense_sharing::{P2PExpenseSharing, UserKeys},
            bevy::receipt_scanner::ReceiptScannerPlugin,
        },
    },
};

/// Bootstrap function to initialize all expense tracker components
pub async fn bootstrap_expense_tracker(
    db_pool: PgPool,
    wallet_service: Arc<dyn WalletService>,
    budget_service: Arc<dyn BudgetService>,
    p2p_manager: Arc<cpc_net::p2p::P2PManager>,
    user_keys: UserKeys,
) -> Result<(
    Arc<dyn ExpenseService>,
    OCRService,
    P2PExpenseSharing,
    ReceiptScannerPlugin,
), Box<dyn std::error::Error>> {
    // Initialize repositories
    let expense_repo = Arc::new(PostgresExpenseRepository::new(db_pool.clone()));
    let receipt_repo = Arc::new(PostgresReceiptRepository::new(db_pool.clone()));
    let sharing_preference_repo = Arc::new(PostgresExpenseSharingPreferenceRepository::new(db_pool.clone()));
    
    // Initialize expense service
    let expense_service = Arc::new(ExpenseServiceImpl::new(
        expense_repo.clone(),
        receipt_repo.clone(),
        sharing_preference_repo,
        wallet_service,
        budget_service,
    ));
    
    // Initialize OCR service
    let ocr_service = OCRService::new(receipt_repo.clone());
    
    // Initialize p2p sharing service
    let p2p_sharing = P2PExpenseSharing::new(
        p2p_manager,
        user_keys,
        expense_service.clone(),
    );
    
    // Initialize Bevy plugin
    let receipt_scanner_plugin = ReceiptScannerPlugin;
    
    Ok((
        expense_service,
        ocr_service,
        p2p_sharing,
        receipt_scanner_plugin,
    ))
}

/// Example of how to use the bootstrapped components
pub async fn example_usage(
    expense_service: Arc<dyn ExpenseService>,
    ocr_service: &OCRService,
    p2p_sharing: &P2PExpenseSharing,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::domain::{
        expense_tracker::{ExpenseCategory, ReceiptImageData},
        primitives::{Money, Currency},
    };
    use uuid::Uuid;
    use chrono::Utc;
    use rust_decimal_macros::dec;
    
    // Example 1: Create an expense
    let user_id = Uuid::new_v4();
    let expense = expense_service.create_expense(
        user_id,
        Money::new(dec!(15.75), Currency::Dabloons),
        ExpenseCategory::Food,
        Utc::now(),
        "Lunch at cafe".to_string(),
    ).await?;
    
    println!("Created expense: {}", expense.id);
    
    // Example 2: Save and process a receipt
    let receipt = expense_service.save_receipt(
        user_id,
        ReceiptImageData::Base64Data("base64_encoded_image_data".to_string()),
    ).await?;
    
    // Process receipt with OCR (in a real implementation)
    // ocr_service.process_receipt(receipt.id).await?;
    
    println!("Saved receipt: {}", receipt.id);
    
    // Example 3: Share an expense
    // p2p_sharing.share_expense(
    //     expense.id,
    //     vec!["recipient_node_id".to_string()],
    //     user_id,
    // ).await?;
    
    // Example 4: Update sharing preferences
    let preferences = expense_service.update_sharing_preferences(
        user_id,
        true,  // Enable sharing
        true,  // Anonymize data
        vec![ExpenseCategory::Food, ExpenseCategory::Entertainment],
    ).await?;
    
    println!("Updated sharing preferences for user: {}", preferences.user_id);
    
    Ok(())
}

/// Configuration struct for the expense tracker
#[derive(Debug, Clone)]
pub struct ExpenseTrackerConfig {
    pub enable_ocr: bool,
    pub enable_p2p_sharing: bool,
    pub enable_receipt_scanning: bool,
    pub default_currency: crate::domain::primitives::Currency,
}

impl Default for ExpenseTrackerConfig {
    fn default() -> Self {
        Self {
            enable_ocr: true,
            enable_p2p_sharing: true,
            enable_receipt_scanning: true,
            default_currency: crate::domain::primitives::Currency::USD,
        }
    }
}

/// Conditional initialization based on configuration
pub async fn bootstrap_with_config(
    db_pool: PgPool,
    wallet_service: Arc<dyn WalletService>,
    budget_service: Arc<dyn BudgetService>,
    p2p_manager: Option<Arc<cpc_net::p2p::P2PManager>>,
    user_keys: Option<UserKeys>,
    config: ExpenseTrackerConfig,
) -> Result<(
    Arc<dyn ExpenseService>,
    Option<OCRService>,
    Option<P2PExpenseSharing>,
    Option<ReceiptScannerPlugin>,
), Box<dyn std::error::Error>> {
    // Initialize repositories
    let expense_repo = Arc::new(PostgresExpenseRepository::new(db_pool.clone()));
    let receipt_repo = Arc::new(PostgresReceiptRepository::new(db_pool.clone()));
    let sharing_preference_repo = Arc::new(PostgresExpenseSharingPreferenceRepository::new(db_pool.clone()));
    
    // Initialize expense service
    let expense_service = Arc::new(ExpenseServiceImpl::new(
        expense_repo.clone(),
        receipt_repo.clone(),
        sharing_preference_repo,
        wallet_service,
        budget_service,
    ));
    
    // Conditionally initialize OCR service
    let ocr_service = if config.enable_ocr {
        Some(OCRService::new(receipt_repo.clone()))
    } else {
        None
    };
    
    // Conditionally initialize p2p sharing service
    let p2p_sharing = if config.enable_p2p_sharing {
        if let (Some(p2p_manager), Some(user_keys)) = (p2p_manager, user_keys) {
            Some(P2PExpenseSharing::new(
                p2p_manager,
                user_keys,
                expense_service.clone(),
            ))
        } else {
            return Err("p2p sharing enabled but p2p_manager or user_keys not provided".into());
        }
    } else {
        None
    };
    
    // Conditionally initialize Bevy plugin
    let receipt_scanner_plugin = if config.enable_receipt_scanning {
        Some(ReceiptScannerPlugin)
    } else {
        None
    };
    
    Ok((
        expense_service,
        ocr_service,
        p2p_sharing,
        receipt_scanner_plugin,
    ))
}
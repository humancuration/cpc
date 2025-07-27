// This module has been refactored and moved to packages/cpc-core/finance/
// The code in this directory is deprecated and will be removed in a future release.
// Please use the finance module instead.
// REFACTORED: This module has been moved to packages/cpc-core/finance/. DO NOT MODIFY.
// See packages/cpc-core/finance/MIGRATION_GUIDE.md for migration instructions.
//! Personal Finance Management System
//!
//! A comprehensive personal finance management application built with Rust,
//! featuring budgeting, expense tracking, savings goals, and financial insights.
//!
//! # Architecture
//!
//! This application follows hexagonal (clean) architecture with vertical slices:
//!
//! - **Budgeting Slice**: Monthly allocation, category tracking, utilization analytics
//! - **Expense Tracking Slice**: Receipt processing, categorization, spending tracking
//! - **Savings Goals Slice**: Goal planning, auto-deduction, progress tracking
//! - **Shared Module**: Cross-cutting concerns (finance primitives)
//!
//! # Key Features
//!
//! - Budget creation and management
//! - Expense tracking with categories
//! - Savings goal planning
//! - Receipt scanning via OCR
//! - Financial insights and trends
//! - UBI (Universal Basic Income) integration
//! - Treasury service for financial analytics

// Vertical slices
pub mod budgeting;
pub mod expense_tracking;
pub mod savings_goals;
pub mod shared;

// Legacy modules (to be deprecated)
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod web;

/// Re-export commonly used types
pub use budgeting::domain::{
    models::{Budget, BudgetPeriod},
};

pub use expense_tracking::domain::{
    models::{Expense, Receipt, ExpenseCategory},
};

pub use savings_goals::domain::{
    models::{SavingsGoal, SavingsProgress},
};

pub use shared::domain::{
    Money, Currency, Amount, FinancialAccount, AccountType, DateRange
};

/// Initialize the personal finance service with default configuration
pub async fn init_service(
    database_url: String,
    ubi_config: Option<infrastructure::services::ubi_service::UbiServiceConfig>,
    treasury_config: Option<infrastructure::services::treasury_service::TreasuryServiceConfig>,
    ocr_config: Option<infrastructure::services::ocr_service::OcrServiceConfig>,
) -> Result<application::finance_service::FinanceService, domain::models::FinanceError> {
    use sqlx::postgres::PgPoolOptions;
    
    // Initialize database connection
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| domain::models::FinanceError::DatabaseError(e.to_string()))?;

    // Initialize repositories
    let budget_repo = infrastructure::PostgresBudgetRepository::new(pool.clone());
    let expense_repo = infrastructure::PostgresExpenseRepository::new(pool.clone());
    let savings_repo = infrastructure::PostgresSavingsRepository::new(pool.clone());
    let data_sharing_repo = infrastructure::PostgresDataSharingRepository::new(pool.clone());

    // Initialize services
    let ubi_service = match ubi_config {
        Some(config) => {
            let service = infrastructure::services::HttpUbiService::new(config);
            std::sync::Arc::new(service) as std::sync::Arc<dyn infrastructure::services::ubi_service::UbiService>
        }
        None => {
            std::sync::Arc::new(infrastructure::services::MockUbiService) as std::sync::Arc<dyn infrastructure::services::ubi_service::UbiService>
        }
    };

    let treasury_service = match treasury_config {
        Some(config) => {
            let service = infrastructure::services::HttpTreasuryService::new(config);
            std::sync::Arc::new(service) as std::sync::Arc<dyn infrastructure::services::treasury_service::TreasuryService>
        }
        None => {
            std::sync::Arc::new(infrastructure::services::MockTreasuryService) as std::sync::Arc<dyn infrastructure::services::treasury_service::TreasuryService>
        }
    };

    let ocr_service = match ocr_config {
        Some(config) => {
            let service = infrastructure::services::HttpOcrService::new(config);
            std::sync::Arc::new(service) as std::sync::Arc<dyn infrastructure::services::ocr_service::OcrService>
        }
        None => {
            std::sync::Arc::new(infrastructure::services::MockOcrService) as std::sync::Arc<dyn infrastructure::services::ocr_service::OcrService>
        }
    };

    // Create services
    let budget_service = std::sync::Arc::new(budgeting::domain::budget_service::BudgetServiceImpl::new(
        std::sync::Arc::new(budget_repo),
        ubi_service,
    ));

    let expense_service = std::sync::Arc::new(expense_tracking::domain::expense_service::ExpenseServiceImpl::new(
        std::sync::Arc::new(expense_repo),
        treasury_service,
        ocr_service,
    ));

    let savings_service = std::sync::Arc::new(savings_goals::domain::savings_service::SavingsServiceImpl::new(
        std::sync::Arc::new(savings_repo),
        std::sync::Arc::new(data_sharing_repo),
    ));

    // Create the finance service
    let finance_service = application::finance_service::FinanceService::new(
        budget_service,
        expense_service,
        savings_service,
    );

    Ok(finance_service)
}
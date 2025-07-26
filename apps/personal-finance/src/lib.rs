//! Personal Finance Management System
//!
//! A comprehensive personal finance management application built with Rust,
//! featuring budgeting, expense tracking, savings goals, and financial insights.
//!
//! # Architecture
//!
//! This application follows hexagonal (clean) architecture with the following layers:
//!
//! - **Domain Layer**: Core business logic and rules
//! - **Application Layer**: Use cases and orchestration
//! - **Infrastructure Layer**: External services and persistence
//! - **Web Layer**: HTTP API (GraphQL and REST)
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

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod web;

/// Re-export commonly used types
pub use domain::{
    models::{Budget, Expense, SavingsGoal, FinanceError},
    budget_service::{BudgetService, UbiService},
    expense_service::{ExpenseService, TreasuryService, OcrService},
    savings_service::SavingsService,
};

pub use application::{
    finance_service::FinanceService,
};

pub use infrastructure::{
    repositories::{BudgetRepository, ExpenseRepository, SavingsRepository},
    services::{UbiServiceConfig, TreasuryServiceConfig, OcrServiceConfig},
};

pub use web::{
    dto::*,
    graphql::*,
    handlers::*,
};

/// Initialize the personal finance service with default configuration
pub async fn init_service(
    database_url: String,
    ubi_config: Option<UbiServiceConfig>,
    treasury_config: Option<TreasuryServiceConfig>,
    ocr_config: Option<OcrServiceConfig>,
) -> Result<FinanceService, FinanceError> {
    use sqlx::postgres::PgPoolOptions;
    
    // Initialize database connection
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

    // Initialize repositories
    let budget_repo = infrastructure::repositories::PostgresBudgetRepository::new(pool.clone());
    let expense_repo = infrastructure::repositories::PostgresExpenseRepository::new(pool.clone());
    let savings_repo = infrastructure::repositories::PostgresSavingsRepository::new(pool.clone());

    // Initialize services
    let ubi_service = match ubi_config {
        Some(config) => {
            let service = infrastructure::services::HttpUbiService::new(config);
            std::sync::Arc::new(service) as std::sync::Arc<dyn UbiService>
        }
        None => {
            std::sync::Arc::new(infrastructure::services::MockUbiService) as std::sync::Arc<dyn UbiService>
        }
    };

    let treasury_service = match treasury_config {
        Some(config) => {
            let service = infrastructure::services::HttpTreasuryService::new(config);
            std::sync::Arc::new(service) as std::sync::Arc<dyn TreasuryService>
        }
        None => {
            std::sync::Arc::new(infrastructure::services::MockTreasuryService) as std::sync::Arc<dyn TreasuryService>
        }
    };

    let ocr_service = match ocr_config {
        Some(config) => {
            let service = infrastructure::services::HttpOcrService::new(config);
            std::sync::Arc::new(service) as std::sync::Arc<dyn OcrService>
        }
        None => {
            std::sync::Arc::new(infrastructure::services::MockOcrService) as std::sync::Arc<dyn OcrService>
        }
    };

    // Create services
    let budget_service = std::sync::Arc::new(domain::budget_service::BudgetServiceImpl::new(
        budget_repo,
        ubi_service,
    ));

    let expense_service = std::sync::Arc::new(domain::expense_service::ExpenseServiceImpl::new(
        expense_repo,
        treasury_service,
        ocr_service,
    ));

    let savings_service = std::sync::Arc::new(domain::savings_service::SavingsServiceImpl::new(
        savings_repo,
    ));

    // Create the finance service
    let finance_service = application::finance_service::FinanceService::new(
        budget_service,
        expense_service,
        savings_service,
    );

    Ok(finance_service)
}
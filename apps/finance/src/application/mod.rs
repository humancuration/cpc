//! Application layer for the finance module
//!
//! Contains the application services that orchestrate domain logic and interact with infrastructure.

pub mod budget_service;
pub mod expense_service;
pub mod subscription_service;
pub mod savings_service;
pub mod investment_service;
pub mod debt_service;
pub mod rewards_service;
pub mod wallet_service;
pub mod finance_aggregator;
pub mod events;
pub mod expense_tracker;
pub mod user_preferences;
pub mod currency;
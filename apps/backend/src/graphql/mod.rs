pub mod social;
pub mod media_mutations;
pub mod financial_forecasting;
pub mod financial_forecasting_dashboard;
pub mod invoicing;
pub mod asset_browser;
pub mod impact;
pub mod expenses;
pub mod community;
pub mod supply_chain;
pub mod cooperative;
pub mod project;
pub mod document_editor;

pub mod finance;

// New GraphQL modules for android-rust-migration
pub mod user_management;
pub mod social_interactions;
pub mod forum_system;
pub mod governance_system;
pub mod subscription_events;

// Modular architecture modules
pub mod static_schema;
pub mod schema_builder;

#[cfg(test)]
mod schema_builder_test;

#[cfg(test)]
mod schema_merging_test;
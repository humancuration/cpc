//! # CPC Financial Core
//!
//! Core financial utilities for the CPC platform, providing precise monetary calculations
//! with audit trail support.
//!
//! This crate provides:
//! - `MonetaryAmount` for precise decimal-based monetary values
//! - Rounding strategies for financial calculations
//! - Currency handling with proper decimal places
//! - Audit trail integration for all financial operations

pub mod monetary;
pub mod rounding;
pub mod currency;
pub mod audit;

// Re-export key types for convenience
pub use monetary::MonetaryAmount;
pub use rounding::{RoundingStrategy, round_with_strategy};
pub use currency::CurrencyCode;
pub use audit::FinancialAuditable;
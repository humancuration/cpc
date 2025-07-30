//! Currency module for the finance domain
//!
//! This module provides comprehensive currency support including:
//! - ISO 4217 currency codes
//! - Exchange rate management
//! - Currency conversion
//! - Locale-aware formatting

pub mod code;
pub mod model;
pub mod registry;
pub mod exchange_rate;

pub use code::CurrencyCode;
pub use model::Currency;
pub use registry::CurrencyRegistry;
pub use exchange_rate::{ExchangeRate, ExchangeRateService, ExchangeRateProvider, ExchangeRateError};
//! Localization module for currency formatting
//!
//! This module provides locale-aware formatting for currencies and numbers.

pub mod formatter;

pub use formatter::{CurrencyFormatter, Locale};
//! # CPC Wallet Crate
//!
//! A reusable wallet crate for CPC apps supporting the internal currency (Dabloons)
//! and traditional currencies. Provides functionality for managing user wallets,
//! transactions, and Universal Income distribution.
//!
//! ## Features
//! - Wallet management for Dabloons and traditional currencies
//! - Transaction recording and history
//! - Transfer functionality between users
//! - Integration with Universal Income system
//! - Integration with common_utils for standardized error handling, logging, crypto, and datetime
//!
//! ## Integration with common_utils
//! This crate integrates with the common_utils package to provide:
//! - Standardized error handling through CommonError
//! - Unified logging through common_utils::logging
//! - Crypto functions through common_utils::crypto
//! - DateTime handling through common_utils::datetime
//!
//! The integration is controlled by the "common-utils-integration" feature flag.

pub mod domain;
pub mod application;
pub mod graphql;

// Re-export key types
pub use domain::{Wallet, WalletTransaction, TransactionType, Money, Currency};
pub use application::{WalletService, WalletRepository, TipService};

// Error compatibility shim for common_utils integration
#[cfg(feature = "common-utils-integration")]
pub mod error_shim;
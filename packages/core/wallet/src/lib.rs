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

pub mod domain;
pub mod application;
pub mod graphql;

// Re-export key types
pub use domain::{Wallet, WalletTransaction, TransactionType, Money, Currency};
pub use application::{WalletService, WalletRepository, TipService};
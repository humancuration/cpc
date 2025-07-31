//! Domain models for the wallet system

pub mod wallet;
pub mod primitives;

pub use wallet::{Wallet, WalletTransaction, TransactionType};
pub use primitives::{Money, Currency, FinancialError, Amount};
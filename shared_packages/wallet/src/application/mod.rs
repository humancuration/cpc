//! Application services for the wallet system

pub mod wallet_service;
pub mod tip_service;

#[cfg(test)]
pub mod wallet_service_test;

pub use wallet_service::{WalletService, WalletRepository, WalletServiceImpl};
pub use tip_service::TipService;
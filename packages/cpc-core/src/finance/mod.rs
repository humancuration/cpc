//! Financial module for the CPC platform
//!
//! This module provides comprehensive financial functionality including:
//! - Wallet management
//! - Transaction processing
//! - Treasury operations
//! - Royalty distribution
//! - Payment processing

pub mod transactions;
pub mod treasury;
pub mod royalty_engine;
pub mod royalty_service;
pub mod wallets;

// Re-export commonly used types
pub use royalty_engine::*;
pub use royalty_service::*;
pub use transactions::*;
pub use treasury::*;
pub use wallets::*;
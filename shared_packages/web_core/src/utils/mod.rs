//! Utility functions for CPC web applications
//!
//! This module provides utility functions that can be used
//! across all web applications in the CPC ecosystem.

pub mod storage;
pub mod error_handling;
pub mod error_reporting;
pub mod error_recovery;

// Re-export utilities for convenience
pub use storage::Storage;
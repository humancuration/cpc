//! Infrastructure layer for the finance module
//!
//! Contains implementations for database repositories, external service clients, and other infrastructure concerns.

pub mod database;
pub mod adapters;
#[cfg(feature = "p2p")]
pub mod p2p;
//! Infrastructure components for the CPC platform
//!
//! This crate provides infrastructure implementations for various services
//! including database adapters, network clients, and sync mechanisms.

pub mod grpc;
pub mod sled;
pub mod core;
pub mod sync;

#[cfg(test)]
mod integration_test;

#[cfg(test)]
mod compile_test;
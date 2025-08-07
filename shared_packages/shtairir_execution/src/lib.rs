//! Shtairir Execution Scheduler
//!
//! This crate provides deterministic execution scheduling for Shtairir graphs
//! with support for concurrent execution of independent nodes.

pub mod scheduler;
pub mod executor;
pub mod graph;
pub mod concurrency;
pub mod registry;

pub use scheduler::Scheduler;
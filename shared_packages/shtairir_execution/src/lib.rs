//! Shtairir Execution System
//!
//! This crate provides deterministic execution scheduling for Shtairir graphs
//! with support for concurrent execution of independent nodes, memory management,
//! and parallel execution planning.

pub mod scheduler;
pub mod executor;
pub mod graph;
pub mod concurrency;
pub mod registry;
pub mod memory;
pub mod planning;

pub use scheduler::Scheduler;
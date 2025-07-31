//! Infrastructure layer for the storage abstraction
//! 
//! This module contains the concrete implementations of storage backends.

pub mod sled;
pub mod postgres;
pub mod in_memory;
pub mod dual_write;
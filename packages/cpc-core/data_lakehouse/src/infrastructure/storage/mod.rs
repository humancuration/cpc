//! Storage implementations for the data lakehouse

pub mod postgres;
pub mod sled;
pub mod webm_columnar;

// Re-export the DataFrame type for convenience
pub use polars::prelude::DataFrame;
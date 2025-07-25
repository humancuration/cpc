//! Product display components module
//! Provides comprehensive product information display with validation status

pub mod display;
pub mod header;
pub mod details;
pub mod cost_breakdown;
pub mod supply_chain;
pub mod validation_status;

pub use display::*;
pub use header::*;
pub use details::*;
pub use cost_breakdown::*;
pub use supply_chain::*;
pub use validation_status::*;
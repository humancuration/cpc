//! Finance-Sheets web application
//!
//! This module provides a financial spreadsheet application with currency
//! internationalization features built on top of the base Sheets functionality.

pub mod components;
pub mod services;
pub mod styles;
pub mod app;

#[cfg(test)]
pub mod tests;

pub use app::App;
pub use components::mobile::*;
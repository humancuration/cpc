//! Expense Tracker module for the CPC finance system
//!
//! This module provides functionality for tracking expenses with dual-currency support
//! (traditional currency + Dabloons), receipt scanning, automatic categorization,
//! and secure p2p sharing with granular consent controls.

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;
pub mod bootstrap;
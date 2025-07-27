//! # CPC Invoicing & Quoting Module
//!
//! A vertical slice implementation for invoicing and quoting functionality
//! following the hexagonal architecture pattern.

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;

// Re-export key types
pub use domain::{Invoice, Quote, InvoiceItem, QuoteItem, PaymentStatus, QuoteStatus};
pub use application::{InvoiceService, QuoteService};
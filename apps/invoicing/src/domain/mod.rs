//! Domain models for the invoicing and quoting system

pub mod primitives;

pub use primitives::{Invoice, Quote, InvoiceItem, QuoteItem, PaymentStatus, QuoteStatus};
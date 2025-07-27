//! Infrastructure implementations for invoicing and quoting

pub mod database;
pub mod p2p;

pub use database::{PgInvoiceRepository, PgQuoteRepository};
pub use p2p::P2PInvoiceSharing;
//! Infrastructure services module
//! 
//! This module contains implementations for external service integrations:
//! - UBI (Universal Basic Income) service
//! - Treasury service for financial insights
//! - OCR service for receipt processing

pub mod ubi_service;
pub mod treasury_service;
pub mod ocr_service;

pub use ubi_service::{UbiServiceConfig, HttpUbiService, MockUbiService};
pub use treasury_service::{TreasuryServiceConfig, HttpTreasuryService, MockTreasuryService};
pub use ocr_service::{OcrServiceConfig, HttpOcrService, MockOcrService};
pub use ocr_service::{ReceiptData, ReceiptItem};
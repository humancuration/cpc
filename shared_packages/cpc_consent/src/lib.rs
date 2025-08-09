// CPC Consent management module
pub mod consent;
pub mod privacy;

pub use consent::*;
pub use privacy::*;

// Re-export consent_manager types
pub use consent_manager::*;

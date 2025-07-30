//! P2P implementations for the CRM module
//!
//! This module contains p2panda implementations for CRM data sharing.

pub mod data_sharing;
pub mod session_manager;

pub use data_sharing::{P2PCrmSharing, CrmSharingError};
pub use session_manager::SessionManager;
//! Authentication service for the Messenger web application
//!
//! This module now re-exports the shared implementation from web_core

pub use web_core::auth::{AuthService, User, Token};
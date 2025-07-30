//! CPC Authentication Library
//!
//! This crate provides authentication services for CPC applications,
//! including user registration, login, session management, and OAuth integration.

pub mod models;
pub mod auth_service;
pub mod session_service;
pub mod oauth;
pub mod middleware;
pub mod error;
pub mod session;
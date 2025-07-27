//! Advanced CRM module for the CPC platform
//!
//! This module extends the simple CRM with advanced features for SMB needs including:
//! - Lead scoring
//! - Email marketing campaigns
//! - HR integration for sales performance tracking
//! - Advanced reporting and analytics
//!
//! # Architecture
//!
//! The module follows the hexagonal architecture pattern with vertical slicing:
//!
//! ```text
//! packages/cpc-core/advanced_crm/
//! ├── domain/          # Pure business logic
//! ├── application/     # Service orchestration
//! ├── infrastructure/  # Database and external service implementations
//! └── presentation/    # UI components (Yew/Bevy)
//! ```

/// Domain layer containing pure business logic
pub mod domain;

/// Application layer containing service implementations
pub mod application;

/// Infrastructure layer containing concrete implementations
pub mod infrastructure;

/// Presentation layer containing UI components
pub mod presentation;
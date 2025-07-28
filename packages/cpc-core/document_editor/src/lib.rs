//! Document Editor Module
//!
//! This module provides a word processor with basic formatting, image insertion,
//! and PDF/DOCX export capabilities.

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;
pub mod web;
pub mod web;
pub mod modular_module;
pub mod module_registry;
pub mod crdt;
pub mod collaboration;

// Re-export key types
pub use domain::models::{Document, DocumentShare, DocumentVersion};
pub use domain::value_objects::{DocumentTitle, DocumentContent};
pub use domain::errors::DocumentError;

pub use application::document_service::DocumentService;
pub use application::export_service::ExportService;
pub use application::collaboration_service::CollaborationService;
pub use collaboration::service::RealtimeCollaborationService;
pub use collaboration::panda_network::{PandaNetwork, PandaSyncService};
pub use modular_module::ModularDocumentEditor;
pub use module_registry::{Module, DependencyRequirement};

#[cfg(feature = "p2p")]
pub mod transport;

#[cfg(test)]
mod tests;
//! Collaborative Workspace crate
//!
//! Domain layer for collaborative documents, project boards, file versioning,
//! and meeting rooms, following ADR-0008. This crate exposes domain models
//! and trait interfaces only (no implementations).

pub mod domain {
    pub mod models;
    pub mod repository;
    pub mod service;
}

pub use domain::{models, repository, service};
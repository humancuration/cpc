//! # Storage Abstraction Layer
//! 
//! Provides unified storage interfaces with smart routing between different storage backends
//! (Sled for edge, PostgreSQL for cloud), addressing the dual-write pattern.
//! 
//! This module implements a storage abstraction layer that allows applications to store data
//! without needing to know the underlying storage mechanism. It provides smart routing based
//! on data sensitivity and access patterns, and implements dual-write patterns with fallback
//! strategies.

/// Domain layer containing core business logic and entities
pub mod domain;

/// Application layer containing use cases and service orchestration
pub mod application;

/// Infrastructure layer containing adapters for external systems
pub mod infrastructure;
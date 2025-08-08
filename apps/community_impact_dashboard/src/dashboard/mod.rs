//! Dashboard module for the Unified Community Impact Dashboard
//!
//! This module contains the core implementation of the unified dashboard that
//! integrates all four impact measurement systems.

pub mod unified_dashboard;
pub mod individual_view;
pub mod community_view;
pub mod visualization_components;
pub mod impact_interconnection;
pub mod community_transformation;
pub mod member_views;
pub mod community_engagement;

pub use unified_dashboard::UnifiedImpactDashboard;
//! Launch support system for the Unified Community Impact Dashboard
//!
//! This module provides community help desk system with peer support,
//! real-time issue tracking and resolution, knowledge base with
//! community-contributed solutions, community feedback triage and
//! response system, and translation support for multilingual communities.

/// Community help desk system with peer support
pub mod help_desk;

/// Real-time issue tracking and resolution
pub mod issue_tracking;

/// Knowledge base with community-contributed solutions
pub mod knowledge_base;

/// Community feedback triage and response system
pub mod feedback_triage;

/// Translation support for multilingual communities
pub mod translation;

// Re-export key components
pub use help_desk::CommunityHelpDesk;
pub use issue_tracking::IssueTracker;
pub use knowledge_base::KnowledgeBase;
pub use feedback_triage::FeedbackTriage;
pub use translation::TranslationSupport;
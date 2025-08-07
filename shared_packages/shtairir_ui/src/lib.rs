//! Shared UI components for Shtairir applications
//!
//! This crate provides reusable Yew components for building Shtairir-based applications,
//! including visual editors, block browsers, and workflow management interfaces.

pub mod components;
pub mod hooks;
pub mod utils;

// Re-export key components
pub use components::{
    block_browser::BlockBrowser,
    node_editor::NodeEditor,
    workflow_canvas::WorkflowCanvas,
    property_panel::PropertyPanel,
    connection_line::ConnectionLine,
};

/// Theme definitions for consistent styling across Shtairir applications
pub mod theme {
    use yew::Classes;
    
    /// Get CSS classes for a primary button
    pub fn primary_button() -> Classes {
        Classes::from("shtairir-btn shtairir-btn-primary")
    }
    
    /// Get CSS classes for a secondary button
    pub fn secondary_button() -> Classes {
        Classes::from("shtairir-btn shtairir-btn-secondary")
    }
    
    /// Get CSS classes for a danger button
    pub fn danger_button() -> Classes {
        Classes::from("shtairir-btn shtairir-btn-danger")
    }
    
    /// Get CSS classes for a node component
    pub fn node() -> Classes {
        Classes::from("shtairir-node")
    }
    
    /// Get CSS classes for a port component
    pub fn port() -> Classes {
        Classes::from("shtairir-port")
    }
}
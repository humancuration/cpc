//! UI Toolkit for CPC applications
//!
//! This crate provides a comprehensive set of UI components, themes, and hooks
//! for building consistent and accessible user interfaces across CPC applications.

pub mod themes;
pub mod hooks;
pub mod components;
pub mod examples;

// Re-export commonly used items
pub use themes::*;
pub use hooks::*;
pub use components::*;
pub use examples::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn ui_toolkit_structure() {
        // This is a placeholder test to verify the module structure
        // Actual tests would be in the respective modules
        assert!(true);
    }
}

#[cfg(test)]
mod integration_test;
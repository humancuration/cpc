//! User Profile Management Workflow
//!
//! A workflow for managing user profile data with validation and transformation.
//! This module demonstrates how to compose Shtairir standard library blocks into a user management workflow.

pub mod workflow;
pub mod validation;

// Re-export key types and functions
pub use workflow::{UserProfile, WorkflowMetrics, execute_workflow, generate_mock_profiles};
pub use validation::{ValidationResult, validate_profile, normalize_name, trim_email, create_display_name};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
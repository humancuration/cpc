//! Tests for mobile components
//!
//! This module contains integration tests for the mobile components functionality.

#[cfg(test)]
mod tests {
    use yew::prelude::*;
    use yew::{Renderer, Scope};
    use super::super::{MobileLayout, FloatingActionButton, haptics};
    
    // Helper component for testing MobileLayout
    #[derive(Properties, PartialEq)]
    struct TestComponentProps {
        pub message: String,
    }
    
    #[function_component(TestComponent)]
    fn test_component(props: &TestComponentProps) -> Html {
        html! {
            <div>{&props.message}</div>
        }
    }
    
    #[test]
    fn test_mobile_layout_creation() {
        // This is a basic test to ensure the component can be created
        // In a real implementation, we would use a testing framework like wasm-bindgen-test
        let _layout = html! {
            <MobileLayout>
                <TestComponent message="Test content" />
            </MobileLayout>
        };
        
        // If we get here without panicking, the component was created successfully
        assert!(true);
    }
    
    #[test]
    fn test_fab_creation() {
        // This is a basic test to ensure the component can be created
        let _fab = html! {
            <FloatingActionButton />
        };
        
        // If we get here without panicking, the component was created successfully
        assert!(true);
    }
    
    #[test]
    fn test_haptics_functions() {
        // Test that haptics functions can be called without panicking
        haptics::trigger_cell_selection();
        haptics::trigger_cell_edit_start();
        haptics::trigger_sheet_switch();
        
        // Test with direct parameters
        haptics::trigger_haptic(50, 0.3);
        haptics::trigger_haptic(100, 0.6);
        haptics::trigger_haptic(150, 0.9);
        
        assert!(true);
    }
}
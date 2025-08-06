//! Tests for the error boundary component
//!
//! This module contains tests for the error boundary component functionality.

use wasm_bindgen_test::*;
use web_core::components::{ErrorBoundary, ErrorBoundaryProps};
use web_core::components::base::{BaseComponent, CommonProps};
use web_core::utils::error_handling::WebError;
use yew::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_error_boundary_creation_with_default_properties() {
    // Create error boundary props with default values
    let props = ErrorBoundaryProps::default();
    
    // Test default property values
    assert_eq!(props.common.class, None);
    assert!(props.children.is_empty());
    assert!(props.on_error.is_empty());
    assert_eq!(props.fallback, None);
}

#[wasm_bindgen_test]
fn test_error_boundary_creation_with_custom_properties() {
    // Test that we can create ErrorBoundaryProps with custom values
    let fallback_html = html! { <div>{"Fallback UI"}</div> };
    
    let props = ErrorBoundaryProps {
        common: CommonProps {
            class: Some("custom-class".to_string()),
            ..Default::default()
        },
        children: yew::Children::new(vec![html! { <div>{"Child"}</div> }]),
        on_error: yew::Callback::from(|_: WebError| {}),
        fallback: Some(fallback_html.clone()),
    };
    
    // Test that the props are set correctly
    assert_eq!(props.common.class, Some("custom-class".to_string()));
    assert_eq!(props.fallback, Some(fallback_html));
}

#[wasm_bindgen_test]
fn test_error_boundary_base_component_implementation() {
    // Test BaseComponent implementation
    let props = ErrorBoundaryProps::default();
    let error_boundary = ErrorBoundary::create(&props);
    
    // Test that the error boundary is created correctly
    assert_eq!(error_boundary.has_error(), false);
    assert_eq!(error_boundary.error_message(), None);
    
    // Test view method
    let html = error_boundary.view();
    assert!(!html.is_empty());
}

#[wasm_bindgen_test]
fn test_error_boundary_base_component_with_children() {
    // Test BaseComponent implementation with children
    let child_html = html! { <div>{"Child content"}</div> };
    let props = ErrorBoundaryProps {
        children: yew::Children::new(vec![child_html.clone()]),
        ..Default::default()
    };
    let error_boundary = ErrorBoundary::create(&props);
    
    // Test view method
    let html = error_boundary.view();
    assert!(!html.is_empty());
}

#[wasm_bindgen_test]
fn test_error_boundary_base_component_with_custom_class() {
    // Test BaseComponent implementation with custom class
    let props = ErrorBoundaryProps {
        common: CommonProps {
            class: Some("custom-error-boundary".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    let error_boundary = ErrorBoundary::create(&props);
    
    // Test view method
    let html = error_boundary.view();
    assert!(!html.is_empty());
}

#[wasm_bindgen_test]
fn test_error_boundary_update_props() {
    // Test updating props
    let initial_props = ErrorBoundaryProps::default();
    let mut error_boundary = ErrorBoundary::create(&initial_props);
    
    assert_eq!(error_boundary.has_error(), false);
    assert_eq!(error_boundary.error_message(), None);
    
    let new_props = ErrorBoundaryProps {
        common: CommonProps {
            class: Some("updated-class".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    error_boundary.update_props(new_props);
    
    // For BaseComponent, updating props doesn't change the state
    assert_eq!(error_boundary.has_error(), false);
    assert_eq!(error_boundary.error_message(), None);
}

#[wasm_bindgen_test]
fn test_error_boundary_default_implementation() {
    // Test default implementation
    let error_boundary = ErrorBoundary::default();
    
    assert_eq!(error_boundary.has_error(), false);
    assert_eq!(error_boundary.error_message(), None);
}

#[wasm_bindgen_test]
fn test_error_boundary_new_method() {
    // Test new method
    let props = ErrorBoundaryProps::default();
    let error_boundary = ErrorBoundary::new(props);
    
    assert_eq!(error_boundary.has_error(), false);
    assert_eq!(error_boundary.error_message(), None);
}

#[wasm_bindgen_test]
fn test_error_boundary_state_methods() {
    // Test state methods
    let props = ErrorBoundaryProps::default();
    let error_boundary = ErrorBoundary::new(props);
    
    assert_eq!(error_boundary.has_error(), false);
    assert_eq!(error_boundary.error_message(), None);
}

#[wasm_bindgen_test]
fn test_error_boundary_error_state() {
    // Test error state functionality
    let props = ErrorBoundaryProps::default();
    let mut error_boundary = ErrorBoundary::new(props);
    
    // Simulate an error occurring
    error_boundary.state.has_error = true;
    error_boundary.state.error_message = Some("Test error message".to_string());
    
    assert_eq!(error_boundary.has_error(), true);
    assert_eq!(error_boundary.error_message(), Some(&"Test error message".to_string()));
}

#[wasm_bindgen_test]
fn test_error_boundary_fallback_ui() {
    // Test fallback UI rendering when an error occurs
    let fallback_html = html! { <div class="custom-fallback">{"Something went wrong!"}</div> };
    let props = ErrorBoundaryProps {
        fallback: Some(fallback_html.clone()),
        ..Default::default()
    };
    let mut error_boundary = ErrorBoundary::new(props);
    
    // Simulate an error occurring
    error_boundary.state.has_error = true;
    
    // For BaseComponent interface, we can't test the full error boundary functionality
    // But we can test that the component renders without errors
    let html = error_boundary.view();
    assert!(!html.is_empty());
}

#[wasm_bindgen_test]
fn test_error_boundary_default_error_ui() {
    // Test default error UI rendering when an error occurs
    let props = ErrorBoundaryProps::default();
    let mut error_boundary = ErrorBoundary::new(props);
    
    // Simulate an error occurring
    error_boundary.state.has_error = true;
    error_boundary.state.error_message = Some("Test error message".to_string());
    
    // For BaseComponent interface, we can't test the full error boundary functionality
    // But we can test that the component renders without errors
    let html = error_boundary.view();
    assert!(!html.is_empty());
}

#[wasm_bindgen_test]
fn test_error_boundary_reset_functionality() {
    // Test reset functionality
    let props = ErrorBoundaryProps::default();
    let mut error_boundary = ErrorBoundary::new(props);
    
    // Simulate an error occurring
    error_boundary.state.has_error = true;
    error_boundary.state.error_message = Some("Test error message".to_string());
    
    assert_eq!(error_boundary.has_error(), true);
    assert_eq!(error_boundary.error_message(), Some(&"Test error message".to_string()));
    
    // Simulate reset
    error_boundary.state.has_error = false;
    error_boundary.state.error_message = None;
    
    assert_eq!(error_boundary.has_error(), false);
    assert_eq!(error_boundary.error_message(), None);
}
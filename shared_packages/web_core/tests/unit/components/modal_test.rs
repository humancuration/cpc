//! Tests for the modal component
//!
//! This module contains tests for the modal component functionality.

use wasm_bindgen_test::*;
use web_core::components::{Modal, ModalProps};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_modal_props_creation_with_default_properties() {
    // Create modal props with default values
    let props = ModalProps::default();
    
    // Test default property values
    assert_eq!(props.open, false);
    assert_eq!(props.title, "");
    assert!(props.children.is_empty());
    assert!(props.onclose.is_empty());
    assert_eq!(props.show_close_button, false);
    assert!(props.common.class.is_none());
}

#[wasm_bindgen_test]
fn test_modal_props_creation_with_custom_properties() {
    // Test that we can create ModalProps with custom values
    let props = ModalProps {
        open: true,
        title: "Test Modal".to_string(),
        show_close_button: true,
        ..Default::default()
    };
    
    // Test that the props are set correctly
    assert_eq!(props.open, true);
    assert_eq!(props.title, "Test Modal");
    assert_eq!(props.show_close_button, true);
}

#[wasm_bindgen_test]
fn test_modal_creation_with_default_properties() {
    // Create modal with default values
    let props = ModalProps::default();
    let modal = Modal::new(props);
    
    // Test that the modal is created correctly
    assert_eq!(modal.props.open, false);
    assert_eq!(modal.props.title, "");
    assert_eq!(modal.props.show_close_button, false);
}

#[wasm_bindgen_test]
fn test_modal_creation_with_custom_properties() {
    let props = ModalProps {
        open: true,
        title: "Test Modal".to_string(),
        show_close_button: true,
        ..Default::default()
    };
    let modal = Modal::new(props);
    
    assert_eq!(modal.props.open, true);
    assert_eq!(modal.props.title, "Test Modal");
    assert_eq!(modal.props.show_close_button, true);
}

#[wasm_bindgen_test]
fn test_modal_props_creation_with_custom_classes() {
    let props = ModalProps {
        common: web_core::components::CommonProps {
            class: Some("custom-class".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    
    assert_eq!(props.common.class, Some("custom-class".to_string()));
}

#[wasm_bindgen_test]
fn test_modal_creation_with_custom_classes() {
    let props = ModalProps {
        common: web_core::components::CommonProps {
            class: Some("custom-class".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    let modal = Modal::new(props);
    
    assert_eq!(modal.props.common.class, Some("custom-class".to_string()));
}

#[wasm_bindgen_test]
fn test_modal_update_props() {
    let initial_props = ModalProps {
        open: false,
        title: "Initial Title".to_string(),
        show_close_button: false,
        ..Default::default()
    };
    let mut modal = Modal::new(initial_props);
    
    assert_eq!(modal.props.open, false);
    assert_eq!(modal.props.title, "Initial Title");
    assert_eq!(modal.props.show_close_button, false);
    
    let new_props = ModalProps {
        open: true,
        title: "New Title".to_string(),
        show_close_button: true,
        ..Default::default()
    };
    modal.update_props(new_props);
    
    assert_eq!(modal.props.open, true);
    assert_eq!(modal.props.title, "New Title");
    assert_eq!(modal.props.show_close_button, true);
}

#[wasm_bindgen_test]
fn test_modal_view_when_open() {
    let props = ModalProps {
        open: true,
        title: "Test Modal".to_string(),
        show_close_button: true,
        ..Default::default()
    };
    let modal = Modal::new(props);
    
    // Render the modal
    let html = modal.view();
    
    // The modal should render when open
    assert!(!html.is_empty());
}

#[wasm_bindgen_test]
fn test_modal_view_when_closed() {
    let props = ModalProps {
        open: false,
        title: "Test Modal".to_string(),
        ..Default::default()
    };
    let modal = Modal::new(props);
    
    // Render the modal
    let html = modal.view();
    
    // The modal should not render when closed
    assert!(html.is_empty());
}
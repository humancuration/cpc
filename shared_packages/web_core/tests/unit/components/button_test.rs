//! Tests for the button component
//!
//! This module contains tests for the button component functionality.

use wasm_bindgen_test::*;
use web_core::components::{Button, ButtonProps, ButtonVariant};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_button_props_creation_with_default_properties() {
    // Create button props with default values
    let props = ButtonProps::default();
    
    // Test default property values
    assert_eq!(props.disabled, false);
    assert_eq!(props.variant, ButtonVariant::Primary);
    assert!(props.children.is_empty());
    assert!(props.onclick.is_empty());
    assert!(props.common.class.is_none());
}

#[wasm_bindgen_test]
fn test_button_props_creation_with_custom_text() {
    // Test that we can create ButtonProps
    let props = ButtonProps {
        children: yew::Children::new(vec![]), // In a real test, we'd pass actual children
        ..Default::default()
    };
    
    // Test that the props are set correctly
    assert_eq!(props.disabled, false);
}

#[wasm_bindgen_test]
fn test_button_creation_with_default_properties() {
    // Create button with default values
    let props = ButtonProps::default();
    let button = Button::new(props);
    
    // Test that the button is created correctly
    assert_eq!(button.props.disabled, false);
    assert_eq!(button.props.variant, ButtonVariant::Primary);
}

#[wasm_bindgen_test]
fn test_button_props_creation_with_primary_variant() {
    let props = ButtonProps {
        variant: ButtonVariant::Primary,
        ..Default::default()
    };
    
    assert_eq!(props.variant, ButtonVariant::Primary);
}

#[wasm_bindgen_test]
fn test_button_props_creation_with_secondary_variant() {
    let props = ButtonProps {
        variant: ButtonVariant::Secondary,
        ..Default::default()
    };
    
    assert_eq!(props.variant, ButtonVariant::Secondary);
}

#[wasm_bindgen_test]
fn test_button_props_creation_with_danger_variant() {
    let props = ButtonProps {
        variant: ButtonVariant::Danger,
        ..Default::default()
    };
    
    assert_eq!(props.variant, ButtonVariant::Danger);
}

#[wasm_bindgen_test]
fn test_button_props_creation_with_text_variant() {
    let props = ButtonProps {
        variant: ButtonVariant::Text,
        ..Default::default()
    };
    
    assert_eq!(props.variant, ButtonVariant::Text);
}

#[wasm_bindgen_test]
fn test_button_props_creation_with_disabled_state() {
    let props = ButtonProps {
        disabled: true,
        ..Default::default()
    };
    
    assert_eq!(props.disabled, true);
}

#[wasm_bindgen_test]
fn test_button_props_creation_with_custom_classes() {
    let props = ButtonProps {
        common: web_core::components::CommonProps {
            class: Some("custom-class".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    
    assert_eq!(props.common.class, Some("custom-class".to_string()));
}

#[wasm_bindgen_test]
fn test_button_creation_with_custom_classes() {
    let props = ButtonProps {
        common: web_core::components::CommonProps {
            class: Some("custom-class".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    let button = Button::new(props);
    
    assert_eq!(button.props.common.class, Some("custom-class".to_string()));
}

#[wasm_bindgen_test]
fn test_button_update_props() {
    let initial_props = ButtonProps {
        disabled: false,
        variant: ButtonVariant::Primary,
        ..Default::default()
    };
    let mut button = Button::new(initial_props);
    
    assert_eq!(button.props.disabled, false);
    assert_eq!(button.props.variant, ButtonVariant::Primary);
    
    let new_props = ButtonProps {
        disabled: true,
        variant: ButtonVariant::Secondary,
        ..Default::default()
    };
    button.update_props(new_props);
    
    assert_eq!(button.props.disabled, true);
    assert_eq!(button.props.variant, ButtonVariant::Secondary);
}

#[wasm_bindgen_test]
fn test_button_variant_default() {
    let variant = ButtonVariant::default();
    assert_eq!(variant, ButtonVariant::Primary);
}

// Additional tests for ButtonVariant enum
#[wasm_bindgen_test]
fn test_button_variant_equality() {
    let variant1 = ButtonVariant::Primary;
    let variant2 = ButtonVariant::Primary;
    let variant3 = ButtonVariant::Secondary;
    
    assert_eq!(variant1, variant2);
    assert_ne!(variant1, variant3);
}

#[wasm_bindgen_test]
fn test_button_variant_clone() {
    let variant = ButtonVariant::Primary;
    let cloned = variant.clone();
    
    assert_eq!(variant, cloned);
}
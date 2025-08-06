//! Tests for the select component
//!
//! This module contains tests for the select component functionality.

use wasm_bindgen_test::*;
use web_core::components::{Select, SelectProps, SelectOption};
use web_core::components::base::CommonProps;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_select_creation_with_default_properties() {
    let props = SelectProps::default();
    let select = Select::new(props);
    
    assert_eq!(select.value(), "");
}

#[wasm_bindgen_test]
fn test_select_creation_with_custom_value() {
    let props = SelectProps {
        value: "test_value".to_string(),
        ..Default::default()
    };
    let select = Select::new(props);
    
    assert_eq!(select.value(), "test_value");
}

#[wasm_bindgen_test]
fn test_select_creation_with_options() {
    let options = vec![
        SelectOption {
            value: "option1".to_string(),
            label: "Option 1".to_string(),
            disabled: false,
        },
        SelectOption {
            value: "option2".to_string(),
            label: "Option 2".to_string(),
            disabled: true,
        },
    ];
    
    let props = SelectProps {
        options,
        ..Default::default()
    };
    let select = Select::new(props);
    
    assert_eq!(select.value(), "");
}

#[wasm_bindgen_test]
fn test_select_creation_with_placeholder() {
    let props = SelectProps {
        placeholder: Some("Select an option".to_string()),
        ..Default::default()
    };
    let select = Select::new(props);
    
    assert_eq!(select.value(), "");
}

#[wasm_bindgen_test]
fn test_select_creation_with_disabled_state() {
    let props = SelectProps {
        disabled: true,
        ..Default::default()
    };
    let select = Select::new(props);
    
    assert_eq!(select.value(), "");
}

#[wasm_bindgen_test]
fn test_select_creation_with_multiple_selection() {
    let props = SelectProps {
        multiple: true,
        ..Default::default()
    };
    let select = Select::new(props);
    
    assert_eq!(select.value(), "");
}

#[wasm_bindgen_test]
fn test_select_update_props() {
    let initial_props = SelectProps {
        value: "initial".to_string(),
        ..Default::default()
    };
    let mut select = Select::new(initial_props);
    
    assert_eq!(select.value(), "initial");
    
    let new_props = SelectProps {
        value: "updated".to_string(),
        ..Default::default()
    };
    select.update_props(new_props);
    
    assert_eq!(select.value(), "updated");
}

#[wasm_bindgen_test]
fn test_select_value_retrieval() {
    let props = SelectProps {
        value: "test_value".to_string(),
        ..Default::default()
    };
    let select = Select::new(props);
    
    assert_eq!(select.value(), "test_value");
}

#[wasm_bindgen_test]
fn test_select_option_creation() {
    let option = SelectOption {
        value: "option1".to_string(),
        label: "Option 1".to_string(),
        disabled: false,
    };
    
    assert_eq!(option.value, "option1");
    assert_eq!(option.label, "Option 1");
    assert_eq!(option.disabled, false);
}

#[wasm_bindgen_test]
fn test_select_option_partial_eq() {
    let option1 = SelectOption {
        value: "option1".to_string(),
        label: "Option 1".to_string(),
        disabled: false,
    };
    
    let option2 = SelectOption {
        value: "option1".to_string(),
        label: "Option 1".to_string(),
        disabled: false,
    };
    
    let option3 = SelectOption {
        value: "option2".to_string(),
        label: "Option 2".to_string(),
        disabled: true,
    };
    
    assert_eq!(option1, option2);
    assert_ne!(option1, option3);
}
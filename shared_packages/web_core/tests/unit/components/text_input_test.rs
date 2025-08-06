//! Tests for the text input component
//!
//! This module contains tests for the text input component functionality.

use wasm_bindgen_test::*;
use web_core::components::{TextInput, TextInputProps};
use web_core::components::base::CommonProps;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_text_input_creation() {
    let props = TextInputProps {
        common: CommonProps::default(),
        value: "test".to_string(),
        onchange: yew::Callback::default(),
        placeholder: "Enter text".to_string(),
        input_type: web_core::components::text_input::InputType::Text,
        disabled: false,
        required: false,
        max_length: None,
    };
    
    let text_input = TextInput::new(props);
    assert_eq!(text_input.value(), "test");
}

#[wasm_bindgen_test]
fn test_input_type_conversion() {
    use web_core::components::text_input::InputType;
    
    assert_eq!(InputType::Text.to_string(), "text");
    assert_eq!(InputType::Password.to_string(), "password");
    assert_eq!(InputType::Email.to_string(), "email");
    assert_eq!(InputType::Number.to_string(), "number");
    assert_eq!(InputType::Search.to_string(), "search");
    assert_eq!(InputType::Url.to_string(), "url");
    assert_eq!(InputType::Tel.to_string(), "tel");
}
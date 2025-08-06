# Button Component Test Plan

## Overview

This document outlines the test plan for the Button component in the web_core package. The Button component is implemented as a struct-based component that implements the BaseComponent trait, making it consistent with other components in the project and easier to test.

## Component Analysis

The Button component has the following properties:
- `children`: The text content to display on the button
- `onclick`: Callback when the button is clicked
- `disabled`: Whether the button is disabled
- `variant`: The styling variant (Primary, Secondary, Danger, Text)
- `class`: Additional CSS classes to apply

## Testing Approach

Since the Button component is a struct-based component that implements the BaseComponent trait, we'll follow the same testing approach as other components like Select. The tests will focus on:

1. Component rendering with various property combinations
2. Event handling (click callbacks)
3. Property updates
4. Styling verification

## Test Cases

### 1. Button Component Creation with Various Property Combinations

#### 1.1 Basic Creation with Default Properties
```rust
#[wasm_bindgen_test]
fn test_button_creation_with_default_properties() {
    // Create button with default values
    let props = ButtonProps::default();
    let button = Button::new(props);
    
    // Test that the button is created correctly
    assert_eq!(button.props.disabled, false);
    assert_eq!(button.props.variant, ButtonVariant::Primary);
}
```

#### 1.2 Creation with Different Variants
```rust
#[wasm_bindgen_test]
fn test_button_creation_with_primary_variant() {
    let props = ButtonProps {
        variant: ButtonVariant::Primary,
        ..Default::default()
    };
    let button = Button::new(props);
    
    assert_eq!(button.props.variant, ButtonVariant::Primary);
}

#[wasm_bindgen_test]
fn test_button_creation_with_secondary_variant() {
    let props = ButtonProps {
        variant: ButtonVariant::Secondary,
        ..Default::default()
    };
    let button = Button::new(props);
    
    assert_eq!(button.props.variant, ButtonVariant::Secondary);
}

#[wasm_bindgen_test]
fn test_button_creation_with_danger_variant() {
    let props = ButtonProps {
        variant: ButtonVariant::Danger,
        ..Default::default()
    };
    let button = Button::new(props);
    
    assert_eq!(button.props.variant, ButtonVariant::Danger);
}

#[wasm_bindgen_test]
fn test_button_creation_with_text_variant() {
    let props = ButtonProps {
        variant: ButtonVariant::Text,
        ..Default::default()
    };
    let button = Button::new(props);
    
    assert_eq!(button.props.variant, ButtonVariant::Text);
}
```

#### 1.3 Creation with Disabled State
```rust
#[wasm_bindgen_test]
fn test_button_creation_with_disabled_state() {
    let props = ButtonProps {
        disabled: true,
        ..Default::default()
    };
    let button = Button::new(props);
    
    assert_eq!(button.props.disabled, true);
}
```

#### 1.4 Creation with Custom CSS Classes
```rust
#[wasm_bindgen_test]
fn test_button_creation_with_custom_classes() {
    let props = ButtonProps {
        common: CommonProps {
            class: Some("custom-class".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    let button = Button::new(props);
    
    assert_eq!(button.props.common.class, Some("custom-class".to_string()));
}
```

### 2. Property Updates

#### 2.1 Testing Property Updates
```rust
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
```

### 3. Component Rendering

#### 3.1 Testing Rendering with Different Property Combinations
```rust
#[wasm_bindgen_test]
fn test_button_rendering_with_all_properties() {
    // Test that the button renders correctly with all properties set
    let props = ButtonProps {
        children: yew::Children::new(vec![Html::from("Click me")]),
        disabled: true,
        variant: ButtonVariant::Primary,
        common: CommonProps {
            class: Some("custom-class".to_string()),
            ..Default::default()
        },
    };
    
    let button = Button::new(props);
    let html = button.view();
    
    // Verify the rendered output
    // This would require a DOM testing utility to verify the actual HTML output
}
```

## Implementation Considerations

1. **Testing Struct-Based Components**: Since the Button is now a struct-based component that implements the BaseComponent trait, we can test it using the same patterns as other components in the project.

2. **Event Testing**: We'll need to simulate click events and verify that callbacks are properly triggered.

3. **Styling Verification**: We'll need to check that the correct CSS classes are applied based on the variant and other properties.

4. **Accessibility**: We should verify that the button has appropriate accessibility attributes.

## Test File Structure

The tests will be implemented in `shared_packages/web_core/tests/unit/components/button_test.rs` and will follow the same pattern as the existing component tests:

```rust
//! Tests for the button component
//!
//! This module contains tests for the button component functionality.

use wasm_bindgen_test::*;
use web_core::components::{Button, ButtonProps, ButtonVariant};

wasm_bindgen_test_configure!(run_in_browser);

// Test cases here
```

The test module will be added to `shared_packages/web_core/tests/unit/components/mod.rs`:

```rust
//! Unit tests for web core components

#[cfg(test)]
mod select_test;
#[cfg(test)]
mod text_input_test;
#[cfg(test)]
mod button_test;
```

## Dependencies

The tests will depend on:
- `wasm_bindgen_test` for WASM testing
- `web_core::components::{Button, ButtonProps, ButtonVariant}` for the component being tested

## Future Considerations

The Button component has now been refactored to implement the BaseComponent trait, which standardizes the testing approach across all components. This makes it easier to test component creation, property updates, and rendering.
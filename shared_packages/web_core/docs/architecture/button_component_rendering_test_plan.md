# Button Component Rendering Test Plan

## Overview

This document outlines a plan for implementing rendering tests for the Button component in the web_core package. These tests would verify that the Button component renders correctly in the DOM and responds to user interactions as expected.

## Update

The Button component has been refactored to implement the `BaseComponent` trait, which makes it easier to test component creation, property updates, and rendering. This document now outlines how to implement comprehensive rendering tests for the Button component.

## Current State

The Button component is now implemented as a struct-based component that implements the `BaseComponent` trait. This makes it consistent with other components in the project and allows for easier testing of:

1. Component creation with `Button::new()`
2. Property updates with `update_props()`
3. Rendering by calling `view()` and verifying the output

## Implementation Approach

To implement proper rendering tests for the Button component, we can now:

### 1. Test Component Rendering

Verify that the component renders with the correct HTML structure and classes by calling the `view()` method:

```rust
#[wasm_bindgen_test]
fn test_button_renders_correctly() {
    let props = ButtonProps {
        children: yew::Children::new(vec![Html::from("Click me")]),
        variant: ButtonVariant::Primary,
        ..Default::default()
    };
    
    let button = Button::new(props);
    let html = button.view();
    
    // Verify the rendered output
    // This would require a DOM testing utility to verify the actual HTML output
}
```


### 3. Test Event Handling

Verify that event handlers work correctly:

```rust
#[wasm_bindgen_test]
fn test_button_click_handler() {
    let clicked = Rc::new(Cell::new(false));
    let clicked_clone = clicked.clone();
    
    let callback = Callback::from(move |_| {
        clicked_clone.set(true);
    });
    
    let props = ButtonProps {
        children: yew::Children::new(vec![Html::from("Click me")]),
        onclick: callback,
        ..Default::default()
    };
    
    let button = Button::new(props);
    let html = button.view();
    
    // Simulate a click event on the rendered HTML
    // This would require a DOM testing utility to simulate the click
    
    // Verify the callback was called
    assert!(clicked.get());
}
```

### 4. Test Property Updates

Verify that the component correctly reflects property changes:

```rust
#[wasm_bindgen_test]
fn test_button_property_updates() {
    // Create button with initial props
    let initial_props = ButtonProps {
        disabled: false,
        ..Default::default()
    };
    let mut button = Button::new(initial_props);
    
    // Update props
    let new_props = ButtonProps {
        disabled: true,
        ..Default::default()
    };
    button.update_props(new_props);
    
    // Verify the button reflects the new props
    assert_eq!(button.props.disabled, true);
    
    // Verify the rendered output reflects the new props
    let html = button.view();
    // This would require a DOM testing utility to verify the actual HTML output
}
```

## Dependencies

To implement these tests, we would need:

1. A Yew component testing framework or utility
2. DOM manipulation utilities for the test environment
3. Event simulation capabilities

## Recommendation

The Button component has already been refactored to implement the `BaseComponent` trait, which makes it easier to test. We should now implement comprehensive rendering tests that:

1. Verify the component renders with the correct HTML structure and classes
2. Test event handling by simulating user interactions
3. Verify property updates are correctly reflected in the rendered output

To implement these tests, we should:

1. Evaluate existing Yew testing libraries like `yew-agent` or `yew-test`
2. Consider implementing a custom testing utility for our specific needs
3. Implement the rendering tests following the patterns outlined in this document
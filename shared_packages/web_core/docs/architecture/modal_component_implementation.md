# Modal Component Implementation Plan

This document provides detailed implementation instructions for refactoring the Modal component to implement the BaseComponent trait.

## Overview

The Modal component needs to be refactored from a functional component to a struct-based component that implements the BaseComponent trait, following the same pattern as Button, Select, and TextInput components.

## Implementation Steps

### 1. Update ModalProps Structure

First, update the ModalProps struct to include CommonProps and derive Clone:

```rust
/// Properties for the Modal component
#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    /// Common properties
    ///
    /// These are common properties that all components support.
    #[prop_or_default]
    pub common: CommonProps,
    
    /// Whether the modal is open
    pub open: bool,
    
    /// Callback when the modal is closed
    #[prop_or_default]
    pub onclose: Callback<()>,
    
    /// The title of the modal
    #[prop_or_default]
    pub title: String,
    
    /// The content to display in the modal
    #[prop_or_default]
    pub children: Children,
    
    /// Whether to show the close button
    #[prop_or_default]
    pub show_close_button: bool,
}
```

### 2. Refactor Component to Struct-Based Implementation

Replace the functional component with a struct-based implementation:

```rust
/// A reusable modal component
#[styled_component(Modal)]
pub struct Modal {
    props: ModalProps,
}

impl BaseComponent for Modal {
    type Properties = ModalProps;
    
    fn create(props: &Self::Properties) -> Self {
        Self { props: props.clone() }
    }
    
    fn update_props(&mut self, props: Self::Properties) {
        self.props = props;
    }
    
    fn view(&self) -> Html {
        // Implementation will be moved from the functional component
    }
}

impl Modal {
    /// Create a new modal component
    pub fn new(props: ModalProps) -> Self {
        Self::create(&props)
    }
}
```

### 3. Move Implementation to view() Method

Move the existing implementation from the functional component to the view() method, making necessary adjustments:

```rust
fn view(&self) -> Html {
    let modal_style = style!(
        r#"
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 1000;
    "#
    );
    
    let modal_content_style = style!(
        r#"
        background-color: white;
        border-radius: 0.5rem;
        box-shadow: 0 0.5rem 1rem rgba(0, 0, 0, 0.15);
        max-width: 500px;
        width: 90%;
        max-height: 90vh;
        overflow-y: auto;
    "#
    );
    
    let modal_header_style = style!(
        r#"
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1rem 1.5rem;
        border-bottom: 1px solid #e9ecef;
    "#
    );
    
    let modal_title_style = style!(
        r#"
        margin: 0;
        font-size: 1.25rem;
        font-weight: 500;
    "#
    );
    
    let modal_close_button_style = style!(
        r#"
        background: none;
        border: none;
        font-size: 1.5rem;
        cursor: pointer;
        color: #6c757d;
        
        &:hover {
            color: #000;
        }
    "#
    );
    
    let modal_body_style = style!(
        r#"
        padding: 1.5rem;
    "#
    );
    
    let on_close = {
        let onclose = self.props.onclose.clone();
        Callback::from(move |_| onclose.emit(()))
    };
    
    if !self.props.open {
        return html! {};
    }
    
    let classes = classes!(
        modal_style.get_class_name(),
        self.props.common.class.clone()
    );
    
    html! {
        <div class={classes}>
            <div class={modal_content_style.get_class_name()}>
                <div class={modal_header_style.get_class_name()}>
                    <h2 class={modal_title_style.get_class_name()}>{ &self.props.title }</h2>
                    if self.props.show_close_button {
                        <button
                            class={modal_close_button_style.get_class_name()}
                            onclick={on_close.clone()}
                        >
                            {"×"}
                        </button>
                    }
                </div>
                <div class={modal_body_style.get_class_name()}>
                    { for self.props.children.iter() }
                </div>
            </div>
        </div>
    }
}
```

### 4. Key Implementation Details

1. **Conditional Rendering**: The conditional check for `self.props.open` should return `html! {}` when the modal is not open.

2. **Event Handling**: The `on_close` callback should properly clone and emit the callback.

3. **CommonProps Integration**: The common classes should be applied to the root element using the `classes!` macro.

4. **Styling**: All styling should remain the same, using the stylist crate as before.

### 5. Testing Implementation

Create a test file `modal_test.rs` in the `tests/unit/components/` directory:

```rust
//! Tests for the modal component
//!
//! This module contains tests for the modal component functionality.

use wasm_bindgen_test::*;
use web_core::components::{Modal, ModalProps};
use web_core::components::base::CommonProps;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_modal_creation_with_default_properties() {
    let props = ModalProps::default();
    let modal = Modal::new(props);
    
    assert_eq!(modal.props.open, false);
    assert_eq!(modal.props.title, "");
    assert!(modal.props.children.is_empty());
    assert_eq!(modal.props.show_close_button, false);
    assert!(modal.props.onclose.is_empty());
    assert!(modal.props.common.class.is_none());
}

#[wasm_bindgen_test]
fn test_modal_creation_with_custom_properties() {
    let children = yew::Children::new(vec![html! { <div>{"Test content"}</div> }]);
    
    let props = ModalProps {
        open: true,
        title: "Test Modal".to_string(),
        children: children.clone(),
        show_close_button: true,
        onclose: yew::Callback::default(),
        common: CommonProps::default(),
    };
    
    let modal = Modal::new(props);
    
    assert_eq!(modal.props.open, true);
    assert_eq!(modal.props.title, "Test Modal");
    assert_eq!(modal.props.children.len(), 1);
    assert_eq!(modal.props.show_close_button, true);
}

#[wasm_bindgen_test]
fn test_modal_update_props() {
    let initial_props = ModalProps {
        open: false,
        title: "Initial".to_string(),
        ..Default::default()
    };
    let mut modal = Modal::new(initial_props);
    
    assert_eq!(modal.props.open, false);
    assert_eq!(modal.props.title, "Initial");
    
    let new_props = ModalProps {
        open: true,
        title: "Updated".to_string(),
        ..Default::default()
    };
    modal.update_props(new_props);
    
    assert_eq!(modal.props.open, true);
    assert_eq!(modal.props.title, "Updated");
}

#[wasm_bindgen_test]
fn test_modal_common_props_integration() {
    let props = ModalProps {
        common: CommonProps {
            class: Some("custom-class".to_string()),
            id: Some("modal-id".to_string()),
            disabled: false,
            style: Some("color: red;".to_string()),
        },
        ..Default::default()
    };
    
    let modal = Modal::new(props);
    
    assert_eq!(modal.props.common.class, Some("custom-class".to_string()));
    assert_eq!(modal.props.common.id, Some("modal-id".to_string()));
    assert_eq!(modal.props.common.style, Some("color: red;".to_string()));
}
```

### 6. Update Test Module

Add the modal test module to `tests/unit/components/mod.rs`:

```rust
#[cfg(test)]
mod modal_test;
```

## Implementation Considerations

### Backward Compatibility
- Ensure the component can still be used in the same way as before
- Maintain the same property names and types (with addition of CommonProps)
- Preserve all existing functionality

### Performance
- Profile the component to ensure no performance regressions
- Optimize styling if needed
- Ensure efficient rendering

### Error Handling
- Handle edge cases gracefully
- Provide clear error messages for developers
- Maintain proper error boundaries

## Code Quality

### Documentation
- Update all doc comments to match the new implementation
- Provide clear examples of usage
- Document all public methods and properties

### Code Style
- Follow the existing code style in the project
- Use consistent naming conventions
- Maintain proper code organization

## Testing Requirements

### Test Coverage
- Achieve at least 80% code coverage
- Test all public methods
- Test all property combinations
- Test edge cases and error conditions

### Test Execution
- All tests should pass in the browser environment
- Tests should be deterministic
- Test execution should be efficient

## Review Process

### Code Review
- Review implementation against this plan
- Verify adherence to architectural principles
- Check for potential issues or improvements

### Testing Review
- Verify all tests pass
- Check test coverage reports
- Review test quality and completeness

## Deployment

### Integration
- Ensure the component integrates properly with the rest of the system
- Verify compatibility with existing components
- Test in example applications

### Documentation
- Update any relevant documentation
- Provide migration guide if needed
- Ensure examples are up to date

## Timeline

The implementation should be completed within one development cycle, including:
- Implementation: 2 days
- Testing: 1 day
- Review and refinement: 1 day
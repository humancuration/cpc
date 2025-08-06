# Modal Component Refactor Plan

This document outlines the detailed architectural plan for refactoring the Modal component to implement the BaseComponent trait, following the same pattern as other components in the web_core library.

## Current State Analysis

The Modal component is currently implemented as a functional component using the `#[styled_component]` attribute:

```rust
#[styled_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    // Implementation
}
```

The current properties are:

```rust
#[derive(Properties, PartialEq)]
pub struct ModalProps {
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

## Proposed Enhancement

The Modal component will be refactored to implement the BaseComponent trait following the same pattern as Button, Select, and TextInput components.

### Component Structure

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
        // Implementation moved from the functional component
    }
}

impl Modal {
    /// Create a new modal component
    pub fn new(props: ModalProps) -> Self {
        Self::create(&props)
    }
}
```

### Implementation Changes

1. **Add CommonProps to ModalProps**: To align with other components, the ModalProps struct will be updated to include CommonProps:

```rust
#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    /// Common properties
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

2. **Convert to Struct-Based Component**: The component will be converted from a functional component to a struct-based component that implements BaseComponent.

3. **Move Implementation to view() Method**: The existing implementation will be moved from the functional component to the view() method of the BaseComponent implementation.

4. **Add new() Constructor Method**: A new() method will be added for easier instantiation.

5. **Add Clone Derive**: The ModalProps struct will derive Clone to support the BaseComponent trait requirements.

## Implementation Considerations

### Backward Compatibility
- The existing API will be maintained for backward compatibility
- The component will still be usable in the same way as before
- Property names and types will remain the same (with the addition of CommonProps)

### Styling Consistency
- The existing styling implementation will be preserved
- Styling will be moved from the functional component to the view() method
- CSS classes will be properly applied using the stylist crate

### Conditional Rendering
- The conditional rendering based on the `open` property will be handled in the view() method
- When the modal is not open, the view() method will return an empty Html fragment: `html! {}`

### Event Handling
- Event handling for the close button will be maintained
- The on_close callback will be properly implemented in the view() method

### Accessibility
- The modal will maintain proper accessibility attributes
- Focus management will be preserved
- Keyboard navigation will continue to work as expected

## Component API

### Properties

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| common | CommonProps | No | Common properties for all components |
| open | bool | Yes | Whether the modal is open |
| onclose | Callback<()> | No | Callback when the modal is closed |
| title | String | No | The title of the modal |
| children | Children | No | The content to display in the modal |
| show_close_button | bool | No | Whether to show the close button |

### Methods

| Method | Description |
|--------|-------------|
| new(props: ModalProps) -> Self | Create a new modal component |
| create(props: &ModalProps) -> Self | Create a new modal component (BaseComponent trait) |
| update_props(props: ModalProps) | Update the component properties (BaseComponent trait) |
| view() -> Html | Render the component (BaseComponent trait) |

## Testing Approach

### Unit Tests

1. **Component Creation Tests**:
   - Test creating a modal with default properties
   - Test creating a modal with custom properties
   - Test creating a modal with common properties (classes, IDs, etc.)

2. **Property Update Tests**:
   - Test updating modal properties
   - Test updating the open state
   - Test updating the title
   - Test updating children content

3. **Rendering Tests**:
   - Test that the modal renders when open is true
   - Test that the modal doesn't render when open is false
   - Test that the title is displayed correctly
   - Test that children are rendered correctly
   - Test that the close button is displayed when show_close_button is true
   - Test that the close button is not displayed when show_close_button is false

4. **Event Handling Tests**:
   - Test that the onclose callback is called when the close button is clicked
   - Test that the onclose callback is not called when the modal is not open

### Integration Tests

1. **Theming Integration**:
   - Test that common properties (classes, IDs, etc.) are applied correctly
   - Test that the modal works with the theme system

2. **Component Composition**:
   - Test that the modal works correctly with other components as children
   - Test that nested modals work correctly

### Accessibility Tests

1. **Keyboard Navigation**:
   - Test that the close button can be focused
   - Test that the close button can be activated with the keyboard

2. **Screen Reader Support**:
   - Test that the modal has appropriate ARIA attributes
   - Test that the title is properly associated with the modal

### Visual Regression Tests

1. **Styling Consistency**:
   - Test that the modal renders consistently across different themes
   - Test that the modal styling matches the design system

2. **Responsive Behavior**:
   - Test that the modal is responsive
   - Test that the modal works on different screen sizes

## Implementation Steps

1. **Update ModalProps**:
   - Add CommonProps field
   - Add Clone derive
   - Update documentation

2. **Refactor Component Implementation**:
   - Convert from functional component to struct-based component
   - Implement BaseComponent trait
   - Move existing implementation to view() method
   - Add new() constructor method

3. **Update Styling**:
   - Ensure styling is properly applied in the view() method
   - Apply common classes from CommonProps

4. **Testing**:
   - Create unit tests for the refactored component
   - Ensure all existing functionality is preserved
   - Add tests for new features (CommonProps integration)

5. **Documentation**:
   - Update component documentation
   - Update examples if necessary

## Benefits

1. **Consistency**: The Modal component will follow the same pattern as other components in the library
2. **Extensibility**: The component will be easier to extend with new features
3. **Maintainability**: The component will be easier to maintain and debug
4. **Theming Support**: The component will properly support the theme system through CommonProps
5. **Testability**: The component will be easier to test in isolation

## Risks and Mitigations

1. **Breaking Changes**: 
   - Risk: Changes might break existing usage
   - Mitigation: Maintain backward compatibility and thoroughly test existing functionality

2. **Performance Impact**:
   - Risk: Struct-based implementation might have different performance characteristics
   - Mitigation: Profile the component and optimize if necessary

3. **Testing Gaps**:
   - Risk: Some edge cases might not be covered by tests
   - Mitigation: Implement comprehensive test coverage including edge cases

## Dependencies

- web_core::components::base::{BaseComponent, CommonProps}
- yew::prelude::*
- stylist::{style, yew::styled_component}

## Timeline

The implementation is expected to be completed in one development cycle, with testing and documentation updates included.
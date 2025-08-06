# Component Library

The web_core component library provides a set of reusable UI components for building CPC web applications.

## Base Component

All components in the library implement the `BaseComponent` trait, which defines a common interface:

```rust
pub trait BaseComponent: Sized {
    type Properties: Properties;
    
    fn create(props: &Self::Properties) -> Self;
    fn update_props(&mut self, props: Self::Properties);
    fn view(&self) -> Html;
}
```

## Available Components

### Button

A flexible button component with multiple variants:

```rust
use web_core::components::{Button, ButtonVariant};

html! {
    <Button 
        variant={ButtonVariant::Primary} 
        onclick={on_click}
    >
        {"Click me"}
    </Button>
}
```

### Modal

A modal dialog component:

```rust
use web_core::components::Modal;

html! {
    <Modal 
        open={is_open}
        onclose={on_close}
        title="My Modal"
    >
        <p>{"This is the modal content"}</p>
    </Modal>
}
```

### ErrorBoundary

An error boundary component that catches and handles errors in child components:

```rust
use web_core::components::ErrorBoundary;

html! {
    <ErrorBoundary on_error={on_error}>
        <MyComponent />
    </ErrorBoundary>
}
```

## Creating Custom Components

To create a custom component that follows the library patterns:

1. Implement the `BaseComponent` trait
2. Use the `CommonProps` struct for common properties
3. Follow the theming guidelines
4. Implement proper error handling

```rust
use web_core::components::{BaseComponent, CommonProps};
use yew::prelude::*;

pub struct MyComponent {
    props: MyComponentProps,
}

#[derive(Properties, PartialEq, Clone)]
pub struct MyComponentProps {
    #[prop_or_default]
    pub common: CommonProps,
    pub value: String,
}

impl BaseComponent for MyComponent {
    type Properties = MyComponentProps;
    
    fn create(props: &Self::Properties) -> Self {
        Self { props: props.clone() }
    }
    
    fn update_props(&mut self, props: Self::Properties) {
        self.props = props;
    }
    
    fn view(&self) -> Html {
        html! {
            <div class={self.props.common.class.clone()}>
                { &self.props.value }
            </div>
        }
    }
}
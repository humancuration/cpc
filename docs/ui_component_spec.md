# Cross-Platform UI Component Specification

## Design Principles
1. **Adaptive Rendering**: Components should render appropriately for web/native contexts
2. **Theme Propagation**: Use CSS variables for theme consistency
3. **Responsive First**: Design for mobile, tablet, and desktop breakpoints
4. **Accessibility**: Follow WCAG 2.1 AA standards

## Core Components
| Component       | Web Implementation       | Desktop Implementation  | Shared Props           |
|-----------------|--------------------------|-------------------------|------------------------|
| Button          | `<button>`               | Tauri-native button     | `variant`, `size`, `on_click` |
| Input           | `<input>`                | Custom-drawn input      | `value`, `on_change`, `placeholder` |
| Card            | `<div>` with CSS         | Custom-drawn container  | `elevation`, `padding` |
| Navigation      | Yew Router               | Tray-based navigation   | `routes`, `current_path` |
| Icon            | SVG inline               | System icon library     | `name`, `size`, `color` |

## Implementation Guidelines
```rust
// Example: Adaptive Button Component
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub label: String,
    pub on_click: Callback<()>,
    #[prop_or_default]
    pub variant: ButtonVariant,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    // Web implementation
    #[cfg(target_arch = "wasm32")]
    return html! {
        <button class={classes!("btn", props.variant.as_class())} 
                onclick={props.on_click.reform(|_| ())}>
            { &props.label }
        </button>
    };

    // Desktop implementation
    #[cfg(not(target_arch = "wasm32"))]
    return html! {
        <TauriButton label={props.label.clone()} 
                     on_click={props.on_click.clone()} 
                     variant={props.variant} />
    };
}
```

## Theming System
```css
:root {
  --primary-color: #0066cc;
  --secondary-color: #e6f2ff;
  --border-radius: 4px;
  --shadow: 0 2px 4px rgba(0,0,0,0.1);
}

/* Desktop overrides */
.desktop {
  --border-radius: 6px;
  --shadow: 0 4px 8px rgba(0,0,0,0.15);
}
```

## Testing Requirements
1. Visual regression tests for both platforms
2. Interaction tests using simulated events
3. Accessibility audits (axe-core integration)
# UI Toolkit

A comprehensive UI component library for CPC applications built with Rust and Yew.

## Features

- **Theme System**: Comprehensive light/dark mode support with system preference detection
- **Responsive Design**: Mobile-first approach with breakpoints and responsive components
- **Accessible Components**: WCAG compliant UI components
- **Consistent Styling**: Design system with color palettes, typography, and spacing scales
- **Cross-Platform**: Works on both web and desktop platforms

## Theme System

The UI toolkit provides a powerful theme system that supports:

### Color Schemes

- Light mode
- Dark mode
- System preference detection

### Theme Provider

The `ThemeProvider` component manages the theme state and provides it to child components through React context.

```rust
use ui_toolkit::components::theme_provider::ThemeProvider;

html! {
    <ThemeProvider>
        // Your app content here
    </ThemeProvider>
}
```

### Using Themes in Components

Components can access the theme through the `use_theme` hook:

```rust
use ui_toolkit::hooks::use_theme::use_theme;

#[function_component(MyComponent)]
fn my_component() -> Html {
    let theme_context = use_theme();
    
    // Access theme properties
    let primary_color = &theme_context.theme_manager.design_system.colors.primary;
    
    // Toggle theme
    let onclick = {
        let toggle_theme = theme_context.toggle_theme.clone();
        Callback::from(move |_| toggle_theme.emit(()))
    };
    
    html! {
        <button onclick={onclick}>
            { "Toggle Theme" }
        </button>
    }
}
```

### CSS Variables

The theme system generates CSS variables that can be used directly in component styles:

- `--cpc-primary`: Primary color
- `--cpc-secondary`: Secondary color
- `--cpc-success`: Success color
- `--cpc-warning`: Warning color
- `--cpc-danger`: Danger color
- `--cpc-info`: Info color
- `--cpc-text`: Text color
- `--cpc-background`: Background color
- `--cpc-surface`: Surface color (for cards, etc.)
- `--cpc-border`: Border color
- `--cpc-spacing-*`: Spacing scale variables
- `--cpc-font-size-*`: Font size variables
- `--cpc-border-radius-*`: Border radius variables

## Components

- Button
- Card
- Container
- Input
- And more...

## Hooks

- `use_theme`: Access the current theme
- `use_platform`: Detect web/desktop platform
- `use_breakpoint`: Detect screen size breakpoints
- `use_media_query`: Custom media query detection

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ui_toolkit = { path = "../shared_packages/ui_toolkit" }
```

## Usage

```rust
use yew::prelude::*;
use ui_toolkit::components::*;
use ui_toolkit::hooks::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <ThemeProvider>
            <Container>
                <Card>
                    <h1>{"Hello, World!"}</h1>
                    <Button>{"Click me"}</Button>
                </Card>
            </Container>
        </ThemeProvider>
    }
}
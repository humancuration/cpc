# Theme System

The web_core theme system provides a consistent design language across all CPC web applications.

## DesignSystem

The `DesignSystem` struct defines the visual language for the application:

```rust
use web_core::theme::DesignSystem;

let theme = DesignSystem::default();

// Access color palette
let primary_color = &theme.colors.primary;

// Access spacing scale
let medium_spacing = &theme.spacing.md;

// Access typography
let font_family = &theme.typography.font_family;
```

## Color Palette

The color palette includes semantic colors for different purposes:

```rust
use web_core::theme::DesignSystem;

let theme = DesignSystem::default();

// Primary action colors
let primary = &theme.colors.primary;    // #007bff
let secondary = &theme.colors.secondary; // #6c757d

// Status colors
let success = &theme.colors.success;    // #28a745
let warning = &theme.colors.warning;    // #ffc107
let danger = &theme.colors.danger;      // #dc3545
let info = &theme.colors.info;          // #17a2b8

// Neutral colors
let light = &theme.colors.light;        // #f8f9fa
let dark = &theme.colors.dark;          // #343a40
let white = &theme.colors.white;        // #ffffff
let black = &theme.colors.black;        // #000000

// Gray scale
let gray_100 = &theme.colors.gray.100;  // #f8f9fa
let gray_500 = &theme.colors.gray.500;  // #adb5bd
let gray_900 = &theme.colors.gray.900;  // #212529
```

## Spacing Scale

The spacing scale provides consistent spacing values:

```rust
use web_core::theme::DesignSystem;

let theme = DesignSystem::default();

let xs = &theme.spacing.xs;   // 0.25rem (4px)
let sm = &theme.spacing.sm;   // 0.5rem (8px)
let md = &theme.spacing.md;   // 1rem (16px)
let lg = &theme.spacing.lg;   // 1.5rem (24px)
let xl = &theme.spacing.xl;   // 2rem (32px)
let xxl = &theme.spacing.xxl; // 3rem (48px)
```

## Typography

The typography system defines font families, sizes, and weights:

```rust
use web_core::theme::DesignSystem;

let theme = DesignSystem::default();

// Font family
let font_family = &theme.typography.font_family;

// Font sizes
let font_xs = &theme.typography.font_sizes.xs;   // 0.75rem (12px)
let font_sm = &theme.typography.font_sizes.sm;   // 0.875rem (14px)
let font_md = &theme.typography.font_sizes.md;   // 1rem (16px)
let font_lg = &theme.typography.font_sizes.lg;   // 1.125rem (18px)
let font_xl = &theme.typography.font_sizes.xl;   // 1.25rem (20px)
let font_xxl = &theme.typography.font_sizes.xxl; // 1.5rem (24px)
let font_xxxl = &theme.typography.font_sizes.xxxl; // 2rem (32px)

// Font weights
let regular = theme.typography.font_weights.regular; // 400
let bold = theme.typography.font_weights.bold;       // 700

// Line heights
let normal = theme.typography.line_heights.normal;   // 1.5
```

## Border Radius

Border radius values for consistent corner styling:

```rust
use web_core::theme::DesignSystem;

let theme = DesignSystem::default();

let radius_sm = &theme.border_radius.sm;  // 0.25rem (4px)
let radius_md = &theme.border_radius.md;  // 0.5rem (8px)
let radius_lg = &theme.border_radius.lg;  // 0.75rem (12px)
let radius_xl = &theme.border_radius.xl;  // 1rem (16px)
let radius_full = &theme.border_radius.full; // 9999px
```

## Shadows

Shadow values for depth and elevation:

```rust
use web_core::theme::DesignSystem;

let theme = DesignSystem::default();

let shadow_sm = &theme.shadows.sm;  // 0 0.125rem 0.25rem rgba(0, 0, 0, 0.075)
let shadow_md = &theme.shadows.md;  // 0 0.5rem 1rem rgba(0, 0, 0, 0.15)
let shadow_lg = &theme.shadows.lg;  // 0 1rem 3rem rgba(0, 0, 0, 0.175)
let shadow_xl = &theme.shadows.xl;  // 0 2rem 4rem rgba(0, 0, 0, 0.2)
```

## Using Themes in Components

Components can use the theme system to ensure consistent styling:

```rust
use web_core::theme::DesignSystem;
use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[styled_component(MyComponent)]
fn my_component() -> Html {
    let theme = DesignSystem::default();
    
    let component_style = style!(
        r#"
        background-color: ${primary_color};
        color: ${text_color};
        padding: ${padding};
        border-radius: ${border_radius};
        font-family: ${font_family};
        font-size: ${font_size};
        "#,
        primary_color = theme.colors.primary,
        text_color = theme.colors.white,
        padding = theme.spacing.md,
        border_radius = theme.border_radius.md,
        font_family = theme.typography.font_family,
        font_size = theme.typography.font_sizes.md,
    );
    
    html! {
        <div class={component_style.get_class_name()}>
            {"My themed component"}
        </div>
    }
}
```

## Customizing Themes

You can create custom themes by modifying the default design system:

```rust
use web_core::theme::DesignSystem;

let mut theme = DesignSystem::default();

// Customize colors
theme.colors.primary = "#ff6b6b".to_string();
theme.colors.secondary = "#4ecdc4".to_string();

// Customize spacing
theme.spacing.md = "1.25rem".to_string();

// Customize typography
theme.typography.font_family = "Inter, system-ui, sans-serif".to_string();
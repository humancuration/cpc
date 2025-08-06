# Theme System Extensions Architecture

This document outlines the architectural plans for extending the web_core theme system with dark mode support, customization options, and responsive design integration.

## Task 1: Dark Mode Support

### Current State
The current DesignSystem only supports a single color palette without explicit dark mode support. Components use fixed color values from the theme.

### Proposed Changes
Implement a comprehensive dark mode solution that includes:
1. Separate color palettes for light and dark modes
2. Automatic mode detection based on system preferences
3. Manual mode switching capability
4. Smooth transitions between modes
5. Component-level theme adaptation

### Implementation Plan
1. Extend the DesignSystem struct to support multiple color schemes
2. Create separate light and dark color palettes
3. Implement theme switching mechanisms
4. Add CSS variables for dynamic theme switching
5. Update components to adapt to theme changes
6. Maintain backward compatibility with existing code

### Extended DesignSystem Structure
```rust
pub struct DesignSystem {
    /// Active color scheme (light or dark)
    pub color_scheme: ColorScheme,
    
    /// Color palette for light mode
    pub light_colors: ColorPalette,
    
    /// Color palette for dark mode
    pub dark_colors: ColorPalette,
    
    /// Currently active color palette (based on color_scheme)
    pub colors: ColorPalette,
    
    /// Spacing scale
    pub spacing: SpacingScale,
    
    /// Typography settings
    pub typography: Typography,
    
    /// Border radius values
    pub border_radius: BorderRadius,
    
    /// Shadow values
    pub shadows: Shadows,
}
```

### ColorScheme Enum
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ColorScheme {
    Light,
    Dark,
    System, // Follow system preference
}
```

### Theme Switching Mechanism
1. System preference detection using `prefers-color-scheme` media query
2. Manual override through API calls
3. Persistent storage of user preference
4. Smooth CSS transitions for theme changes

### CSS Variable Implementation
```css
:root {
  --cpc-primary: #007bff;
  --cpc-secondary: #6c757d;
  --cpc-background: #ffffff;
  --cpc-text: #212529;
  /* ... other variables ... */
}

[data-theme="dark"] {
  --cpc-primary: #0d6efd;
  --cpc-secondary: #6c757d;
  --cpc-background: #121212;
  --cpc-text: #e0e0e0;
  /* ... other variables ... */
}
```

### Benefits
- Improved accessibility for users with visual impairments
- Reduced eye strain in low-light environments
- Modern UI/UX expectations met
- Consistent experience across all components

## Task 2: Theme Customization Options

### Current State
The current theme system allows modification of individual values but doesn't provide a structured way to create custom themes or override default values systematically.

### Proposed Changes
Implement a comprehensive theme customization system that allows:
1. Complete theme overrides
2. Partial theme modifications
3. Theme composition and inheritance
4. Runtime theme switching
5. User-defined theme registration

### Implementation Plan
1. Create a ThemeManager for handling multiple themes
2. Implement theme merging and inheritance
3. Add theme validation
4. Provide hooks for runtime theme changes
5. Create utilities for theme customization

### ThemeManager Structure
```rust
pub struct ThemeManager {
    /// Registered themes
    themes: HashMap<String, DesignSystem>,
    
    /// Active theme name
    active_theme: String,
    
    /// User overrides
    overrides: Option<DesignSystem>,
}

impl ThemeManager {
    /// Register a new theme
    pub fn register_theme(&mut self, name: String, theme: DesignSystem) {
        self.themes.insert(name, theme);
    }
    
    /// Set the active theme
    pub fn set_theme(&mut self, name: &str) -> Result<(), ThemeError> {
        if self.themes.contains_key(name) {
            self.active_theme = name.to_string();
            Ok(())
        } else {
            Err(ThemeError::ThemeNotFound(name.to_string()))
        }
    }
    
    /// Apply partial overrides to the current theme
    pub fn apply_overrides(&mut self, overrides: DesignSystem) {
        self.overrides = Some(overrides);
    }
    
    /// Get the current effective theme
    pub fn get_theme(&self) -> DesignSystem {
        let base_theme = self.themes.get(&self.active_theme)
            .cloned()
            .unwrap_or_default();
            
        if let Some(overrides) = &self.overrides {
            self.merge_themes(base_theme, overrides.clone())
        } else {
            base_theme
        }
    }
}
```

### Theme Composition
1. Base themes (light, dark)
2. Brand themes (company-specific color schemes)
3. User themes (personal preferences)
4. Context themes (temporary overrides for specific views)

### Theme Merging Strategy
1. Deep merge of theme objects
2. Priority system (user > context > brand > base)
3. Validation of merged themes
4. Performance optimization for theme retrieval

### Benefits
- Flexibility for different branding requirements
- User personalization options
- Consistent design across applications
- Easy maintenance and updates

## Task 3: Responsive Design Integration

### Current State
The current theme system provides spacing scales and typography settings but doesn't have explicit responsive design support or breakpoints.

### Proposed Changes
Extend the theme system with responsive design capabilities:
1. Breakpoint definitions
2. Responsive spacing scales
3. Responsive typography
4. Component adaptation utilities
5. Media query helpers

### Implementation Plan
1. Add breakpoint definitions to the DesignSystem
2. Create responsive variants of existing scales
3. Implement responsive utility functions
4. Update components to use responsive features
5. Provide documentation and examples

### Breakpoint System
```rust
#[derive(Debug, Clone)]
pub struct Breakpoints {
    /// Small devices (landscape phones, 576px and up)
    pub sm: String, // 576px
    
    /// Medium devices (tablets, 768px and up)
    pub md: String, // 768px
    
    /// Large devices (desktops, 992px and up)
    pub lg: String, // 992px
    
    /// Extra large devices (large desktops, 1200px and up)
    pub xl: String, // 1200px
    
    /// Extra extra large devices (larger desktops, 1400px and up)
    pub xxl: String, // 1400px
}
```

### Responsive Spacing
```rust
#[derive(Debug, Clone)]
pub struct ResponsiveSpacingScale {
    /// Extra small spacing
    pub xs: SpacingVariants,
    
    /// Small spacing
    pub sm: SpacingVariants,
    
    /// Medium spacing
    pub md: SpacingVariants,
    
    /// Large spacing
    pub lg: SpacingVariants,
    
    /// Extra large spacing
    pub xl: SpacingVariants,
}

#[derive(Debug, Clone)]
pub struct SpacingVariants {
    pub base: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}
```

### Responsive Typography
```rust
#[derive(Debug, Clone)]
pub struct ResponsiveTypography {
    pub font_family: String,
    pub font_sizes: ResponsiveFontSizes,
    pub font_weights: FontWeights,
    pub line_heights: ResponsiveLineHeights,
}

#[derive(Debug, Clone)]
pub struct ResponsiveFontSizes {
    pub xs: FontSizeVariants,
    pub sm: FontSizeVariants,
    pub md: FontSizeVariants,
    pub lg: FontSizeVariants,
    pub xl: FontSizeVariants,
}

#[derive(Debug, Clone)]
pub struct FontSizeVariants {
    pub base: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}
```

### Responsive Utility Functions
1. Media query generators
2. Breakpoint matching functions
3. Responsive style helpers
4. Component adaptation utilities

### Benefits
- Consistent responsive behavior across components
- Easier maintenance of responsive design
- Better mobile user experience
- Reduced CSS duplication

## Implementation Roadmap

1. **Phase 1**: Implement dark mode support with automatic detection
2. **Phase 2**: Add theme customization and ThemeManager
3. **Phase 3**: Integrate responsive design features
4. **Phase 4**: Create theme switching UI components
5. **Phase 5**: Add theme persistence and user preferences
6. **Phase 6**: Performance optimization and testing
7. **Phase 7**: Documentation and examples

## Testing Strategy

- Visual regression tests for theme switching
- Cross-browser compatibility testing
- Accessibility testing for color contrast
- Performance tests for theme switching
- Unit tests for theme merging and validation
- Integration tests with components

## Documentation Requirements

- Theme customization guide
- Dark mode implementation examples
- Responsive design patterns
- Theme migration guide for existing applications
- Best practices for accessible theming
- Component styling guidelines
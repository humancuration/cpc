# Accessibility Demo Visualization Example

This example demonstrates advanced accessibility features in the CPC visualization system through practical examples.

## Overview

This example shows:
- Implementing comprehensive accessibility metadata
- Creating screen reader-optimized visualizations
- Designing keyboard-navigable interfaces
- Testing accessibility features effectively
- Understanding compliance requirements for accessibility

## Prerequisites

- CPC development environment
- Understanding of accessibility concepts
- Familiarity with screen readers (NVDA, JAWS, VoiceOver)
- Access to API Gateway service

## Running the Example

Navigate to this directory and run the example:

```bash
cargo run
```

Note: This example focuses on accessibility features and doesn't require the API Gateway to be running, as it demonstrates the accessibility logic rather than making actual visualization requests.

## Key Components

### Accessibility Manager

The example implements an accessibility manager that handles different accessibility modes:

```rust
pub enum AccessibilityMode {
    Standard,
    ScreenReader,
    HighContrast,
    CognitiveSupport,
}
```

### Keyboard Navigation

Comprehensive keyboard navigation support:

```rust
pub struct KeyboardNavigation {
    shortcuts: HashMap<String, String>,
    current_focus: usize,
    total_elements: usize,
}
```

### Color Contrast

Dynamic color contrast adjustment:

```rust
pub enum ColorContrast {
    Standard,
    High,
    Enhanced,
}
```

## Expected Output

When running successfully, you should see output similar to:

```
Starting Accessibility Demo Example...
=== Bar Chart Accessibility Demo ===
Standard Mode:
  Alt Text: Bar chart showing 4 data points. Values range from 120 to 220. Highest value is 220 in Q3 Sales.

Screen Reader Mode:
  Alt Text: Bar chart with 4 categories. Data points: Q1 Sales: 120, Q2 Sales: 180, Q3 Sales: 220, Q4 Sales: 195. Use keyboard arrows to navigate between bars.

Cognitive Support Mode:
  Alt Text: Simple bar chart. The highest bar is Q3 Sales with a value of 220. This shows the largest amount in the data.

=== Keyboard Navigation Demo ===
Keyboard Navigation Shortcuts:
  Tab: Move to next element
  Shift+Tab: Move to previous element
  Enter: Activate current element
  ArrowUp: Move up in chart
  ArrowDown: Move down in chart
  ...

=== Comprehensive Accessibility Testing ===
...
Accessibility Demo Example completed successfully!
```

## Accessibility Features Demonstrated

1. **Multiple Accessibility Modes**
   - Standard mode for typical users
   - Screen reader mode with detailed navigation
   - High contrast mode for visual impairments
   - Cognitive support mode for simplified understanding

2. **Alternative Text Generation**
   - Context-aware alt text for different chart types
   - Detailed descriptions for screen reader users
   - Simplified text for cognitive support

3. **Keyboard Navigation**
   - Comprehensive shortcut system
   - Focus management
   - Spatial navigation in charts

4. **Visual Customization**
   - Text scaling options
   - Color contrast adjustments
   - Motion preference controls

## Testing with Screen Readers

The example includes functionality to test with screen readers:

1. **NVDA (Windows)**
2. **VoiceOver (macOS)**
3. **JAWS (Windows)**

## Compliance Features

The example demonstrates compliance with:

- **WCAG 2.1 Standards**
  - Perceivable, Operable, Understandable, Robust principles
- **Section 508 Compliance**
  - Federal accessibility requirements

## Customization

You can customize this example by:

1. Adding new accessibility modes
2. Implementing additional chart types
3. Extending keyboard navigation
4. Adding support for more screen readers
5. Implementing automated accessibility testing

## Related Documentation

- [Accessibility Demo Tutorial](../../../docs/tutorials/accessibility_demo.md)
- [Visualization User Guide](../../../docs/user/visualization_guide.md)
- [Complex Dashboard Tutorial](../../../docs/tutorials/complex_dashboard.md)
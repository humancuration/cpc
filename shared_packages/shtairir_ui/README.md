# Shtairir UI Components

A shared UI component library for the Shtairir visual scripting system.

## Overview

This package provides reusable UI components for building visual scripting editors and related tools within the CPC ecosystem. The components are designed to be modular, accessible, and customizable.

## Components

### BlockBrowser

A component for browsing and selecting available code blocks. Features include:

- Search functionality
- Category filtering
- Detailed block information display
- Purity indicators (pure vs effectful blocks)
- Version information
- Tagging system

### VisualNode

A draggable node component for visual scripting editors:

- Configurable ports for inputs/outputs
- Customizable styling
- Drag and drop support
- Connection handling

### PropertyPanel

An editor panel for viewing and modifying node properties:

- Dynamic form generation based on property types
- Validation support
- Undo/redo functionality

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
shtairir_ui = { path = "../shared_packages/shtairir_ui" }
```

## Usage

```rust
use shtairir_ui::components::BlockBrowser;
use shtairir_ui::models::BlockDefinition;

// Create a block browser component
let block_browser = BlockBrowser::new(
    vec![/* block definitions */],
    |selected_block| {
        // Handle block selection
        println!("Selected block: {}", selected_block.name);
    }
);
```

## Styling

The components use CSS variables for easy customization. Import the base styles:

```css
@import '../shared_packages/shtairir_ui/assets/styles.css';
```

Then override variables as needed:

```css
:root {
    --shtairir-primary: #your-color;
    --shtairir-secondary: #your-color;
    /* ... */
}
```

## Contributing

See the main Shtairir Editor documentation for contribution guidelines.

## License

This package is part of the CPC software ecosystem and is licensed under the CPC license.
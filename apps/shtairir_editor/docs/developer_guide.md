# Shtairir Visual Editor Developer Guide

This guide is for developers who want to extend or customize the Shtairir Visual Editor.

## Architecture Overview

The Shtairir Visual Editor follows a modular architecture built on Rust and WebAssembly:

```
shtairir_editor/
├── src/
│   ├── app.rs           # Main application component
│   ├── lib.rs           # Public API and WASM bindings
│   ├── main.rs          # Entry point (placeholder for native)
│   ├── models.rs        # Data models and structures
│   ├── components/      # UI components
│   ├── registry.rs      # Block registry integration
│   ├── serializer.rs    # TOML serialization
│   └── validator.rs     # Workflow validation
├── assets/              # Static assets (CSS, images)
├── examples/            # Example workflows
├── docs/                # Documentation
├── tests/               # Unit and integration tests
├── Cargo.toml          # Package manifest
├── Makefile            # Build automation
└── index.html          # Web entry point
```

## Key Components

### Data Models

The `models.rs` file contains the core data structures:

- `Graph`: Represents a complete workflow
- `Node`: Individual processing units
- `Connection`: Links between nodes
- `Port`: Input/output points on nodes

### UI Components

The editor uses Yew components organized in `src/components/`:

- `BlockLibrary`: Browser for available blocks
- `Canvas`: Main workspace for node placement
- `PropertiesPanel`: Configuration interface
- `Toolbar`: Top-level controls
- `ConnectionView`: Visual representation of connections

### Integration Points

The editor integrates with several Shtairir systems:

- **Registry**: Loading and managing block definitions
- **Execution**: Running workflows (future feature)
- **Serialization**: Import/export in TOML format

## Extending the Editor

### Adding New Components

To add a new UI component:

1. Create a new file in `src/components/`
2. Implement a Yew function component
3. Export it in `src/components/mod.rs`
4. Use it in your application

Example component structure:

```rust
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MyComponentProps {
    pub data: String,
    #[prop_or_default]
    pub on_change: Callback<String>,
}

#[function_component(MyComponent)]
pub fn my_component(props: &MyComponentProps) -> Html {
    let on_click = {
        let on_change = props.on_change.clone();
        let data = props.data.clone();
        Callback::from(move |_| {
            on_change.emit(data.clone());
        })
    };
    
    html! {
        <div class="my-component" onclick={on_click}>
            {&props.data}
        </div>
    }
}
```

### Custom Block Types

To support new block types:

1. Extend the `BlockSpec` model in `shtairir_registry`
2. Update the block library component to handle new types
3. Create specialized parameter editors
4. Add validation rules

### New Connection Policies

To add connection policies:

1. Extend the `EdgePolicy` enum in `models.rs`
2. Update the connection rendering in `ConnectionView`
3. Add policy configuration to the properties panel
4. Implement the policy logic in the execution system

## Styling and Themes

The editor uses CSS for styling with a component-based approach:

- Each component has scoped CSS classes
- Variables are defined in `:root` for easy theming
- Responsive design using media queries

To customize the theme:

1. Modify CSS variables in `assets/style.css`
2. Add new component-specific styles
3. Test across different screen sizes

## API Integration

The editor provides a public API through `src/lib.rs`:

```rust
use shtairir_editor::api::*;

// Create and manipulate graphs programmatically
let mut graph = Graph::new();
// ... modify graph ...

// Serialize to TOML
let toml = Serializer::graph_to_toml(&graph)?;

// Validate workflows
let result = Validator::validate_graph(&graph);
```

## Testing

The editor includes several types of tests:

### Unit Tests

Located in `tests/` directory:

```bash
make test
```

### Integration Tests

Test complete workflows and user interactions.

### Browser Testing

Manual testing in different browsers to ensure compatibility.

## Build Process

### Development Build

```bash
make build-dev
```

### Production Build

```bash
make build-web
```

### Serving Locally

```bash
make serve
```

## Performance Considerations

### Rendering Optimization

- Use `use_memo` for expensive computations
- Implement virtual scrolling for large lists
- Debounce frequent updates

### Memory Management

- Clean up event listeners
- Avoid circular references
- Use efficient data structures

## Security

The editor runs in a browser sandbox with these considerations:

- All data stays client-side by default
- File operations use browser security model
- No external network requests without user action

## Internationalization

To add new languages:

1. Create translation files
2. Update the localization system
3. Add language selector to UI
4. Test with RTL languages

## Contributing

### Code Style

Follow the Rust formatting guidelines:

```bash
make fmt
make lint
```

### Documentation

All public APIs should include documentation comments.

### Testing

Add tests for new functionality:

- Unit tests for pure functions
- Component tests for UI elements
- Integration tests for workflows

## Release Process

1. Update version in `Cargo.toml` and `MODULE.toml`
2. Run all tests
3. Build production artifacts
4. Update documentation
5. Create git tag
6. Publish to package registry

## Troubleshooting

### Common Build Issues

**Missing wasm-pack**: Install with `make install-wasm-pack`

**Compilation errors**: Check that all dependencies are up to date

**Runtime errors**: Check browser console for detailed error messages

### Performance Issues

**Slow rendering**: Check for unnecessary re-renders
**Memory leaks**: Use browser dev tools to monitor memory usage
**Large workflows**: Implement virtualization for canvas elements

## Advanced Topics

### Custom Renderers

The editor can support different rendering backends:

- SVG for vector graphics
- Canvas API for complex visualizations
- WebGL for 3D workflows

### Plugin System

Future versions may support plugins for:

- Custom block types
- New connection policies
- Additional export formats
- Third-party integrations

### Collaboration Features

Real-time collaboration requires:

- Conflict-free replicated data types (CRDTs)
- WebSocket communication
- User presence indicators
- Operation history

## Conclusion

The Shtairir Visual Editor is designed to be extensible and customizable. By following the patterns established in the codebase, you can add new features, support additional block types, and integrate with other systems in the CPC ecosystem.

For major architectural changes, please discuss with the core team before implementation.
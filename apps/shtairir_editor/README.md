# Shtairir Visual Editor

A web-based visual programming editor for creating and editing Shtairir workflows. This editor provides a graphical interface for composing complex data processing pipelines, machine learning workflows, and other computational graphs using the Shtairir visual programming system.

## Features

### Core Editor Framework
- **Web-based Interface**: Built with Yew and WebAssembly for seamless browser deployment
- **Responsive Design**: Works on various screen sizes and devices
- **Theming Support**: Clean, modern interface that matches the CPC platform aesthetics
- **Accessibility**: Keyboard navigation and screen reader support

### Visual Workflow Composition
- **Interactive Canvas**: Drag and drop nodes, create connections between ports
- **Real-time Validation**: Visual feedback for valid and invalid connections
- **Zoom and Pan**: Navigate large workflows with ease
- **Auto-layout**: Automatic organization of nodes for better visualization

### Block Management
- **Block Library**: Browse and search available blocks from the registry
- **Categorization**: Filter blocks by category, functionality, or tags
- **Documentation**: Inline help and examples for each block
- **Purity Indicators**: Visual distinction between pure and effectful blocks

### Connection System
- **Type Compatibility**: Automatic validation of port type compatibility
- **Visual Feedback**: Color-coded connections for valid/invalid status
- **Connection Policies**: Support for advanced connection behaviors (merge, filter, buffer, etc.)
- **Cycle Detection**: Warning system for circular dependencies

### Workflow Management
- **TOML Export/Import**: Generate and load workflow specifications in TOML format
- **Undo/Redo**: Full history management for workflow changes
- **Performance Metrics**: Overlay for monitoring workflow execution performance
- **Simulation Mode**: Test workflows without full execution

## Architecture

The Shtairir Visual Editor is built using a modular architecture:

```
shtairir_editor/
├── src/
│   ├── app.rs           # Main application component
│   ├── models.rs        # Data models for graphs, nodes, connections
│   ├── components/      # UI components (block library, canvas, properties panel)
│   ├── registry.rs      # Block registry management
│   ├── serializer.rs    # TOML serialization/deserialization
│   └── validator.rs     # Workflow validation logic
├── assets/
│   └── style.css        # Styling for the editor
└── Cargo.toml           # Project dependencies
```

## Getting Started

### Prerequisites

- Rust toolchain (latest stable)
- wasm-pack for WebAssembly compilation
- A modern web browser

### Building

```bash
# Clone the repository
git clone <repository-url>
cd apps/shtairir_editor

# Build for WebAssembly
wasm-pack build --target web

# Serve the application
# (Use any static file server, e.g., Python's http.server)
python -m http.server 8000
```

### Development

```bash
# Build in development mode with watch
wasm-pack build --target web --dev --watch
```

## Usage

1. **Browse Blocks**: Use the block library panel to find available blocks
2. **Add Nodes**: Drag blocks from the library to the canvas
3. **Connect Nodes**: Click on output ports and drag to input ports to create connections
4. **Configure Parameters**: Select nodes to edit their parameters in the properties panel
5. **Export Workflow**: Use the toolbar to export your workflow as TOML

## Key Components

### Canvas
The main workspace where nodes are placed and connected. Supports zooming, panning, and drag-and-drop operations.

### Block Library
Sidebar containing all available blocks organized by category. Search and filter capabilities for finding specific blocks.

### Properties Panel
Context-sensitive panel that shows configuration options for the selected node, including parameter editing and port information.

### Toolbar
Top-level controls for file operations, editing, view options, and help resources.

## Integration

The editor integrates with the broader Shtairir ecosystem:

- **Shtairir Registry**: Loads block definitions and metadata
- **Shtairir Execution**: Can execute workflows created in the editor
- **TOML Format**: Exports workflows in the standard Shtairir graph specification format

## Contributing

Contributions are welcome! Please see the main CPC project guidelines for contributing.

## License

This project is part of the CPC software ecosystem and is licensed under the CPC license.

## Support

For issues, questions, or feature requests, please open an issue in the main CPC repository.
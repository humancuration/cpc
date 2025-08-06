# Shtairir Ecosystem

## Overview

The Shtairir ecosystem consists of multiple components that work together to provide a unified scripting experience across all CPC applications.

## Core Components

### 1. Shtairir Language (`shared_packages/shtairir`)

The core parsing and execution engine for the Shtairir language.

**Features:**
- Grammar definition using Pest
- Abstract Syntax Tree (AST) representation
- Execution engine with adapter pattern
- Comprehensive test suite

**Key Modules:**
- `grammar.pest` - Language grammar definition
- `parser.rs` - Script parser
- `ast.rs` - Abstract Syntax Tree structures
- `engine.rs` - Execution engine

### 2. Visual Editor (`shared_packages/shtairir_editor`)

Web-based visual editor for creating and editing Shtairir scripts using a node-based interface.

**Features:**
- Yew-based web components
- Node graph representation
- Drag and drop functionality
- Real-time script generation

### 3. CLI Tool (`apps/shtairir_cli`)

Command-line interface for executing Shtairir scripts.

**Features:**
- Script file execution
- Inline script execution
- Multiple output formats
- Error reporting

## Integration Patterns

### App Adapter Pattern

Applications integrate with Shtairir through the `AppAdapter` trait:

```rust
pub trait AppAdapter {
    fn execute(&self, command: &Command) -> Result<Value, String>;
}
```

Each application implements this trait to handle commands directed at it.

### Script Format

Shtairir scripts follow a simple format:
```
app_name:function_name(arg1, arg2, ...)
```

Multiple commands can be combined:
```
bevy:create_entity()
redis:set("key", "value")
ffmpeg:convert("input.mp4", "output.webm")
```

## Data Flow

1. **Parsing**: Scripts are parsed into AST using Pest
2. **Validation**: AST is validated for correctness
3. **Execution**: Commands are routed to appropriate app adapters
4. **Result Collection**: Results are collected and returned

## Example Workflows

### Media Processing Pipeline
```
// Convert video format
ffmpeg:convert("input.mp4", "output.webm")

// Extract audio track
ffmpeg:extract_audio("input.mp4", "audio.opus")

// Generate thumbnails
ffmpeg:generate_thumbnails("input.mp4", "thumb_%d.jpg")

// Store metadata
redis:set("media:input", "input.mp4")
redis:set("media:output", "output.webm")
```

### Game Entity Creation
```
// Create game entities
bevy:create_entity()
bevy:add_component("player", "Transform")
bevy:add_component("player", "Sprite")
bevy:set_component("player", "Transform", {x=100, y=200})

// Create camera
bevy:create_entity()
bevy:add_component("camera", "Camera")
```

## Development Guidelines

### Adding New Applications

1. Implement the `AppAdapter` trait for your application
2. Register the adapter with the execution context
3. Test with sample scripts

### Extending the Grammar

1. Update `grammar.pest` with new rules
2. Update the parser to handle new constructs
3. Add appropriate AST nodes
4. Update the execution engine if needed

## Future Roadmap

### Short Term
- Enhanced visual editor features
- Improved error handling and reporting
- Performance optimizations
- Additional example applications

### Long Term
- Package management system
- Security sandboxing
- WebAssembly integration
- Cross-platform support
- Community script repository

## Testing

The ecosystem includes comprehensive tests:
- Unit tests for parsing and execution
- Integration tests for complete workflows
- Performance benchmarks
- Example scripts for manual testing

## License

All components of the Shtairir ecosystem are licensed under the CPC License.
# Shtairir - Unified Scripting Language

Shtairir is a unified scripting language that combines Arabic and Hebrew elements for liberation. It enables cross-app interoperability across the CPC ecosystem.

## Features

- Cross-app scripting capabilities
- Unified grammar for all CPC applications
- Visual editor integration
- Sandboxed execution environment
- Extensible adapter system

## Grammar

The Shtairir grammar is defined in `src/grammar.pest` and supports:

- App commands: `app:function(arg1, arg2, ...)`
- Data types: numbers, strings, identifiers, objects, arrays
- Complex data structures

Example:
```
bevy:create_entity()
ffmpeg:convert("input.mp4", "output.webm")
redis:set("key", "value")
```

## Architecture

### Core Components

1. **Parser** - Uses Pest to parse scripts into AST
2. **AST** - Abstract Syntax Tree representation of scripts
3. **Engine** - Execution engine with adapter system
4. **Adapters** - App-specific implementations

### Adapter System

Apps integrate with Shtairir through adapters:

```rust
pub trait AppAdapter {
    fn execute(&self, command: &Command) -> Result<Value, String>;
}
```

## Usage

Add to your Cargo.toml:
```toml
[dependencies]
shtairir = { path = "../shared_packages/shtairir" }
```

Basic usage:
```rust
use shtairir::{parse_script, ExecutionContext, AppAdapter};

// Parse a script
let script = parse_script("bevy:create_entity()")?;

// Create execution context
let mut context = ExecutionContext::new();
context.register_adapter("bevy".to_string(), Box::new(BevyAdapter));

// Execute
let results = shtairir::execute_script(&script, &context)?;
```

## Visual Editor

The `shtairir_editor` package provides a Yew-based visual editor for creating and editing Shtairir scripts through a node-based interface.

## License

This project is licensed under the CPC License - see the LICENSE file for details.
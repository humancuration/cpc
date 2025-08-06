# Shtairir Unified Scripting Language

## Overview

Shtairir is a unified scripting language that combines Arabic and Hebrew elements for liberation. It enables cross-app interoperability across the CPC ecosystem by providing a common grammar and execution environment for all applications.

## Language Design

The language is designed to be simple yet powerful, allowing users to compose complex workflows by chaining commands from different applications.

### Syntax

The basic syntax follows this pattern:
```
app_name:function_name(arg1, arg2, ...)
```

### Supported Data Types

1. **Numbers**: Integer and floating-point values
   ```
   42
   3.14159
   -7.5
   ```

2. **Strings**: Text enclosed in double quotes
   ```
   "Hello, world!"
   "This is a string with \"escaped\" quotes"
   ```

3. **Identifiers**: Variable names and unquoted strings
   ```
   entity_id
   component_name
   ```

4. **Objects**: Key-value pairs enclosed in braces
   ```
   {name="player", health=100, position={x=10, y=20}}
   ```

5. **Arrays**: Lists of values enclosed in brackets
   ```
   [1, 2, 3, 4]
   ["apple", "banana", "cherry"]
   ```

### Examples

Simple command:
```
bevy:create_entity()
```

Command with arguments:
```
ffmpeg:convert("input.mp4", "output.webm")
```

Command with complex arguments:
```
redis:set("user:123", {name="Alice", age=30, active=true})
```

Multiple commands:
```
bevy:create_entity()
bevy:add_component("entity1", "Position")
bevy:set_component("entity1", "Position", {x=100, y=200})
ffmpeg:convert("video.mp4", "video.webm")
```

## Architecture

### Parser

The parser uses Pest grammar to convert script text into an Abstract Syntax Tree (AST).

### AST

The AST represents the script as structured data:
```rust
pub struct Script {
    pub commands: Vec<Command>,
}

pub struct Command {
    pub app: String,
    pub function: String,
    pub args: Vec<Value>,
}

pub enum Value {
    Number(f64),
    String(String),
    Identifier(String),
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
}
```

### Execution Engine

The execution engine uses an adapter pattern to interface with different applications:

```rust
pub trait AppAdapter {
    fn execute(&self, command: &Command) -> Result<Value, String>;
}
```

## Integration Guide

### Adding Shtairir to Your App

1. Add shtairir as a dependency in your Cargo.toml:
   ```toml
   [dependencies]
   shtairir = { path = "../shared_packages/shtairir" }
   ```

2. Implement an AppAdapter for your application:
   ```rust
   struct MyAppAdapter;
   
   impl AppAdapter for MyAppAdapter {
       fn execute(&self, command: &Command) -> Result<Value, String> {
           // Handle commands for your app
           match command.function.as_str() {
               "my_function" => {
                   // Implementation here
                   Ok(Value::String("result".to_string()))
               },
               _ => Err("Unknown function".to_string())
           }
       }
   }
   ```

3. Register your adapter and execute scripts:
   ```rust
   let mut context = ExecutionContext::new();
   context.register_adapter("myapp".to_string(), Box::new(MyAppAdapter));
   
   let script = parse_script("myapp:my_function()")?;
   let results = execute_script(&script, &context)?;
   ```

## Visual Editor

The Shtairir Editor provides a visual interface for creating and editing scripts. It represents scripts as node graphs where each node corresponds to a command.

## Future Extensions

Planned features include:
- Visual scripting with node-based editor
- Script debugging and profiling
- Package management for reusable script libraries
- Security sandboxing for untrusted scripts
- WebAssembly integration for browser execution

## License

This project is licensed under the CPC License.
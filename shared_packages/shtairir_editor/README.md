# Shtairir Editor

Visual editor for the Shtairir unified scripting language, built with Yew.

## Quick Start

Get started with the Shtairir Visual Editor in just a few steps:

```rust
use shtairir_editor::{VisualEditor, Graph, Node, Value, Port, PortType, PortDirection};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let graph = use_state(|| {
        let mut graph = Graph::new();
        
        // Create a simple math operation node
        let node = Node {
            id: "node1".to_string(),
            app: "math".to_string(),
            function: "add".to_string(),
            args: vec![Value::Number(5.0), Value::Number(3.0)],
            position: (100, 100),
            input_ports: vec![
                Port {
                    id: "in1".to_string(),
                    name: "a".to_string(),
                    port_type: PortType::Number,
                    direction: PortDirection::Input,
                },
                Port {
                    id: "in2".to_string(),
                    name: "b".to_string(),
                    port_type: PortType::Number,
                    direction: PortDirection::Input,
                },
            ],
            output_ports: vec![
                Port {
                    id: "out1".to_string(),
                    name: "result".to_string(),
                    port_type: PortType::Number,
                    direction: PortDirection::Output,
                },
            ],
        };
        
        graph.add_node(node);
        graph
    });
    
    let on_graph_change = {
        let graph = graph.clone();
        Callback::from(move |new_graph| {
            graph.set(new_graph);
        })
    };
    
    html! {
        <div class="app">
            <h1>{"Shtairir Visual Editor"}</h1>
            <VisualEditor
                graph={(*graph).clone()}
                on_graph_change={on_graph_change}
            />
        </div>
    }
}
```

## Features

- Node-based visual scripting interface
- Enhanced port system with type validation
- Visual connection validation with real-time feedback
- Drag and drop functionality
- Real-time script generation with data flow
- Integration with Shtairir core
- Web-based UI using Yew framework
- Parameter editing controls for different value types
- Visual data flow connections with SVG rendering
- Script conversion improvements (visual ↔ script)
- Backward compatibility with legacy graphs

## Enhanced Web Example

The `web_example.rs` binary demonstrates all documented features with a comprehensive interactive demo:

**Key Features Demonstrated:**
- **Diverse Node Types**: Math operations, string manipulation, boolean logic, and custom object/array nodes
- **Connection Validation**: Real-time type checking with visual feedback (blue for valid, red for invalid)
- **Parameter Editing**: Type-specific controls for numbers, strings, booleans, arrays, and objects
- **Script Conversion**: Bidirectional conversion between visual graphs and Shtairir scripts
- **Interactive UI**: Drag-and-drop nodes, port connections, and real-time parameter updates

**Running the Example:**
```bash
cargo run --bin web_example
```

For detailed documentation of the enhanced example, see [WEB_EXAMPLE_README.md](WEB_EXAMPLE_README.md).

## Architecture

The editor is built as a reusable Yew component that can be integrated into any web application:

```rust
use shtairir_editor::{VisualEditor, Graph};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let graph = use_state(|| Graph::new());
    
    let on_graph_change = {
        let graph = graph.clone();
        Callback::from(move |new_graph| {
            graph.set(new_graph);
        })
    };
    
    html! {
        <VisualEditor 
            graph={(*graph).clone()} 
            on_graph_change={on_graph_change} 
        />
    }
}
```

## Components

### Graph
Represents the visual script as a collection of nodes and connections:

```rust
pub struct Graph {
    pub nodes: HashMap<String, Node>,
    pub connections: Vec<Connection>,
}
```

### Node
Represents a visual node in the editor:

```rust
pub struct Node {
    pub id: String,
    pub app: String,
    pub function: String,
    pub args: Vec<Value>,
    pub position: (i32, i32),
    pub input_ports: Vec<Port>,
    pub output_ports: Vec<Port>,
}
```

### Connection
Represents a connection between nodes:

```rust
pub struct Connection {
    pub id: String,
    pub from_node: String,
    pub from_port: String,
    pub to_node: String,
    pub to_port: String,
}
```

### Port
Represents a connection point on a node:

```rust
pub struct Port {
    pub id: String,
    pub name: String,
    pub port_type: PortType,
    pub direction: PortDirection,
}
```

## Conversion

The editor can convert between visual graphs and Shtairir scripts with proper data flow:

```rust
// Graph to Script
let script = graph.to_script();

// Script to Graph
let graph = Graph::from_script(&script);

// Migrate legacy graphs
graph.migrate_legacy();
```

## Components

### VisualEditor
Main component for the visual editor interface with SVG connection rendering.

### NodeEditor
Component for editing node parameters and configuration.

## License

This project is licensed under the CPC License - see the LICENSE file for details.
# Shtairir Editor

Visual editor for the Shtairir unified scripting language, built with Yew.

## Features

- Node-based visual scripting interface
- Drag and drop functionality
- Real-time script generation
- Integration with Shtairir core
- Web-based UI using Yew framework

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

## Conversion

The editor can convert between visual graphs and Shtairir scripts:

```rust
// Graph to Script
let script = graph.to_script();

// Script to Graph
let graph = Graph::from_script(&script);
```

## License

This project is licensed under the CPC License - see the LICENSE file for details.
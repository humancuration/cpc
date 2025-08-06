# Shtairir Visual Editor Documentation

The Shtairir Visual Editor provides a node-based interface for creating and editing Shtairir scripts visually. This document covers the advanced features and usage patterns of the visual editor.

## Table of Contents

1. [Feature Explanations](#feature-explanations)
2. [API Reference](#api-reference)
3. [Best Practices](#best-practices)
4. [Troubleshooting](#troubleshooting)

## Feature Explanations

### Port System

The port system is the foundation of data flow in the visual editor. Each node has input and output ports that define how data flows through the script.

#### Port Types

Ports can be of several types, determining what kind of data they can handle:

- **Any**: Compatible with any port type
- **Number**: Numeric values (integers, floats)
- **String**: Text values
- **Boolean**: True/False values
- **Object**: Key-value pairs/structs
- **Array**: Lists of values
- **Custom(String)**: User-defined types

#### Input/Output Ports

- **Input Ports**: Receive data from other nodes or user input
- **Output Ports**: Send data to other nodes

#### Port Compatibility

Ports can only be connected if they are compatible:

```rust
// Check compatibility manually
let is_compatible = PortType::are_compatible(&PortType::Number, &PortType::Number); // true
let is_incompatible = PortType::are_compatible(&PortType::Number, &PortType::String); // false
```

Compatibility rules:
- Any type can connect to Any
- Identical types can connect
- Custom types must match exactly
- Otherwise, types are incompatible

#### Creating Custom Ports

```rust
let custom_input_port = Port {
    id: "data_input".to_string(),
    name: "Data".to_string(),
    port_type: PortType::Object,
    direction: PortDirection::Input,
};

let custom_output_port = Port {
    id: "processed_output".to_string(),
    name: "Processed".to_string(),
    port_type: PortType::Array,
    direction: PortDirection::Output,
};
```

### Connection Validation

The visual editor provides real-time validation of connections between nodes.

#### Visual Feedback

- **Valid Connections**: Displayed in the default connection color
- **Invalid Connections**: Displayed in red to indicate type mismatches
- **Hover Effects**: Visual feedback when hovering over ports

#### Validation Rules

Connections are validated based on:
1. **Port Type Compatibility**: Source and destination ports must be compatible
2. **Direction**: Output ports can only connect to input ports
3. **Node Existence**: Both source and destination nodes must exist
4. **Port Existence**: Both source and destination ports must exist

```rust
// Connection validation is automatic
let connection = Connection {
    id: "conn1".to_string(),
    from_node: "node1".to_string(),
    from_port: "output".to_string(),
    to_node: "node2".to_string(),
    to_port: "input".to_string(),
};

if connection.is_valid(&graph.nodes) {
    // Valid connection
    graph.connections.push(connection);
} else {
    // Invalid connection - show error to user
    println!("Cannot connect incompatible port types");
}
```

### Parameter Controls

Nodes can have parameters that users can edit through the NodeEditor component.

#### Value Type Editors

The visual editor automatically provides appropriate controls based on the parameter type:

- **Numbers**: Numeric input field with validation
- **Strings**: Text input field
- **Booleans**: Checkbox or toggle
- **Objects**: JSON editor or structured form
- **Arrays**: List editor with add/remove controls
- **Identifiers**: Display-only (derived from connections)

#### Real-time Updates

Parameter changes are immediately reflected in the node and the generated script:

```rust
// Parameter editing is handled by the NodeEditor component
// This example shows how parameter changes are processed:

let on_arg_change = {
    let node = node.clone();
    let on_node_change = props.on_node_change.clone();
    Callback::from(move |(index, value): (usize, String)| {
        let mut new_node = (*node).clone();
        // Update the argument with the new value
        new_node.args[index] = Value::String(value);
        node.set(new_node.clone());
        on_node_change.emit(new_node);
    })
};
```

### Script Conversion

The visual editor can convert between visual graphs and Shtairir scripts while preserving data flow.

#### Graph to Script Conversion

```rust
let script = graph.to_script();
// script.commands contains the converted Shtairir commands
```

The conversion process:
1. Identifies starting nodes (nodes without incoming connections)
2. Traverses the graph in execution order
3. Resolves connections into identifier references
4. Generates Shtairir commands with proper argument flow

#### Script to Graph Conversion

```rust
let graph = Graph::from_script(&script);
```

The conversion process:
1. Creates nodes for each command in the script
2. Generates default input/output ports
3. Positions nodes in a default layout
4. Preserves command arguments as node parameters

#### Data Flow Preservation

Connections are represented as identifier references in the script:

```rust
// Original connection
Connection {
    from_node: "node1".to_string(),
    from_port: "output".to_string(),
    to_node: "node2".to_string(),
    to_port: "input".to_string(),
}

// Becomes in script
Command {
    app: "app2".to_string(),
    function: "func2".to_string(),
    args: vec![Value::Identifier("node1.output".to_string())],
}
```

### Data Flow Visualization

The editor uses SVG to render connections between nodes, providing a clear visual representation of data flow.

#### Connection Rendering

```rust
// Connections are rendered as SVG paths
let path_data = format!("M {} {} C {} {}, {} {}, {} {}",
    start.0, start.1,
    start.0 + 50, start.1,
    end.0 - 50, end.1,
    end.0, end.1
);

html! {
    <path
        d={path_data}
        stroke={if is_valid { "var(--connection-color)" } else { "red" }}
        stroke-width="2"
        fill="none"
        class="connection"
    />
}
```

#### Visual Features

- **Curved Paths**: Bezier curves for smooth connection lines
- **Color Coding**: Different colors for valid/invalid connections
- **Z-Indexing**: Connections rendered behind nodes for clarity
- **Hover States**: Visual feedback on interaction

## API Reference

### Graph

The main container for nodes and connections.

```rust
pub struct Graph {
    pub nodes: HashMap<String, Node>,
    pub connections: Vec<Connection>,
}
```

#### Key Methods

```rust
// Create a new empty graph
let graph = Graph::new();

// Add a node to the graph
graph.add_node(node);

// Remove a node and its connections
graph.remove_node("node_id");

// Convert graph to Shtairir script
let script = graph.to_script();

// Create graph from Shtairir script
let graph = Graph::from_script(&script);

// Migrate legacy graph format
graph.migrate_legacy();
```

### Node

Represents a single node in the visual graph.

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

#### Key Methods

```rust
// Get the index of an input port
let index = node.get_input_index("port_id");
```

### Connection

Represents a data flow connection between two nodes.

```rust
pub struct Connection {
    pub id: String,
    pub from_node: String,
    pub from_port: String,
    pub to_node: String,
    pub to_port: String,
}
```

#### Key Methods

```rust
// Validate connection compatibility
let is_valid = connection.is_valid(&nodes);
```

### Port

Represents a connection point on a node.

```rust
pub struct Port {
    pub id: String,
    pub name: String,
    pub port_type: PortType,
    pub direction: PortDirection,
}
```

#### Key Methods

```rust
// Create default input port
let input_port = Port::default_input();

// Create default output port
let output_port = Port::default_output();
```

### PortType

Defines the type of data a port can handle.

```rust
pub enum PortType {
    Any,
    Number,
    String,
    Boolean,
    Object,
    Array,
    Custom(String),
}
```

#### Key Methods

```rust
// Check if two port types are compatible
let is_compatible = PortType::are_compatible(&type1, &type2);
```

### PortDirection

Specifies whether a port is input or output.

```rust
pub enum PortDirection {
    Input,
    Output,
}
```

### VisualEditor

The main Yew component for the visual editor interface.

```rust
#[function_component(VisualEditor)]
pub fn visual_editor(props: &VisualEditorProps) -> Html
```

#### Props

```rust
pub struct VisualEditorProps {
    pub graph: Graph,
    pub on_graph_change: Callback<Graph>,
}
```

### NodeEditor

Component for editing node parameters.

```rust
#[function_component(NodeEditor)]
pub fn node_editor(props: &NodeEditorProps) -> Html
```

#### Props

```rust
pub struct NodeEditorProps {
    pub node: Node,
    pub on_node_change: Callback<Node>,
}
```

## Best Practices

### Creating Custom Nodes

When creating custom nodes, follow these guidelines:

1. **Use Descriptive Names**: Make node and port names clear and meaningful
2. **Define Clear Port Types**: Use specific types instead of `Any` when possible
3. **Provide Default Values**: Set sensible defaults for parameters
4. **Document Behavior**: Include comments explaining what the node does

```rust
// Good example
let custom_node = Node {
    id: "data_processor".to_string(),
    app: "analytics".to_string(),
    function: "process_metrics".to_string(),
    args: vec![Value::Number(1.0)], // Sensible default
    position: (100, 100),
    input_ports: vec![
        Port {
            id: "metrics_input".to_string(),
            name: "Metrics".to_string(),
            port_type: PortType::Array, // Specific type
            direction: PortDirection::Input,
        }
    ],
    output_ports: vec![
        Port {
            id: "results_output".to_string(),
            name: "Results".to_string(),
            port_type: PortType::Object, // Specific type
            direction: PortDirection::Output,
        }
    ],
};
```

### Defining Port Configurations

1. **Balance Input/Output Ports**: Ensure nodes have appropriate numbers of each
2. **Use Consistent Naming**: Follow a naming convention for port IDs
3. **Group Related Ports**: Use prefixes or suffixes for related ports
4. **Consider Data Flow**: Design ports to match expected data flow patterns

```rust
// Good port configuration
input_ports: vec![
    Port {
        id: "primary_input".to_string(),
        name: "Primary Data".to_string(),
        port_type: PortType::Object,
        direction: PortDirection::Input,
    },
    Port {
        id: "secondary_input".to_string(),
        name: "Secondary Data".to_string(),
        port_type: PortType::Object,
        direction: PortDirection::Input,
    },
    Port {
        id: "config_input".to_string(),
        name: "Configuration".to_string(),
        port_type: PortType::Object,
        direction: PortDirection::Input,
    }
],
output_ports: vec![
    Port {
        id: "main_output".to_string(),
        name: "Result".to_string(),
        port_type: PortType::Object,
        direction: PortDirection::Output,
    },
    Port {
        id: "error_output".to_string(),
        name: "Error".to_string(),
        port_type: PortType::String,
        direction: PortDirection::Output,
    }
]
```

### Handling Complex Data Types

1. **Use Custom Types**: Define custom port types for domain-specific data
2. **Document Structure**: Include documentation for complex data structures
3. **Validate Data**: Add validation for complex data where possible
4. **Provide Examples**: Show example data structures in documentation

```rust
// Define custom types for complex data
let custom_port_type = PortType::Custom("UserData".to_string());

// Use in port configuration
Port {
    id: "user_data".to_string(),
    name: "User Data".to_string(),
    port_type: custom_port_type,
    direction: PortDirection::Input,
}
```

### Optimizing Graph Performance

1. **Minimize Connections**: Avoid unnecessary connections between nodes
2. **Use Subgraphs**: Group related nodes into subgraphs when possible
3. **Avoid Cycles**: Design graphs to avoid circular dependencies
4. **Cache Results**: Cache node outputs when they don't change frequently

```rust
// Good graph structure
// Linear flow with clear inputs and outputs
// No circular dependencies
// Minimal cross-connections
```

## Troubleshooting

### Connection Compatibility Issues

#### Problem: Ports won't connect

**Possible Causes:**
- Port types are incompatible
- Trying to connect input to input or output to output
- Port IDs don't exist

**Solutions:**

1. **Check Port Types**
```rust
// Debug port types
let from_port = from_node.output_ports.iter().find(|p| p.id == from_port_id);
let to_port = to_node.input_ports.iter().find(|p| p.id == to_port_id);

if let (Some(fp), Some(tp)) = (from_port, to_port) {
    println!("From port type: {:?}, To port type: {:?}", fp.port_type, tp.port_type);
    println!("Are compatible: {}", PortType::are_compatible(&fp.port_type, &tp.port_type));
}
```

2. **Verify Port Direction**
```rust
// Ensure output connects to input
assert!(from_port.direction == PortDirection::Output);
assert!(to_port.direction == PortDirection::Input);
```

3. **Check Port Existence**
```rust
// Verify ports exist
let from_port_exists = from_node.output_ports.iter().any(|p| p.id == from_port_id);
let to_port_exists = to_node.input_ports.iter().any(|p| p.id == to_port_id);
```

### Script Conversion Errors

#### Problem: Generated script doesn't match expected behavior

**Possible Causes:**
- Graph traversal order is incorrect
- Connections not properly resolved
- Missing or invalid node arguments

**Solutions:**

1. **Debug Graph Traversal**
```rust
// Add logging to see traversal order
let script = graph.to_script();
for (i, command) in script.commands.iter().enumerate() {
    println!("Command {}: {}.{}", i, command.app, command.function);
    println!("Arguments: {:?}", command.args);
}
```

2. **Check Connection Resolution**
```rust
// Verify connections are properly converted
for conn in &graph.connections {
    println!("Connection: {}.{} -> {}.{}", 
        conn.from_node, conn.from_port, 
        conn.to_node, conn.to_port);
}
```

3. **Validate Node Arguments**
```rust
// Check that all arguments are valid
for node in graph.nodes.values() {
    for (i, arg) in node.args.iter().enumerate() {
        println!("Node {} arg {}: {:?}", node.id, i, arg);
    }
}
```

### Legacy Graph Migration

#### Problem: Old graphs don't work with the current version

**Possible Causes:**
- Missing port definitions
- Changed node structure
- Updated connection format

**Solutions:**

1. **Run Migration**
```rust
// Apply migration to legacy graphs
graph.migrate_legacy();

// Check that ports were added
for node in graph.nodes.values() {
    if node.input_ports.is_empty() {
        println!("Warning: Node {} still has no input ports", node.id);
    }
    if node.output_ports.is_empty() {
        println!("Warning: Node {} still has no output ports", node.id);
    }
}
```

2. **Manual Migration for Complex Cases**
```rust
// For complex legacy graphs, manual migration may be needed
for node in graph.nodes.values_mut() {
    // Add specific ports based on node type
    if node.app == "math" && node.function == "add" {
        node.input_ports = vec![
            Port {
                id: "a".to_string(),
                name: "A".to_string(),
                port_type: PortType::Number,
                direction: PortDirection::Input,
            },
            Port {
                id: "b".to_string(),
                name: "B".to_string(),
                port_type: PortType::Number,
                direction: PortDirection::Input,
            }
        ];
        node.output_ports = vec![
            Port {
                id: "result".to_string(),
                name: "Result".to_string(),
                port_type: PortType::Number,
                direction: PortDirection::Output,
            }
        ];
    }
}
```

### Performance Issues

#### Problem: Editor becomes slow with large graphs

**Possible Causes:**
- Too many nodes or connections
- Inefficient rendering
- Complex connection validation

**Solutions:**

1. **Optimize Graph Structure**
```rust
// Break large graphs into smaller subgraphs
// Remove unnecessary connections
// Use node groups for related functionality
```

2. **Implement Lazy Loading**
```rust
// Only render visible portions of large graphs
// Virtualize node rendering for better performance
```

3. **Cache Validation Results**
```rust
// Cache connection validation results
// Recalculate only when graph changes
```

### Component Integration Issues

#### Problem: VisualEditor component doesn't integrate properly

**Possible Causes:**
- Missing CSS styles
- Incorrect prop types
- State management issues

**Solutions:**

1. **Check CSS Requirements**
```css
/* Add required CSS classes */
.visual-editor {
    position: relative;
    width: 100%;
    height: 100%;
}

.node {
    position: absolute;
    background: white;
    border: 1px solid #ccc;
    border-radius: 4px;
    padding: 8px;
    min-width: 150px;
}

.port {
    display: inline-block;
    padding: 4px 8px;
    margin: 2px;
    background: #f0f0f0;
    border-radius: 3px;
    cursor: pointer;
}

.port:hover {
    background: #e0e0e0;
}

.connection {
    pointer-events: none;
}
```

2. **Verify Props**
```rust
// Ensure props are correctly typed
let on_graph_change = {
    let graph = graph.clone();
    Callback::from(move |new_graph: Graph| {
        graph.set(new_graph);
    })
};
```

3. **Debug State Changes**
```rust
// Add logging to track graph changes
let on_graph_change = {
    let graph = graph.clone();
    Callback::from(move |new_graph: Graph| {
        println!("Graph changed: {} nodes, {} connections", 
            new_graph.nodes.len(), 
            new_graph.connections.len());
        graph.set(new_graph);
    })
};
```

## Getting Help

For additional support:
- Check the main Shtairir documentation
- Review the test cases in `shared_packages/shtairir_editor/tests/`
- Examine the web example in `shared_packages/shtairir_editor/src/bin/web_example.rs`
- Submit issues to the project repository
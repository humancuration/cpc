use shtairir_editor::{Graph, Node, Connection, Port, PortType, PortDirection, Value};
mod test_utils;

#[test]
fn test_port_type_compatibility() {
    // Test compatible port types
    assert!(PortType::are_compatible(&PortType::Number, &PortType::Number));
    assert!(PortType::are_compatible(&PortType::String, &PortType::String));
    assert!(PortType::are_compatible(&PortType::Any, &PortType::Number));
    assert!(PortType::are_compatible(&PortType::String, &PortType::Any));
    
    // Test incompatible port types
    assert!(!PortType::are_compatible(&PortType::Number, &PortType::String));
    assert!(!PortType::are_compatible(&PortType::Boolean, &PortType::Object));
}

#[test]
fn test_connection_validation() {
    let mut graph = Graph::new();
    
    // Create nodes with ports
    let node1 = Node {
        id: "node1".to_string(),
        app: "test".to_string(),
        function: "func1".to_string(),
        args: vec![],
        position: (0, 0),
        input_ports: vec![],
        output_ports: vec![
            Port {
                id: "out1".to_string(),
                name: "output".to_string(),
                port_type: PortType::Number,
                direction: PortDirection::Output,
            }
        ],
    };
    
    let node2 = Node {
        id: "node2".to_string(),
        app: "test".to_string(),
        function: "func2".to_string(),
        args: vec![],
        position: (0, 0),
        input_ports: vec![
            Port {
                id: "in1".to_string(),
                name: "input".to_string(),
                port_type: PortType::Number,
                direction: PortDirection::Input,
            }
        ],
        output_ports: vec![],
    };
    
    graph.add_node(node1);
    graph.add_node(node2);
    
    // Create a valid connection
    let valid_connection = Connection {
        id: "conn1".to_string(),
        from_node: "node1".to_string(),
        from_port: "out1".to_string(),
        to_node: "node2".to_string(),
        to_port: "in1".to_string(),
    };
    
    assert!(valid_connection.is_valid(&graph.nodes));
    
    // Create an invalid connection (incompatible types)
    let node3 = Node {
        id: "node3".to_string(),
        app: "test".to_string(),
        function: "func3".to_string(),
        args: vec![],
        position: (0, 0),
        input_ports: vec![
            Port {
                id: "in1".to_string(),
                name: "input".to_string(),
                port_type: PortType::String,
                direction: PortDirection::Input,
            }
        ],
        output_ports: vec![],
    };
    
    graph.add_node(node3);
    
    let invalid_connection = Connection {
        id: "conn2".to_string(),
        from_node: "node1".to_string(),
        from_port: "out1".to_string(),
        to_node: "node3".to_string(),
        to_port: "in1".to_string(),
    };
    
    assert!(!invalid_connection.is_valid(&graph.nodes));
}

#[test]
fn test_script_conversion() {
    let mut graph = Graph::new();
    
    // Create nodes
    let node1 = Node {
        id: "node1".to_string(),
        app: "math".to_string(),
        function: "add".to_string(),
        args: vec![Value::Number(5.0), Value::Number(3.0)],
        position: (0, 0),
        input_ports: vec![
            Port::default_input(),
        ],
        output_ports: vec![
            Port::default_output(),
        ],
    };
    
    let node2 = Node {
        id: "node2".to_string(),
        app: "math".to_string(),
        function: "multiply".to_string(),
        args: vec![Value::Identifier("node1.output".to_string()), Value::Number(2.0)],
        position: (0, 0),
        input_ports: vec![
            Port::default_input(),
        ],
        output_ports: vec![
            Port::default_output(),
        ],
    };
    
    graph.add_node(node1);
    graph.add_node(node2);
    
    // Add connection
    let connection = Connection {
        id: "conn1".to_string(),
        from_node: "node1".to_string(),
        from_port: "default_output".to_string(),
        to_node: "node2".to_string(),
        to_port: "default_input".to_string(),
    };
    graph.connections.push(connection);
    
    // Convert to script
    let script = graph.to_script();
    
    assert_eq!(script.commands.len(), 2);
    assert_eq!(script.commands[0].app, "math");
    assert_eq!(script.commands[0].function, "add");
    assert_eq!(script.commands[1].app, "math");
    assert_eq!(script.commands[1].function, "multiply");
}

// Test custom port type compatibility
#[test]
fn test_custom_port_compatibility() {
    // Ref: docs/shtairir_visual_editor.md#feature-explanations
    assert!(PortType::are_compatible(
        &PortType::Custom("UserData".to_string()),
        &PortType::Custom("UserData".to_string())
    ));
    assert!(!PortType::are_compatible(
        &PortType::Custom("UserData".to_string()),
        &PortType::Custom("ProductData".to_string())
    ));
}

// Test port creation and validation
#[test]
fn test_custom_port_creation() {
    // Ref: docs/shtairir_visual_editor.md#feature-explanations
    let custom_port = Port {
        id: "custom_port".to_string(),
        name: "Custom Data".to_string(),
        port_type: PortType::Custom("AnalyticsData".to_string()),
        direction: PortDirection::Input,
    };
    
    assert_eq!(custom_port.id, "custom_port");
    assert_eq!(custom_port.port_type, PortType::Custom("AnalyticsData".to_string()));
}

// Test port type edge cases
#[test]
fn test_port_type_edge_cases() {
    // Ref: docs/shtairir_visual_editor.md#feature-explanations
    assert!(PortType::are_compatible(&PortType::Any, &PortType::Custom("AnyType".to_string())));
    assert!(!PortType::are_compatible(&PortType::Number, &PortType::Boolean));
}

// Test directional validation
#[test]
fn test_directional_validation() {
    // Ref: docs/shtairir_visual_editor.md#connection-validation
    let mut graph = test_utils::create_test_graph();
    
    // Attempt invalid connection (input to input)
    let invalid_connection = Connection {
        id: "invalid_conn".to_string(),
        from_node: "node1".to_string(),
        from_port: "input1".to_string(),
        to_node: "node2".to_string(),
        to_port: "input1".to_string(),
    };
    
    assert!(!invalid_connection.is_valid(&graph.nodes), "Input to input connection should be invalid");
}

// Test non-existent node/port handling
#[test]
fn test_nonexistent_validation() {
    // Ref: docs/shtairir_visual_editor.md#connection-validation
    let mut graph = test_utils::create_test_graph();
    
    let missing_node_conn = Connection {
        id: "missing_node".to_string(),
        from_node: "ghost_node".to_string(),
        from_port: "output".to_string(),
        to_node: "node1".to_string(),
        to_port: "input1".to_string(),
    };
    
    let missing_port_conn = Connection {
        id: "missing_port".to_string(),
        from_node: "node1".to_string(),
        from_port: "phantom_port".to_string(),
        to_node: "node2".to_string(),
        to_port: "input1".to_string(),
    };
    
    assert!(!missing_node_conn.is_valid(&graph.nodes), "Connection with missing node should be invalid");
    assert!(!missing_port_conn.is_valid(&graph.nodes), "Connection with missing port should be invalid");
}

// Test parameter updates
#[test]
fn test_parameter_updates() {
    // Ref: docs/shtairir_visual_editor.md#parameter-controls
    let mut node = test_utils::create_node_with_params();
    
    // Simulate parameter change
    node.args[0] = Value::Number(10.0);
    node.args[1] = Value::String("updated".to_string());
    
    assert_eq!(node.args[0], Value::Number(10.0), "Parameter update failed for number");
    assert_eq!(node.args[1], Value::String("updated".to_string()), "Parameter update failed for string");
}

// Test parameter persistence
#[test]
fn test_parameter_persistence() {
    // Ref: docs/shtairir_visual_editor.md#parameter-controls
    let original_node = test_utils::create_node_with_params();
    let serialized = serde_json::to_string(&original_node).expect("Serialization failed");
    let deserialized: Node = serde_json::from_str(&serialized).expect("Deserialization failed");
    
    assert_eq!(original_node.args, deserialized.args, "Parameter persistence failed after serialization");
}

// Test complex graph conversion
#[test]
fn test_complex_graph_conversion() {
    // Ref: docs/shtairir_visual_editor.md#script-conversion
    let mut graph = test_utils::create_complex_graph();
    let script = graph.to_script();
    
    // Verify command count and order
    assert_eq!(script.commands.len(), 5, "Complex graph should produce 5 commands");
    assert_eq!(script.commands[0].function, "start", "First command should be 'start'");
    assert_eq!(script.commands[4].function, "end", "Last command should be 'end'");
    
    // Verify connection preservation
    assert!(script.commands[2].args.contains(&Value::Identifier("branch1.output".to_string())),
        "Connection should be preserved as identifier reference");
}

// Test round-trip conversion
#[test]
fn test_round_trip_conversion() {
    // Ref: docs/shtairir_visual_editor.md#script-conversion
    let original_graph = test_utils::create_test_graph();
    let script = original_graph.to_script();
    let new_graph = Graph::from_script(&script);
    
    // Compare essential properties (ignore positions)
    assert_eq!(original_graph.nodes.len(), new_graph.nodes.len(), "Node count should be preserved");
    assert_eq!(original_graph.connections.len(), new_graph.connections.len(), "Connection count should be preserved");
}

// Test legacy migration
#[test]
fn test_legacy_migration() {
    // Ref: docs/shtairir_visual_editor.md#troubleshooting
    let mut legacy_graph = test_utils::load_legacy_graph();
    legacy_graph.migrate_legacy();
    
    for node in legacy_graph.nodes.values() {
        assert!(!node.input_ports.is_empty(), "Node should have input ports after migration");
        assert!(!node.output_ports.is_empty(), "Node should have output ports after migration");
    }
}
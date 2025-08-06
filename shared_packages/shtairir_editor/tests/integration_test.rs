use shtairir_editor::{VisualEditor, NodeEditor, Graph, Node};
use yew::LocalServerRenderer;
use yew::prelude::*;
mod test_utils;

// Test VisualEditor rendering
#[tokio::test]
async fn test_visual_editor_rendering() {
    // Ref: docs/shtairir_visual_editor.md#api-reference
    let graph = test_utils::create_test_graph();
    let editor = LocalServerRenderer::<VisualEditor>::with_props(VisualEditorProps {
        graph,
        on_graph_change: Callback::noop(),
    });
    
    let rendered = editor.render().await;
    assert!(rendered.contains("class=\"node\""), "Rendered output should contain node elements");
    assert!(rendered.contains("class=\"connection\""), "Rendered output should contain connection elements");
}

// Test NodeEditor parameter controls
#[tokio::test]
async fn test_node_editor_controls() {
    // Ref: docs/shtairir_visual_editor.md#api-reference
    let node = test_utils::create_test_node();
    let editor = LocalServerRenderer::<NodeEditor>::with_props(NodeEditorProps {
        node,
        on_node_change: Callback::noop(),
    });
    
    let rendered = editor.render().await;
    assert!(rendered.contains("type=\"text\""), "Rendered output should contain text input for parameters");
}

// Test complex graph with custom port types
#[test]
fn test_custom_port_types_graph() {
    // Ref: docs/shtairir_visual_editor.md#feature-explanations
    use shtairir_editor::{Port, PortType, PortDirection};
    
    let mut graph = Graph::new();
    
    let node_with_custom_ports = Node {
        id: "custom_node".to_string(),
        app: "analytics".to_string(),
        function: "process".to_string(),
        args: vec![],
        position: (0, 0),
        input_ports: vec![
            Port {
                id: "input1".to_string(),
                name: "User Data".to_string(),
                port_type: PortType::Custom("UserData".to_string()),
                direction: PortDirection::Input,
            }
        ],
        output_ports: vec![
            Port {
                id: "output1".to_string(),
                name: "Analytics Result".to_string(),
                port_type: PortType::Custom("AnalyticsData".to_string()),
                direction: PortDirection::Output,
            }
        ],
    };
    
    graph.add_node(node_with_custom_ports);
    
    let node = graph.nodes.get("custom_node").expect("Node should exist");
    assert_eq!(node.input_ports[0].port_type, PortType::Custom("UserData".to_string()));
    assert_eq!(node.output_ports[0].port_type, PortType::Custom("AnalyticsData".to_string()));
}

// Test script conversion with complex data types
#[test]
fn test_complex_data_types_conversion() {
    // Ref: docs/shtairir_visual_editor.md#script-conversion
    use shtairir_editor::{Port, PortType, PortDirection};
    use std::collections::HashMap;
    
    let mut graph = Graph::new();
    
    let node_with_complex_data = Node {
        id: "complex_node".to_string(),
        app: "data".to_string(),
        function: "process".to_string(),
        args: vec![
            Value::Object({
                let mut map = HashMap::new();
                map.insert("key1".to_string(), Value::String("value1".to_string()));
                map.insert("key2".to_string(), Value::Number(42.0));
                map
            }),
            Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ]),
        ],
        position: (0, 0),
        input_ports: vec![
            Port {
                id: "input1".to_string(),
                name: "Object Input".to_string(),
                port_type: PortType::Object,
                direction: PortDirection::Input,
            },
            Port {
                id: "input2".to_string(),
                name: "Array Input".to_string(),
                port_type: PortType::Array,
                direction: PortDirection::Input,
            }
        ],
        output_ports: vec![
            Port {
                id: "output1".to_string(),
                name: "Result".to_string(),
                port_type: PortType::Object,
                direction: PortDirection::Output,
            }
        ],
    };
    
    graph.add_node(node_with_complex_data);
    let script = graph.to_script();
    
    assert_eq!(script.commands.len(), 1);
    assert_eq!(script.commands[0].args.len(), 2);
    
    // Verify complex data types are preserved
    match &script.commands[0].args[0] {
        Value::Object(map) => {
            assert!(map.contains_key("key1"));
            assert!(map.contains_key("key2"));
        },
        _ => panic!("First argument should be an object"),
    }
    
    match &script.commands[0].args[1] {
        Value::Array(arr) => {
            assert_eq!(arr.len(), 3);
        },
        _ => panic!("Second argument should be an array"),
    }
}

// Test graph migration with complex scenarios
#[test]
fn test_complex_migration_scenario() {
    // Ref: docs/shtairir_visual_editor.md#troubleshooting
    let mut graph = Graph::new();
    
    // Create a mix of legacy and modern nodes
    let legacy_node = Node {
        id: "legacy_node".to_string(),
        app: "old".to_string(),
        function: "process".to_string(),
        args: vec![Value::Number(42.0)],
        position: (0, 0),
        input_ports: vec![],
        output_ports: vec![],
    };
    
    let modern_node = Node {
        id: "modern_node".to_string(),
        app: "new".to_string(),
        function: "process".to_string(),
        args: vec![Value::Number(24.0)],
        position: (100, 0),
        input_ports: vec![
            Port {
                id: "input1".to_string(),
                name: "Input".to_string(),
                port_type: PortType::Number,
                direction: PortDirection::Input,
            }
        ],
        output_ports: vec![
            Port {
                id: "output1".to_string(),
                name: "Output".to_string(),
                port_type: PortType::Number,
                direction: PortDirection::Output,
            }
        ],
    };
    
    graph.add_node(legacy_node);
    graph.add_node(modern_node);
    
    // Before migration
    assert!(graph.nodes.get("legacy_node").unwrap().input_ports.is_empty());
    assert!(!graph.nodes.get("modern_node").unwrap().input_ports.is_empty());
    
    // Apply migration
    graph.migrate_legacy();
    
    // After migration
    assert!(!graph.nodes.get("legacy_node").unwrap().input_ports.is_empty());
    assert!(!graph.nodes.get("modern_node").unwrap().input_ports.is_empty());
    
    // Verify migration didn't affect existing ports
    assert_eq!(graph.nodes.get("modern_node").unwrap().input_ports.len(), 1);
    assert_eq!(graph.nodes.get("modern_node").unwrap().input_ports[0].id, "input1");
}
use shtairir_editor::{Graph, Node, Connection, Port, PortType, PortDirection, Value};
use shtairir::ast::Script;
use std::collections::HashMap;

pub fn create_test_graph() -> Graph {
    let mut graph = Graph::new();
    
    // Create nodes with ports
    let node1 = Node {
        id: "node1".to_string(),
        app: "test".to_string(),
        function: "func1".to_string(),
        args: vec![Value::Number(5.0)],
        position: (0, 0),
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
    
    let node2 = Node {
        id: "node2".to_string(),
        app: "test".to_string(),
        function: "func2".to_string(),
        args: vec![Value::Number(3.0)],
        position: (200, 0),
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
    
    graph.add_node(node1);
    graph.add_node(node2);
    
    // Add connection
    let connection = Connection {
        id: "conn1".to_string(),
        from_node: "node1".to_string(),
        from_port: "output1".to_string(),
        to_node: "node2".to_string(),
        to_port: "input1".to_string(),
    };
    graph.connections.push(connection);
    
    graph
}

pub fn create_complex_graph() -> Graph {
    let mut graph = Graph::new();
    
    // Create nodes for a complex graph with branching connections
    let start_node = Node {
        id: "start".to_string(),
        app: "test".to_string(),
        function: "start".to_string(),
        args: vec![],
        position: (0, 100),
        input_ports: vec![],
        output_ports: vec![
            Port {
                id: "output".to_string(),
                name: "Output".to_string(),
                port_type: PortType::Any,
                direction: PortDirection::Output,
            }
        ],
    };
    
    let branch1_node = Node {
        id: "branch1".to_string(),
        app: "test".to_string(),
        function: "process".to_string(),
        args: vec![Value::Number(10.0)],
        position: (200, 0),
        input_ports: vec![
            Port {
                id: "input".to_string(),
                name: "Input".to_string(),
                port_type: PortType::Any,
                direction: PortDirection::Input,
            }
        ],
        output_ports: vec![
            Port {
                id: "output".to_string(),
                name: "Output".to_string(),
                port_type: PortType::Number,
                direction: PortDirection::Output,
            }
        ],
    };
    
    let branch2_node = Node {
        id: "branch2".to_string(),
        app: "test".to_string(),
        function: "process".to_string(),
        args: vec![Value::String("test".to_string())],
        position: (200, 200),
        input_ports: vec![
            Port {
                id: "input".to_string(),
                name: "Input".to_string(),
                port_type: PortType::Any,
                direction: PortDirection::Input,
            }
        ],
        output_ports: vec![
            Port {
                id: "output".to_string(),
                name: "Output".to_string(),
                port_type: PortType::String,
                direction: PortDirection::Output,
            }
        ],
    };
    
    let merge_node = Node {
        id: "merge".to_string(),
        app: "test".to_string(),
        function: "merge".to_string(),
        args: vec![Value::Object(HashMap::new())],
        position: (400, 100),
        input_ports: vec![
            Port {
                id: "input1".to_string(),
                name: "Input 1".to_string(),
                port_type: PortType::Number,
                direction: PortDirection::Input,
            },
            Port {
                id: "input2".to_string(),
                name: "Input 2".to_string(),
                port_type: PortType::String,
                direction: PortDirection::Input,
            }
        ],
        output_ports: vec![
            Port {
                id: "output".to_string(),
                name: "Output".to_string(),
                port_type: PortType::Object,
                direction: PortDirection::Output,
            }
        ],
    };
    
    let end_node = Node {
        id: "end".to_string(),
        app: "test".to_string(),
        function: "end".to_string(),
        args: vec![],
        position: (600, 100),
        input_ports: vec![
            Port {
                id: "input".to_string(),
                name: "Input".to_string(),
                port_type: PortType::Any,
                direction: PortDirection::Input,
            }
        ],
        output_ports: vec![],
    };
    
    graph.add_node(start_node);
    graph.add_node(branch1_node);
    graph.add_node(branch2_node);
    graph.add_node(merge_node);
    graph.add_node(end_node);
    
    // Add connections
    let connections = vec![
        Connection {
            id: "conn1".to_string(),
            from_node: "start".to_string(),
            from_port: "output".to_string(),
            to_node: "branch1".to_string(),
            to_port: "input".to_string(),
        },
        Connection {
            id: "conn2".to_string(),
            from_node: "start".to_string(),
            from_port: "output".to_string(),
            to_node: "branch2".to_string(),
            to_port: "input".to_string(),
        },
        Connection {
            id: "conn3".to_string(),
            from_node: "branch1".to_string(),
            from_port: "output".to_string(),
            to_node: "merge".to_string(),
            to_port: "input1".to_string(),
        },
        Connection {
            id: "conn4".to_string(),
            from_node: "branch2".to_string(),
            from_port: "output".to_string(),
            to_node: "merge".to_string(),
            to_port: "input2".to_string(),
        },
        Connection {
            id: "conn5".to_string(),
            from_node: "merge".to_string(),
            from_port: "output".to_string(),
            to_node: "end".to_string(),
            to_port: "input".to_string(),
        },
    ];
    
    graph.connections = connections;
    graph
}

pub fn load_legacy_graph() -> Graph {
    let mut graph = Graph::new();
    
    // Create nodes without ports (legacy format)
    let legacy_node1 = Node {
        id: "legacy1".to_string(),
        app: "legacy".to_string(),
        function: "old_func".to_string(),
        args: vec![Value::Number(42.0)],
        position: (0, 0),
        input_ports: vec![],
        output_ports: vec![],
    };
    
    let legacy_node2 = Node {
        id: "legacy2".to_string(),
        app: "legacy".to_string(),
        function: "old_func".to_string(),
        args: vec![Value::String("legacy_value".to_string())],
        position: (200, 0),
        input_ports: vec![],
        output_ports: vec![],
    };
    
    graph.add_node(legacy_node1);
    graph.add_node(legacy_node2);
    
    graph
}

pub fn create_node_with_params() -> Node {
    Node {
        id: "param_test".to_string(),
        app: "test".to_string(),
        function: "param_test".to_string(),
        args: vec![
            Value::Number(5.0),
            Value::String("test".to_string()),
            Value::Boolean(true),
            Value::Array(vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]),
        ],
        position: (0, 0),
        input_ports: vec![
            Port {
                id: "input1".to_string(),
                name: "Number Input".to_string(),
                port_type: PortType::Number,
                direction: PortDirection::Input,
            },
            Port {
                id: "input2".to_string(),
                name: "String Input".to_string(),
                port_type: PortType::String,
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
    }
}

pub fn create_test_node() -> Node {
    Node {
        id: "test_node".to_string(),
        app: "test".to_string(),
        function: "test_function".to_string(),
        args: vec![
            Value::Number(10.0),
            Value::String("hello".to_string()),
            Value::Boolean(false),
        ],
        position: (0, 0),
        input_ports: vec![
            Port {
                id: "number_input".to_string(),
                name: "Number".to_string(),
                port_type: PortType::Number,
                direction: PortDirection::Input,
            },
            Port {
                id: "string_input".to_string(),
                name: "String".to_string(),
                port_type: PortType::String,
                direction: PortDirection::Input,
            },
            Port {
                id: "bool_input".to_string(),
                name: "Boolean".to_string(),
                port_type: PortType::Boolean,
                direction: PortDirection::Input,
            }
        ],
        output_ports: vec![
            Port {
                id: "output".to_string(),
                name: "Output".to_string(),
                port_type: PortType::Any,
                direction: PortDirection::Output,
            }
        ],
    }
}
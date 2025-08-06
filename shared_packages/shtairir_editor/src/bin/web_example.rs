use shtairir_editor::{VisualEditor, Graph, Node, Value, Port, PortType, PortDirection, Connection, NodeEditor};
use yew::prelude::*;
use std::collections::HashMap;
use shtairir::ast::Script;

#[function_component(App)]
fn app() -> Html {
    let graph = use_state(|| create_demo_graph());
    let selected_node = use_state(|| Option::<Node>::None);
    let script_output = use_state(|| String::new());
    let show_script = use_state(|| false);
    
    let on_graph_change = {
        let graph = graph.clone();
        Callback::from(move |new_graph| {
            graph.set(new_graph);
        })
    };
    
    let on_node_select = {
        let selected_node = selected_node.clone();
        Callback::from(move |node: Node| {
            selected_node.set(Some(node));
        })
    };
    
    let on_node_change = {
        let graph = graph.clone();
        let selected_node = selected_node.clone();
        Callback::from(move |new_node: Node| {
            let mut new_graph = (*graph).clone();
            new_graph.add_node(new_node.clone());
            graph.set(new_graph);
            selected_node.set(Some(new_node));
        })
    };
    
    let convert_to_script = {
        let graph = graph.clone();
        let script_output = script_output.clone();
        let show_script = show_script.clone();
        Callback::from(move |_| {
            let script = (*graph).to_script();
            let script_text = format_script(&script);
            script_output.set(script_text);
            show_script.set(true);
        })
    };
    
    let convert_from_script = {
        let graph = graph.clone();
        let script_output = script_output.clone();
        let show_script = show_script.clone();
        Callback::from(move |_| {
            // For demo purposes, create a sample script and convert it
            let sample_script = create_sample_script();
            let new_graph = Graph::from_script(&sample_script);
            graph.set(new_graph);
            script_output.set(format_script(&sample_script));
            show_script.set(true);
        })
    };
    
    let migrate_legacy = {
        let graph = graph.clone();
        Callback::from(move |_| {
            let mut new_graph = (*graph).clone();
            new_graph.migrate_legacy();
            graph.set(new_graph);
        })
    };
    
    let add_math_node = {
        let graph = graph.clone();
        Callback::from(move |_| {
            let mut new_graph = (*graph).clone();
            let node_id = format!("math_node_{}", new_graph.nodes.len());
            let position = (100 + (new_graph.nodes.len() as i32 * 50) % 500, 100 + (new_graph.nodes.len() as i32 * 30) % 300);
            
            let node = create_math_node(&node_id, position);
            new_graph.add_node(node);
            graph.set(new_graph);
        })
    };
    
    let add_string_node = {
        let graph = graph.clone();
        Callback::from(move |_| {
            let mut new_graph = (*graph).clone();
            let node_id = format!("string_node_{}", new_graph.nodes.len());
            let position = (100 + (new_graph.nodes.len() as i32 * 50) % 500, 100 + (new_graph.nodes.len() as i32 * 30) % 300);
            
            let node = create_string_node(&node_id, position);
            new_graph.add_node(node);
            graph.set(new_graph);
        })
    };
    
    let add_boolean_node = {
        let graph = graph.clone();
        Callback::from(move |_| {
            let mut new_graph = (*graph).clone();
            let node_id = format!("bool_node_{}", new_graph.nodes.len());
            let position = (100 + (new_graph.nodes.len() as i32 * 50) % 500, 100 + (new_graph.nodes.len() as i32 * 30) % 300);
            
            let node = create_boolean_node(&node_id, position);
            new_graph.add_node(node);
            graph.set(new_graph);
        })
    };
    
    let add_custom_node = {
        let graph = graph.clone();
        Callback::from(move |_| {
            let mut new_graph = (*graph).clone();
            let node_id = format!("custom_node_{}", new_graph.nodes.len());
            let position = (100 + (new_graph.nodes.len() as i32 * 50) % 500, 100 + (new_graph.nodes.len() as i32 * 30) % 300);
            
            let node = create_custom_node(&node_id, position);
            new_graph.add_node(node);
            graph.set(new_graph);
        })
    };
    
    let add_invalid_connection = {
        let graph = graph.clone();
        Callback::from(move |_| {
            let mut new_graph = (*graph).clone();
            
            // Add nodes for invalid connection demo if they don't exist
            if !new_graph.nodes.contains_key("number_source") {
                let number_source = Node {
                    id: "number_source".to_string(),
                    app: "math".to_string(),
                    function: "constant".to_string(),
                    args: vec![Value::Number(42.0)],
                    position: (700, 100),
                    input_ports: vec![],
                    output_ports: vec![Port {
                        id: "output".to_string(),
                        name: "value".to_string(),
                        port_type: PortType::Number,
                        direction: PortDirection::Output,
                    }],
                };
                new_graph.add_node(number_source);
            }
            
            if !new_graph.nodes.contains_key("string_target") {
                let string_target = Node {
                    id: "string_target".to_string(),
                    app: "text".to_string(),
                    function: "process".to_string(),
                    args: vec![Value::String("default".to_string())],
                    position: (700, 200),
                    input_ports: vec![Port {
                        id: "input".to_string(),
                        name: "text".to_string(),
                        port_type: PortType::String,
                        direction: PortDirection::Input,
                    }],
                    output_ports: vec![],
                };
                new_graph.add_node(string_target);
            }
            
            // Add invalid connection (Number -> String)
            let invalid_conn = Connection {
                id: "invalid_conn".to_string(),
                from_node: "number_source".to_string(),
                from_port: "output".to_string(),
                to_node: "string_target".to_string(),
                to_port: "input".to_string(),
            };
            
            new_graph.connections.push(invalid_conn);
            graph.set(new_graph);
        })
    };
    
    html! {
        <div class="app">
            <style>
                {include_str!("style.css")}
            </style>
            
            <div class="header">
                <h1>{"Shtairir Visual Editor - Enhanced Demo"}</h1>
                <p class="subtitle">{"Demonstrating all documented features"}</p>
            </div>
            
            <div class="toolbar">
                <div class="toolbar-section">
                    <h3>{"Add Nodes"}</h3>
                    <button onclick={add_math_node}>{"Add Math Node"}</button>
                    <button onclick={add_string_node}>{"Add String Node"}</button>
                    <button onclick={add_boolean_node}>{"Add Boolean Node"}</button>
                    <button onclick={add_custom_node}>{"Add Custom Node"}</button>
                </div>
                
                <div class="toolbar-section">
                    <h3>{"Connections"}</h3>
                    <button onclick={add_invalid_connection}>{"Add Invalid Connection"}</button>
                </div>
                
                <div class="toolbar-section">
                    <h3>{"Script Conversion"}</h3>
                    <button onclick={convert_to_script}>{"Graph to Script"}</button>
                    <button onclick={convert_from_script}>{"Script to Graph"}</button>
                    <button onclick={migrate_legacy}>{"Migrate Legacy"}</button>
                </div>
            </div>
            
            <div class="main-content">
                <div class="editor-panel">
                    <div class="panel-header">
                        <h2>{"Visual Editor"}</h2>
                        <span class="info">{format!("Nodes: {} | Connections: {}", graph.nodes.len(), graph.connections.len())}</span>
                    </div>
                    <VisualEditor
                        graph={(*graph).clone()}
                        on_graph_change={on_graph_change}
                        on_node_select={on_node_select}
                    />
                </div>
                
                <div class="side-panel">
                    { if let Some(node) = &*selected_node {
                        html! {
                            <div class="panel-section">
                                <div class="panel-header">
                                    <h2>{"Node Editor"}</h2>
                                    <button class="close-btn" onclick={Callback::from(move |_| selected_node.set(None))}>{"×"}</button>
                                </div>
                                <NodeEditor
                                    node={node.clone()}
                                    on_node_change={on_node_change}
                                />
                                <div class="node-info">
                                    <h3>{"Node Information"}</h3>
                                    <p><strong>{"ID: "}</strong>{&node.id}</p>
                                    <p><strong>{"App: "}</strong>{&node.app}</p>
                                    <p><strong>{"Function: "}</strong>{&node.function}</p>
                                    <p><strong>{"Position: "}</strong>{format!("({}, {})", node.position.0, node.position.1)}</p>
                                    <h4>{"Input Ports:"}</h4>
                                    <ul>
                                        { for node.input_ports.iter().map(|port| {
                                            html! {
                                                <li>
                                                    <span class={"port-indicator input-port"}></span>
                                                    {format!("{}: {:?}", port.name, port.port_type)}
                                                </li>
                                            }
                                        })}
                                    </ul>
                                    <h4>{"Output Ports:"}</h4>
                                    <ul>
                                        { for node.output_ports.iter().map(|port| {
                                            html! {
                                                <li>
                                                    <span class={"port-indicator output-port"}></span>
                                                    {format!("{}: {:?}", port.name, port.port_type)}
                                                </li>
                                            }
                                        })}
                                    </ul>
                                </div>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="panel-section">
                                <div class="panel-header">
                                    <h2>{"Node Editor"}</h2>
                                </div>
                                <p class="placeholder">{"Select a node to edit its parameters"}</p>
                            </div>
                        }
                    }}
                    
                    { if *show_script {
                        html! {
                            <div class="panel-section">
                                <div class="panel-header">
                                    <h2>{"Generated Script"}</h2>
                                    <button class="close-btn" onclick={Callback::from(move |_| show_script.set(false))}>{"×"}</button>
                                </div>
                                <div class="script-output">
                                    <pre>{(*script_output).clone()}</pre>
                                </div>
                            </div>
                        }
                    } else {
                        html! {}
                    }}
                    
                    <div class="panel-section">
                        <div class="panel-header">
                            <h2>{"Legend"}</h2>
                        </div>
                        <div class="legend">
                            <div class="legend-item">
                                <div class="legend-color valid-connection"></div>
                                <span>{"Valid Connection"}</span>
                            </div>
                            <div class="legend-item">
                                <div class="legend-color invalid-connection"></div>
                                <span>{"Invalid Connection"}</span>
                            </div>
                            <div class="legend-item">
                                <span class="port-indicator input-port"></span>
                                <span>{"Input Port"}</span>
                            </div>
                            <div class="legend-item">
                                <span class="port-indicator output-port"></span>
                                <span>{"Output Port"}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="footer">
                <h3>{"Features Demonstrated"}</h3>
                <div class="features-grid">
                    <div class="feature-item">
                        <h4>{"✓ Diverse Node Types"}</h4>
                        <p>{"Math operations, string manipulation, boolean logic, and custom object/array nodes"}</p>
                    </div>
                    <div class="feature-item">
                        <h4>{"✓ Connection Validation"}</h4>
                        <p>{"Real-time type checking with visual feedback for valid (blue) and invalid (red) connections"}</p>
                    </div>
                    <div class="feature-item">
                        <h4>{"✓ Parameter Editing"}</h4>
                        <p>{"Interactive controls for numbers, strings, booleans, arrays, and objects"}</p>
                    </div>
                    <div class="feature-item">
                        <h4>{"✓ Script Conversion"}</h4>
                        <p>{"Bidirectional conversion between visual graphs and Shtairir scripts"}</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn create_demo_graph() -> Graph {
    let mut graph = Graph::new();
    
    // Math operations node (add)
    let math_add = create_math_node("math_add", (100, 100));
    graph.add_node(math_add);
    
    // Math operations node (multiply)
    let math_multiply = create_math_node("math_multiply", (400, 100));
    // Change function to multiply
    if let Some(node) = graph.nodes.get_mut("math_multiply") {
        node.function = "multiply".to_string();
        node.args = vec![Value::Number(0.0), Value::Number(2.0)];
    }
    
    // String manipulation node (concatenate)
    let string_concat = create_string_node("string_concat", (100, 250));
    graph.add_node(string_concat);
    
    // String manipulation node (uppercase)
    let string_upper = create_string_node("string_upper", (400, 250));
    if let Some(node) = graph.nodes.get_mut("string_upper") {
        node.function = "uppercase".to_string();
    }
    
    // Boolean logic node (AND)
    let bool_and = create_boolean_node("bool_and", (100, 400));
    graph.add_node(bool_and);
    
    // Boolean logic node (OR)
    let bool_or = create_boolean_node("bool_or", (400, 400));
    if let Some(node) = graph.nodes.get_mut("bool_or") {
        node.function = "or".to_string();
        node.args = vec![Value::Boolean(false), Value::Boolean(false)];
    }
    
    // Custom node with object/array ports
    let custom_node = create_custom_node("custom_processor", (700, 250));
    graph.add_node(custom_node);
    
    // Valid connections
    let valid_connections = vec![
        Connection {
            id: "conn1".to_string(),
            from_node: "math_add".to_string(),
            from_port: "result".to_string(),
            to_node: "math_multiply".to_string(),
            to_port: "a".to_string(),
        },
        Connection {
            id: "conn2".to_string(),
            from_node: "string_concat".to_string(),
            from_port: "result".to_string(),
            to_node: "string_upper".to_string(),
            to_port: "input".to_string(),
        },
        Connection {
            id: "conn3".to_string(),
            from_node: "bool_and".to_string(),
            from_port: "result".to_string(),
            to_node: "bool_or".to_string(),
            to_port: "a".to_string(),
        },
        Connection {
            id: "conn4".to_string(),
            from_node: "math_multiply".to_string(),
            from_port: "result".to_string(),
            to_node: "custom_processor".to_string(),
            to_port: "number_input".to_string(),
        },
        Connection {
            id: "conn5".to_string(),
            from_node: "string_upper".to_string(),
            from_port: "result".to_string(),
            to_node: "custom_processor".to_string(),
            to_port: "string_input".to_string(),
        },
    ];
    
    graph.connections = valid_connections;
    graph
}

fn create_math_node(id: &str, position: (i32, i32)) -> Node {
    Node {
        id: id.to_string(),
        app: "math".to_string(),
        function: "add".to_string(),
        args: vec![Value::Number(5.0), Value::Number(3.0)],
        position,
        input_ports: vec![
            Port {
                id: "a".to_string(),
                name: "a".to_string(),
                port_type: PortType::Number,
                direction: PortDirection::Input,
            },
            Port {
                id: "b".to_string(),
                name: "b".to_string(),
                port_type: PortType::Number,
                direction: PortDirection::Input,
            },
        ],
        output_ports: vec![
            Port {
                id: "result".to_string(),
                name: "result".to_string(),
                port_type: PortType::Number,
                direction: PortDirection::Output,
            },
        ],
    }
}

fn create_string_node(id: &str, position: (i32, i32)) -> Node {
    Node {
        id: id.to_string(),
        app: "text".to_string(),
        function: "concatenate".to_string(),
        args: vec![Value::String("Hello".to_string()), Value::String("World".to_string())],
        position,
        input_ports: vec![
            Port {
                id: "input1".to_string(),
                name: "text1".to_string(),
                port_type: PortType::String,
                direction: PortDirection::Input,
            },
            Port {
                id: "input2".to_string(),
                name: "text2".to_string(),
                port_type: PortType::String,
                direction: PortDirection::Input,
            },
        ],
        output_ports: vec![
            Port {
                id: "result".to_string(),
                name: "result".to_string(),
                port_type: PortType::String,
                direction: PortDirection::Output,
            },
        ],
    }
}

fn create_boolean_node(id: &str, position: (i32, i32)) -> Node {
    Node {
        id: id.to_string(),
        app: "logic".to_string(),
        function: "and".to_string(),
        args: vec![Value::Boolean(true), Value::Boolean(false)],
        position,
        input_ports: vec![
            Port {
                id: "a".to_string(),
                name: "a".to_string(),
                port_type: PortType::Boolean,
                direction: PortDirection::Input,
            },
            Port {
                id: "b".to_string(),
                name: "b".to_string(),
                port_type: PortType::Boolean,
                direction: PortDirection::Input,
            },
        ],
        output_ports: vec![
            Port {
                id: "result".to_string(),
                name: "result".to_string(),
                port_type: PortType::Boolean,
                direction: PortDirection::Output,
            },
        ],
    }
}

fn create_custom_node(id: &str, position: (i32, i32)) -> Node {
    let mut object_data = HashMap::new();
    object_data.insert("name".to_string(), Value::String("test".to_string()));
    object_data.insert("value".to_string(), Value::Number(42.0));
    
    Node {
        id: id.to_string(),
        app: "custom".to_string(),
        function: "process_data".to_string(),
        args: vec![
            Value::Object(object_data),
            Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ]),
        ],
        position,
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
                id: "object_input".to_string(),
                name: "Object".to_string(),
                port_type: PortType::Object,
                direction: PortDirection::Input,
            },
            Port {
                id: "array_input".to_string(),
                name: "Array".to_string(),
                port_type: PortType::Array,
                direction: PortDirection::Input,
            },
        ],
        output_ports: vec![
            Port {
                id: "object_output".to_string(),
                name: "Processed Object".to_string(),
                port_type: PortType::Object,
                direction: PortDirection::Output,
            },
            Port {
                id: "array_output".to_string(),
                name: "Processed Array".to_string(),
                port_type: PortType::Array,
                direction: PortDirection::Output,
            },
        ],
    }
}

fn create_sample_script() -> Script {
    use shtairir::ast::{Command, Value};
    
    Script {
        commands: vec![
            Command {
                app: "math".to_string(),
                function: "add".to_string(),
                args: vec![Value::Number(10.0), Value::Number(5.0)],
            },
            Command {
                app: "text".to_string(),
                function: "uppercase".to_string(),
                args: vec![Value::String("hello world".to_string())],
            },
            Command {
                app: "logic".to_string(),
                function: "and".to_string(),
                args: vec![Value::Boolean(true), Value::Boolean(false)],
            },
        ],
    }
}

fn format_script(script: &Script) -> String {
    let mut output = String::new();
    output.push_str("# Generated Shtairir Script\n\n");
    
    for (i, command) in script.commands.iter().enumerate() {
        output.push_str(&format!("{}.{}.{}", i + 1, command.app, command.function));
        
        if !command.args.is_empty() {
            output.push_str("(");
            for (j, arg) in command.args.iter().enumerate() {
                if j > 0 {
                    output.push_str(", ");
                }
                match arg {
                    Value::Number(n) => output.push_str(&n.to_string()),
                    Value::String(s) => output.push_str(&format!("\"{}\"", s)),
                    Value::Boolean(b) => output.push_str(&b.to_string()),
                    Value::Identifier(id) => output.push_str(id),
                    Value::Object(_) => output.push_str("{object}"),
                    Value::Array(_) => output.push_str("[array]"),
                }
            }
            output.push(')');
        }
        output.push('\n');
    }
    
    output
}

fn main() {
    yew::Renderer::<App>::new().render();
}
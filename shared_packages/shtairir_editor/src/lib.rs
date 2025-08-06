use shtairir::ast::{Command, Value, Script};
use yew::prelude::*;
use std::collections::{HashMap, HashSet};

mod port;
pub use port::{Port, PortType, PortDirection};

mod node_editor;
pub use node_editor::{NodeEditor, NodeEditorProps};

#[derive(Clone, PartialEq)]
pub struct Node {
    pub id: String,
    pub app: String,
    pub function: String,
    pub args: Vec<Value>,
    pub position: (i32, i32),
    pub input_ports: Vec<Port>,
    pub output_ports: Vec<Port>,
}

impl Node {
    pub fn get_input_index(&self, port_id: &str) -> Option<usize> {
        self.input_ports.iter().position(|p| p.id == port_id)
    }
}

#[derive(Clone, PartialEq)]
pub struct Connection {
    pub id: String,
    pub from_node: String,
    pub from_port: String,
    pub to_node: String,
    pub to_port: String,
}

#[derive(Clone, PartialEq)]
pub struct Graph {
    pub nodes: HashMap<String, Node>,
    pub connections: Vec<Connection>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            connections: Vec::new(),
        }
    }
    
    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }
    
    pub fn remove_node(&mut self, node_id: &str) {
        self.nodes.remove(node_id);
        self.connections.retain(|c| c.from_node != node_id && c.to_node != node_id);
    }
    
    pub fn to_script(&self) -> Script {
        let mut commands = vec![];
        let mut visited = HashSet::new();
        
        // Find starting nodes (no incoming connections)
        let start_nodes = self.nodes.values()
            .filter(|node| !self.connections.iter().any(|c| c.to_node == node.id))
            .collect::<Vec<_>>();
        
        for node in start_nodes {
            self.traverse_node(node, &mut commands, &mut visited);
        }
        
        Script { commands }
    }
    
    fn traverse_node(&self, node: &Node, commands: &mut Vec<Command>, visited: &mut HashSet<String>) {
        if visited.contains(&node.id) {
            return;
        }
        visited.insert(node.id.clone());
        
        // Create command with resolved arguments
        let mut command = Command {
            app: node.app.clone(),
            function: node.function.clone(),
            args: node.args.clone(),
        };
        
        // Process output connections
        for conn in self.connections.iter().filter(|c| c.from_node == node.id) {
            if let Some(target_node) = self.nodes.get(&conn.to_node) {
                // Handle argument passing through connections
                if let Some(arg_index) = target_node.get_input_index(&conn.to_port) {
                    command.args[arg_index] = Value::Identifier(format!("{}.{}", node.id, conn.from_port));
                }
                self.traverse_node(target_node, commands, visited);
            }
        }
        
        commands.push(command);
    }
    
    pub fn from_script(script: &Script) -> Self {
        let mut graph = Self::new();
        
        for (i, command) in script.commands.iter().enumerate() {
            let node = Node {
                id: format!("node_{}", i),
                app: command.app.clone(),
                function: command.function.clone(),
                args: command.args.clone(),
                position: (100 * i as i32, 100),
                input_ports: vec![Port::default_input()],
                output_ports: vec![Port::default_output()],
            };
            graph.add_node(node);
        }
        
        graph
    }
    
    pub fn migrate_legacy(&mut self) {
        for node in self.nodes.values_mut() {
            if node.input_ports.is_empty() {
                node.input_ports.push(Port::default_input());
            }
            if node.output_ports.is_empty() {
                node.output_ports.push(Port::default_output());
            }
        }
    }
}

impl Connection {
    pub fn is_valid(&self, nodes: &HashMap<String, Node>) -> bool {
        let Some(from_node) = nodes.get(&self.from_node) else { return false };
        let Some(to_node) = nodes.get(&self.to_node) else { return false };
        
        let from_port = from_node.output_ports.iter()
            .find(|p| p.id == self.from_port);
        let to_port = to_node.input_ports.iter()
            .find(|p| p.id == self.to_port);
        
        match (from_port, to_port) {
            (Some(fp), Some(tp)) => PortType::are_compatible(&fp.port_type, &tp.port_type),
            _ => false
        }
    }
}

// Enhanced Yew component for the visual editor
#[derive(Properties, PartialEq)]
pub struct VisualEditorProps {
    pub graph: Graph,
    pub on_graph_change: Callback<Graph>,
    #[prop_or_default]
    pub on_node_select: Callback<Node>,
}

#[function_component(VisualEditor)]
pub fn visual_editor(props: &VisualEditorProps) -> Html {
    let graph = use_state(|| props.graph.clone());
    let dragging_node = use_state(|| Option::<(String, (i32, i32))>::None);
    let connecting_port = use_state(|| Option::<(String, String)>::None);
    
    // Update graph when props change
    use_effect_with(props.graph.clone(), {
        let graph = graph.clone();
        move |new_graph| {
            graph.set(new_graph);
        }
    });
    
    let on_node_mouse_down = {
        let dragging_node = dragging_node.clone();
        Callback::from(move |(node_id, position): (String, (i32, i32))| {
            dragging_node.set(Some((node_id, position)));
        })
    };
    
    let on_port_click = {
        let connecting_port = connecting_port.clone();
        Callback::from(move |(node_id, port_id): (String, String)| {
            if let Some((from_node, from_port)) = &*connecting_port {
                // Complete the connection
                if from_node != node_id {
                    // This would normally trigger connection creation
                    gloo_console::log!(format!("Connect {}.{} to {}.{}", from_node, from_port, node_id, port_id));
                }
                connecting_port.set(None);
            } else {
                // Start a new connection
                connecting_port.set(Some((node_id, port_id)));
            }
        })
    };
    
    let on_mouse_move = {
        let graph = graph.clone();
        let dragging_node = dragging_node.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some((node_id, start_pos)) = &*dragging_node {
                let mut new_graph = (*graph).clone();
                if let Some(node) = new_graph.nodes.get_mut(node_id) {
                    let dx = e.client_x() - start_pos.0;
                    let dy = e.client_y() - start_pos.1;
                    node.position = (node.position.0 + dx, node.position.1 + dy);
                    graph.set(new_graph);
                    dragging_node.set(Some((node_id.clone(), (e.client_x(), e.client_y()))));
                }
            }
        })
    };
    
    let on_mouse_up = {
        let dragging_node = dragging_node.clone();
        Callback::from(move |_| {
            dragging_node.set(None);
        })
    };
    
    let on_node_click = {
        let on_node_select = props.on_node_select.clone();
        Callback::from(move |node: Node| {
            on_node_select.emit(node);
        })
    };
    
    let render_connections = |connections: &[Connection], nodes: &HashMap<String, Node>| -> Html {
        connections.iter().map(|conn| {
            let from_node = match nodes.get(&conn.from_node) {
                Some(node) => node,
                None => return html! {},
            };
            let to_node = match nodes.get(&conn.to_node) {
                Some(node) => node,
                None => return html! {},
            };
            
            // Calculate port positions
            let from_port_pos = get_port_position(from_node, &conn.from_port, false);
            let to_port_pos = get_port_position(to_node, &conn.to_port, true);
            
            let (start, end) = match (from_port_pos, to_port_pos) {
                (Some(start), Some(end)) => (start, end),
                _ => return html! {},
            };
            
            let path_data = format!("M {} {} C {} {}, {} {}, {} {}",
                start.0, start.1,
                start.0 + 50, start.1,
                end.0 - 50, end.1,
                end.0, end.1
            );
            
            let is_valid = conn.is_valid(nodes);
            
            html! {
                <path
                    d={path_data}
                    stroke={if is_valid { "var(--valid-connection-color)" } else { "var(--invalid-connection-color)" }}
                    stroke-width="2"
                    fill="none"
                    class={if is_valid { "connection valid" } else { "connection invalid" }}
                />
            }
        }).collect::<Html>()
    };
    
    let render_nodes = || -> Html {
        graph.nodes.values().map(|node| {
            let node_clone = node.clone();
            let on_node_mouse_down = on_node_mouse_down.clone();
            let on_port_click = on_port_click.clone();
            let on_node_click = on_node_click.clone();
            
            html! {
                <div
                    class="node"
                    style={format!("position: absolute; left: {}px; top: {}px;", node.position.0, node.position.1)}
                    onmousedown={Callback::from(move |e: MouseEvent| {
                        e.stop_propagation();
                        on_node_mouse_down.emit((node.id.clone(), (e.client_x(), e.client_y())));
                    })}
                    onclick={Callback::from(move |_| {
                        on_node_click.emit(node_clone.clone());
                    })}
                >
                    <div class="node-header">
                        <span class="node-title">{format!("{}:{}", node.app, node.function)}</span>
                        <span class="node-id">{format!("({})", node.id)}</span>
                    </div>
                    <div class="node-body">
                        <div class="input-ports">
                            { for node.input_ports.iter().map(|port| {
                                let port_name = port.name.clone();
                                let node_id = node.id.clone();
                                let port_id = port.id.clone();
                                let on_port_click = on_port_click.clone();
                                
                                html! {
                                    <div
                                        class={format!("port input-port {}", format_port_type(&port.port_type))}
                                        data-port-id={port.id.clone()}
                                        onclick={Callback::from(move |e: MouseEvent| {
                                            e.stop_propagation();
                                            on_port_click.emit((node_id.clone(), port_id.clone()));
                                        })}
                                        title={format!("{}: {:?}", port_name, port.port_type)}
                                    >
                                        <span class="port-name">{port_name}</span>
                                    </div>
                                }
                            }) }
                        </div>
                        <div class="output-ports">
                            { for node.output_ports.iter().map(|port| {
                                let port_name = port.name.clone();
                                let node_id = node.id.clone();
                                let port_id = port.id.clone();
                                let on_port_click = on_port_click.clone();
                                
                                html! {
                                    <div
                                        class={format!("port output-port {}", format_port_type(&port.port_type))}
                                        data-port-id={port.id.clone()}
                                        onclick={Callback::from(move |e: MouseEvent| {
                                            e.stop_propagation();
                                            on_port_click.emit((node_id.clone(), port_id.clone()));
                                        })}
                                        title={format!("{}: {:?}", port_name, port.port_type)}
                                    >
                                        <span class="port-name">{port_name}</span>
                                    </div>
                                }
                            }) }
                        </div>
                    </div>
                </div>
            }
        }).collect::<Html>()
    };
    
    html! {
        <div class="visual-editor"
             onmousemove={on_mouse_move}
             onmouseup={on_mouse_up}
        >
            <div class="canvas">
                <svg class="connections" width="100%" height="100%">
                    { render_connections(&graph.connections, &graph.nodes) }
                </svg>
                { render_nodes() }
            </div>
        </div>
    }
}

fn get_port_position(node: &Node, port_id: &str, is_input: bool) -> Option<(i32, i32)> {
    let ports = if is_input { &node.input_ports } else { &node.output_ports };
    let port_index = ports.iter().position(|p| p.id == port_id)?;
    
    let port_x = if is_input { node.position.0 } else { node.position.0 + 200 };
    let port_y = node.position.1 + 40 + (port_index as i32 * 25);
    
    Some((port_x, port_y))
}

fn format_port_type(port_type: &PortType) -> &'static str {
    match port_type {
        PortType::Number => "number-port",
        PortType::String => "string-port",
        PortType::Boolean => "boolean-port",
        PortType::Object => "object-port",
        PortType::Array => "array-port",
        PortType::Custom(_) => "custom-port",
        PortType::Any => "any-port",
    }
}
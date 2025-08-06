use shtairir::ast::{Command, Value, Script};
use yew::prelude::*;
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct Node {
    pub id: String,
    pub app: String,
    pub function: String,
    pub args: Vec<Value>,
    pub position: (i32, i32),
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
        let commands = self.nodes.values().map(|node| Command {
            app: node.app.clone(),
            function: node.function.clone(),
            args: node.args.clone(),
        }).collect();
        
        Script { commands }
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
            };
            graph.add_node(node);
        }
        
        graph
    }
}

// Basic Yew component for the visual editor
#[derive(Properties, PartialEq)]
pub struct VisualEditorProps {
    pub graph: Graph,
    pub on_graph_change: Callback<Graph>,
}

#[function_component(VisualEditor)]
pub fn visual_editor(props: &VisualEditorProps) -> Html {
    let graph = use_state(|| props.graph.clone());
    
    let on_add_node = {
        let graph = graph.clone();
        Callback::from(move |_| {
            // In a real implementation, this would open a dialog to configure the node
            gloo_console::log!("Add node clicked");
        })
    };
    
    html! {
        <div class="visual-editor">
            <div class="toolbar">
                <button onclick={on_add_node}>{"Add Node"}</button>
            </div>
            <div class="canvas">
                { for graph.nodes.values().map(|node| {
                    html! {
                        <div 
                            class="node"
                            style={format!("position: absolute; left: {}px; top: {}px;", node.position.0, node.position.1)}
                        >
                            <div class="node-header">
                                <span>{format!("{}:{}", node.app, node.function)}</span>
                            </div>
                            <div class="node-body">
                                // Node content would be rendered here
                            </div>
                        </div>
                    }
                }) }
            </div>
        </div>
    }
}
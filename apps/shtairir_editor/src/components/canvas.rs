use yew::prelude::*;
use crate::models::{Graph, Node, Connection};
use crate::components::ConnectionView;

#[derive(Properties, PartialEq)]
pub struct CanvasProps {
    pub graph: Graph,
    pub on_graph_update: Callback<Graph>,
    pub on_node_select: Callback<Node>,
}

#[function_component(Canvas)]
pub fn canvas(props: &CanvasProps) -> Html {
    let dragging_node = use_state(|| Option::<(String, (i32, i32))>::None);
    let connecting_port = use_state(|| Option::<(String, String, bool)>::None); // (node_id, port_id, is_input)
    let zoom_level = use_state(|| 1.0);
    
    let on_node_mouse_down = {
        let dragging_node = dragging_node.clone();
        Callback::from(move |(node_id, position): (String, (i32, i32))| {
            dragging_node.set(Some((node_id, position)));
        })
    };
    
    let on_port_click = {
        let connecting_port = connecting_port.clone();
        let graph = use_state(|| props.graph.clone());
        let on_graph_update = props.on_graph_update.clone();
        Callback::from(move |(node_id, port_id, is_input): (String, String, bool)| {
            if let Some((from_node, from_port, from_is_input)) = &*connecting_port {
                // Complete the connection
                if from_node != &node_id && from_is_input != &is_input {
                    // Create a new connection
                    let mut new_graph = (*graph).clone();
                    let connection_id = format!("conn_{}_{}", uuid::Uuid::new_v4().to_string(), new_graph.connections.len());
                    
                    let connection = if *from_is_input {
                        // from_port is input, so this click is on output
                        Connection {
                            id: connection_id,
                            from_node: node_id.clone(),
                            from_port: port_id.clone(),
                            to_node: from_node.clone(),
                            to_port: from_port.clone(),
                            policy: None,
                        }
                    } else {
                        // from_port is output, so this click is on input
                        Connection {
                            id: connection_id,
                            from_node: from_node.clone(),
                            from_port: from_port.clone(),
                            to_node: node_id.clone(),
                            to_port: port_id.clone(),
                            policy: None,
                        }
                    };
                    
                    new_graph.connections.push(connection);
                    graph.set(new_graph.clone());
                    on_graph_update.emit(new_graph);
                }
                connecting_port.set(None);
            } else {
                // Start a new connection
                connecting_port.set(Some((node_id, port_id, is_input)));
            }
        })
    };
    
    let on_mouse_move = {
        let graph = use_state(|| props.graph.clone());
        let dragging_node = dragging_node.clone();
        let on_graph_update = props.on_graph_update.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some((node_id, start_pos)) = &*dragging_node {
                let mut new_graph = (*graph).clone();
                if let Some(node) = new_graph.nodes.get_mut(node_id) {
                    let dx = e.client_x() - start_pos.0;
                    let dy = e.client_y() - start_pos.1;
                    node.position = (node.position.0 + dx, node.position.1 + dy);
                    graph.set(new_graph.clone());
                    on_graph_update.emit(new_graph);
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
    
    let on_zoom_in = {
        let zoom_level = zoom_level.clone();
        Callback::from(move |_| {
            zoom_level.set((*zoom_level * 1.1).min(3.0));
        })
    };
    
    let on_zoom_out = {
        let zoom_level = zoom_level.clone();
        Callback::from(move |_| {
            zoom_level.set((*zoom_level * 0.9).max(0.1));
        })
    };
    
    let on_zoom_reset = {
        let zoom_level = zoom_level.clone();
        Callback::from(move |_| {
            zoom_level.set(1.0);
        })
    };
    
    html! {
        <div class="canvas-container">
            <div class="canvas-toolbar">
                <button onclick={on_zoom_in} class="zoom-btn">{"+"}</button>
                <button onclick={on_zoom_reset} class="zoom-btn">{"1:1"}</button>
                <button onclick={on_zoom_out} class="zoom-btn">{"-"}</button>
                <span class="zoom-level">{ format!("{:.1}x", *zoom_level) }</span>
            </div>
            
            <div 
                class="canvas" 
                onmousemove={on_mouse_move}
                onmouseup={on_mouse_up}
                style={format!("transform: scale({}); transform-origin: 0 0;", *zoom_level)}
            >
                <svg class="connections-layer" width="2000" height="2000">
                    { for props.graph.connections.iter().map(|conn| {
                        html! {
                            <ConnectionView 
                                connection={conn.clone()}
                                nodes={props.graph.nodes.clone()}
                            />
                        }
                    })}
                </svg>
                
                { for props.graph.nodes.values().map(|node| {
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
                                <div class="node-title-row">
                                    <span class="node-title">{&node.block_spec.title}</span>
                                    <span class="node-id">{&node.id}</span>
                                </div>
                                {
                                    if let Some(status) = &node.status {
                                        let status_indicator = match status {
                                            crate::models::NodeStatus::Pending => html! { <span class="status-indicator pending" title="Pending">{"⏳"}</span> },
                                            crate::models::NodeStatus::Running => html! { <span class="status-indicator running" title="Running">{"▶"}</span> },
                                            crate::models::NodeStatus::Completed => html! { <span class="status-indicator completed" title="Completed">{"✓"}</span> },
                                            crate::models::NodeStatus::Failed(msg) => html! { <span class="status-indicator failed" title={format!("Failed: {}", msg)}>{"✗"}</span> },
                                        };
                                        html! { <div class="node-status">{status_indicator}</div> }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>
                            <div class="node-body">
                                <div class="input-ports">
                                    { for node.input_ports.iter().map(|port| {
                                        let node_id = node.id.clone();
                                        let port_id = port.id.clone();
                                        let on_port_click = on_port_click.clone();
                                        
                                        html! {
                                            <div
                                                class="port input-port"
                                                data-port-id={port.id.clone()}
                                                onclick={Callback::from(move |e: MouseEvent| {
                                                    e.stop_propagation();
                                                    on_port_click.emit((node_id.clone(), port_id.clone(), true));
                                                })}
                                                title={format!("{}: {}", port.name, port.port_type)}
                                            >
                                                <div class="port-visual input"></div>
                                                <span class="port-name">{&port.name}</span>
                                            </div>
                                        }
                                    }) }
                                </div>
                                <div class="output-ports">
                                    { for node.output_ports.iter().map(|port| {
                                        let node_id = node.id.clone();
                                        let port_id = port.id.clone();
                                        let on_port_click = on_port_click.clone();
                                        
                                        html! {
                                            <div
                                                class="port output-port"
                                                data-port-id={port.id.clone()}
                                                onclick={Callback::from(move |e: MouseEvent| {
                                                    e.stop_propagation();
                                                    on_port_click.emit((node_id.clone(), port_id.clone(), false));
                                                })}
                                                title={format!("{}: {}", port.name, port.port_type)}
                                            >
                                                <span class="port-name">{&port.name}</span>
                                                <div class="port-visual output"></div>
                                            </div>
                                        }
                                    }) }
                                </div>
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}
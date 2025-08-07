use yew::prelude::*;
use crate::models::{Connection, Node};
use std::collections::HashMap;

#[derive(Properties, PartialEq)]
pub struct ConnectionViewProps {
    pub connection: Connection,
    pub nodes: HashMap<String, Node>,
}

#[function_component(ConnectionView)]
pub fn connection_view(props: &ConnectionViewProps) -> Html {
    let from_node = match props.nodes.get(&props.connection.from_node) {
        Some(node) => node,
        None => return html! {},
    };
    
    let to_node = match props.nodes.get(&props.connection.to_node) {
        Some(node) => node,
        None => return html! {},
    };
    
    // Calculate port positions
    let from_port_pos = get_port_position(from_node, &props.connection.from_port, false);
    let to_port_pos = get_port_position(to_node, &props.connection.to_port, true);
    
    let (start, end) = match (from_port_pos, to_port_pos) {
        (Some(start), Some(end)) => (start, end),
        _ => return html! {},
    };
    
    // Create a curved path
    let path_data = format!("M {} {} C {} {}, {} {}, {} {}",
        start.0, start.1,
        start.0 + 100, start.1,
        end.0 - 100, end.1,
        end.0, end.1
    );
    
    // Check if connection is valid (this would need proper type checking)
    let is_valid = true; // TODO: Implement proper validation
    
    html! {
        <g class="connection-group">
            <path
                d={path_data}
                stroke={if is_valid { "var(--connection-valid-color, #2ecc71)" } else { "var(--connection-invalid-color, #e74c3c)" }}
                stroke-width="2"
                fill="none"
                class={if is_valid { "connection valid" } else { "connection invalid" }}
            />
            // Arrowhead
            <polygon
                points={format!("{},{} {},{} {},{}", end.0-10, end.1-5, end.0, end.1, end.0-10, end.1+5)}
                fill={if is_valid { "var(--connection-valid-color, #2ecc71)" } else { "var(--connection-invalid-color, #e74c3c)" }}
            />
        </g>
    }
}

fn get_port_position(node: &Node, port_id: &str, is_input: bool) -> Option<(i32, i32)> {
    let ports = if is_input { &node.input_ports } else { &node.output_ports };
    let port_index = ports.iter().position(|p| p.id == port_id)?;
    
    let port_x = if is_input { node.position.0 } else { node.position.0 + 200 };
    let port_y = node.position.1 + 40 + (port_index as i32 * 25);
    
    Some((port_x, port_y))
}
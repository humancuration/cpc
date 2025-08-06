use shared_packages::shtairir_editor::{Port, PortType};

pub struct NodeTemplate {
    pub id: String,
    pub name: String,
    pub category: String,
    pub inputs: Vec<PortTemplate>,
    pub outputs: Vec<PortTemplate>,
}

pub struct PortTemplate {
    pub id: String,
    pub name: String,
    pub port_type: PortType,
}

pub fn dashboard_nodes() -> Vec<NodeTemplate> {
    vec![
        // Data Source node
        NodeTemplate {
            id: "data_source".to_string(),
            name: "Data Source".to_string(),
            category: "Data".to_string(),
            inputs: vec![],
            outputs: vec![
                PortTemplate {
                    id: "dataset".to_string(),
                    name: "Dataset".to_string(),
                    port_type: PortType::Custom("Dataset".to_string()),
                }
            ],
        },
        
        // Data Filter node
        NodeTemplate {
            id: "data_filter".to_string(),
            name: "Data Filter".to_string(),
            category: "Data".to_string(),
            inputs: vec![
                PortTemplate {
                    id: "dataset".to_string(),
                    name: "Dataset".to_string(),
                    port_type: PortType::Custom("Dataset".to_string()),
                },
                PortTemplate {
                    id: "filter_condition".to_string(),
                    name: "Filter Condition".to_string(),
                    port_type: PortType::String,
                }
            ],
            outputs: vec![
                PortTemplate {
                    id: "filtered_dataset".to_string(),
                    name: "Filtered Dataset".to_string(),
                    port_type: PortType::Custom("Dataset".to_string()),
                }
            ],
        },
        
        // Data Transform node
        NodeTemplate {
            id: "data_transform".to_string(),
            name: "Data Transform".to_string(),
            category: "Data".to_string(),
            inputs: vec![
                PortTemplate {
                    id: "dataset".to_string(),
                    name: "Dataset".to_string(),
                    port_type: PortType::Custom("Dataset".to_string()),
                },
                PortTemplate {
                    id: "transform_function".to_string(),
                    name: "Transform Function".to_string(),
                    port_type: PortType::String,
                }
            ],
            outputs: vec![
                PortTemplate {
                    id: "transformed_dataset".to_string(),
                    name: "Transformed Dataset".to_string(),
                    port_type: PortType::Custom("Dataset".to_string()),
                }
            ],
        },
        
        // Create Visualization node
        NodeTemplate {
            id: "create_visualization".to_string(),
            name: "Create Visualization".to_string(),
            category: "Visualization".to_string(),
            inputs: vec![
                PortTemplate {
                    id: "dataset".to_string(),
                    name: "Dataset".to_string(),
                    port_type: PortType::Custom("Dataset".to_string()),
                },
                PortTemplate {
                    id: "visualization_type".to_string(),
                    name: "Visualization Type".to_string(),
                    port_type: PortType::String,
                },
                PortTemplate {
                    id: "title".to_string(),
                    name: "Title".to_string(),
                    port_type: PortType::String,
                }
            ],
            outputs: vec![
                PortTemplate {
                    id: "visualization".to_string(),
                    name: "Visualization".to_string(),
                    port_type: PortType::Custom("Visualization".to_string()),
                }
            ],
        },
        
        // Display Visualization node
        NodeTemplate {
            id: "display_visualization".to_string(),
            name: "Display Visualization".to_string(),
            category: "Visualization".to_string(),
            inputs: vec![
                PortTemplate {
                    id: "visualization".to_string(),
                    name: "Visualization".to_string(),
                    port_type: PortType::Custom("Visualization".to_string()),
                }
            ],
            outputs: vec![],
        },
        
        // Get Dashboard Data node
        NodeTemplate {
            id: "get_dashboard_data".to_string(),
            name: "Get Dashboard Data".to_string(),
            category: "Dashboard".to_string(),
            inputs: vec![
                PortTemplate {
                    id: "dashboard_id".to_string(),
                    name: "Dashboard ID".to_string(),
                    port_type: PortType::String,
                }
            ],
            outputs: vec![
                PortTemplate {
                    id: "dashboard_data".to_string(),
                    name: "Dashboard Data".to_string(),
                    port_type: PortType::Object,
                }
            ],
        },
        
        // Update Dashboard node
        NodeTemplate {
            id: "update_dashboard".to_string(),
            name: "Update Dashboard".to_string(),
            category: "Dashboard".to_string(),
            inputs: vec![
                PortTemplate {
                    id: "dashboard_id".to_string(),
                    name: "Dashboard ID".to_string(),
                    port_type: PortType::String,
                },
                PortTemplate {
                    id: "dashboard_data".to_string(),
                    name: "Dashboard Data".to_string(),
                    port_type: PortType::Object,
                }
            ],
            outputs: vec![],
        },
        
        // HTTP Request node
        NodeTemplate {
            id: "http_request".to_string(),
            name: "HTTP Request".to_string(),
            category: "Network".to_string(),
            inputs: vec![
                PortTemplate {
                    id: "url".to_string(),
                    name: "URL".to_string(),
                    port_type: PortType::String,
                },
                PortTemplate {
                    id: "method".to_string(),
                    name: "Method".to_string(),
                    port_type: PortType::String,
                },
                PortTemplate {
                    id: "headers".to_string(),
                    name: "Headers".to_string(),
                    port_type: PortType::Object,
                },
                PortTemplate {
                    id: "body".to_string(),
                    name: "Body".to_string(),
                    port_type: PortType::String,
                }
            ],
            outputs: vec![
                PortTemplate {
                    id: "response".to_string(),
                    name: "Response".to_string(),
                    port_type: PortType::Object,
                },
                PortTemplate {
                    id: "status_code".to_string(),
                    name: "Status Code".to_string(),
                    port_type: PortType::Number,
                }
            ],
        },
        
        // Timer node
        NodeTemplate {
            id: "timer".to_string(),
            name: "Timer".to_string(),
            category: "Flow".to_string(),
            inputs: vec![
                PortTemplate {
                    id: "interval_seconds".to_string(),
                    name: "Interval (seconds)".to_string(),
                    port_type: PortType::Number,
                }
            ],
            outputs: vec![
                PortTemplate {
                    id: "trigger".to_string(),
                    name: "Trigger".to_string(),
                    port_type: PortType::Boolean,
                }
            ],
        },
    ]
}

impl NodeTemplate {
    pub fn to_node(&self, node_id: &str) -> shared_packages::shtairir_editor::Node {
        let inputs = self.inputs.iter().map(|port| Port {
            id: port.id.clone(),
            name: port.name.clone(),
            port_type: port.port_type.clone(),
            direction: shared_packages::shtairir_editor::PortDirection::Input,
        }).collect();
        
        let outputs = self.outputs.iter().map(|port| Port {
            id: port.id.clone(),
            name: port.name.clone(),
            port_type: port.port_type.clone(),
            direction: shared_packages::shtairir_editor::PortDirection::Output,
        }).collect();
        
        shared_packages::shtairir_editor::Node {
            id: node_id.to_string(),
            app: "dashboard".to_string(),
            function: self.id.clone(),
            args: vec![],
            position: (100, 100),
            input_ports: inputs,
            output_ports: outputs,
            status: None,
        }
    }
}
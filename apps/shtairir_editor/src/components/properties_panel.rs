use yew::prelude::*;
use crate::models::Node;
use shtairir::ast::Value;

#[derive(Properties, PartialEq)]
pub struct PropertiesPanelProps {
    pub node: Option<Node>,
    pub on_node_update: Callback<Node>,
    pub on_close: Callback<()>,
}

#[function_component(PropertiesPanel)]
pub fn properties_panel(props: &PropertiesPanelProps) -> Html {
    let node = match &props.node {
        Some(n) => n,
        None => return html! {},
    };
    
    let on_param_change = {
        let node = node.clone();
        let on_node_update = props.on_node_update.clone();
        Callback::from(move |(param_name, new_value): (String, Value)| {
            let mut updated_node = node.clone();
            updated_node.params.insert(param_name, new_value);
            on_node_update.emit(updated_node);
        })
    };
    
    html! {
        <div class="properties-panel">
            <div class="panel-header">
                <h2>{"Properties"}</h2>
                <button class="close-btn" onclick={props.on_close.clone()}>{"×"}</button>
            </div>
            
            <div class="panel-content">
                <div class="node-info">
                    <h3>{&node.block_spec.title}</h3>
                    <p class="node-description">{&node.block_spec.description}</p>
                    <div class="node-meta">
                        <span class="node-id">{"ID: "}{&node.id}</span>
                        <span class="node-module">{"Module: "}{&node.block_spec.namespace}</span>
                    </div>
                </div>
                
                if !node.block_spec.params.is_empty() {
                    <div class="params-section">
                        <h4>{"Parameters"}</h4>
                        { for node.block_spec.params.iter().map(|param_spec| {
                            let param_value = node.params.get(&param_spec.name).cloned().unwrap_or_else(|| {
                                // Use default value or create a default based on type
                                Value::String("".to_string())
                            });
                            
                            let on_change = {
                                let param_name = param_spec.name.clone();
                                let on_param_change = on_param_change.clone();
                                Callback::from(move |new_value: Value| {
                                    on_param_change.emit((param_name.clone(), new_value));
                                })
                            };
                            
                            html! {
                                <div class="param-item">
                                    <label>{&param_spec.name}</label>
                                    <ParamEditor 
                                        param_type={param_spec.ty.clone()}
                                        value={param_value}
                                        on_change={on_change}
                                    />
                                    if let Some(allowed) = &param_spec.allowed {
                                        if let Some(enum_values) = &allowed.enum_values {
                                            <div class="allowed-values">
                                                <span>{"Allowed: "}</span>
                                                { for enum_values.iter().map(|val| {
                                                    html! {
                                                        <span class="allowed-value">{format!("{:?}", val)}</span>
                                                    }
                                                })}
                                            </div>
                                        }
                                    }
                                </div>
                            }
                        })}
                    </div>
                }
                
                <div class="ports-section">
                    <h4>{"Input Ports"}</h4>
                    { for node.input_ports.iter().map(|port| {
                        html! {
                            <div class="port-info">
                                <span class="port-name">{&port.name}</span>
                                <span class="port-type">{&port.port_type}</span>
                            </div>
                        }
                    })}
                    
                    <h4>{"Output Ports"}</h4>
                    { for node.output_ports.iter().map(|port| {
                        html! {
                            <div class="port-info">
                                <span class="port-name">{&port.name}</span>
                                <span class="port-type">{&port.port_type}</span>
                            </div>
                        }
                    })}
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ParamEditorProps {
    param_type: String,
    value: Value,
    on_change: Callback<Value>,
}

#[function_component(ParamEditor)]
fn param_editor(props: &ParamEditorProps) -> Html {
    match &props.value {
        Value::String(s) => {
            let on_input = {
                let on_change = props.on_change.clone();
                Callback::from(move |e: InputEvent| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    on_change.emit(Value::String(input.value()));
                })
            };
            
            html! {
                <input
                    type="text"
                    value={s.clone()}
                    oninput={on_input}
                    class="param-input string-input"
                />
            }
        },
        Value::Number(n) => {
            let on_input = {
                let on_change = props.on_change.clone();
                Callback::from(move |e: InputEvent| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    if let Ok(num) = input.value().parse::<f64>() {
                        on_change.emit(Value::Number(num));
                    }
                })
            };
            
            html! {
                <input
                    type="number"
                    step="any"
                    value={n.to_string()}
                    oninput={on_input}
                    class="param-input number-input"
                />
            }
        },
        Value::Boolean(b) => {
            let on_change = {
                let on_change = props.on_change.clone();
                Callback::from(move |e: Event| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    on_change.emit(Value::Boolean(input.checked()));
                })
            };
            
            html! {
                <label class="boolean-toggle">
                    <input
                        type="checkbox"
                        checked={*b}
                        onchange={on_change}
                    />
                    <span class="toggle-slider"></span>
                </label>
            }
        },
        Value::Object(_) => {
            html! {
                <div class="object-editor">
                    <span>{"Object (complex type)"}</span>
                </div>
            }
        },
        Value::Array(_) => {
            html! {
                <div class="array-editor">
                    <span>{"Array (complex type)"}</span>
                </div>
            }
        },
        Value::Identifier(id) => {
            html! {
                <div class="identifier-display">
                    <span>{"Connected to: "}{id}</span>
                </div>
            }
        },
    }
}
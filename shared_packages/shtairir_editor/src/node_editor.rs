use yew::prelude::*;
use crate::{Node, Value};
use std::collections::HashMap;

#[derive(Properties, PartialEq)]
pub struct NodeEditorProps {
    pub node: Node,
    pub on_node_change: Callback<Node>,
}

#[function_component(NodeEditor)]
pub fn node_editor(props: &NodeEditorProps) -> Html {
    let node = use_state(|| props.node.clone());
    
    let on_arg_change = {
        let node = node.clone();
        let on_node_change = props.on_node_change.clone();
        Callback::from(move |(index, new_value): (usize, Value)| {
            let mut new_node = (*node).clone();
            new_node.args[index] = new_value;
            node.set(new_node.clone());
            on_node_change.emit(new_node);
        })
    };
    
    let on_string_change = {
        let on_arg_change = on_arg_change.clone();
        Callback::from(move |(index, value): (usize, String)| {
            on_arg_change.emit((index, Value::String(value)));
        })
    };
    
    let on_number_change = {
        let on_arg_change = on_arg_change.clone();
        Callback::from(move |(index, value): (usize, String)| {
            if let Ok(num) = value.parse::<f64>() {
                on_arg_change.emit((index, Value::Number(num)));
            }
        })
    };
    
    let on_boolean_change = {
        let on_arg_change = on_arg_change.clone();
        Callback::from(move |(index, checked): (usize, bool)| {
            on_arg_change.emit((index, Value::Boolean(checked)));
        })
    };
    
    let on_array_change = {
        let on_arg_change = on_arg_change.clone();
        Callback::from(move |(index, values): (usize, Vec<String>)| {
            let array_values = values.iter()
                .filter_map(|s| s.parse::<f64>().ok().map(Value::Number))
                .collect();
            on_arg_change.emit((index, Value::Array(array_values)));
        })
    };
    
    let on_object_change = {
        let on_arg_change = on_arg_change.clone();
        Callback::from(move |(index, key_values): (usize, Vec<(String, String)>)| {
            let obj_values: HashMap<String, Value> = key_values.iter()
                .filter_map(|(k, v)| {
                    if v.parse::<f64>().is_ok() {
                        Some((k.clone(), Value::Number(v.parse().unwrap())))
                    } else {
                        Some((k.clone(), Value::String(v.clone())))
                    }
                })
                .collect();
            on_arg_change.emit((index, Value::Object(obj_values)));
        })
    };
    
    html! {
        <div class="node-editor">
            <h2>{ format!("{}:{}", node.app, node.function) }</h2>
            <div class="params">
                {for node.args.iter().enumerate().map(|(i, arg)| {
                    let on_string_change = on_string_change.clone();
                    let on_number_change = on_number_change.clone();
                    let on_boolean_change = on_boolean_change.clone();
                    let on_array_change = on_array_change.clone();
                    let on_object_change = on_object_change.clone();
                    
                    html! {
                        <div class="param" key={i}>
                            <label>{ format!("Argument {}", i+1) }</label>
                            { match arg {
                                Value::String(s) => html! {
                                    <div class="string-editor">
                                        <input
                                            type="text"
                                            value={s.clone()}
                                            onchange={move |e: Event| {
                                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                on_string_change.emit((i, input.value()));
                                            }}
                                            placeholder="Enter text..."
                                        />
                                    </div>
                                },
                                Value::Number(n) => html! {
                                    <div class="number-editor">
                                        <input
                                            type="number"
                                            step="0.1"
                                            value={n.to_string()}
                                            onchange={move |e: Event| {
                                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                on_number_change.emit((i, input.value()));
                                            }}
                                            placeholder="Enter number..."
                                        />
                                    </div>
                                },
                                Value::Boolean(b) => html! {
                                    <div class="boolean-editor">
                                        <label class="toggle">
                                            <input
                                                type="checkbox"
                                                checked={*b}
                                                onchange={move |e: Event| {
                                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                    on_boolean_change.emit((i, input.checked()));
                                                }}
                                            />
                                            <span class="toggle-slider"></span>
                                            <span class="toggle-label">{if *b { "True" } else { "False" }}</span>
                                        </label>
                                    </div>
                                },
                                Value::Identifier(id) => html! {
                                    <div class="identifier-editor">
                                        <input
                                            type="text"
                                            value={id.clone()}
                                            readonly=true
                                            class="readonly"
                                        />
                                        <span class="info">{"(Identifier - derived from connections)"}</span>
                                    </div>
                                },
                                Value::Array(arr) => html! {
                                    <ArrayEditor
                                        values={arr.iter().map(|v| match v {
                                            Value::Number(n) => n.to_string(),
                                            _ => "0".to_string(),
                                        }).collect()}
                                        on_change={on_array_change.reform(move |vals| (i, vals))}
                                    />
                                },
                                Value::Object(obj) => html! {
                                    <ObjectEditor
                                        key_values={obj.iter().map(|(k, v)| {
                                            match v {
                                                Value::Number(n) => (k.clone(), n.to_string()),
                                                Value::String(s) => (k.clone(), s.clone()),
                                                _ => (k.clone(), "".to_string()),
                                            }
                                        }).collect()}
                                        on_change={on_object_change.reform(move |kvs| (i, kvs))}
                                    />
                                },
                            }}
                        </div>
                    }
                })}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ArrayEditorProps {
    values: Vec<String>,
    on_change: Callback<Vec<String>>,
}

#[function_component(ArrayEditor)]
fn array_editor(props: &ArrayEditorProps) -> Html {
    let values = use_state(|| props.values.clone());
    let new_value = use_state(|| String::new());
    
    let on_add = {
        let values = values.clone();
        let new_value = new_value.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |_| {
            if !new_value.is_empty() {
                let mut new_values = (*values).clone();
                new_values.push((*new_value).clone());
                values.set(new_values.clone());
                on_change.emit(new_values);
                new_value.set(String::new());
            }
        })
    };
    
    let on_remove = {
        let values = values.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |index: usize| {
            let mut new_values = (*values).clone();
            new_values.remove(index);
            values.set(new_values.clone());
            on_change.emit(new_values);
        })
    };
    
    let on_value_change = {
        let new_value = new_value.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            new_value.set(input.value());
        })
    };
    
    html! {
        <div class="array-editor">
            <div class="array-items">
                { for values.iter().enumerate().map(|(i, value)| {
                    let on_remove = on_remove.clone();
                    html! {
                        <div class="array-item">
                            <span>{value}</span>
                            <button class="remove-btn" onclick={move |_| on_remove.emit(i)}>{"×"}</button>
                        </div>
                    }
                })}
            </div>
            <div class="array-add">
                <input
                    type="number"
                    step="0.1"
                    value={(*new_value).clone()}
                    onchange={on_value_change}
                    placeholder="Add number..."
                />
                <button onclick={on_add}>{"+"}</button>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ObjectEditorProps {
    key_values: Vec<(String, String)>,
    on_change: Callback<Vec<(String, String)>>,
}

#[function_component(ObjectEditor)]
fn object_editor(props: &ObjectEditorProps) -> Html {
    let key_values = use_state(|| props.key_values.clone());
    let new_key = use_state(|| String::new());
    let new_value = use_state(|| String::new());
    
    let on_add = {
        let key_values = key_values.clone();
        let new_key = new_key.clone();
        let new_value = new_value.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |_| {
            if !new_key.is_empty() && !new_value.is_empty() {
                let mut new_kvs = (*key_values).clone();
                new_kvs.push(((*new_key).clone(), (*new_value).clone()));
                key_values.set(new_kvs.clone());
                on_change.emit(new_kvs);
                new_key.set(String::new());
                new_value.set(String::new());
            }
        })
    };
    
    let on_remove = {
        let key_values = key_values.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |index: usize| {
            let mut new_kvs = (*key_values).clone();
            new_kvs.remove(index);
            key_values.set(new_kvs.clone());
            on_change.emit(new_kvs);
        })
    };
    
    let on_key_change = {
        let new_key = new_key.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            new_key.set(input.value());
        })
    };
    
    let on_value_change = {
        let new_value = new_value.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            new_value.set(input.value());
        })
    };
    
    html! {
        <div class="object-editor">
            <div class="object-items">
                { for key_values.iter().enumerate().map(|(i, (key, value))| {
                    let on_remove = on_remove.clone();
                    html! {
                        <div class="object-item">
                            <span class="key">{key}: </span>
                            <span class="value">{value}</span>
                            <button class="remove-btn" onclick={move |_| on_remove.emit(i)}>{"×"}</button>
                        </div>
                    }
                })}
            </div>
            <div class="object-add">
                <input
                    type="text"
                    value={(*new_key).clone()}
                    onchange={on_key_change}
                    placeholder="Key..."
                />
                <input
                    type="text"
                    value={(*new_value).clone()}
                    onchange={on_value_change}
                    placeholder="Value..."
                />
                <button onclick={on_add}>{"+"}</button>
            </div>
        </div>
    }
}
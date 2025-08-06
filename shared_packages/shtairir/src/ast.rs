use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Identifier(String),
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub app: String,
    pub function: String,
    pub args: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub commands: Vec<Command>,
}
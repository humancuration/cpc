use std::collections::HashMap;
use crate::ast::{Command, Script, Value};

pub trait AppAdapter {
    fn execute(&self, command: &Command) -> Result<Value, String>;
}

pub struct ExecutionContext {
    adapters: HashMap<String, Box<dyn AppAdapter>>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
        }
    }
    
    pub fn register_adapter(&mut self, app_name: String, adapter: Box<dyn AppAdapter>) {
        self.adapters.insert(app_name, adapter);
    }
    
    pub fn execute_script(&self, script: &Script) -> Result<Vec<Value>, String> {
        let mut results = Vec::new();
        
        for command in &script.commands {
            let app_name = &command.app;
            
            if let Some(adapter) = self.adapters.get(app_name) {
                let result = adapter.execute(command)?;
                results.push(result);
            } else {
                return Err(format!("No adapter registered for app: {}", app_name));
            }
        }
        
        Ok(results)
    }
}

pub fn execute_script(script: &Script, context: &ExecutionContext) -> Result<Vec<Value>, String> {
    context.execute_script(script)
}
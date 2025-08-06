use std::collections::HashMap;
use shtairir::{ast::{Command, Script, Value}, engine::{ExecutionContext, AppAdapter}, dashboard_adapter::DashboardAdapter};
use dashmap::DashMap;
use futures::future::FutureExt;

/// Represents the status of a node during execution
#[derive(Debug, Clone, PartialEq)]
pub enum NodeStatus {
    Pending,
    Running,
    Completed,
    Failed(String), // Error message
}

/// Represents errors that can occur during script execution
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionError {
    HandlerNotFound,
    ExecutionFailed(String),
    TimeoutError,
    ValidationError(String),
    VisualizationError(String),
    NetworkError(String),
    DataError(String),
}

impl std::fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionError::HandlerNotFound => write!(f, "Handler not found"),
            ExecutionError::ExecutionFailed(msg) => write!(f, "Execution failed: {}", msg),
            ExecutionError::TimeoutError => write!(f, "Execution timed out"),
            ExecutionError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ExecutionError::VisualizationError(msg) => write!(f, "Visualization error: {}", msg),
            ExecutionError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ExecutionError::DataError(msg) => write!(f, "Data error: {}", msg),
        }
    }
}

impl std::error::Error for ExecutionError {}

/// Result of script execution
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionResult {
    pub success: bool,
    pub results: Vec<Value>,
    pub node_status: HashMap<String, NodeStatus>,
    pub errors: Vec<ExecutionError>,
    pub execution_time_ms: u64,
}

/// Context for script execution with variables and node status tracking
pub struct ExecutionContextExtended {
    variables: DashMap<String, Value>,
    node_status: DashMap<String, NodeStatus>,
    errors: Vec<ExecutionError>,
    start_time: std::time::Instant,
}

impl ExecutionContextExtended {
    pub fn new() -> Self {
        Self {
            variables: DashMap::new(),
            node_status: DashMap::new(),
            errors: Vec::new(),
            start_time: std::time::Instant::now(),
        }
    }

    pub fn set_variable(&self, key: String, value: Value) {
        self.variables.insert(key, value);
    }

    pub fn get_variable(&self, key: &str) -> Option<Value> {
        self.variables.get(key).map(|v| v.clone())
    }

    pub fn set_node_status(&self, node_id: String, status: NodeStatus) {
        self.node_status.insert(node_id, status);
    }

    pub fn get_node_status(&self, node_id: &str) -> Option<NodeStatus> {
        self.node_status.get(node_id).map(|s| s.clone())
    }

    pub fn add_error(&mut self, error: ExecutionError) {
        self.errors.push(error);
    }

    pub fn into_result(self) -> ExecutionResult {
        let execution_time_ms = self.start_time.elapsed().as_millis() as u64;
        let node_status: HashMap<String, NodeStatus> = self.node_status.into_iter().collect();
        
        ExecutionResult {
            success: self.errors.is_empty(),
            results: Vec::new(), // Will be populated during execution
            node_status,
            errors: self.errors,
            execution_time_ms,
        }
    }
}

/// Executes a graph script with proper error handling and status tracking
pub async fn execute_graph(graph: &mut shtairir_editor::Graph) -> ExecutionResult {
    // Compile graph to script
    let script = graph.to_script();
    
    // Initialize execution context
    let mut context = ExecutionContextExtended::new();
    
    // Set up execution engine with dashboard adapter
    let mut engine = ExecutionContext::new();
    engine.register_adapter("dashboard".to_string(), Box::new(DashboardAdapter));
    
    // Reset all node statuses to pending at the start
    for (node_id, node) in graph.nodes.iter_mut() {
        node.status = Some(shtairir_editor::NodeStatus::Pending);
    }
    
    // Process commands
    for (index, command) in script.commands.iter().enumerate() {
        let node_id = format!("node_{}", index);
        
        // Update node status in graph to running
        if let Some(node) = graph.nodes.get_mut(&node_id) {
            node.status = Some(shtairir_editor::NodeStatus::Running);
        }
        
        // Update UI: Set node status to running
        context.set_node_status(node_id.clone(), NodeStatus::Running);
        
        // Execute command with timeout
        let result = execute_command_with_timeout(&command, &engine, 30000) // 30 second timeout
            .await;
        
        // Update context with results
        match result {
            Ok(value) => {
                // Store result in variables
                context.set_variable(format!("{}.result", node_id), value.clone());
                
                // Update node status to completed in graph
                if let Some(node) = graph.nodes.get_mut(&node_id) {
                    node.status = Some(shtairir_editor::NodeStatus::Completed);
                }
                
                // Update node status to completed in context
                context.set_node_status(node_id, NodeStatus::Completed);
            }
            Err(error) => {
                // Add error to context
                let error_msg = error.to_string();
                context.add_error(error.clone());
                
                // Update node status to failed in graph
                if let Some(node) = graph.nodes.get_mut(&node_id) {
                    node.status = Some(shtairir_editor::NodeStatus::Failed(error_msg.clone()));
                }
                
                // Update node status to failed in context
                context.set_node_status(node_id, NodeStatus::Failed(error_msg));
            }
        }
    }
    
    context.into_result()
}

/// Executes a single command with a timeout
async fn execute_command_with_timeout(
    command: &Command,
    engine: &ExecutionContext,
    timeout_ms: u64
) -> Result<Value, ExecutionError> {
    let future = async {
        // Execute the command using the engine
        match engine.execute_command(command) {
            Ok(value) => Ok(value),
            Err(error) => Err(ExecutionError::ExecutionFailed(error)),
        }
    };
    
    // Apply timeout
    match wasm_timer::Delay::new(std::time::Duration::from_millis(timeout_ms)).race(future).await {
        Ok(result) => result,
        Err(_) => Err(ExecutionError::TimeoutError),
    }
}

/// Extension trait for ExecutionContext to support single command execution
trait ExecutionContextExt {
    fn execute_command(&self, command: &Command) -> Result<Value, String>;
}

impl ExecutionContextExt for ExecutionContext {
    fn execute_command(&self, command: &Command) -> Result<Value, String> {
        let app_name = &command.app;
        
        if let Some(adapter) = self.adapters.get(app_name) {
            adapter.execute(command)
        } else {
            Err(format!("No adapter registered for app: {}", app_name))
        }
    }
}

/// Executes a script synchronously for non-async contexts
pub fn execute_script_sync(script: &Script) -> ExecutionResult {
    // Initialize execution context
    let mut context = ExecutionContextExtended::new();
    
    // Set up execution engine with dashboard adapter
    let mut engine = ExecutionContext::new();
    engine.register_adapter("dashboard".to_string(), Box::new(DashboardAdapter));
    
    // Process commands
    for (index, command) in script.commands.iter().enumerate() {
        let node_id = format!("node_{}", index);
        
        // Update UI: Set node status to running
        context.set_node_status(node_id.clone(), NodeStatus::Running);
        
        // Execute command
        let result = engine.execute_command(command);
        
        // Update context with results
        match result {
            Ok(value) => {
                // Store result in variables
                context.set_variable(format!("{}.result", node_id), value);
                
                // Update node status to completed
                context.set_node_status(node_id, NodeStatus::Completed);
            }
            Err(error) => {
                // Add error to context
                context.add_error(ExecutionError::ExecutionFailed(error));
                
                // Update node status to failed
                context.set_node_status(node_id, NodeStatus::Failed("Execution failed".to_string()));
            }
        }
    }
    
    context.into_result()
}
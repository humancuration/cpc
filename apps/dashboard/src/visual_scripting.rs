use shared_packages::shtairir_editor::{VisualEditor, Graph, Node};
use yew::prelude::*;
use crate::dashboard_nodes::dashboard_nodes;
use crate::execution::{execute_graph, ExecutionResult, NodeStatus};
use std::collections::HashMap;

pub struct VisualScriptingView {
    graph: Graph,
    node_templates: Vec<crate::dashboard_nodes::NodeTemplate>,
    node_status: HashMap<String, NodeStatus>,
    execution_in_progress: bool,
    execution_result: Option<ExecutionResult>,
}

impl VisualScriptingView {
    pub fn new() -> Self {
        let mut graph = Graph::new();
        let node_templates = dashboard_nodes();
        
        // Add some initial nodes for demonstration
        Self {
            graph,
            node_templates,
            node_status: HashMap::new(),
            execution_in_progress: false,
            execution_result: None,
        }
    }
    
    fn on_graph_change(&mut self, new_graph: Graph) {
        self.graph = new_graph;
    }
    fn execute_script(&mut self, ctx: &Context<Self>) {
        if self.execution_in_progress {
            return; // Prevent concurrent executions
        }
        
        self.execution_in_progress = true;
        self.node_status.clear();
        self.execution_result = None;
        
        let mut graph = self.graph.clone();
        let callback = ctx.link().callback(|result| Msg::ExecutionComplete(result));
        
        // Spawn async task
        wasm_bindgen_futures::spawn_local(async move {
            let result = execute_graph(&mut graph).await;
            callback.emit(result);
        });
    }
    }
    fn on_execution_complete(&mut self, result: ExecutionResult) -> bool {
        self.execution_in_progress = false;
        self.execution_result = Some(result.clone());
        self.node_status = result.node_status;
        
        // Update the graph with the node statuses from the execution result
        // In a real implementation, we would need to update the graph with the actual node statuses
        // For now, we'll just mark the execution as complete
        
        true
    }
    }
}

impl Component for VisualScriptingView {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::new()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GraphChanged(new_graph) => {
                self.on_graph_change(new_graph);
                true
            }
            Msg::NodeSelected(_node) => {
                // Handle node selection if needed
                false
            }
            Msg::ExecuteScript => {
                self.execute_script(ctx);
                true
            }
            Msg::ExecutionComplete(result) => {
                self.on_execution_complete(result)
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_graph_change = ctx.link().callback(Msg::GraphChanged);
        let on_node_select = ctx.link().callback(Msg::NodeSelected);
        let execute_callback = ctx.link().callback(|_| Msg::ExecuteScript);

        html! {
            <div class="visual-scripting-container">
                <div class="node-palette">
                    <h3>{"Dashboard Nodes"}</h3>
                    { for self.node_templates.iter().map(|template| {
                        let template = template.clone();
                        html! {
                            <div class="node-template-item">
                                <span class="node-name">{&template.name}</span>
                                <span class="node-category">{&template.category}</span>
                            </div>
                        }
                    }) }
                </div>
                <div class="editor-area">
                    <VisualEditor
                        graph={self.graph.clone()}
                        on_graph_change={on_graph_change}
                        on_node_select={on_node_select}
                    />
                </div>
                <div class="script-controls">
                    <button
                        class={classes!("execute-button", self.execution_in_progress.then(|| "executing"))}
                        onclick={execute_callback}
                        disabled={self.execution_in_progress}
                    >
                        { if self.execution_in_progress { "Executing..." } else { "Execute Script" } }
                    </button>
                    <button class="save-button">{"Save Script"}</button>
                    <button class="load-button">{"Load Script"}</button>
                </div>
                
                // Display execution results if available
                if let Some(result) = &self.execution_result {
                    <div class="execution-results">
                        <h3>{"Execution Results"}</h3>
                        <div class="result-summary">
                            <span class={classes!("status", if result.success { "success" } else { "failed" })}>
                                { if result.success { "Success" } else { "Failed" } }
                            </span>
                            <span class="execution-time">
                                { format!("Execution time: {}ms", result.execution_time_ms) }
                            </span>
                        </div>
                        
                        if !result.errors.is_empty() {
                            <div class="error-list">
                                <h4>{"Errors"}</h4>
                                { for result.errors.iter().map(|error| {
                                    html! {
                                        <div class="error-item">
                                            <span class="error-message">{ error.to_string() }</span>
                                        </div>
                                    }
                                }) }
                            </div>
                        }
                        
                        <div class="node-status-list">
                            <h4>{"Node Status"}</h4>
                            { for result.node_status.iter().map(|(node_id, status)| {
                                let status_class = match status {
                                    NodeStatus::Pending => "pending",
                                    NodeStatus::Running => "running",
                                    NodeStatus::Completed => "completed",
                                    NodeStatus::Failed(_) => "failed",
                                };
                                
                                html! {
                                    <div class={classes!("node-status-item", status_class)}>
                                        <span class="node-id">{ node_id }</span>
                                        <span class="status-indicator"></span>
                                        <span class="status-text">
                                            { match status {
                                                NodeStatus::Pending => "Pending",
                                                NodeStatus::Running => "Running",
                                                NodeStatus::Completed => "Completed",
                                                NodeStatus::Failed(msg) => &format!("Failed: {}", msg),
                                            } }
                                        </span>
                                    </div>
                                }
                            }) }
                        </div>
                    </div>
                }
            </div>
        }
    }
}

pub enum Msg {
    GraphChanged(Graph),
    NodeSelected(Node),
    ExecuteScript,
    ExecutionComplete(ExecutionResult),
}
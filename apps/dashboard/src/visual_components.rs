use yew::prelude::*;
use crate::execution::NodeStatus;

/// Component that displays the status of a node with visual indicators
#[derive(Properties, PartialEq)]
pub struct NodeStatusIndicatorProps {
    pub status: NodeStatus,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(NodeStatusIndicator)]
pub fn node_status_indicator(props: &NodeStatusIndicatorProps) -> Html {
    let status_class = match props.status {
        NodeStatus::Pending => "status-pending",
        NodeStatus::Running => "status-running",
        NodeStatus::Completed => "status-completed",
        NodeStatus::Failed(_) => "status-failed",
    };
    
    let title = match &props.status {
        NodeStatus::Pending => "Pending",
        NodeStatus::Running => "Running",
        NodeStatus::Completed => "Completed",
        NodeStatus::Failed(msg) => &format!("Failed: {}", msg),
    };
    
    html! {
        <div 
            class={classes!("node-status-indicator", status_class, props.class.clone())}
            title={title.to_string()}
        >
            <div class="status-icon"></div>
            <span class="status-text">{title}</span>
        </div>
    }
}

/// Component that displays the progress of script execution
#[derive(Properties, PartialEq)]
pub struct ExecutionProgressProps {
    pub in_progress: bool,
    pub progress: f64, // 0.0 to 1.0
    #[prop_or_default]
    pub message: String,
}

#[function_component(ExecutionProgress)]
pub fn execution_progress(props: &ExecutionProgressProps) -> Html {
    if !props.in_progress {
        return html! {};
    }
    
    let progress_percent = props.progress * 100.0;
    let message = if props.message.is_empty() {
        format!("Executing... {:.1}%", progress_percent)
    } else {
        props.message.clone()
    };
    
    html! {
        <div class="execution-progress">
            <div class="progress-bar-container">
                <div 
                    class="progress-bar"
                    style={format!("width: {}%", progress_percent)}
                ></div>
            </div>
            <div class="progress-message">{message}</div>
        </div>
    }
}

/// Component that displays execution results with expandable details
#[derive(Properties, PartialEq)]
pub struct ExecutionResultsProps {
    pub success: bool,
    pub results: Vec<shtairir::ast::Value>,
    pub errors: Vec<String>,
    pub execution_time_ms: u64,
    #[prop_or_default]
    pub node_status: std::collections::HashMap<String, NodeStatus>,
}

#[function_component(ExecutionResults)]
pub fn execution_results(props: &ExecutionResultsProps) -> Html {
    let details_expanded = use_state(|| false);
    
    let toggle_details = {
        let details_expanded = details_expanded.clone();
        Callback::from(move |_| {
            details_expanded.set(!*details_expanded);
        })
    };
    
    let result_class = if props.success { "results-success" } else { "results-failed" };
    
    html! {
        <div class={classes!("execution-results", result_class)}>
            <div class="results-summary">
                <div class="result-status">
                    {
                        if props.success {
                            html! { <span class="success-icon">{"✓"}</span> }
                        } else {
                            html! { <span class="failed-icon">{"✗"}</span> }
                        }
                    }
                    <span class="result-text">
                        { if props.success { "Execution Completed" } else { "Execution Failed" } }
                    </span>
                </div>
                <div class="execution-time">
                    { format!("{}ms", props.execution_time_ms) }
                </div>
                <button 
                    class={classes!("details-toggle", *details_expanded.then(|| "expanded"))}
                    onclick={toggle_details}
                >
                    { if *details_expanded { "Hide Details" } else { "Show Details" } }
                </button>
            </div>
            
            if *details_expanded {
                <div class="results-details">
                    if !props.errors.is_empty() {
                        <div class="errors-section">
                            <h4>{"Errors"}</h4>
                            <div class="error-list">
                                { for props.errors.iter().map(|error| {
                                    html! {
                                        <div class="error-item">
                                            <span class="error-message">{error}</span>
                                        </div>
                                    }
                                }) }
                            </div>
                        </div>
                    }
                    
                    if !props.node_status.is_empty() {
                        <div class="node-status-section">
                            <h4>{"Node Status"}</h4>
                            <div class="node-status-list">
                                { for props.node_status.iter().map(|(node_id, status)| {
                                    let status = status.clone();
                                    html! {
                                        <div class="node-status-item">
                                            <span class="node-id">{node_id}</span>
                                            <NodeStatusIndicator status={status} />
                                        </div>
                                    }
                                }) }
                            </div>
                        </div>
                    }
                    
                    if !props.results.is_empty() {
                        <div class="results-section">
                            <h4>{"Results"}</h4>
                            <div class="results-list">
                                { for props.results.iter().enumerate().map(|(i, result)| {
                                    html! {
                                        <div class="result-item">
                                            <span class="result-index">{format!("Result {}:", i+1)}</span>
                                            <span class="result-value">{format_result_value(result)}</span>
                                        </div>
                                    }
                                }) }
                            </div>
                        </div>
                    }
                </div>
            }
        </div>
    }
}

/// Formats a result value for display
fn format_result_value(value: &shtairir::ast::Value) -> String {
    match value {
        shtairir::ast::Value::String(s) => s.clone(),
        shtairir::ast::Value::Number(n) => n.to_string(),
        shtairir::ast::Value::Boolean(b) => b.to_string(),
        shtairir::ast::Value::Identifier(id) => format!("${}", id),
        shtairir::ast::Value::Object(obj) => {
            if obj.len() > 3 {
                format!("Object with {} fields", obj.len())
            } else {
                let pairs: Vec<String> = obj.iter()
                    .map(|(k, v)| format!("{}: {}", k, format_result_value(v)))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
        }
        shtairir::ast::Value::Array(arr) => {
            if arr.len() > 3 {
                format!("Array with {} items", arr.len())
            } else {
                let items: Vec<String> = arr.iter()
                    .map(format_result_value)
                    .collect();
                format!("[{}]", items.join(", "))
            }
        }
    }
}

/// Component that displays a loading spinner
#[derive(Properties, PartialEq)]
pub struct LoadingSpinnerProps {
    #[prop_or_default]
    pub size: String, // "small", "medium", "large"
    #[prop_or_default]
    pub message: String,
}

#[function_component(LoadingSpinner)]
pub fn loading_spinner(props: &LoadingSpinnerProps) -> Html {
    let size_class = match props.size.as_str() {
        "small" => "spinner-small",
        "large" => "spinner-large",
        _ => "spinner-medium", // default
    };
    
    let message = if props.message.is_empty() {
        "Loading..."
    } else {
        &props.message
    };
    
    html! {
        <div class={classes!("loading-spinner", size_class)}>
            <div class="spinner"></div>
            <div class="spinner-message">{message}</div>
        </div>
    }
}

/// Component that displays an error message with retry button
#[derive(Properties, PartialEq)]
pub struct ErrorDisplayProps {
    pub error: String,
    #[prop_or_default]
    pub on_retry: Option<Callback<()>>,
}

#[function_component(ErrorDisplay)]
pub fn error_display(props: &ErrorDisplayProps) -> Html {
    html! {
        <div class="error-display">
            <div class="error-icon">{"⚠️"}</div>
            <div class="error-message">{props.error.clone()}</div>
            {
                if let Some(on_retry) = &props.on_retry {
                    html! {
                        <button 
                            class="retry-button"
                            onclick={on_retry.reform(|_| ())}
                        >
                            {"Retry"}
                        </button>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

/// Component that displays a success message
#[derive(Properties, PartialEq)]
pub struct SuccessDisplayProps {
    pub message: String,
    #[prop_or_default]
    pub action: Option<(String, Callback<()>)>,
}

#[function_component(SuccessDisplay)]
pub fn success_display(props: &SuccessDisplayProps) -> Html {
    html! {
        <div class="success-display">
            <div class="success-icon">{"✓"}</div>
            <div class="success-message">{props.message.clone()}</div>
            {
                if let Some((label, callback)) = &props.action {
                    html! {
                        <button 
                            class="action-button"
                            onclick={callback.reform(|_| ())}
                        >
                            {label}
                        </button>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
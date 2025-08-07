//! User profile workflow implementation
//!
//! This module implements the user profile management workflow using the Shtairir execution engine.

use anyhow::Result;
use shtairir_core::{Graph, Runtime};
use shtairir_registry::Registry;
use tracing::info;

/// Represents a user profile
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub name: String,
    pub email: String,
    pub age: i64,
    pub display_name: Option<String>,
}

/// Represents workflow metrics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkflowMetrics {
    pub total_profiles: usize,
    pub valid_profiles: usize,
    pub processing_time_ms: f64,
}

/// Execute the user profile workflow
pub async fn execute_workflow(registry: &Registry, profile_count: i64) -> Result<(String, Vec<UserProfile>)> {
    info!("Loading user profile workflow graph");
    
    // Load the graph from the registry
    let graph = registry.find_graph("demos.shtairir.user_profiles/user_profile_workflow")?
        .ok_or_else(|| anyhow::anyhow!("Graph not found"))?;
    
    info!("Creating runtime for workflow execution");
    
    // Create a runtime
    let mut runtime = Runtime::new(registry.clone());
    
    // Set input values
    let inputs = serde_json::json!({
        "mock_data.count": profile_count
    });
    
    info!("Executing workflow with {} mock profiles", profile_count);
    
    // Execute the graph
    let outputs = runtime.execute_graph(&graph, inputs).await?;
    
    // Extract the summary report
    let summary = outputs.get("summary")
        .ok_or_else(|| anyhow::anyhow!("Summary not found in outputs"))?
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Summary is not a string"))?
        .to_string();
    
    // Extract the processed profiles
    let profiles: Vec<UserProfile> = if let Some(profiles_value) = outputs.get("processed_profiles") {
        serde_json::from_value(profiles_value.clone())?
    } else {
        Vec::new()
    };
    
    info!("Workflow execution completed successfully");
    
    Ok((summary, profiles))
}

/// Generate mock user profiles
pub fn generate_mock_profiles(count: usize) -> Vec<UserProfile> {
    use rand::Rng;
    
    let mut rng = rand::thread_rng();
    let mut profiles = Vec::with_capacity(count);
    
    let names = vec![
        "Alice Johnson", "Bob Smith", "Charlie Brown", "Diana Prince", 
        "Edward Norton", "Fiona Gallagher", "George Washington", "Helen Keller"
    ];
    
    let domains = vec!["example.com", "test.org", "demo.net", "sample.io"];
    
    for i in 0..count {
        let name = names[rng.gen_range(0..names.len())].to_string();
        let domain = domains[rng.gen_range(0..domains.len())];
        let email = format!("user{}@{}", i, domain);
        
        profiles.push(UserProfile {
            id: format!("user-{}", i),
            name,
            email,
            age: rng.gen_range(18..80),
            display_name: None,
        });
    }
    
    profiles
}
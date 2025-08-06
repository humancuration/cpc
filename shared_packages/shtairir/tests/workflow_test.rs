use shtairir::{parse_script, ExecutionContext, AppAdapter};
use shtairir::ast::{Command, Value};

// Mock adapters for testing
struct MockBevyAdapter;
struct MockRedisAdapter;
struct MockFFmpegAdapter;

impl AppAdapter for MockBevyAdapter {
    fn execute(&self, command: &Command) -> Result<Value, String> {
        match command.function.as_str() {
            "create_entity" => Ok(Value::String("entity_created".to_string())),
            "add_component" => Ok(Value::String("component_added".to_string())),
            _ => Err("Unknown bevy function".to_string())
        }
    }
}

impl AppAdapter for MockRedisAdapter {
    fn execute(&self, command: &Command) -> Result<Value, String> {
        match command.function.as_str() {
            "set" => Ok(Value::String("key_set".to_string())),
            "get" => Ok(Value::String("value_retrieved".to_string())),
            _ => Err("Unknown redis function".to_string())
        }
    }
}

impl AppAdapter for MockFFmpegAdapter {
    fn execute(&self, command: &Command) -> Result<Value, String> {
        match command.function.as_str() {
            "convert" => Ok(Value::String("conversion_complete".to_string())),
            _ => Err("Unknown ffmpeg function".to_string())
        }
    }
}

#[test]
fn test_complete_workflow() {
    let script_content = r#"
bevy:create_entity()
bevy:add_component("entity1", "Position")
redis:set("entity1:pos", {x=100, y=200})
ffmpeg:convert("intro.mp4", "intro.webm")
"#;

    // Parse the script
    let script = parse_script(script_content).expect("Failed to parse script");
    assert_eq!(script.commands.len(), 4);

    // Create execution context and register adapters
    let mut context = ExecutionContext::new();
    context.register_adapter("bevy".to_string(), Box::new(MockBevyAdapter));
    context.register_adapter("redis".to_string(), Box::new(MockRedisAdapter));
    context.register_adapter("ffmpeg".to_string(), Box::new(MockFFmpegAdapter));

    // Execute the script
    let results = shtairir::execute_script(&script, &context).expect("Failed to execute script");
    assert_eq!(results.len(), 4);

    // Verify results
    if let Value::String(s) = &results[0] {
        assert_eq!(s, "entity_created");
    } else {
        panic!("Expected string result");
    }

    if let Value::String(s) = &results[1] {
        assert_eq!(s, "component_added");
    } else {
        panic!("Expected string result");
    }

    if let Value::String(s) = &results[2] {
        assert_eq!(s, "key_set");
    } else {
        panic!("Expected string result");
    }

    if let Value::String(s) = &results[3] {
        assert_eq!(s, "conversion_complete");
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_error_handling() {
    let script_content = r#"
unknown_app:some_function()
"#;

    let script = parse_script(script_content).expect("Failed to parse script");
    
    let context = ExecutionContext::new(); // No adapters registered
    
    let result = shtairir::execute_script(&script, &context);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No adapter registered for app: unknown_app"));
}
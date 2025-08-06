use shtairir::{parse_script, ExecutionContext, AppAdapter};
use shtairir::ast::{Command, Value};
use std::collections::HashMap;

// Example adapter for a "bevy" app
struct BevyAdapter;

impl AppAdapter for BevyAdapter {
    fn execute(&self, command: &Command) -> Result<Value, String> {
        match command.function.as_str() {
            "create_entity" => {
                // In a real implementation, this would create an entity in Bevy
                Ok(Value::String(format!("Created entity with {} components", command.args.len())))
            },
            "add_component" => {
                if command.args.len() >= 2 {
                    if let (Value::String(entity), Value::String(component)) = (&command.args[0], &command.args[1]) {
                        Ok(Value::String(format!("Added component {} to entity {}", component, entity)))
                    } else {
                        Err("Invalid arguments for add_component".to_string())
                    }
                } else {
                    Err("Not enough arguments for add_component".to_string())
                }
            },
            _ => Err(format!("Unknown function: {}", command.function))
        }
    }
}

// Example adapter for a "ffmpeg" app
struct FFmpegAdapter;

impl AppAdapter for FFmpegAdapter {
    fn execute(&self, command: &Command) -> Result<Value, String> {
        match command.function.as_str() {
            "convert" => {
                if command.args.len() >= 2 {
                    if let (Value::String(input), Value::String(output)) = (&command.args[0], &command.args[1]) {
                        // In a real implementation, this would convert a video file
                        Ok(Value::String(format!("Converted {} to {}", input, output)))
                    } else {
                        Err("Invalid arguments for convert".to_string())
                    }
                } else {
                    Err("Not enough arguments for convert".to_string())
                }
            },
            _ => Err(format!("Unknown function: {}", command.function))
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Shtairir Scripting Language Example");
    
    // Example script that uses multiple apps
    let script_text = r#"
bevy:create_entity()
bevy:add_component("entity1", "Position")
ffmpeg:convert("input.mp4", "output.webm")
"#;
    
    println!("\nParsing script:");
    println!("{}", script_text);
    
    // Parse the script
    let script = parse_script(script_text)?;
    
    // Create execution context and register adapters
    let mut context = ExecutionContext::new();
    context.register_adapter("bevy".to_string(), Box::new(BevyAdapter));
    context.register_adapter("ffmpeg".to_string(), Box::new(FFmpegAdapter));
    
    // Execute the script
    println!("\nExecuting script:");
    let results = shtairir::execute_script(&script, &context)?;
    
    // Print results
    for (i, result) in results.iter().enumerate() {
        println!("Result {}: {:?}", i, result);
    }
    
    Ok(())
}
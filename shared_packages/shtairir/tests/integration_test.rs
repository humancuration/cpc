use shtairir::{parse_script, ExecutionContext, AppAdapter};
use shtairir::ast::{Command, Value};

// Test adapter
struct TestAdapter;

impl AppAdapter for TestAdapter {
    fn execute(&self, command: &Command) -> Result<Value, String> {
        match command.function.as_str() {
            "echo" => {
                if let Some(Value::String(s)) = command.args.get(0) {
                    Ok(Value::String(s.clone()))
                } else {
                    Err("Invalid argument".to_string())
                }
            },
            "add" => {
                if command.args.len() >= 2 {
                    if let (Value::Number(a), Value::Number(b)) = (&command.args[0], &command.args[1]) {
                        Ok(Value::Number(a + b))
                    } else {
                        Err("Invalid arguments for add".to_string())
                    }
                } else {
                    Err("Not enough arguments for add".to_string())
                }
            },
            _ => Err(format!("Unknown function: {}", command.function))
        }
    }
}

#[test]
fn test_parsing_and_execution() {
    let script_text = r#"
test:echo("hello world")
test:add(2.5, 3.7)
"#;
    
    let script = parse_script(script_text).expect("Failed to parse script");
    assert_eq!(script.commands.len(), 2);
    
    let mut context = ExecutionContext::new();
    context.register_adapter("test".to_string(), Box::new(TestAdapter));
    
    let results = shtairir::execute_script(&script, &context).expect("Failed to execute script");
    assert_eq!(results.len(), 2);
    
    if let Value::String(s) = &results[0] {
        assert_eq!(s, "hello world");
    } else {
        panic!("Expected string result");
    }
    
    if let Value::Number(n) = &results[1] {
        assert_eq!(*n, 6.2);
    } else {
        panic!("Expected number result");
    }
}
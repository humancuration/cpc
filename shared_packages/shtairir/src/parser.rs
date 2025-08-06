use pest::Parser;
use pest_derive::Parser;
use crate::ast::{Command, Script, Value};
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ShtairirParser;

#[derive(Debug)]
pub enum ParseError {
    PestError(pest::error::Error<Rule>),
    ConversionError(String),
}

impl From<pest::error::Error<Rule>> for ParseError {
    fn from(error: pest::error::Error<Rule>) -> Self {
        ParseError::PestError(error)
    }
}

pub fn parse_script(input: &str) -> Result<Script, ParseError> {
    let pairs = ShtairirParser::parse(Rule::command, input)?;
    
    let mut commands = Vec::new();
    
    for pair in pairs {
        if pair.as_rule() == Rule::command {
            let command = parse_command(pair)?;
            commands.push(command);
        }
    }
    
    Ok(Script { commands })
}

fn parse_command(pair: pest::iterators::Pair<Rule>) -> Result<Command, ParseError> {
    let mut inner = pair.into_inner();
    
    let app_pair = inner.next().ok_or(ParseError::ConversionError("Missing app name".to_string()))?;
    let function_pair = inner.next().ok_or(ParseError::ConversionError("Missing function call".to_string()))?;
    
    let app = app_pair.as_str().to_string();
    
    let mut function_inner = function_pair.into_inner();
    let function_name_pair = function_inner.next().ok_or(ParseError::ConversionError("Missing function name".to_string()))?;
    let function_name = function_name_pair.as_str().to_string();
    
    let args_pair = function_inner.next();
    let args = if let Some(args_pair) = args_pair {
        parse_args(args_pair)?
    } else {
        Vec::new()
    };
    
    Ok(Command {
        app,
        function: function_name,
        args,
    })
}

fn parse_args(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Value>, ParseError> {
    let mut args = Vec::new();
    
    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::value {
            let value = parse_value(inner_pair)?;
            args.push(value);
        }
    }
    
    Ok(args)
}

fn parse_value(pair: pest::iterators::Pair<Rule>) -> Result<Value, ParseError> {
    let rule = pair.as_rule();
    
    match rule {
        Rule::number => {
            let num_str = pair.as_str();
            let num = num_str.parse::<f64>()
                .map_err(|_| ParseError::ConversionError(format!("Invalid number: {}", num_str)))?;
            Ok(Value::Number(num))
        },
        Rule::string => {
            let inner_str = pair.into_inner()
                .map(|p| p.as_str())
                .collect::<String>();
            Ok(Value::String(inner_str))
        },
        Rule::ident => {
            Ok(Value::Identifier(pair.as_str().to_string()))
        },
        Rule::object => {
            let mut map = HashMap::new();
            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() == Rule::key_value {
                    let mut kv_inner = inner_pair.into_inner();
                    let key_pair = kv_inner.next().ok_or(ParseError::ConversionError("Missing key".to_string()))?;
                    let value_pair = kv_inner.next().ok_or(ParseError::ConversionError("Missing value".to_string()))?;
                    
                    let key = key_pair.as_str().to_string();
                    let value = parse_value(value_pair)?;
                    map.insert(key, value);
                }
            }
            Ok(Value::Object(map))
        },
        Rule::array => {
            let mut values = Vec::new();
            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() == Rule::value {
                    let value = parse_value(inner_pair)?;
                    values.push(value);
                }
            }
            Ok(Value::Array(values))
        },
        _ => Err(ParseError::ConversionError(format!("Unexpected rule: {:?}", rule))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_command() {
        let input = r#"bevy:create_entity()"#;
        let script = parse_script(input).expect("Failed to parse");
        
        assert_eq!(script.commands.len(), 1);
        let command = &script.commands[0];
        assert_eq!(command.app, "bevy");
        assert_eq!(command.function, "create_entity");
        assert_eq!(command.args.len(), 0);
    }

    #[test]
    fn test_parse_command_with_args() {
        let input = r#"ffmpeg:convert("input.mp4", "output.webm")"#;
        let script = parse_script(input).expect("Failed to parse");
        
        assert_eq!(script.commands.len(), 1);
        let command = &script.commands[0];
        assert_eq!(command.app, "ffmpeg");
        assert_eq!(command.function, "convert");
        assert_eq!(command.args.len(), 2);
        
        if let Value::String(s) = &command.args[0] {
            assert_eq!(s, "input.mp4");
        } else {
            panic!("Expected string argument");
        }
        
        if let Value::String(s) = &command.args[1] {
            assert_eq!(s, "output.webm");
        } else {
            panic!("Expected string argument");
        }
    }

    #[test]
    fn test_parse_multiple_commands() {
        let input = r#"
bevy:create_entity()
redis:set("key", "value")
"#;
        let script = parse_script(input).expect("Failed to parse");
        
        assert_eq!(script.commands.len(), 2);
        
        let command1 = &script.commands[0];
        assert_eq!(command1.app, "bevy");
        assert_eq!(command1.function, "create_entity");
        
        let command2 = &script.commands[1];
        assert_eq!(command2.app, "redis");
        assert_eq!(command2.function, "set");
    }
}
use clap::Parser;
use shtairir::{parse_script, ExecutionContext};
use std::fs;
use std::process;

/// Shtairir CLI - Unified Scripting Language
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Script file to execute
    #[arg(short, long)]
    script: Option<String>,

    /// Inline script to execute
    #[arg(short, long)]
    execute: Option<String>,

    /// Output format (json, text)
    #[arg(short, long, default_value = "text")]
    format: String,
}

fn main() {
    let cli = Cli::parse();

    let script_content = if let Some(script_file) = cli.script {
        match fs::read_to_string(&script_file) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading script file {}: {}", script_file, e);
                process::exit(1);
            }
        }
    } else if let Some(script_text) = cli.execute {
        script_text
    } else {
        eprintln!("Error: Either --script or --execute must be provided");
        process::exit(1);
    };

    // Parse the script
    let script = match parse_script(&script_content) {
        Ok(script) => script,
        Err(e) => {
            eprintln!("Error parsing script: {:?}", e);
            process::exit(1);
        }
    };

    // Create execution context
    let context = ExecutionContext::new();
    // Note: In a real implementation, we would register app adapters here

    // For this example, we'll just print the parsed script
    println!("Parsed script with {} commands:", script.commands.len());
    for (i, command) in script.commands.iter().enumerate() {
        println!("  {}. {}:{}({})", 
            i + 1, 
            command.app, 
            command.function,
            command.args.iter().map(|arg| format!("{:?}", arg)).collect::<Vec<_>>().join(", ")
        );
    }

    println!("\nNote: This is a demonstration CLI. In a full implementation, app adapters would be registered and scripts would be executed.");
    
    // In a real implementation, we would execute the script:
    // match shtairir::execute_script(&script, &context) {
    //     Ok(results) => {
    //         // Output results based on format
    //         match cli.format.as_str() {
    //             "json" => println!("{}", serde_json::to_string_pretty(&results).unwrap()),
    //             _ => {
    //                 for (i, result) in results.iter().enumerate() {
    //                     println!("Result {}: {:?}", i, result);
    //                 }
    //             }
    //         }
    //     },
    //     Err(e) => {
    //         eprintln!("Error executing script: {}", e);
    //         process::exit(1);
    //     }
    // }
}
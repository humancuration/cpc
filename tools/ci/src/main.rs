use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use std::process;

#[derive(Parser)]
#[command(name = "ci")]
#[command(about = "CI utilities for the CPC project")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check documentation consistency
    CheckDocsConsistency,
    /// Check GraphQL schema consistency
    CheckSchema {
        /// Write the schema snapshot to file
        #[arg(long)]
        write_snapshot: bool,
    },
}

fn check_docs_consistency() -> Result<()> {
    let needles_path = "tools/ci/needles.txt";
    
    // Check if needles.txt exists
    if !Path::new(needles_path).exists() {
        eprintln!("Missing tools/ci/needles.txt. Please create it with lines like: <file>|<required_substring>. See docs/dev/docs-consistency-checks.md.");
        std::process::exit(1);
    }

    // Read needles file
    let content = fs::read_to_string(needles_path)
        .with_context(|| format!("Failed to read {}", needles_path))?;
    
    let mut failed = false;
    
    for line in content.lines() {
        let line = line.trim();
        
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        // Split on first '|' to get file and required substring
        if let Some(pos) = line.find('|') {
            let file_path = line[..pos].trim();
            let required_substring = line[pos + 1..].trim();
            
            // Check if file exists
            if !Path::new(file_path).exists() {
                eprintln!("File not found: {}", file_path);
                failed = true;
                continue;
            }
            
            // Read file content
            let file_content = fs::read_to_string(file_path)
                .with_context(|| format!("Failed to read {}", file_path))?;
            
            // Check if required substring exists
            if !file_content.contains(required_substring) {
                eprintln!("Missing required substring in {}: {}", file_path, required_substring);
                eprintln!("Please either:");
                eprintln!("  a) Add a reference (link or text containing the required substring) to {}, or", file_path);
                eprintln!("  b) If the rule is obsolete, update tools/ci/needles.txt accordingly.");
                failed = true;
            }
        } else {
            eprintln!("Invalid rule format (missing '|'): {}", line);
            failed = true;
        }
    }
    
    if failed {
        std::process::exit(1);
    }
    
    println!("All documentation consistency checks passed!");
    Ok(())
}

fn check_schema(write_snapshot: bool) -> Result<()> {
    // This is a placeholder implementation
    // In a real implementation, this would check the GraphQL schema
    if write_snapshot {
        println!("Writing schema snapshot...");
        // In a real implementation, this would generate and write the schema snapshot
    } else {
        println!("Checking schema consistency...");
        // In a real implementation, this would check the schema against the snapshot
    }
    
    println!("Schema check completed!");
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::CheckDocsConsistency => check_docs_consistency(),
        Commands::CheckSchema { write_snapshot } => check_schema(*write_snapshot),
    }
}
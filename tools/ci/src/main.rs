use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use async_graphql::Schema;
use clap::{Args, Parser, Subcommand};
use similar::{ChangeTag, TextDiff};

/// CLI entry for CPC CI utilities.
/// Local and CI command for schema check:
/// cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
#[derive(Parser, Debug)]
#[command(author, version, about = "CPC CI tools")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate GraphQL SDL and compare to snapshot for drift.
    CheckSchema(CheckSchemaArgs),

    /// Legacy docs consistency checker (kept for now).
    CheckDocsConsistency,
}

#[derive(Args, Debug)]
struct CheckSchemaArgs {
    /// Write the generated SDL to the snapshot path.
    #[arg(long)]
    write_snapshot: bool,
}

fn normalize_lf_and_trim(s: &str) -> String {
    let lf = s.replace("\r\n", "\n");
    let mut out = String::with_capacity(lf.len());
    for line in lf.lines() {
        out.push_str(line.trim_end());
        out.push('\n');
    }
    out
}

fn resolved_snapshot_path() -> PathBuf {
    if let Ok(p) = env::var("SCHEMA_SNAPSHOT_PATH") {
        PathBuf::from(p)
    } else {
        PathBuf::from("docs/api_server/schema.graphql")
    }
}

fn print_mismatch_header(path: &Path) {
    println!("Schema drift detected.");
    println!("Snapshot path: {}", path.to_string_lossy());
    println!("Re-run locally: cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema");
    println!("Windows note: use forward slashes (/); string comparisons are case-sensitive; normalize LF before diff.");
}

/// Produce a truncated diff summary (first 3 hunks).
fn print_truncated_diff(old: &str, new: &str) {
    let diff = TextDiff::from_lines(old, new);

    let mut hunk_count = 0usize;
    let mut additional = 0usize;

    for h in diff.grouped_ops(3) {
        if hunk_count >= 3 {
            // Count remaining changes roughly by added/removed lines in remaining ops
            additional += h.iter().map(|op| {
                diff.iter_inline_changes(op)
                    .filter(|c| matches!(c.tag(), ChangeTag::Delete | ChangeTag::Insert | ChangeTag::Replace))
                    .count()
            }).sum::<usize>();
            continue;
        }

        println!("--- hunk {}", hunk_count + 1);
        for op in h {
            for change in diff.iter_changes(op) {
                let sign = match change.tag() {
                    ChangeTag::Delete => "-",
                    ChangeTag::Insert => "+",
                    ChangeTag::Equal => " ",
                };
                // Only show added/removed/modified lines; keep equals for minimal context
                let text = change.to_string();
                for line in text.lines() {
                    println!("{}{}", sign, line);
                }
            }
        }
        hunk_count += 1;
    }

    if additional > 0 {
        println!("…and {} more.", additional);
    }
}

fn ensure_stub_envs_off_for_schema() {
    // Per ADR 0009, schema-affecting stubs must be OFF for determinism.
    // We explicitly clear known toggles here. If new ones are added, extend this list.
    // See docs/adr/0009-bootstrap-stub-toggles.md
    env::remove_var("VOLUNTEER_REPUTATION");
}

/* intentionally removed: old unused build_schema_for_ci with wrong signature */

fn build_schema_for_ci_concrete() -> api_server::graphql::ci_schema::CiSchema {
    api_server::graphql::ci_schema::build_schema_for_ci()
}

fn run_check_schema(args: &CheckSchemaArgs) -> Result<()> {
    ensure_stub_envs_off_for_schema();

    let schema = build_schema_for_ci_concrete();
    let generated = schema.sdl();

    let normalized_generated = normalize_lf_and_trim(&generated);

    let snapshot_path = resolved_snapshot_path();

    if args.write_snapshot {
        if let Some(parent) = snapshot_path.parent() {
            fs::create_dir_all(parent).ok();
        }
        fs::write(&snapshot_path, &normalized_generated)
            .with_context(|| format!("failed to write snapshot to {}", snapshot_path.to_string_lossy()))?;
        println!("Snapshot updated: {} — please review and commit.", snapshot_path.to_string_lossy());
        return Ok(());
    }

    if !snapshot_path.exists() {
        print_mismatch_header(&snapshot_path);
        println!("Snapshot file does not exist. Create it with:");
        println!("  cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema --write-snapshot");
        return Err(anyhow!("snapshot missing"));
    }

    let snapshot_raw = fs::read_to_string(&snapshot_path)
        .with_context(|| format!("failed to read snapshot {}", snapshot_path.to_string_lossy()))?;
    let normalized_snapshot = normalize_lf_and_trim(&snapshot_raw);

    if normalized_snapshot == normalized_generated {
        // Match
        return Ok(());
    }

    // Mismatch
    print_mismatch_header(&snapshot_path);
    print_truncated_diff(&normalized_snapshot, &normalized_generated);
    Err(anyhow!("schema drift"))
}

fn usage_and_exit() -> ! {
    eprintln!("Usage: cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema");
    std::process::exit(2);
}

fn run_check_docs_consistency() -> Result<()> {
    // Keep legacy behavior to avoid breaking other jobs while we transition.
    let raw = fs::read_to_string("tools/ci/needles.txt")
        .map_err(|_| anyhow!("Missing tools/ci/needles.txt. See docs/dev/docs-consistency-checks.md."))?;
    let mut rules = Vec::new();
    for (idx, line) in raw.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let mut parts = trimmed.splitn(2, '|');
        let left = parts.next().unwrap_or("").trim();
        let right = parts.next().unwrap_or("").trim();
        if left.is_empty() || right.is_empty() {
            return Err(anyhow!("Invalid config at tools/ci/needles.txt line {}: expected '<file>|<required_substring>'", idx + 1));
        }
        rules.push((left.to_string(), right.to_string()));
    }
    if rules.is_empty() {
        return Err(anyhow!("tools/ci/needles.txt contains no rules. Add lines like '<file>|<required_substring>'."));
    }

    let mut errors: Vec<String> = Vec::new();
    for (target_file, required_substring) in rules {
        if !Path::new(&target_file).exists() {
            errors.push(format!("File not found: {target_file}"));
            continue;
        }
        match fs::read_to_string(&target_file) {
            Ok(content) => {
                if !content.contains(&required_substring) {
                    errors.push(format!(
                        "Discoverability check failed:\n  File: {target_file}\n  Missing substring: {required_substring}\n  How to fix: Add a reference containing the substring to {target_file}, or update tools/ci/needles.txt if the path/phrase changed intentionally."
                    ));
                }
            }
            Err(e) => errors.push(format!("Could not read '{target_file}': {e}")),
        }
    }

    if errors.is_empty() {
        println!("[docs-consistency] OK: required references are present.");
        Ok(())
    } else {
        let mut msg = String::from("[docs-consistency] ERROR: One or more checks failed:\n");
        for e in errors {
            msg.push_str("- ");
            msg.push_str(&e);
            msg.push('\n');
        }
        Err(anyhow!(msg))
    }
}

fn main() {
    // Support both clap subcommands and the previous ad-hoc arg style.
    let mut args_iter = env::args().peekable();
    // If only program name provided, show usage
    if args_iter.clone().count() == 1 {
        usage_and_exit();
    }

    // Quick path: if second arg is a known token, bypass clap error noise.
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if let Some(cmd) = args.get(0).map(|s| s.as_str()) {
        match cmd {
            "check-schema" => {
                let write_snapshot = args.iter().any(|a| a == "--write-snapshot");
                let cs_args = CheckSchemaArgs { write_snapshot };
                match run_check_schema(&cs_args) {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("{}", e);
                        std::process::exit(1);
                    }
                }
                return;
            }
            "check-docs-consistency" => {
                match run_check_docs_consistency() {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("{}", e);
                        std::process::exit(1);
                    }
                }
                return;
            }
            _ => { /* fallthrough to clap for nicer errors */ }
        }
    }

    let cli = Cli::parse();
    match cli.command {
        Commands::CheckSchema(a) => {
            if let Err(e) = run_check_schema(&a) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        Commands::CheckDocsConsistency => {
            if let Err(e) = run_check_docs_consistency() {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
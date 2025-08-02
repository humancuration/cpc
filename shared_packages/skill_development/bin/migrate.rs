use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::process::exit;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Apply all pending migrations
    Up,
    /// Roll back the latest migration
    Down,
    /// Show current migration status
    Status,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let cli = Cli::parse();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let migrator = sqlx::migrate!("./migrations");

    match cli.command {
        Commands::Up => {
            migrator.run(&pool).await?;
            println!("Migrations applied successfully");
        }
        Commands::Down => {
            migrator.undo(&pool).await?;
            println!("Latest migration reverted");
        }
        Commands::Status => {
            let applied: Vec<String> = sqlx::query_scalar("SELECT version FROM _sqlx_migrations ORDER BY version")
                .fetch_all(&pool)
                .await?;
            
            println!("Applied migrations:");
            for version in applied {
                println!("- {}", version);
            }
            
            // Show pending migrations
            let mut pending = Vec::new();
            for migration in migrator.iter() {
                let version = migration.version().to_string();
                if !applied.contains(&version) {
                    pending.push(version);
                }
            }
            
            if !pending.is_empty() {
                println!("\nPending migrations:");
                for version in pending {
                    println!("- {}", version);
                }
            } else {
                println!("\nNo pending migrations");
            }
        }
    }

    Ok(())
}
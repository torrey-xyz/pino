//! Pino CLI - Command-line interface for the Pino Solana framework

use clap::{Parser, Subcommand};
use anyhow::Result;
use colored::*;

mod commands;
mod config;
mod utils;

#[derive(Parser)]
#[command(name = "pino")]
#[command(about = "A CLI tool for the Pino Solana framework")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new Pino project
    New {
        /// Project name
        name: String,
        /// Project template
        #[arg(short, long, default_value = "basic")]
        template: String,
    },
    /// Generate code from templates
    Generate {
        /// Type of code to generate (instruction, account, program)
        #[arg(value_name = "TYPE")]
        generate_type: String,
        /// Name of the generated item
        name: String,
    },
    /// Build the current project
    Build {
        /// Build in release mode
        #[arg(short, long)]
        release: bool,
    },
    /// Run tests
    Test {
        /// Run specific test
        #[arg(short, long)]
        test: Option<String>,
    },
    /// Start development server
    Dev {
        /// Port to run on
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    /// Deploy the program
    Deploy {
        /// Network to deploy to
        #[arg(short, long, default_value = "devnet")]
        network: String,
    },
    /// Analyze program performance
    Analyze {
        /// Show detailed analysis
        #[arg(short, long)]
        detailed: bool,
    },
    /// Migrate from other frameworks
    Migrate {
        /// Source framework (anchor, native)
        #[arg(short, long)]
        from: String,
        /// Source directory
        source: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Print banner
    println!("{}", "🌲 Pino Framework CLI".green().bold());
    println!("{}", "High-performance Solana development toolkit".dimmed());
    println!();

    match cli.command {
        Commands::New { name, template } => {
            commands::new::execute(&name, &template).await?;
        }
        Commands::Generate { generate_type, name } => {
            commands::generate::execute(&generate_type, &name).await?;
        }
        Commands::Build { release } => {
            commands::build::execute(release).await?;
        }
        Commands::Test { test } => {
            commands::test::execute(test.as_deref()).await?;
        }
        Commands::Dev { port } => {
            commands::dev::execute(port).await?;
        }
        Commands::Deploy { network } => {
            commands::deploy::execute(&network).await?;
        }
        Commands::Analyze { detailed } => {
            commands::analyze::execute(detailed).await?;
        }
        Commands::Migrate { from, source } => {
            commands::migrate::execute(&from, &source).await?;
        }
    }

    Ok(())
} 
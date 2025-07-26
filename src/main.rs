use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use config::Config;

mod config;
mod file_helpers;
mod list;
mod new;
mod project;
mod write;

#[derive(Parser)]
#[command(
    name = "pm",
    version = "1.0",
    about = "A CLI for managing project folders"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project
    New {
        name: String,
        #[arg(short, long, value_delimiter = ',')]
        categories: Option<Vec<String>>,
    },
    /// List all active projects
    List,
    /// Open the notes for a project
    Note { name: String },
    /// Archive a project or check for stale projects
    Archive {
        name: Option<String>,
        #[arg(long)]
        check_stale: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::get().context("Failed to get config")?;

    match cli.command {
        Commands::New { name, categories } => config
            .new_command(&name, categories)
            .with_context(|| format!("Failed to create project '{name}'")),
        Commands::List => config.list_command(),
        // Commands::Note { name } => {},
        // Commands::Archive { name, check_stale } => {},
        _ => todo!(),
    }
}

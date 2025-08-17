use anyhow::{Context, Result};
use cli::{Commands, parse};
use config::Config;

mod cli;
mod config;
mod file_helpers;
mod list;
mod new;
mod project;
mod write;

fn main() -> Result<()> {
    let cli = parse();
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

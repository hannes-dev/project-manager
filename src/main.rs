use anyhow::{Context, Result};
use cli::{Commands, parse};
use commands::list;
use config::Config;
use workspace::Workspace;

mod cli;
mod commands;
mod config;
mod file_helpers;
mod project;
mod workspace;
mod write;

#[cfg(test)]
mod test_utils;

fn main() -> Result<()> {
    let cli = parse();
    let config = Config::get().context("Failed to get config")?;
    let workspace = Workspace::new(&config);

    match cli.command {
        Commands::New { name, categories } => config
            .new_command(&name, categories)
            .with_context(|| format!("Failed to create project '{name}'")),
        Commands::List => list::handle(&workspace),
        Commands::Init { path } => {
            println!("{}", path.canonicalize()?.display());
            Ok(())
        }
        // Commands::Note { name } => {},
        // Commands::Archive { name, check_stale } => {},
        _ => todo!(),
    }
}

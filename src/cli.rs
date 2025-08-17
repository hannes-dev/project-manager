use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "pm",
    version = "1.0",
    about = "A CLI for managing project folders"
)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new project
    New {
        name: String,
        #[arg(short, long, value_delimiter = ',')]
        categories: Option<Vec<String>>,
    },
    Init {
        #[arg(default_value_os = ".")]
        path: PathBuf,
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

pub fn parse() -> CLI {
    CLI::parse()
}

use std::path::PathBuf;

use crate::workspace::Workspace;
use anyhow::{Ok, Result};

pub fn handle(workspace: &Workspace) -> Result<()> {
    for name in workspace.project_names()? {
        println!("{name}");
    }

    Ok(())
}

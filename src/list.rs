use crate::{config::Config, project::Project};
use anyhow::{Ok, Result};

impl Config {
    pub fn list_command(&self) -> Result<()> {
        let non_projects = std::fs::read_dir(&self.projects_dir)?
            .filter_map(Result::ok)
            // only allow directories
            .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_dir()))
            // only print projects, and cound non-projects
            .fold(0, |count, entry| {
                if Project::is_project(&entry.path()) {
                    println!("{}", entry.file_name().to_string_lossy());
                    count
                } else {
                    count + 1
                }
            });
        if non_projects > 0 {
            eprintln!("\n... and {non_projects} non-project folders");
        }

        Ok(())
    }
}

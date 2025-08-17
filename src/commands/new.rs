use anyhow::{Result, bail};

use crate::{config::Config, file_helpers::create_dir, project::Project, write::Write};

impl Config {
    pub fn new_command(&self, name: &String, categories: Option<Vec<String>>) -> Result<()> {
        if name.contains(' ') {
            bail!("Name cannot contain a space");
        }

        let proj_dir = self.project_dir.join(&name);
        if proj_dir.exists() {
            let error_msg = if Project::exists(&proj_dir) {
                format!("Project already exists at {proj_dir:?}",)
            } else {
                format!("Folder already exists at {proj_dir:?}")
            };
            bail!(error_msg);
        }

        create_dir(&proj_dir)?;
        let project = match categories {
            Some(categories) => Project::with_categories(categories),
            None => Project::new(),
        };
        proj_dir.write(project)?;

        Ok(())
    }
}

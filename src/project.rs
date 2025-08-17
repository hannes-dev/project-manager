use std::path::{Path, PathBuf};

use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::write::Write;

const METADATA_FILE: &str = ".project_metadata.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub created_at: DateTime<Utc>,
    pub last_modified_at: DateTime<Utc>,
    pub categories: Vec<String>,
}

impl Project {
    pub fn new() -> Self {
        Self {
            created_at: Utc::now(),
            last_modified_at: Utc::now(),
            categories: Vec::new(),
        }
    }

    pub fn with_categories(categories: Vec<String>) -> Self {
        let mut project = Self::new();
        project.categories = categories;
        project
    }

    /// Checks if path has a valid metadata file
    pub fn is_project(path: &Path) -> bool {
        path.join(METADATA_FILE).exists()
    }
}

impl Write<Project> for PathBuf {
    fn write(&self, metadata: Project) -> anyhow::Result<()> {
        let toml_string = toml::to_string(&metadata)?;
        let metadata_path = self.join(METADATA_FILE);
        std::fs::write(&metadata_path, toml_string)
            .with_context(|| format!("Failed to write metadata file to '{metadata_path:?}'"))?;
        Ok(())
    }
}

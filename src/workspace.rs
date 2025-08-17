use std::path::PathBuf;

use anyhow::Result;

use crate::{config::Config, project::Project};

pub struct Workspace<'a> {
    config: &'a Config,
}

impl<'a> Workspace<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    /// all names of projects in the project folder
    pub fn project_names(&self) -> Result<impl Iterator<Item = String>> {
        Ok(self.project_paths()?.filter_map(|p| {
            p.file_name()
                .map(|name| name.to_string_lossy().into_owned())
        }))
    }

    /// all paths of projects in the project folder
    pub fn project_paths(&self) -> Result<impl Iterator<Item = PathBuf>> {
        Ok(self.all_paths()?.filter(|p| Project::exists(&p)))
    }

    /// all paths of directories in the project folder
    pub fn all_paths(&self) -> Result<impl Iterator<Item = PathBuf>> {
        Ok(std::fs::read_dir(&self.config.project_dir)?
            .filter_map(Result::ok)
            // only allow directories
            .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_dir()))
            .map(|x| x.path()))
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{PROJECT_NAMES, TempConfig};

    #[test]
    fn listing_projects_skip_other() {
        let temp_config = TempConfig::new();
        temp_config.populate_projects();
        temp_config.populate_other_folders();

        let project_names: Vec<String> = temp_config.workspace().project_names().unwrap().collect();

        assert_eq!(project_names.len(), PROJECT_NAMES.len());

        for name in project_names {
            assert!(PROJECT_NAMES.contains(&name.as_str()))
        }
    }
}

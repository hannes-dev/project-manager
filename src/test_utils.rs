use std::fs::create_dir;

use assert_fs::{
    TempDir,
    prelude::{FileTouch, PathChild},
};

use crate::{config::Config, project::METADATA_FILE, workspace::Workspace};

pub struct TempConfig {
    pub _temp_dir: TempDir,
    pub config: Config,
}

pub static PROJECT_NAMES: &[&str] = &["cheese", "cars", "oathbringer"];
pub static OTHER_FOLDER_NAMES: &[&str] = &["goat", "bike", "river"];

impl TempConfig {
    pub fn new() -> Self {
        let temp_dir = assert_fs::TempDir::new().unwrap();

        // make sure the directories exist
        let projects_dir = temp_dir.path().join("projects").to_owned();
        create_dir(&projects_dir).unwrap();
        let archive_dir = temp_dir.path().join("archive").to_owned();
        create_dir(&archive_dir).unwrap();

        let config = Config {
            project_dir: projects_dir,
            archive_dir,
        };

        Self {
            _temp_dir: temp_dir,
            config,
        }
    }

    pub fn workspace(&self) -> Workspace {
        Workspace::new(&self.config)
    }

    /// makes a couple project folders and metadata files
    pub fn populate_projects(&self) {
        for name in PROJECT_NAMES {
            self._temp_dir
                .child("projects")
                .child(name)
                .child(METADATA_FILE)
                .touch()
                .unwrap();
        }
    }

    /// makes a couple project folders and metadata files
    pub fn populate_other_folders(&self) {
        for name in OTHER_FOLDER_NAMES {
            self._temp_dir
                .child("projects")
                .child(name)
                .touch()
                .unwrap();
        }
    }
}

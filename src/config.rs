use crate::file_helpers::create_dir_all;
use anyhow::{Context, Result};
use directories::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub projects_dir: PathBuf,
    pub archive_dir: PathBuf,
}

impl Config {
    pub fn get() -> Result<Self> {
        let dirs = ProjectDirs::from("", "", "pm").unwrap();
        let config_dir = dirs.config_dir();
        let config_path = config_dir.join("config.toml");

        let home_dir = UserDirs::new().unwrap().home_dir().to_owned();

        if !config_path.exists() {
            create_dir_all(config_dir)?;
        }

        if !config_path.exists() {
            let default_config = Config {
                projects_dir: home_dir.join("projects"),
                archive_dir: home_dir.join("archive"),
            };

            let toml_string =
                toml::to_string(&default_config).context("Could not serialize default config")?;
            std::fs::write(&config_path, toml_string).with_context(|| {
                format!(
                    "Could not write default config to '{}'",
                    config_path.display()
                )
            })?;
        }

        let toml_string = std::fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&toml_string)?;

        if !config.projects_dir.exists() {
            create_dir_all(&config.projects_dir)?;
        }
        if !config.archive_dir.exists() {
            create_dir_all(&config.archive_dir)?;
        }

        Ok(config)
    }
}

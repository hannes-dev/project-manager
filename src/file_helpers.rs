use std::{fmt::Debug, fs, path::Path};

use anyhow::{Context, Result};

pub fn create_dir<P>(path: P) -> Result<()>
where
    P: AsRef<Path> + Debug,
{
    fs::create_dir(&path).with_context(|| format!("Failed to create dir at {path:?}"))
}

pub fn create_dir_all<P>(path: P) -> Result<()>
where
    P: AsRef<Path> + Debug,
{
    fs::create_dir_all(&path).with_context(|| format!("Failed to create all dirs at {path:?}"))
}

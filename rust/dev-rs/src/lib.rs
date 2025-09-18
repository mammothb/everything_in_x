use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

pub fn get_config_path() -> Result<PathBuf> {
    let mut path = env::home_dir().context("cannot find home directory")?;

    path.push(".config");
    fs::create_dir_all(&path)?;

    path.push("dev.toml");

    Ok(path)
}

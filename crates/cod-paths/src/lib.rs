use anyhow::Context;
use std::path::PathBuf;

pub fn token_directory() -> anyhow::Result<PathBuf> {
    dirs::data_dir()
        .context("Couldn't find data directory for saving the token.")
        .map(|data_dir| data_dir.join(".cod"))
}

pub fn token_path() -> anyhow::Result<PathBuf> {
    token_directory().map(|token_dir| token_dir.join("TOKEN"))
}

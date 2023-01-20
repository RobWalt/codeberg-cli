use std::path::PathBuf;

use anyhow::Context;

pub fn get_repo_name() -> anyhow::Result<String> {
    std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
        .context("Couldn't detect git repository")
        .map(|output| String::from_utf8_lossy(output.stdout.as_slice()).to_string())
        .map(PathBuf::from)
        .and_then(|git_path| {
            git_path
                .file_name()
                .map(|name| name.to_owned())
                .context("Couldn't find repo name in git path")
        })
        .and_then(|repo_name| {
            repo_name
                .to_str()
                .map(|repo_name| repo_name.to_owned())
                .context("Couldn't convert repo name into string")
        })
        .map(|repo_name| repo_name.trim().to_owned())
}

#[test]
fn cod_repo_name() -> anyhow::Result<()> {
    let repo_name = get_repo_name()?;
    assert_eq!(repo_name.as_str(), "codeberg-cli");
    Ok(())
}

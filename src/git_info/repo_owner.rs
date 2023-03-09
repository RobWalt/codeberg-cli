use std::path::PathBuf;

use anyhow::Context;

#[derive(Debug, Clone)]
pub struct RepoAndOwner {
    pub repo: String,
    pub owner: String,
}

pub fn get_repo_owner() -> anyhow::Result<RepoAndOwner> {
    std::process::Command::new("git")
        .arg("remote")
        .arg("-v")
        .output()
        .context("Couldn't detect git repository")
        .map(|output| String::from_utf8_lossy(output.stdout.as_slice()).to_string())
        .and_then(|output| {
            output
                .lines()
                .next()
                .and_then(|repo| repo.split_whitespace().nth(1).map(|repo| repo.to_owned()))
                .context("Couldn't detect git repository")
        })
        .map(PathBuf::from)
        .and_then(|mut git_path| {
            let repo = git_path
                .file_name()
                .map(|name| name.to_owned())
                .context("Couldn't find repo name in git path")?;
            git_path.pop();
            let owner = git_path
                .file_name()
                .map(|name| name.to_owned())
                .context("Couldn't find repo owner in git path")?;
            Ok((repo, owner))
        })
        .and_then(|(repo, owner)| {
            let repo = repo
                .to_str()
                .map(|repo_name| repo_name.trim().to_owned())
                .context("Couldn't convert repo name into string")?;
            // owner needs additional cleanup since original url can be something like
            // git@codeberg.org:UserName/RepoName
            let owner = owner
                .to_str()
                .map(|repo_name| {
                    repo_name
                        .chars()
                        .rev()
                        .take_while(char::is_ascii_alphanumeric)
                        .collect::<String>()
                        .chars()
                        .rev()
                        .collect::<String>()
                })
                .context("Couldn't convert repo name into string")?;
            Ok(RepoAndOwner { repo, owner })
        })
}

#[test]
fn berg_repo_name() -> anyhow::Result<()> {
    let repo_and_owner = get_repo_owner()?;
    assert_eq!(repo_and_owner.repo.as_str(), "codeberg-cli");
    assert_eq!(repo_and_owner.owner.as_str(), "RobWalt");
    Ok(())
}

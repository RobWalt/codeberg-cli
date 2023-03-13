use std::ffi::OsStr;
use std::path::PathBuf;

use anyhow::Context;

#[derive(Debug, Clone)]
pub struct RepoAndOwner {
    pub repo: String,
    pub owner: String,
}

pub fn get_repo_owner() -> anyhow::Result<RepoAndOwner> {
    // execute git command to get remotes
    std::process::Command::new("git")
        .arg("remote")
        .arg("-v")
        .output()
        .context("Couldn't detect git repository")
        // convert cli output to string
        .map(|output| String::from_utf8_lossy(output.stdout.as_slice()).to_string())
        // try to take first line
        .and_then(|output| {
            output
                .lines()
                .next()
                .map(str::to_owned)
                .context("Couldn't detect git repository")
        })
        // try to take second word in line
        .and_then(|repo| {
            repo.split_whitespace()
                .nth(1)
                .map(str::to_owned)
                .context("Couldn't detect git repository")
        })
        // convert to path type
        .map(PathBuf::from)
        .and_then(|mut git_path| {
            // split away last part of path
            let repo = git_path
                .file_name()
                .map(OsStr::to_owned)
                .context("Couldn't find repo name in git path")?;
            git_path.pop();
            // split away second last part of path
            let owner = git_path
                .file_name()
                .map(OsStr::to_owned)
                .context("Couldn't find repo owner in git path")?;
            Ok((repo, owner))
        })
        .and_then(|(repo, owner)| {
            // cleanup + conversion to string
            let repo = repo
                .to_str()
                .map(|repo_name| {
                    // repo name could be something like myCoolRepo.git
                    repo_name
                        .chars()
                        .take_while(alphanumeric_or_dash)
                        .collect::<String>()
                })
                .context("Couldn't convert repo name into string")?;
            // owner needs additional cleanup since original url can be something like
            // git@codeberg.org:UserName/RepoName
            let owner = owner
                .to_str()
                .map(|owner_name| {
                    owner_name
                        .chars()
                        .rev()
                        .take_while(alphanumeric_or_dash)
                        .collect::<String>()
                        .chars()
                        .rev()
                        .collect::<String>()
                })
                .context("Couldn't convert repo name into string")?;
            Ok(RepoAndOwner { repo, owner })
        })
}

fn alphanumeric_or_dash(char: &char) -> bool {
    char.is_alphanumeric() || '-'.eq(char)
}

#[test]
fn berg_repo_name() -> anyhow::Result<()> {
    let repo_and_owner = get_repo_owner()?;
    assert_eq!(repo_and_owner.repo.as_str(), "codeberg-cli");
    assert_eq!(repo_and_owner.owner.as_str(), "RobWalt");
    Ok(())
}

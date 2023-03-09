use clap::Parser;

/// Clone a repository
#[derive(Parser, Debug)]
pub struct RepoCloneArgs {
    /// Repository to be cloned
    #[arg(value_name = "OWNER/REPO")]
    pub owner_and_repo: String,
}

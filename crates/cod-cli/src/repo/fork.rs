use clap::Parser;

/// Fork a repository
#[derive(Parser, Debug)]
pub struct RepoForkArgs {
    /// Repository to be forked
    #[arg(value_name = "OWNER/REPO")]
    pub owner_and_repo: String,
}

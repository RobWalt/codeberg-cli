use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Fork a repository")]
pub struct RepoForkArgs {
    #[arg(id = "OWNER/REPO", help = "The repository given in OWNER/REPO format")]
    pub owner_and_repo: String,
}

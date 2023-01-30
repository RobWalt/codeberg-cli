use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Clone a repository")]
pub struct RepoCloneArgs {
    #[arg(id = "OWNER/REPO", help = "The repository given in OWNER/REPO format")]
    pub owner_and_repo: String,
}

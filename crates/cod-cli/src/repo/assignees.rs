use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "List available assignee candidates")]
pub struct RepoAssigneesArgs {}

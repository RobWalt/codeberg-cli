pub mod create;
pub mod list;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
#[command(author, version, about = "Issue commands", long_about = None)]
pub enum PullRequestArgs {
    List(list::ListPullRequestArgs),
    Create(create::CreatePullRequestArgs),
}

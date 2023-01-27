pub mod create;
pub mod edit;
pub mod list;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
#[command(author, version, about = "Pull request commands", long_about = None)]
pub enum PullRequestArgs {
    List(list::ListPullRequestArgs),
    Create(create::CreatePullRequestArgs),
    Edit(edit::EditPullRequestArgs),
}

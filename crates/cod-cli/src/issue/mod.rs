pub mod create;
pub mod edit;
pub mod list;
pub mod view;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
#[command(author, version, about = "Issue commands", long_about = None)]
pub enum IssueArgs {
    List(list::ListIssueArgs),
    Create(create::CreateIssueArgs),
    View(view::ViewIssueArgs),
    Edit(edit::EditIssueArgs),
}

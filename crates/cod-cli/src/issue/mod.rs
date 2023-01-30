pub mod comment;
pub mod create;
pub mod edit;
pub mod list;
pub mod view;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
#[command(about = "Issue subcommands")]
pub enum IssueArgs {
    List(list::ListIssueArgs),
    Create(create::CreateIssueArgs),
    Comment(comment::CommentIssueArgs),
    View(view::ViewIssueArgs),
    Edit(edit::EditIssueArgs),
}

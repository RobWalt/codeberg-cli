pub mod comment;
pub mod create;
pub mod edit;
pub mod list;
pub mod view;

use clap::Subcommand;

/// Issue subcommands
#[derive(Subcommand, Debug)]
pub enum IssueArgs {
    List(list::ListIssueArgs),
    Create(create::CreateIssueArgs),
    Comment(comment::CommentIssueArgs),
    View(view::ViewIssueArgs),
    Edit(edit::EditIssueArgs),
}

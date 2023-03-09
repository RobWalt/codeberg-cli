pub mod comment;
pub mod create;
pub mod edit;
pub mod list;
pub mod view;

use clap::Subcommand;

/// Pull request subcommands
#[derive(Subcommand, Debug)]
pub enum PullRequestArgs {
    List(list::ListPullRequestArgs),
    Create(create::CreatePullRequestArgs),
    Edit(edit::EditPullRequestArgs),
    View(view::ViewPullRequestsArgs),
    Comment(comment::CommentPullRequestArgs),
}

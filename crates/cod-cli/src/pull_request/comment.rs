use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Add a comment to a selected pull request")]
pub struct CommentPullRequestArgs {}

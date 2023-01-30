use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Add a comment to a selected issue")]
pub struct CommentIssueArgs {}

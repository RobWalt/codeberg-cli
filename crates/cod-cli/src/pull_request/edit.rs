use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Edit properties of an existing pull request")]
pub struct EditPullRequestArgs {}

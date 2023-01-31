use clap::Parser;

/// Create an issue
#[derive(Parser, Debug)]
pub struct CreateIssueArgs {
    /// Title or summary
    #[arg(short, long)]
    pub title: Option<String>,

    /// List of label ids
    #[arg(short, long)]
    pub labels: Option<Vec<String>>,

    /// Main description of issue
    #[arg(id = "description", short, long)]
    pub body: Option<String>,

    /// List of assignee names
    #[arg(short, long)]
    pub assignees: Option<Vec<String>>,
}

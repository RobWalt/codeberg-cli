use clap::Parser;

/// Create an issue
#[derive(Parser, Debug)]
pub struct CreateIssueArgs {
    /// Title or summary
    #[arg(short, long)]
    pub title: Option<String>,

    /// Comma-delimited list of label ids
    #[arg(short, long, value_name = "LABEL,...", value_delimiter = ',')]
    pub labels: Option<Vec<String>>,

    /// Main description of issue
    #[arg(id = "description", short, long)]
    pub body: Option<String>,

    /// Comma-delimited list of assignee names
    #[arg(short, long, value_name = "ASSIGNEE,...", value_delimiter = ',')]
    pub assignees: Option<Vec<String>>,
}

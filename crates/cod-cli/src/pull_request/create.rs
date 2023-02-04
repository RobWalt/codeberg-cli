use clap::Parser;

/// Create a pull request
#[derive(Parser, Debug)]
pub struct CreatePullRequestArgs {
    /// Comma-delimited list of assignee names
    #[arg(short, long, value_name = "ASSIGNEE,...", value_delimiter = ',')]
    pub assignees: Option<Vec<String>>,

    /// Target branch for the pull request
    #[arg(short, long)]
    pub target_branch: Option<String>,

    /// Main description of the pull request
    #[arg(id = "description", short, long)]
    pub body: Option<String>,

    /// Source branch of the pull request
    #[arg(short, long)]
    pub source_branch: Option<String>,

    /// Comma-delimited list of labels
    #[arg(short, long, value_name = "LABEL,...", value_delimiter = ',')]
    pub labels: Option<Vec<String>>,

    /// Title or summary
    #[arg(short, long)]
    pub title: Option<String>,
}

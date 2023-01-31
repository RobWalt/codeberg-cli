use clap::Parser;

/// Create a pull request
#[derive(Parser, Debug)]
pub struct CreatePullRequestArgs {
    /// List of assignee names
    #[arg(short, long)]
    pub assignees: Option<Vec<String>>,

    /// Target branch for the pull request
    #[arg(short, long)]
    pub base: Option<String>,

    /// Main description of the pull request
    #[arg(id = "description", short, long)]
    pub body: Option<String>,

    /// Source branch of the pull request
    #[arg(short, long)]
    pub head: Option<String>,

    /// List of labels
    #[arg(short, long)]
    pub labels: Option<Vec<String>>,

    /// Title or summary
    #[arg(short, long)]
    pub title: Option<String>,
}

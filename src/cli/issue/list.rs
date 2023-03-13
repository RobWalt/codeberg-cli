use clap::Parser;

/// List all issues in the current repository
#[derive(Parser, Debug)]
pub struct ListIssueArgs {
    /// Number of issues to be displayed
    #[arg(short, long, value_name = "N", default_value_t = 5)]
    pub count: usize,
}

use clap::Parser;

/// Create an issue
#[derive(Parser, Debug)]
pub struct CreateMilestoneArgs {
    /// Title or summary
    #[arg(short, long)]
    pub title: Option<String>,

    /// Main description of milestone
    #[arg(id = "description", short, long)]
    pub body: Option<String>,
}

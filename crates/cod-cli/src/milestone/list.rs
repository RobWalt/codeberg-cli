use clap::Parser;

/// List all milestones in the current repository
#[derive(Parser, Debug)]
pub struct ListMilestonesArgs {
    /// Number of milestones to be displayed
    #[arg(short, long, value_name = "N", default_value_t = 5)]
    pub count: usize,
}

pub mod list;

use clap::Subcommand;

/// Milestone subcommands
#[derive(Subcommand, Debug)]
pub enum MilestoneArgs {
    List(list::ListMilestonesArgs),
}

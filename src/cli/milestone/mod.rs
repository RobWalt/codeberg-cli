pub mod create;
pub mod edit;
pub mod list;
pub mod view;

use clap::Subcommand;

/// Milestone subcommands
#[derive(Subcommand, Debug)]
pub enum MilestoneArgs {
    List(list::ListMilestonesArgs),
    View(view::ViewMilestonesArgs),
    Create(create::CreateMilestoneArgs),
    Edit(edit::EditMilestoneArgs),
}

pub mod create;
pub mod delete;
pub mod list;

use clap::Subcommand;

/// Label subcommands
#[derive(Subcommand, Debug)]
pub enum LabelArgs {
    List(list::ListLabelsArgs),
    Create(create::CreateLabelArgs),
    Delete(delete::DeleteLabelArgs),
}

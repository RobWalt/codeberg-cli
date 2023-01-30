pub mod create;
pub mod delete;
pub mod list;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
#[command(about = "Label subcommands")]
pub enum LabelArgs {
    List(list::ListLabelsArgs),
    Create(create::CreateLabelArgs),
    Delete(delete::DeleteLabelArgs),
}

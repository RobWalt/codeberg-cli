pub mod list;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
#[command(author, version, about = "Label commands", long_about = None)]
pub enum LabelArgs {
    List(list::ListLabelsArgs),
}

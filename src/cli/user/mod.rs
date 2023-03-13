pub mod info;

use clap::Subcommand;

/// User subcommands
#[derive(Subcommand, Debug)]
pub enum UserArgs {
    Info(info::UserInfoArgs),
}

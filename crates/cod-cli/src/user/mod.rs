pub mod info;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
#[command(about = "User subcommands")]
pub enum UserArgs {
    Info(info::UserInfoArgs),
}

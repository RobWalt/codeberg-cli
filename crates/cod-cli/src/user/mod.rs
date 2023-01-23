pub mod info;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
#[command(author, version, about = "User information", long_about = None)]
pub enum UserArgs {
    Info(info::UserInfoArgs),
}

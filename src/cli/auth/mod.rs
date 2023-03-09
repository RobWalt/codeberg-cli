pub mod login;
pub mod logout;

use clap::Subcommand;

/// Authentication subcommands
#[derive(Subcommand, Debug)]
pub enum AuthArgs {
    Login(login::LoginArgs),
    Logout(logout::LogoutArgs),
}

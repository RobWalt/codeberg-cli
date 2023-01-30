pub mod login;
pub mod logout;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
#[command(about = "Authentification subcommands")]
pub enum AuthArgs {
    Login(login::LoginArgs),
    Logout(logout::LogoutArgs),
}

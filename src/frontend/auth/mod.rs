mod login;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
#[command(author, version, about = "Choose authentification subcommands", long_about = None)]
pub enum AuthArgs {
    Login(login::LoginArgs),
}
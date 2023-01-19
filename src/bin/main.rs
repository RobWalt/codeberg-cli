use clap::{Command, FromArgMatches, Subcommand};
use codeberg_cli_backend::{login, AuthArgs, MainArgs};

#[tokio::main]
async fn main() {
    let cli = Command::new("cod");
    let mut cli = MainArgs::augment_subcommands(cli);
    let args = MainArgs::from_arg_matches(&cli.clone().get_matches()).unwrap();

    let cmd_result = match args {
        MainArgs::Auth(AuthArgs::Login(_login_args)) => login().await,
    };

    if let Err(error) = cmd_result {
        println!("Error: {error:?}\n\n");
        if let Err(e) = cli.print_long_help() {
            println!("Couldn't print help message because: {e:?}");
        }
    }
}

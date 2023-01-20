use anyhow::Context;
use clap::{Command, FromArgMatches, Subcommand};
use codeberg_cli_backend::{info, login, logout, AuthArgs, MainArgs, Token, UserArgs};

#[tokio::main]
async fn main() {
    let cli = Command::new("cod");
    let mut cli = MainArgs::augment_subcommands(cli);
    let args = MainArgs::from_arg_matches(&cli.clone().get_matches()).unwrap();

    let cmd_result = if let MainArgs::Auth(AuthArgs::Login(login_args)) = args {
        login(login_args).await
    } else {
        dispatch_args(args).await
    };

    if let Err(e) = cmd_result.or_else(|app_error| {
        cli.print_long_help()
            .context(format!("{app_error:?}"))
            .and_then(|_| anyhow::bail!("{app_error:?}"))
    }) {
        println!("Error: {e:?}");
    }
}

async fn dispatch_args(args: MainArgs) -> anyhow::Result<()> {
    let token = Token::read_from_data_dir()
        .context("Couldn't find login data. Please use `cod auth login` to authenticate first.")?;
    match args {
        MainArgs::Auth(AuthArgs::Logout(logout_args)) => logout(logout_args),
        MainArgs::Auth(AuthArgs::Login(_)) => unreachable!("was already handled"),
        MainArgs::User(UserArgs::Info(info_args)) => info(info_args, token).await,
    }
}

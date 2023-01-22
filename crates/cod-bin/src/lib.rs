use anyhow::Context;
use clap::{Command, FromArgMatches, Subcommand};
use cod_actions::auth::login::login;
use cod_actions::auth::logout::logout;
use cod_actions::issue::list::list_issues;
use cod_actions::label::create::create_label;
use cod_actions::label::list::list_labels;
use cod_actions::pull_request::list::list_pulls;
use cod_actions::user::info::info;
use cod_cli::auth::AuthArgs;
use cod_cli::issue::IssueArgs;
use cod_cli::label::LabelArgs;
use cod_cli::pull_request::PullRequestArgs;
use cod_cli::user::UserArgs;
use cod_cli::MainArgs;
use cod_types::token::Token;

pub async fn cod_main() {
    tracing_subscriber::fmt::init();

    let cli = Command::new("cod");

    let app_result = run(cli).await;

    if let Err(e) = app_result.or_else(|(mut cli, app_error)| {
        cli.print_long_help()
            .context(format!("{app_error:?}"))
            .and_then(|_| anyhow::bail!("{app_error:?}"))
    }) {
        println!("Error: {e:?}");
    }
}

async fn run(cli: Command) -> Result<(), (Command, anyhow::Error)> {
    let cli = MainArgs::augment_subcommands(cli);
    let args = MainArgs::from_arg_matches(&cli.clone().get_matches())
        .map_err(|error| (cli.clone(), anyhow::Error::from(error)))?;

    let res = if let MainArgs::Auth(AuthArgs::Login(login_args)) = args {
        login(login_args).await
    } else {
        dispatch_args(args).await
    };
    res.map_err(|error| (cli, error))
}

async fn dispatch_args(args: MainArgs) -> anyhow::Result<()> {
    let token = Token::read_from_data_dir()
        .context("Couldn't find login data. Please use `cod auth login` to authenticate first.")?;
    match args {
        MainArgs::Auth(AuthArgs::Logout(logout_args)) => logout(logout_args),
        MainArgs::Auth(AuthArgs::Login(_)) => unreachable!("was already handled"),
        MainArgs::User(UserArgs::Info(info_args)) => info(info_args, token).await,
        MainArgs::Issue(IssueArgs::List(list_args)) => list_issues(list_args, token).await,
        MainArgs::Pull(PullRequestArgs::List(list_args)) => list_pulls(list_args, token).await,
        MainArgs::Label(LabelArgs::List(list_args)) => list_labels(list_args, token).await,
        MainArgs::Label(LabelArgs::Create(create_args)) => create_label(create_args, token).await,
    }
}

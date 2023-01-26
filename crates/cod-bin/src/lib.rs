use anyhow::Context;
use clap::{Command, FromArgMatches, Subcommand};
use cod_actions::auth::login::login_user;
use cod_actions::auth::logout::logout_user;
use cod_actions::issue::comment::comment_issue;
use cod_actions::issue::create::create_issue;
use cod_actions::issue::edit::edit_issue;
use cod_actions::issue::list::list_issue;
use cod_actions::issue::view::view_issue;
use cod_actions::label::create::create_label;
use cod_actions::label::delete::delete_label;
use cod_actions::label::list::list_label;
use cod_actions::pull_request::list::list_pull;
use cod_actions::repo::assignees::assignees_repo;
use cod_actions::repo::clone::clone_repo;
use cod_actions::repo::create::create_repo;
use cod_actions::repo::fork::fork_repo;
use cod_actions::repo::info::info_repo;
use cod_actions::user::info::info_user;
use cod_cli::auth::AuthArgs;
use cod_cli::issue::IssueArgs;
use cod_cli::label::LabelArgs;
use cod_cli::pull_request::PullRequestArgs;
use cod_cli::repo::RepoArgs;
use cod_cli::user::UserArgs;
use cod_cli::MainArgs;
use cod_client::CodebergClient;
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
        login_user(login_args).await
    } else {
        dispatch_args(args).await
    };
    res.map_err(|error| (cli, error))
}

async fn dispatch_args(main_args: MainArgs) -> anyhow::Result<()> {
    tracing::info!("args: {main_args:#?}");
    let token = Token::read_from_data_dir()
        .context("Couldn't find login data. Please use `cod auth login` to authenticate first.")?;
    let client = CodebergClient::new(&token)?;
    match main_args {
        MainArgs::Auth(AuthArgs::Logout(args)) => logout_user(args),
        MainArgs::Auth(AuthArgs::Login(_)) => unreachable!("was already handled"),
        MainArgs::User(UserArgs::Info(args)) => info_user(args, &client).await,
        MainArgs::Repo(RepoArgs::Info(args)) => info_repo(args, &client).await,
        MainArgs::Repo(RepoArgs::Assignees(args)) => assignees_repo(args, &client).await,
        MainArgs::Repo(RepoArgs::Create(args)) => create_repo(args, &client).await,
        MainArgs::Repo(RepoArgs::Clone(args)) => clone_repo(args, &client).await,
        MainArgs::Repo(RepoArgs::Fork(args)) => fork_repo(args, &client).await,
        MainArgs::Issue(IssueArgs::List(args)) => list_issue(args, &client).await,
        MainArgs::Issue(IssueArgs::Create(args)) => create_issue(args, &client).await,
        MainArgs::Issue(IssueArgs::View(args)) => view_issue(args, &client).await,
        MainArgs::Issue(IssueArgs::Edit(args)) => edit_issue(args, &client).await,
        MainArgs::Issue(IssueArgs::Comment(args)) => comment_issue(args, &client).await,
        MainArgs::Pull(PullRequestArgs::List(args)) => list_pull(args, &client).await,
        MainArgs::Label(LabelArgs::List(args)) => list_label(args, &client).await,
        MainArgs::Label(LabelArgs::Create(args)) => create_label(args, &client).await,
        MainArgs::Label(LabelArgs::Delete(args)) => delete_label(args, &client).await,
    }
}

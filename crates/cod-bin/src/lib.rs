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
use cod_actions::label::edit::edit_label;
use cod_actions::label::list::list_label;
use cod_actions::pull_request::comment::comment_pull;
use cod_actions::pull_request::create::create_pull;
use cod_actions::pull_request::edit::edit_pull;
use cod_actions::pull_request::list::list_pull;
use cod_actions::pull_request::view::view_pull;
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
use cod_cli::{generate_completion, MainArgs};
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

    let res = match args {
        MainArgs::Auth(AuthArgs::Login(login_args)) => login_user(login_args).await,
        MainArgs::Completion { shell } => {
            generate_completion(shell, "cod");
            Ok(())
        }
        _ => dispatch_args(args).await,
    };
    res.map_err(|error| (cli, error))
}

async fn dispatch_args(main_args: MainArgs) -> anyhow::Result<()> {
    tracing::debug!("args: {main_args:#?}");
    let token = Token::read_from_data_dir()
        .context("Couldn't find login data. Please use `cod auth login` to authenticate first.")?;
    let client = CodebergClient::new(&token)?;
    use MainArgs::*;
    match main_args {
        Auth(AuthArgs::Logout(args)) => logout_user(args),
        Auth(AuthArgs::Login(_)) | Completion { .. } => unreachable!("was already handled"),
        User(UserArgs::Info(args)) => info_user(args, &client).await,
        Repo(RepoArgs::Info(args)) => info_repo(args, &client).await,
        Repo(RepoArgs::Assignees(args)) => assignees_repo(args, &client).await,
        Repo(RepoArgs::Create(args)) => create_repo(args, &client).await,
        Repo(RepoArgs::Clone(args)) => clone_repo(args, &client).await,
        Repo(RepoArgs::Fork(args)) => fork_repo(args, &client).await,
        Issue(IssueArgs::List(args)) => list_issue(args, &client).await,
        Issue(IssueArgs::Create(args)) => create_issue(args, &client).await,
        Issue(IssueArgs::View(args)) => view_issue(args, &client).await,
        Issue(IssueArgs::Edit(args)) => edit_issue(args, &client).await,
        Issue(IssueArgs::Comment(args)) => comment_issue(args, &client).await,
        Pull(PullRequestArgs::List(args)) => list_pull(args, &client).await,
        Pull(PullRequestArgs::Create(args)) => create_pull(args, &client).await,
        Pull(PullRequestArgs::Edit(args)) => edit_pull(args, &client).await,
        Pull(PullRequestArgs::View(args)) => view_pull(args, &client).await,
        Pull(PullRequestArgs::Comment(args)) => comment_pull(args, &client).await,
        Label(LabelArgs::List(args)) => list_label(args, &client).await,
        Label(LabelArgs::Create(args)) => create_label(args, &client).await,
        Label(LabelArgs::Delete(args)) => delete_label(args, &client).await,
        Label(LabelArgs::Edit(args)) => edit_label(args, &client).await,
    }
}

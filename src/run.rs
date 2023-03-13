use crate::actions::auth::login::login_user;
use crate::actions::auth::logout::logout_user;
use crate::actions::issue::comment::comment_issue;
use crate::actions::issue::create::create_issue;
use crate::actions::issue::edit::edit_issue;
use crate::actions::issue::list::list_issue;
use crate::actions::issue::view::view_issue;
use crate::actions::label::create::create_label;
use crate::actions::label::delete::delete_label;
use crate::actions::label::edit::edit_label;
use crate::actions::label::list::list_label;
use crate::actions::milestone::create::create_milestone;
use crate::actions::milestone::edit::edit_milestone;
use crate::actions::milestone::list::list_milestone;
use crate::actions::milestone::view::view_milestone;
use crate::actions::notification::list::list_notifications;
use crate::actions::notification::view::view_notifications;
use crate::actions::pull_request::comment::comment_pull;
use crate::actions::pull_request::create::create_pull;
use crate::actions::pull_request::edit::edit_pull;
use crate::actions::pull_request::list::list_pull;
use crate::actions::pull_request::view::view_pull;
use crate::actions::repo::assignees::assignees_repo;
use crate::actions::repo::clone::clone_repo;
use crate::actions::repo::create::create_repo;
use crate::actions::repo::fork::fork_repo;
use crate::actions::repo::info::info_repo;
use crate::actions::user::info::info_user;
use crate::cli::auth::AuthArgs;
use crate::cli::issue::IssueArgs;
use crate::cli::label::LabelArgs;
use crate::cli::milestone::MilestoneArgs;
use crate::cli::notification::NotificationArgs;
use crate::cli::pull_request::PullRequestArgs;
use crate::cli::repo::RepoArgs;
use crate::cli::user::UserArgs;
use crate::cli::{generate_completion, MainArgs};
use crate::client::BergClient;
use crate::types::token::Token;
use anyhow::Context;
use clap::{Command, FromArgMatches, Subcommand};

pub async fn berg_main() {
    tracing_subscriber::fmt::init();

    let cli = Command::new("berg");

    let app_result = run(cli).await;

    if let Err(e) = app_result.or_else(|(mut cli, app_error)| {
        cli.print_help()
            .context(format!("{app_error:?}"))
            .and_then(|_| anyhow::bail!("{app_error:?}"))
    }) {
        let error_msg = format!("Error: {e:?}");
        let border = "=".repeat(error_msg.len().min(80));
        println!("\n");
        println!("{border}\n");
        println!("{error_msg}");
        println!("\n{border}");
    }
}

async fn run(cli: Command) -> Result<(), (Command, anyhow::Error)> {
    let cli = MainArgs::augment_subcommands(cli);
    let args = MainArgs::from_arg_matches(&cli.clone().get_matches())
        .map_err(|error| (cli.clone(), anyhow::Error::from(error)))?;

    let res = match args {
        MainArgs::Auth(AuthArgs::Login(login_args)) => login_user(login_args).await,
        MainArgs::Completion { shell } => {
            generate_completion(shell, "berg");
            Ok(())
        }
        _ => dispatch_args(args).await,
    };
    res.map_err(|error| (cli, error))
}

async fn dispatch_args(main_args: MainArgs) -> anyhow::Result<()> {
    tracing::debug!("args: {main_args:#?}");
    let token = Token::read_from_data_dir()
        .context("Couldn't find login data. Please use `berg auth login` to authenticate first.")?;
    let client = BergClient::new(&token)?;
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
        Milestone(MilestoneArgs::List(args)) => list_milestone(args, &client).await,
        Milestone(MilestoneArgs::View(args)) => view_milestone(args, &client).await,
        Milestone(MilestoneArgs::Create(args)) => create_milestone(args, &client).await,
        Milestone(MilestoneArgs::Edit(args)) => edit_milestone(args, &client).await,
        Notification(NotificationArgs::List(args)) => list_notifications(args, &client).await,
        Notification(NotificationArgs::View(args)) => view_notifications(args, &client).await,
    }
}
